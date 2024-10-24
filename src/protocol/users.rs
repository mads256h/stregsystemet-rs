use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use thiserror::Error;

use crate::responses::result_json::HttpStatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct UsernameRequest {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub balance: String,
}

#[serde_as]
#[derive(Error, Debug, Serialize)]
pub enum UserInfoError {
    #[error("database error: {0}")]
    DbError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sqlx::Error,
    ),

    #[error("invalid username: {0}")]
    InvalidUsername(String),
}

impl HttpStatusCode for UserInfoError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            UserInfoError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserInfoError::InvalidUsername(_) => StatusCode::BAD_REQUEST,
        }
    }
}
