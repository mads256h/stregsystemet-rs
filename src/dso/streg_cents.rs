use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use maud::Render;
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize, Serialize, Debug, sqlx::Type, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
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

impl Add for StregCents {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        i64::checked_add(self.0, rhs.0).map(StregCents)
    }
}

impl Mul<u32> for StregCents {
    type Output = Option<Self>;

    fn mul(self, rhs: u32) -> Self::Output {
        i64::checked_mul(self.0, rhs as i64).map(StregCents)
    }
}

pub fn stregcents_sum<I>(mut iterator: I) -> Option<StregCents>
where
    I: Iterator<Item = Option<StregCents>>,
{
    iterator.try_fold(StregCents(0), |a, b| match b {
        Some(v) => a + v,
        None => None,
    })
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
