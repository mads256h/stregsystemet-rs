use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

use crate::{dso::product::ProductId, responses::result_json::HttpStatusCode};

#[derive(Deserialize, Serialize)]
pub struct ActiveProductsResponse {
    pub products: Vec<ActiveProduct>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActiveProduct {
    pub id: ProductId,
    pub name: String,
    pub price: String,
    pub aliases: Vec<String>,
}

// TODO: Should not be here
#[serde_as]
#[derive(Error, Debug, Serialize)]
#[error(transparent)]
pub struct DatabaseError(
    #[serde_as(as = "DisplayFromStr")]
    #[from]
    sqlx::Error,
);

impl HttpStatusCode for DatabaseError {
    fn status_code(&self) -> axum::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
