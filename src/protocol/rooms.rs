use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use thiserror::Error;

use crate::responses::result_json::HttpStatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomIdRequest {
    pub room_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfoResponse {
    pub room_id: i32,
    pub name: String,
}

#[serde_as]
#[derive(Error, Debug, Serialize)]
pub enum RoomInfoError {
    #[error("database error: {0}")]
    DbError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        sqlx::Error,
    ),

    #[error("invalid room id: {0}")]
    InvalidRoom(i32),
}

impl HttpStatusCode for RoomInfoError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            RoomInfoError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RoomInfoError::InvalidRoom(_) => StatusCode::BAD_REQUEST,
        }
    }
}
