use std::num::{NonZeroU32, ParseIntError};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

pub fn parse_quickbuy_query(quickbuy_query: &str) -> Result<QuickBuyType, QuickBuyParseError> {
    let trimmed = quickbuy_query.trim();

    let split = trimmed
        .split(' ')
        .filter(|&s| !s.is_empty())
        .collect::<Vec<&str>>();
    match split.len() {
        0 => Err(QuickBuyParseError::EmptyQuery),
        1 => Ok(QuickBuyType::Username {
            username: split[0].into(),
        }),
        _ => Ok(parse_multi_buy_expression(&split)?),
    }
}

fn parse_multi_buy_expression(split: &[&str]) -> Result<QuickBuyType, MultiBuyParseError> {
    let username = split[0];
    let rest = &split[1..];

    let products = rest
        .iter()
        .map(|product| parse_multi_buy_product(product))
        .collect::<Result<Vec<MultiBuyProduct>, MultiBuyParseError>>()?;

    Ok(QuickBuyType::MultiBuy {
        username: username.into(),
        products,
    })
}

fn parse_multi_buy_product(product_query: &str) -> Result<MultiBuyProduct, MultiBuyParseError> {
    let split = product_query.split(':').collect::<Vec<&str>>();

    match split.len() {
        1 => Ok(MultiBuyProduct {
            product_name: parse_product_name(split[0])?.into(),
            amount: NonZeroU32::new(1).unwrap(),
        }),
        2 => Ok(MultiBuyProduct {
            product_name: parse_product_name(split[0])?.into(),
            amount: split[1].parse::<NonZeroU32>()?,
        }),
        _ => Err(MultiBuyParseError::Syntax),
    }
}

fn parse_product_name(product_name: &str) -> Result<&str, MultiBuyParseError> {
    match product_name.len() {
        0 => Err(MultiBuyParseError::EmptyProduct),
        _ => Ok(product_name),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuickBuyType {
    Username {
        username: String,
    },
    MultiBuy {
        username: String,
        products: Vec<MultiBuyProduct>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiBuyProduct {
    pub product_name: String,
    pub amount: NonZeroU32,
}

#[derive(Error, Debug, Serialize)]
pub enum QuickBuyParseError {
    #[error("query is empty")]
    EmptyQuery,

    #[error(transparent)]
    MultiBuy(#[from] MultiBuyParseError),
}

#[serde_as]
#[derive(Error, Debug, Serialize)]
pub enum MultiBuyParseError {
    #[error("syntax error")]
    Syntax,

    #[error("empty product name")]
    EmptyProduct,

    #[error("invalid amount")]
    InvalidAmount(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        ParseIntError,
    ),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query() {
        parse_and_expect_empty_query("");
    }

    #[test]
    fn whitespace_query() {
        parse_and_expect_empty_query(" ");
    }

    #[test]
    fn username_query() {
        let expected_username: String = "test_user".into();
        let result = parse_quickbuy_query(&expected_username).unwrap();

        assert!(
            matches!(result, QuickBuyType::Username { username } if username == expected_username)
        );
    }

    #[test]
    fn multibuy_query() {
        let result = parse_quickbuy_query("test_user kaffe øl:2 21:3").unwrap();

        match result {
            QuickBuyType::MultiBuy { username, products } => {
                assert_eq!(username, "test_user");

                let kaffe_product = &products[0];
                let øl_product = &products[1];
                let id_product = &products[2];

                assert_eq!(kaffe_product.product_name, "kaffe");
                assert_eq!(kaffe_product.amount, NonZeroU32::new(1).unwrap());

                assert_eq!(øl_product.product_name, "øl");
                assert_eq!(øl_product.amount, NonZeroU32::new(2).unwrap());

                assert_eq!(id_product.product_name, "21");
                assert_eq!(id_product.amount, NonZeroU32::new(3).unwrap());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn empty_product_multibuy_query() {
        let error = parse_quickbuy_query("test_user :2").unwrap_err();

        assert!(matches!(
            error,
            QuickBuyParseError::MultiBuy(MultiBuyParseError::EmptyProduct)
        ))
    }

    #[test]
    fn missing_amount_multibuy_query() {
        parse_and_expect_invalid_amount_multibuy_query("test_user p:");
    }

    #[test]
    fn zero_amount_multibuy_query() {
        parse_and_expect_invalid_amount_multibuy_query("test_user p:0");
    }

    #[test]
    fn negative_amount_multibuy_query() {
        parse_and_expect_invalid_amount_multibuy_query("test_user p:-1");
    }

    #[test]
    fn invalid_amount_multibuy_query() {
        parse_and_expect_invalid_amount_multibuy_query("test_user p:x");
    }

    fn parse_and_expect_empty_query(query: &str) {
        let result = parse_quickbuy_query(query);
        assert!(result.is_err());
        let error = result.unwrap_err();

        assert!(matches!(error, QuickBuyParseError::EmptyQuery));
    }

    fn parse_and_expect_invalid_amount_multibuy_query(query: &str) {
        let error = parse_quickbuy_query(query).unwrap_err();

        assert!(matches!(
            error,
            QuickBuyParseError::MultiBuy(MultiBuyParseError::InvalidAmount(_))
        ))
    }
}
