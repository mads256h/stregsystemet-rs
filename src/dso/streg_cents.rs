use std::{
    fmt::Display,
    num::NonZeroU32,
    ops::{Add, Mul, Sub},
};

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

impl Add for StregCents {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        i64::checked_add(self.0, rhs.0).map(StregCents)
    }
}

impl Sub for StregCents {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        i64::checked_sub(self.0, rhs.0).map(StregCents)
    }
}

impl Mul<NonZeroU32> for StregCents {
    type Output = Option<Self>;

    fn mul(self, rhs: NonZeroU32) -> Self::Output {
        let r: u32 = rhs.into();
        i64::checked_mul(self.0, r as i64).map(StregCents)
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
