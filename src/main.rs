mod dso;
mod protocol;
mod quickbuy;

use std::error::Error;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use dso::{
    product::{Product, ProductId},
    streg_cents::StregCents,
};

use protocol::buy_request::BuyRequest;
use quickbuy::parser::{parse_quickbuy_query, QuickBuyType};
use protocol::products::active_products_response::ActiveProductsReponse;
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
        .route("/api/products/active", get(get_active_products2))
        .route("/api/purchase/quickbuy", post(quickbuy_handler))
        .nest_service("/", ServeDir::new("static"));

    router.with_state(pool)
}

#[debug_handler]
async fn get_active_products2(
    State(pool): State<PgPool>,
) -> Result<Json<ActiveProductsReponse>, (StatusCode, String)> {
    let active_products = get_active_products(&pool).await.map_err(internal_error)?;
    Ok(Json(ActiveProductsReponse {
        products: active_products,
    }))
}

#[debug_handler]
async fn quickbuy_handler(
    Json(buy_request): Json<BuyRequest>,
) -> Result<Json<QuickBuyType>, (StatusCode, String)> {
    let result = parse_quickbuy_query(&buy_request.quickbuy).map_err(internal_error)?;
    Ok(Json(result))
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
