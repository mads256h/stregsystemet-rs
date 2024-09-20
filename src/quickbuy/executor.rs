use serde::Serialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use sqlx::{PgPool, Postgres, Transaction};
use thiserror::Error;

use crate::dso::{
    product::ProductId,
    streg_cents::{stregcents_sum, StregCents},
    user::UserId,
};

use super::parser::MultiBuyProduct;

pub async fn execute_multi_buy_query(
    username: &str,
    multi_buy_products: &[MultiBuyProduct],
    pool: &PgPool,
) -> Result<(), MultiBuyExecutorError> {
    let mut transaction = pool.begin().await?;

    let user_id = get_user_id_by_name(username, &mut transaction)
        .await?
        .ok_or_else(|| MultiBuyExecutorError::InvalidUsername(username.to_string()))?;

    let user_money = get_user_money_by_id(user_id, &mut transaction).await?;

    let multi_buy_products_with_ids =
        get_multi_buy_products_with_ids(multi_buy_products, &mut transaction).await?;

    let product_price_sum =
        get_product_price_sum(&multi_buy_products_with_ids, &mut transaction).await?;

    if user_money < product_price_sum {
        return Err(MultiBuyExecutorError::InsufficientFunds(product_price_sum));
    }

    purchase_products(user_id, &multi_buy_products_with_ids, &mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}

async fn get_user_id_by_name(
    username: &str,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<Option<UserId>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT id as "id: UserId"
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(&mut **transaction)
    .await
}

async fn get_user_money_by_id(
    user_id: UserId,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<StregCents, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT ((SELECT COALESCE(SUM(amount), 0) FROM deposits WHERE user_id = $1) - (SELECT COALESCE(SUM(price), 0) FROM sales WHERE user_id = $1))::bigint as "money!: StregCents"
        "#,
        user_id as UserId)
        .fetch_one(&mut **transaction)
        .await
}

async fn get_product_price_sum(
    mutli_buy_products_with_ids: &[MultiBuyProductProductIdPair<'_>],
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<StregCents, MultiBuyExecutorError> {
    // TODO: How do you do this without having to do a ugly loop?
    let mut products_with_prices = vec![];

    for multi_buy_product_with_id in mutli_buy_products_with_ids {
        products_with_prices.push((
            multi_buy_product_with_id,
            get_product_price(multi_buy_product_with_id, transaction).await?,
        ))
    }

    stregcents_sum(
        products_with_prices
            .into_iter()
            .map(|(p, price)| price * p.multi_buy_product.amount),
    )
    .ok_or(MultiBuyExecutorError::StregCentsOverflow)
}

async fn get_product_price(
    multi_buy_product_with_id: &MultiBuyProductProductIdPair<'_>,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<StregCents, MultiBuyExecutorError> {
    get_product_price_by_id(multi_buy_product_with_id.product_id, transaction)
        .await?
        .ok_or_else(|| {
            MultiBuyExecutorError::InvalidProduct(
                multi_buy_product_with_id
                    .multi_buy_product
                    .product_name
                    .clone(),
            )
        })
}

async fn get_product_price_by_id(
    product_id: ProductId,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<Option<StregCents>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT price as "price: StregCents"
        FROM products
        WHERE id = $1 AND active=true AND (deactivate_after_timestamp IS NULL OR deactivate_after_timestamp > now())
        "#,
        product_id as ProductId
    )
    .fetch_optional(&mut **transaction)
    .await
}

async fn get_multi_buy_products_with_ids<'a>(
    multi_buy_products: &'a [MultiBuyProduct],
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<Vec<MultiBuyProductProductIdPair<'a>>, MultiBuyExecutorError> {
    let mut products_with_ids: Vec<MultiBuyProductProductIdPair<'a>> = vec![];

    for multi_buy_product in multi_buy_products {
        products_with_ids.push(MultiBuyProductProductIdPair {
            multi_buy_product,
            product_id: get_product_id_by_id_or_alias(multi_buy_product, transaction).await?,
        });
    }

    Ok(products_with_ids)
}

async fn get_product_id_by_id_or_alias(
    product: &MultiBuyProduct,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<ProductId, MultiBuyExecutorError> {
    let product_id_or_error = product.product_name.parse::<ProductId>();

    match product_id_or_error {
        Ok(product_id) => Ok(product_id),
        Err(_) => Ok(get_product_id_by_alias(&product.product_name, transaction).await?),
    }
}

async fn get_product_id_by_alias(
    product_name: &str,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<ProductId, MultiBuyExecutorError> {
    sqlx::query_scalar!(
        r#"
        SELECT product_id as "id: ProductId"
        FROM product_aliases
        WHERE alias_name = $1
        "#,
        product_name
    )
    .fetch_optional(&mut **transaction)
    .await?
    .ok_or_else(|| MultiBuyExecutorError::InvalidProduct(product_name.to_string()))
}

async fn purchase_products(
    user_id: UserId,
    multi_buy_products_with_ids: &[MultiBuyProductProductIdPair<'_>],
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<(), sqlx::Error> {
    for multi_buy_product_with_id in multi_buy_products_with_ids {
        for _ in 0..(multi_buy_product_with_id.multi_buy_product.amount.into()) {
            purchase_product(user_id, multi_buy_product_with_id.product_id, transaction).await?;
        }
    }

    Ok(())
}

async fn purchase_product(
    user_id: UserId,
    product_id: ProductId,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<(), sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO sales(price, product_id, user_id)
        VALUES ((SELECT price FROM products WHERE id = $1), $1, $2)
        "#,
        product_id as ProductId,
        user_id as UserId
    )
    .execute(&mut **transaction)
    .await?
    .rows_affected();

    assert_eq!(rows_affected, 1);

    Ok(())
}

#[serde_as]
#[derive(Error, Debug, Serialize)]
pub enum MultiBuyExecutorError {
    #[error("database error: {0}")]
    DbError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sqlx::Error,
    ),

    #[error("invalid username: {0}")]
    InvalidUsername(String),

    #[error("invalid product: {0}")]
    InvalidProduct(String),

    #[error("insufficient funds: {0}")]
    InsufficientFunds(StregCents),

    #[error("stregcents overflow / underflow")]
    StregCentsOverflow,
}

struct MultiBuyProductProductIdPair<'a> {
    multi_buy_product: &'a MultiBuyProduct,
    product_id: ProductId,
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/deposits.sql"
    ))]
    async fn multi_buy_buy_product_by_id(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "1".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(matches!(result, Ok(())));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql",
        "../../fixtures/deposits.sql"
    ))]
    async fn multi_buy_buy_product_by_alias(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "enabled".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(matches!(result, Ok(())));
    }

    #[sqlx::test]
    async fn multi_buy_invalid_username(pool: PgPool) {
        let result = execute_multi_buy_query("i_do_not_exist", &[], &pool).await;

        assert!(
            matches!(result, Err(MultiBuyExecutorError::InvalidUsername(username)) if username == "i_do_not_exist")
        );
    }

    #[sqlx::test(fixtures("../../fixtures/users.sql"))]
    async fn multi_buy_invalid_product_unknown(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "1337".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(
            matches!(result, Err(MultiBuyExecutorError::InvalidProduct(product_name)) if product_name == "1337")
        );
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql"
    ))]
    async fn multi_buy_invalid_product_inactive(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "inactive".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(
            matches!(result, Err(MultiBuyExecutorError::InvalidProduct(product_name)) if product_name == "inactive")
        );
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql"
    ))]
    async fn multi_buy_invalid_product_deactivated_by_timestamp(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "inactive_timestamp".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(
            matches!(result, Err(MultiBuyExecutorError::InvalidProduct(product_name)) if product_name == "inactive_timestamp")
        );
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql"
    ))]
    async fn multi_buy_insufficient_funds_no_money(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "enabled".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(matches!(
            result,
            Err(MultiBuyExecutorError::InsufficientFunds(_))
        ));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql",
        "../../fixtures/deposits.sql"
    ))]
    async fn multi_buy_insufficient_funds_too_expensive(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "expensive".to_string(),
            amount: NonZeroU32::new(1).unwrap(),
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(matches!(
            result,
            Err(MultiBuyExecutorError::InsufficientFunds(_))
        ));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/users.sql",
        "../../fixtures/products.sql",
        "../../fixtures/product_aliases.sql"
    ))]
    async fn multi_buy_streg_cents_overflow(pool: PgPool) {
        let product = MultiBuyProduct {
            product_name: "overflow".to_string(),
            amount: NonZeroU32::MAX,
        };
        let result = execute_multi_buy_query("test_user", &[product], &pool).await;

        assert!(matches!(
            result,
            Err(MultiBuyExecutorError::StregCentsOverflow)
        ));
    }
}
