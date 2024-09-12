use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BuyRequest {
    pub quickbuy: String,
}
