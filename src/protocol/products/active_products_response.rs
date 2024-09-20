use serde::{Deserialize, Serialize};

use crate::dso::product::ProductId;

#[derive(Deserialize, Serialize)]
pub struct ActiveProductsResponse {
    pub products: Vec<ActiveProduct>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActiveProduct {
    pub id: ProductId,
    pub name: String,
    pub price: String,
}
