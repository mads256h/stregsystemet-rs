use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    dso::{product::ProductId, streg_cents::StregCents},
    quickbuy::{executor::MultiBuyExecutorError, parser::QuickBuyParseError},
    responses::result_json::HttpStatusCode,
};

#[derive(Deserialize, Serialize)]
pub struct BuyRequest {
    pub quickbuy: String,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum BuyResponse {
    Username {
        username: String,
    },
    MultiBuy {
        username: String,
        bought_products: Vec<BoughtProduct>,
        product_price_sum: StregCents,
        new_user_balance: StregCents,
    },
}

#[derive(Deserialize, Serialize)]
pub struct BoughtProduct {
    pub product_id: ProductId,
    pub amount: i32,
}

#[derive(Error, Debug, Serialize)]
pub enum BuyError {
    #[error("parser error {0}")]
    Parser(#[from] QuickBuyParseError),

    #[error("executor error {0}")]
    Executor(#[from] MultiBuyExecutorError),
}

impl HttpStatusCode for BuyError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            BuyError::Parser(_) => StatusCode::BAD_REQUEST,
            BuyError::Executor(multi_buy_executor_error) => match multi_buy_executor_error {
                MultiBuyExecutorError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::BAD_REQUEST,
            },
        }
    }
}
