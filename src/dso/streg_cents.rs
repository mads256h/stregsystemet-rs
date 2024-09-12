use std::fmt::Display;

use maud::Render;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct StregCents(i64);

impl Display for StregCents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dollars = self.0 / 100;
        let cents = self.0 % 100;

        write!(f, "{}.{:02}", dollars, cents)
    }
}

impl Render for StregCents {
    fn render_to(&self, buffer: &mut String) {
        self.to_string().render_to(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let streg_cents = StregCents(725);
        let streg_cents_zero_cents = StregCents(800);
        let streg_cents_zero = StregCents(0);

        assert_eq!(streg_cents.to_string(), "7.25");
        assert_eq!(streg_cents_zero_cents.to_string(), "8.00");
        assert_eq!(streg_cents_zero.to_string(), "0.00");
    }
}
