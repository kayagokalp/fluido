use std::{
    num::ParseFloatError,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Concentration {
    pub wrapped: i64,
}

impl Concentration {
    pub const EPSILON: f64 = 0.0001;
    pub fn from_f64(val: f64) -> Self {
        Self {
            wrapped: (val / Self::EPSILON).round() as i64,
        }
    }
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
        let epsilon_corrected = self.wrapped as f64 * Self::EPSILON;
        let scale = 1f64 / Self::EPSILON;
        let truncated = (epsilon_corrected * scale).trunc() / scale;

        write!(f, "{}", truncated)
    }
}

#[cfg(test)]
mod tests {
    use super::Concentration;

    #[test]
    fn test_precision() {
        let num_a = 0.00005;
        let num_b = 0.00009;

        let conc_a = Concentration::from_f64(num_a);
        let conc_b = Concentration::from_f64(num_b);

        assert_eq!(conc_a, conc_b)
    }

    #[test]
    fn test_addition() {
        let num_a: Concentration = 0.01f64.into();
        let num_b: Concentration = 0.01f64.into();

        let expected: Concentration = 0.02f64.into();
        let sum = num_a + num_b;
        assert_eq!(sum, expected)
    }

    #[test]
    fn test_sub() {
        let num_a: Concentration = 0.01f64.into();
        let num_b: Concentration = 0.01f64.into();

        let expected: Concentration = 0f64.into();
        let diff = num_a - num_b;
        assert_eq!(diff, expected)
    }
}
