mod dso;
mod protocol;
mod quickbuy;
mod responses;

use std::{error::Error, num::NonZeroUsize, sync::Arc, time::Duration};

use askama_axum::{Response, Template};
use axum::{
    body::{Body, Bytes},
    debug_handler,
    error_handling::HandleErrorLayer,
    extract::{Request, State},
    http::{header, response::Parts, HeaderName, HeaderValue, Method, StatusCode, Uri},
    middleware::{self, Next},
    routing::{get, post},
    BoxError, Json, Router,
};
use dotenv::dotenv;
use dso::{product::ProductId, streg_cents::StregCents};

use http_body_util::BodyExt;
use lru::LruCache;
use protocol::{
    buy_request::{BuyError, BuyRequest, BuyResponse},
    products::active_products_response::DatabaseError,
};
use protocol::{
    news::ActiveNewsResponse,
    products::active_products_response::{ActiveProduct, ActiveProductsResponse},
};
use quickbuy::{
    executor::{execute_multi_buy_query, username_exists},
    parser::{parse_quickbuy_query, QuickBuyType},
};
use responses::result_json::ResultJson;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, sync::Mutex};
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

type IdempotencyCache = Arc<Mutex<LruCache<(Method, Uri, String), (Parts, Bytes)>>>;
#[derive(Clone)]
struct MyState {
    pool: PgPool,
    idempotency_cache: IdempotencyCache,
}

fn app(pool: PgPool) -> Router {
    let state = MyState {
        pool,
        idempotency_cache: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(256).unwrap()))),
    };

    let router = Router::new()
        .route("/", get(index_handler))
        .route("/api/products/active", get(get_active_products))
        .route("/api/purchase/quickbuy", post(quickbuy_handler))
        .route("/api/news/active", get(get_active_news_handler))
        .nest_service(
            "/static",
            ServiceBuilder::new()
                .layer(middleware::from_fn(guess_mime_type_from_extension))
                .service(ServeDir::new("static")),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            idempotency_key_handller,
        ))
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

    router.with_state(state)
}

// TODO: Clean this shitshow of a function up
// There is unwrap and clone everywhere
// We can't remove everything but god damn.
// TODO: Should we use the request body as a cache key as well?
async fn idempotency_key_handller(
    State(state): State<MyState>,
    request: Request,
    next: Next,
) -> Response {
    static IDEMPOTENCY_HEADER_KEY: HeaderName = HeaderName::from_static("x-idempotency-key");

    let idempotency_key = request.headers().get(&IDEMPOTENCY_HEADER_KEY);

    if request.method() != Method::POST && request.method() != Method::PATCH
        || idempotency_key.is_none()
    {
        // Already an idempotent method or no idempotency key provided: Not handling
        let response = next.run(request).await;
        return response;
    }

    let idempotency_key = idempotency_key.unwrap().to_str().unwrap().to_owned();
    let cache_key = (
        request.method().clone(),
        request.uri().clone(),
        idempotency_key,
    );
    let cached_response = {
        let mut idempotency_cache = state.idempotency_cache.lock().await;
        idempotency_cache.get(&cache_key).cloned()
    };

    match cached_response {
        Some(cached_response) => {
            let (parts, bytes) = cached_response;
            let body: Body = bytes.clone().into();

            Response::from_parts(parts.clone(), body)
        }
        None => {
            let response = next.run(request).await;
            let (parts, body) = response.into_parts();
            let bytes = body.collect().await.unwrap().to_bytes();

            let return_body: Body = bytes.clone().into();
            {
                let mut idempotency_cache = state.idempotency_cache.lock().await;
                idempotency_cache.put(cache_key, (parts.clone(), bytes));
            }

            Response::from_parts(parts, return_body)
        }
    }
}

async fn guess_mime_type_from_extension(request: Request, next: Next) -> Response {
    let uri = request.uri().path();
    let guess = mime_guess::from_path(uri);

    let mut response = next.run(request).await;

    if let Some(mime) = guess.first() {
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime.essence_str()).expect("invalid header value"),
        );
    };

    response
}

#[debug_handler]
async fn get_active_products(
    State(state): State<MyState>,
) -> ResultJson<ActiveProductsResponse, DatabaseError> {
    async {
        let products = sqlx::query!(
            r#"
            SELECT products.id as "id: ProductId", products.name, products.price as "price: StregCents", STRING_AGG(product_aliases.alias_name, ' ') as aliases
            -- ' ' is an illegal character in aliases so it can be used as a separator
            FROM products
            LEFT JOIN product_aliases
            ON products.id=product_aliases.product_id
            WHERE products.active=true AND (products.deactivate_after_timestamp IS NULL OR products.deactivate_after_timestamp > now())
            GROUP BY products.id, products.name, products.price
            ORDER BY products.id
            "#)
            .fetch_all(&state.pool)
            .await?;

        let active_products = products
            .into_iter()
            .map(|p| ActiveProduct {
                id: p.id,
                name: p.name,
                price: p.price.to_string(),
                aliases: p.aliases.map(|a| a.split(' ').map(|a| a.to_string()).collect()).unwrap_or_default(),
            })
            .collect();

        Ok(ActiveProductsResponse {
            products: active_products,
        })
    }.await.into()
}

#[debug_handler]
async fn quickbuy_handler(
    State(state): State<MyState>,
    Json(buy_request): Json<BuyRequest>,
) -> ResultJson<BuyResponse, BuyError> {
    async {
        let quickbuy_type = parse_quickbuy_query(&buy_request.quickbuy)?;
        match quickbuy_type {
            QuickBuyType::Username { username } => {
                username_exists(&username, &state.pool).await?;
                Ok(BuyResponse::Username { username })
            }
            QuickBuyType::MultiBuy { username, products } => {
                let (bought_products, product_price_sum, new_user_balance) =
                    execute_multi_buy_query(&username, &products, &state.pool).await?;
                Ok(BuyResponse::MultiBuy {
                    bought_products,
                    product_price_sum,
                    new_user_balance,
                })
            }
        }
    }
    .await
    .into()
}

#[debug_handler]
async fn get_active_news_handler(
    State(state): State<MyState>,
) -> ResultJson<ActiveNewsResponse, DatabaseError> {
    async {
        let news = sqlx::query_scalar!(
            r#"
            SELECT content
            FROM news
            WHERE active=true AND (deactivate_after_timestamp IS NULL OR deactivate_after_timestamp > now())
            ORDER BY id
            "#)
            .fetch_all(&state.pool)
            .await?;

        Ok(ActiveNewsResponse {
            news,
        })
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
