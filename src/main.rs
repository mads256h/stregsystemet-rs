mod components;
mod dso;
mod protocol;
mod quickbuy;

use std::error::Error;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use components::{
    product_menu_components::{clickable_products_table_component, products_table_component},
    quickbuy_component::quickbuy_component,
};
use dotenv::dotenv;
use dso::{
    product::{Product, ProductId},
    streg_cents::StregCents,
};
use maud::{html, Markup, DOCTYPE};
use protocol::buy_request::BuyRequest;
use quickbuy::{
    executor::execute_multi_buy_query,
    parser::{parse_quickbuy_query, QuickBuyType},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_connection_string = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_string)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app(pool)).await?;

    Ok(())
}

fn app(pool: PgPool) -> Router {
    let router = Router::new()
        .route("/api/user/create", get(create_user))
        .route("/api/user/all", get(get_users))
        .route("/", get(index))
        .route("/buy/", post(buy_handler))
        .nest_service("/static", ServeDir::new("static"));

    router.with_state(pool)
}

#[debug_handler]
async fn create_user(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query!(
        r#"
        INSERT INTO users(username, email, notes) VALUES ('test_user2', 'user2@test.com', 'Test user');
        "#)
        .execute(&pool)
        .await
        .map_err(internal_error)?;

    Ok("Ok".into())
}

#[debug_handler]
async fn get_users(State(pool): State<PgPool>) -> Result<Markup, (StatusCode, String)> {
    let users = sqlx::query!(
        r#"
        SELECT id, username, email, notes, join_timestamp
        FROM users;
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    Ok(html! {
        ol {
            @for user in users {
                li { (user.username) }
            }
        }
    })
}

#[debug_handler]
async fn index(State(pool): State<PgPool>) -> Result<Markup, (StatusCode, String)> {
    let products = get_active_products(&pool).await.map_err(internal_error)?;

    Ok(html! {
        (DOCTYPE)
        html {
            head {
                link rel="stylesheet" href="/static/main.css";
            }
            body {
                (quickbuy_component())
                (products_table_component(&products))
            }
        }
    })
}

async fn buy_handler(
    State(pool): State<PgPool>,
    Form(buy_request): Form<BuyRequest>,
) -> Result<Markup, (StatusCode, String)> {
    let quickbuy_type = parse_quickbuy_query(&buy_request.quickbuy).map_err(internal_error)?;

    // Should not be here.
    let products = get_active_products(&pool).await.map_err(internal_error)?;

    Ok(match quickbuy_type {
        QuickBuyType::Username { username } => html! {
            (DOCTYPE)
            html {
                head {
                    link rel="stylesheet" href="/static/main.css";
                }
                body {
                    (clickable_products_table_component(&products, &username))
                }
            }
        },
        QuickBuyType::MultiBuy { username, products } => {
            execute_multi_buy_query(&username, &products, &pool)
                .await
                .map_err(internal_error)?;

            html! {
                "Username: " (username) "Products: "
                ol {
                    @for product in products {
                        li { (product.product_name) " Amount: " (product.amount) }
                    }
                }
            }
        }
    })
}

async fn get_active_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as!(
        Product,
        r#"
        SELECT id as "id: ProductId", name, price as "price: StregCents"
        FROM products
        WHERE active=true AND (deactivate_after_timestamp IS NULL OR deactivate_after_timestamp > now())
        "#)
        .fetch_all(pool)
        .await
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
