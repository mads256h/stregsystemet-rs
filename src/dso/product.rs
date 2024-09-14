use std::{num::ParseIntError, str::FromStr};

use maud::Render;
use serde::{Deserialize, Serialize};

use super::streg_cents::StregCents;

#[derive(Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq, Clone, Copy)]
#[sqlx(transparent)]
pub struct ProductId(i32);

pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub price: StregCents,
}

impl Render for ProductId {
    fn render_to(&self, buffer: &mut String) {
        self.0.render_to(buffer)
    }
}

impl FromStr for ProductId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProductId(i32::from_str(s)?))
    }
}
