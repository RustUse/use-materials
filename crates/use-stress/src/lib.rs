#![forbid(unsafe_code)]
//! Primitive stress and pressure-like helpers.
//!
//! Initial calculations assume SI units unless otherwise documented.
//!
//! # Examples
//!
//! ```rust
//! use use_stress::{Stress, force_from_stress, normal_stress, shear_stress};
//!
//! let stress = Stress::new(250_000_000.0).unwrap();
//!
//! assert_eq!(stress.megapascals(), 250.0);
//! assert_eq!(normal_stress(1_000.0, 0.01).unwrap(), 100_000.0);
//! assert_eq!(shear_stress(1_000.0, 0.01).unwrap(), 100_000.0);
//! assert_eq!(force_from_stress(100_000.0, 0.01).unwrap(), 1_000.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stress {
    pascals: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressError {
    InvalidStress,
    InvalidForce,
    InvalidArea,
}

fn validate_finite(value: f64, error: StressError) -> Result<f64, StressError> {
    if !value.is_finite() {
        Err(error)
    } else {
        Ok(value)
    }
}

fn validate_area(area_m2: f64) -> Result<f64, StressError> {
    if !area_m2.is_finite() || area_m2 <= 0.0 {
        Err(StressError::InvalidArea)
    } else {
        Ok(area_m2)
    }
}

impl Stress {
    pub fn new(pascals: f64) -> Result<Self, StressError> {
        Ok(Self {
            pascals: validate_finite(pascals, StressError::InvalidStress)?,
        })
    }

    #[must_use]
    pub fn pascals(&self) -> f64 {
        self.pascals
    }

    #[must_use]
    pub fn megapascals(&self) -> f64 {
        self.pascals / 1_000_000.0
    }

    #[must_use]
    pub fn gigapascals(&self) -> f64 {
        self.pascals / 1_000_000_000.0
    }
}

pub fn normal_stress(force_newtons: f64, area_m2: f64) -> Result<f64, StressError> {
    Ok(validate_finite(force_newtons, StressError::InvalidForce)? / validate_area(area_m2)?)
}

pub fn shear_stress(force_newtons: f64, area_m2: f64) -> Result<f64, StressError> {
    normal_stress(force_newtons, area_m2)
}

pub fn force_from_stress(stress_pa: f64, area_m2: f64) -> Result<f64, StressError> {
    Ok(validate_finite(stress_pa, StressError::InvalidStress)? * validate_area(area_m2)?)
}

#[cfg(test)]
mod tests {
    use super::{Stress, StressError, force_from_stress, normal_stress, shear_stress};

    #[test]
    fn computes_stress_and_force_values() {
        let stress = Stress::new(250_000_000.0).unwrap();

        assert_eq!(stress.pascals(), 250_000_000.0);
        assert_eq!(stress.megapascals(), 250.0);
        assert_eq!(stress.gigapascals(), 0.25);
        assert_eq!(normal_stress(1_000.0, 0.01).unwrap(), 100_000.0);
        assert_eq!(shear_stress(1_000.0, 0.01).unwrap(), 100_000.0);
        assert_eq!(force_from_stress(100_000.0, 0.01).unwrap(), 1_000.0);
    }

    #[test]
    fn allows_signed_stress_values() {
        let stress = Stress::new(-50_000.0).unwrap();

        assert_eq!(stress.megapascals(), -0.05);
        assert_eq!(normal_stress(-100.0, 0.01).unwrap(), -10_000.0);
    }

    #[test]
    fn rejects_invalid_force_stress_and_area_inputs() {
        assert_eq!(Stress::new(f64::NAN), Err(StressError::InvalidStress));
        assert_eq!(
            normal_stress(f64::INFINITY, 1.0),
            Err(StressError::InvalidForce)
        );
        assert_eq!(shear_stress(1.0, 0.0), Err(StressError::InvalidArea));
        assert_eq!(force_from_stress(1.0, -1.0), Err(StressError::InvalidArea));
    }
}
