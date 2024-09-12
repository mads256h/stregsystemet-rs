use std::error::Error;

use axum::{debug_handler, extract::State, http::StatusCode, routing::get, Router};
use dotenv::dotenv;
use maud::{html, Markup};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

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
        .route("/api/user/all", get(get_users));

    router.with_state(pool)
}

#[debug_handler]
async fn create_user(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query!(
        r#"
    INSERT INTO users(username, email, notes) VALUES ('test_user2', 'user2@test.com', 'Test user');
    "#
    )
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

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
