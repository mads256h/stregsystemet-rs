use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::responses::result_json::HttpStatusCode;

#[derive(Serialize, Deserialize)]
pub struct ActiveNewsResponse {
    pub news: Vec<String>,
}

#[serde_as]
#[derive(Error, Debug, Serialize)]
pub enum ActiveNewsError {
    #[error("database error: {0}")]
    DbError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sqlx::Error,
    ),
}

impl HttpStatusCode for ActiveNewsError {
    fn status_code(&self) -> axum::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
