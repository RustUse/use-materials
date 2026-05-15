#![forbid(unsafe_code)]
//! Primitive hardness helpers.
//!
//! Hardness values are only compared when they share the same scale. No
//! approximate cross-scale conversions are provided in this first pass.
//!
//! # Examples
//!
//! ```rust
//! use use_hardness::{Hardness, HardnessScale, is_harder_than, same_scale};
//!
//! let first = Hardness::new(200.0, HardnessScale::Vickers).unwrap();
//! let second = Hardness::new(180.0, HardnessScale::Vickers).unwrap();
//!
//! assert!(same_scale(first, second));
//! assert!(is_harder_than(first, second).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardnessScale {
    Brinell,
    Vickers,
    RockwellA,
    RockwellB,
    RockwellC,
    Mohs,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hardness {
    value: f64,
    scale: HardnessScale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardnessError {
    InvalidValue,
    ScaleMismatch,
}

impl Hardness {
    pub fn new(value: f64, scale: HardnessScale) -> Result<Self, HardnessError> {
        if !value.is_finite() || value <= 0.0 {
            return Err(HardnessError::InvalidValue);
        }

        Ok(Self { value, scale })
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn scale(&self) -> HardnessScale {
        self.scale
    }
}

#[must_use]
pub fn same_scale(a: Hardness, b: Hardness) -> bool {
    a.scale == b.scale
}

pub fn is_harder_than(a: Hardness, b: Hardness) -> Result<bool, HardnessError> {
    if !same_scale(a, b) {
        return Err(HardnessError::ScaleMismatch);
    }

    Ok(a.value > b.value)
}

#[cfg(test)]
mod tests {
    use super::{Hardness, HardnessError, HardnessScale, is_harder_than, same_scale};

    #[test]
    fn compares_hardness_values_on_the_same_scale() {
        let first = Hardness::new(200.0, HardnessScale::Vickers).unwrap();
        let second = Hardness::new(180.0, HardnessScale::Vickers).unwrap();

        assert!(same_scale(first, second));
        assert!(is_harder_than(first, second).unwrap());
        assert!(!is_harder_than(second, first).unwrap());
    }

    #[test]
    fn rejects_cross_scale_comparisons() {
        let first = Hardness::new(60.0, HardnessScale::RockwellC).unwrap();
        let second = Hardness::new(8.0, HardnessScale::Mohs).unwrap();

        assert!(!same_scale(first, second));
        assert_eq!(
            is_harder_than(first, second),
            Err(HardnessError::ScaleMismatch)
        );
    }

    #[test]
    fn rejects_invalid_hardness_values() {
        assert_eq!(
            Hardness::new(0.0, HardnessScale::Brinell),
            Err(HardnessError::InvalidValue)
        );
        assert_eq!(
            Hardness::new(f64::NAN, HardnessScale::Mohs),
            Err(HardnessError::InvalidValue)
        );
    }
}
