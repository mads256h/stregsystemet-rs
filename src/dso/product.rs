use maud::Render;
use serde::{Deserialize, Serialize};

use super::streg_cents::StregCents;

#[derive(Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq)]
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
