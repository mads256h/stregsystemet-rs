use std::{num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize};

use super::streg_cents::StregCents;

#[derive(Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq, Clone, Copy, Hash)]
#[sqlx(transparent)]
pub struct ProductId(i32);

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub price: StregCents,
}

impl FromStr for ProductId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProductId(i32::from_str(s)?))
    }
}
