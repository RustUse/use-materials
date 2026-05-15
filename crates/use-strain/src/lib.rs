#![forbid(unsafe_code)]
//! Primitive strain helpers.
//!
//! Initial calculations assume SI units unless otherwise documented.
//!
//! # Examples
//!
//! ```rust
//! use use_strain::{Strain, engineering_strain, percent_strain, strain_from_change};
//!
//! let strain = Strain::new(0.05).unwrap();
//! let engineering = engineering_strain(2.0, 2.1).unwrap();
//! let from_change = strain_from_change(2.0, 0.1).unwrap();
//!
//! assert_eq!(strain.percent(), 5.0);
//! assert!((engineering - 0.05).abs() < 1.0e-12);
//! assert!((from_change - 0.05).abs() < 1.0e-12);
//! assert_eq!(percent_strain(0.05).unwrap(), 5.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Strain {
    ratio: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrainError {
    InvalidStrain,
    InvalidOriginalLength,
    InvalidFinalLength,
    InvalidChangeInLength,
}

fn validate_finite(value: f64, error: StrainError) -> Result<f64, StrainError> {
    if !value.is_finite() {
        Err(error)
    } else {
        Ok(value)
    }
}

fn validate_original_length(value: f64) -> Result<f64, StrainError> {
    if !value.is_finite() || value <= 0.0 {
        Err(StrainError::InvalidOriginalLength)
    } else {
        Ok(value)
    }
}

impl Strain {
    pub fn new(ratio: f64) -> Result<Self, StrainError> {
        Ok(Self {
            ratio: validate_finite(ratio, StrainError::InvalidStrain)?,
        })
    }

    #[must_use]
    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    #[must_use]
    pub fn percent(&self) -> f64 {
        self.ratio * 100.0
    }
}

pub fn engineering_strain(original_length_m: f64, final_length_m: f64) -> Result<f64, StrainError> {
    let original_length_m = validate_original_length(original_length_m)?;
    let final_length_m = validate_finite(final_length_m, StrainError::InvalidFinalLength)?;

    Ok((final_length_m - original_length_m) / original_length_m)
}

pub fn strain_from_change(
    original_length_m: f64,
    change_in_length_m: f64,
) -> Result<f64, StrainError> {
    Ok(
        validate_finite(change_in_length_m, StrainError::InvalidChangeInLength)?
            / validate_original_length(original_length_m)?,
    )
}

pub fn percent_strain(strain: f64) -> Result<f64, StrainError> {
    Ok(validate_finite(strain, StrainError::InvalidStrain)? * 100.0)
}

#[cfg(test)]
mod tests {
    use super::{Strain, StrainError, engineering_strain, percent_strain, strain_from_change};

    #[test]
    fn computes_strain_values() {
        let strain = Strain::new(0.05).unwrap();
        let engineering = engineering_strain(2.0, 2.1).unwrap();
        let from_change = strain_from_change(2.0, 0.1).unwrap();

        assert_eq!(strain.ratio(), 0.05);
        assert_eq!(strain.percent(), 5.0);
        assert!((engineering - 0.05).abs() < 1.0e-12);
        assert!((from_change - 0.05).abs() < 1.0e-12);
        assert_eq!(percent_strain(0.05).unwrap(), 5.0);
    }

    #[test]
    fn allows_negative_strain_for_contraction() {
        assert!((engineering_strain(2.0, 1.9).unwrap() + 0.05).abs() < 1.0e-12);
        assert_eq!(Strain::new(-0.02).unwrap().percent(), -2.0);
    }

    #[test]
    fn rejects_invalid_length_and_strain_inputs() {
        assert_eq!(Strain::new(f64::NAN), Err(StrainError::InvalidStrain));
        assert_eq!(
            engineering_strain(0.0, 2.0),
            Err(StrainError::InvalidOriginalLength)
        );
        assert_eq!(
            engineering_strain(2.0, f64::INFINITY),
            Err(StrainError::InvalidFinalLength)
        );
        assert_eq!(
            strain_from_change(-1.0, 0.1),
            Err(StrainError::InvalidOriginalLength)
        );
        assert_eq!(
            percent_strain(f64::INFINITY),
            Err(StrainError::InvalidStrain)
        );
    }
}
