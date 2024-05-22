use serde::{Deserialize, Serialize};
use std::{
    num::ParseFloatError,
    ops::{Add, Div, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LimitedFloat {
    pub wrapped: i64,
}

pub type Concentration = LimitedFloat;
pub type Volume = LimitedFloat;

impl LimitedFloat {
    pub fn new(wrapped: i64) -> Self {
        Self { wrapped }
    }

    pub fn valid(&self) -> bool {
        self.wrapped >= 0 && self.wrapped as f64 <= 1.0f64 / Self::EPSILON
    }

    pub const EPSILON: f64 = 0.0001;
}

impl Sub for Concentration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_val = self.wrapped;
        let rhs_val = rhs.wrapped;
        let val = self_val - rhs_val;

        Self { wrapped: val }
    }
}

impl Add for Concentration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_val = self.wrapped;
        let rhs_val = rhs.wrapped;
        let val = self_val + rhs_val;

        Self { wrapped: val }
    }
}

impl Div for LimitedFloat {
    type Output = LimitedFloat;

    fn div(self, rhs: Self) -> Self::Output {
        let self_val = self.wrapped;
        let rhs_val = rhs.wrapped;
        let val = self_val / rhs_val;

        Self { wrapped: val }
    }
}

impl From<Concentration> for f64 {
    fn from(value: Concentration) -> Self {
        let epsilon_corrected = value.wrapped as f64 * Concentration::EPSILON;
        let scale = 1f64 / Self::EPSILON;
        (epsilon_corrected * scale).trunc() / scale
    }
}

impl From<f64> for Concentration {
    fn from(value: f64) -> Self {
        Self {
            wrapped: (value / Self::EPSILON).round() as i64,
        }
    }
}

impl FromStr for Concentration {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f64_val = s.parse::<f64>()?;
        let epsilon_corrected = (f64_val / Self::EPSILON).round() as i64;

        Ok(Self {
            wrapped: epsilon_corrected,
        })
    }
}

impl std::fmt::Display for Concentration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.wrapped == 0 {
            write!(f, "0.0")
        } else {
            let epsilon_corrected = self.wrapped as f64 * Self::EPSILON;
            let scale = 1f64 / Self::EPSILON;
            let truncated = (epsilon_corrected * scale).trunc() / scale;

            write!(f, "{}", truncated)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LimitedFloat;

    #[test]
    fn test_precision() {
        let num_a = 0.00005;
        let num_b = 0.00009;

        let conc_a = LimitedFloat::from(num_a);
        let conc_b = LimitedFloat::from(num_b);

        assert_eq!(conc_a, conc_b)
    }

    #[test]
    fn test_addition() {
        let num_a: LimitedFloat = 0.01f64.into();
        let num_b: LimitedFloat = 0.01f64.into();

        let expected: LimitedFloat = 0.02f64.into();
        let sum = num_a + num_b;
        assert_eq!(sum, expected)
    }

    #[test]
    fn test_sub() {
        let num_a: LimitedFloat = 0.01f64.into();
        let num_b: LimitedFloat = 0.01f64.into();

        let expected: LimitedFloat = 0f64.into();
        let diff = num_a - num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_div() {
        let num_a: LimitedFloat = 1.0f64.into();
        let num_b: LimitedFloat = 2.0f64.into();

        let expected: LimitedFloat = 0.5f64.into();
        let diff = num_a / num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_display() {
        let num_a: LimitedFloat = 0.01f64.into();
        let expected = "0.01";
        let num_a_str = format!("{num_a}");
        assert_eq!(num_a_str, expected);
        let num_b: LimitedFloat = 0f64.into();
        let expected = "0.0";
        let num_b_str = format!("{num_b}");
        assert_eq!(num_b_str, expected);
    }
}
