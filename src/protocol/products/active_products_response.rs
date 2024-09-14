use serde::{Deserialize, Serialize};

use crate::dso::product::Product;

#[derive(Deserialize, Serialize)]
pub struct ActiveProductsReponse {
    pub products: Vec<Product>,
}
