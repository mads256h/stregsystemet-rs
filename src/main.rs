mod dso;
mod protocol;
mod quickbuy;
mod responses;

use std::{error::Error, time::Duration};

use askama_axum::Template;
use axum::{
    debug_handler,
    error_handling::HandleErrorLayer,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    BoxError, Json, Router,
};
use dotenv::dotenv;
use dso::{
    product::{Product, ProductId},
    streg_cents::StregCents,
};

use protocol::products::active_products_response::{ActiveProduct, ActiveProductsResponse};
use protocol::{
    buy_request::{BuyError, BuyRequest, BuyResponse},
    products::active_products_response::DatabaseError,
};
use quickbuy::{
    executor::execute_multi_buy_query,
    parser::{parse_quickbuy_query, QuickBuyType},
};
use responses::result_json::ResultJson;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "stregsystemet=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

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
        .route("/", get(index_handler))
        .route("/api/products/active", get(get_active_products))
        .route("/api/purchase/quickbuy", post(quickbuy_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                // this middleware goes above `TimeoutLayer` because it will receive
                // errors returned by `TimeoutLayer`
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(30))),
        )
        .fallback(not_found_handler);

    router.with_state(pool)
}

#[debug_handler]
async fn get_active_products(
    State(pool): State<PgPool>,
) -> ResultJson<ActiveProductsResponse, DatabaseError> {
    async {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id as "id: ProductId", name, price as "price: StregCents"
            FROM products
            WHERE active=true AND (deactivate_after_timestamp IS NULL OR deactivate_after_timestamp > now())
            "#)
            .fetch_all(&pool)
            .await?;

        let active_products = products
            .into_iter()
            .map(|p| ActiveProduct {
                id: p.id,
                name: p.name,
                price: p.price.to_string(),
            })
            .collect();

        Ok(ActiveProductsResponse {
            products: active_products,
        })
    }.await.into()
}

#[debug_handler]
async fn quickbuy_handler(
    State(pool): State<PgPool>,
    Json(buy_request): Json<BuyRequest>,
) -> ResultJson<BuyResponse, BuyError> {
    async {
        let quickbuy_type = parse_quickbuy_query(&buy_request.quickbuy)?;
        match quickbuy_type {
            QuickBuyType::Username { username } => Ok(BuyResponse::Username { username }),
            QuickBuyType::MultiBuy { username, products } => {
                execute_multi_buy_query(&username, &products, &pool).await?;
                Ok(BuyResponse::MultiBuy)
            }
        }
    }
    .await
    .into()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[debug_handler]
async fn index_handler() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {}

#[debug_handler]
async fn not_found_handler() -> (StatusCode, NotFoundTemplate) {
    (StatusCode::NOT_FOUND, NotFoundTemplate {})
}
