#![forbid(unsafe_code)]
//! Primitive thermal-expansion helpers.
//!
//! Initial calculations assume SI units unless otherwise documented. For
//! temperature differences, `ΔK` and `Δ°C` are treated equivalently.
//!
//! # Examples
//!
//! ```rust
//! use use_thermal_expansion::{
//!     LinearExpansionCoefficient, area_expansion_coefficient, coefficient_from_lengths,
//!     final_length, linear_expansion, volume_expansion_coefficient,
//! };
//!
//! let coefficient = LinearExpansionCoefficient::new(12.0e-6).unwrap();
//! let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();
//! let expanded_length = final_length(2.0, 12.0e-6, 50.0).unwrap();
//! let inferred = coefficient_from_lengths(2.0, 2.0012, 50.0).unwrap();
//!
//! assert_eq!(coefficient.per_kelvin(), 12.0e-6);
//! assert!((expansion - 0.0012).abs() < 1.0e-12);
//! assert!((expanded_length - 2.0012).abs() < 1.0e-12);
//! assert!((inferred - 12.0e-6).abs() < 1.0e-12);
//! assert_eq!(area_expansion_coefficient(12.0e-6).unwrap(), 24.0e-6);
//! assert_eq!(volume_expansion_coefficient(12.0e-6).unwrap(), 36.0e-6);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearExpansionCoefficient {
    per_kelvin: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalExpansionError {
    InvalidCoefficient,
    InvalidInitialLength,
    InvalidFinalLength,
    InvalidTemperatureChange,
}

fn validate_coefficient(value: f64) -> Result<f64, ThermalExpansionError> {
    if !value.is_finite() {
        Err(ThermalExpansionError::InvalidCoefficient)
    } else {
        Ok(value)
    }
}

fn validate_initial_length(value: f64) -> Result<f64, ThermalExpansionError> {
    if !value.is_finite() || value <= 0.0 {
        Err(ThermalExpansionError::InvalidInitialLength)
    } else {
        Ok(value)
    }
}

fn validate_temperature_change(value: f64, allow_zero: bool) -> Result<f64, ThermalExpansionError> {
    if !value.is_finite() || (!allow_zero && value == 0.0) {
        Err(ThermalExpansionError::InvalidTemperatureChange)
    } else {
        Ok(value)
    }
}

impl LinearExpansionCoefficient {
    pub fn new(per_kelvin: f64) -> Result<Self, ThermalExpansionError> {
        Ok(Self {
            per_kelvin: validate_coefficient(per_kelvin)?,
        })
    }

    #[must_use]
    pub fn per_kelvin(&self) -> f64 {
        self.per_kelvin
    }
}

pub fn linear_expansion(
    initial_length_m: f64,
    coefficient_per_k: f64,
    delta_temp_k: f64,
) -> Result<f64, ThermalExpansionError> {
    Ok(validate_initial_length(initial_length_m)?
        * validate_coefficient(coefficient_per_k)?
        * validate_temperature_change(delta_temp_k, true)?)
}

pub fn final_length(
    initial_length_m: f64,
    coefficient_per_k: f64,
    delta_temp_k: f64,
) -> Result<f64, ThermalExpansionError> {
    Ok(validate_initial_length(initial_length_m)?
        + linear_expansion(initial_length_m, coefficient_per_k, delta_temp_k)?)
}

pub fn coefficient_from_lengths(
    initial_length_m: f64,
    final_length_m: f64,
    delta_temp_k: f64,
) -> Result<f64, ThermalExpansionError> {
    let initial_length_m = validate_initial_length(initial_length_m)?;

    if !final_length_m.is_finite() {
        return Err(ThermalExpansionError::InvalidFinalLength);
    }

    Ok((final_length_m - initial_length_m)
        / (initial_length_m * validate_temperature_change(delta_temp_k, false)?))
}

pub fn area_expansion_coefficient(
    linear_coefficient_per_k: f64,
) -> Result<f64, ThermalExpansionError> {
    Ok(2.0 * validate_coefficient(linear_coefficient_per_k)?)
}

pub fn volume_expansion_coefficient(
    linear_coefficient_per_k: f64,
) -> Result<f64, ThermalExpansionError> {
    Ok(3.0 * validate_coefficient(linear_coefficient_per_k)?)
}

#[cfg(test)]
mod tests {
    use super::{
        LinearExpansionCoefficient, ThermalExpansionError, area_expansion_coefficient,
        coefficient_from_lengths, final_length, linear_expansion, volume_expansion_coefficient,
    };

    #[test]
    fn computes_thermal_expansion_values() {
        let coefficient = LinearExpansionCoefficient::new(12.0e-6).unwrap();
        let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();
        let expanded_length = final_length(2.0, 12.0e-6, 50.0).unwrap();
        let inferred = coefficient_from_lengths(2.0, 2.0012, 50.0).unwrap();

        assert_eq!(coefficient.per_kelvin(), 12.0e-6);
        assert!((expansion - 0.0012).abs() < 1.0e-12);
        assert!((expanded_length - 2.0012).abs() < 1.0e-12);
        assert!((inferred - 12.0e-6).abs() < 1.0e-12);
        assert_eq!(area_expansion_coefficient(12.0e-6).unwrap(), 24.0e-6);
        assert_eq!(volume_expansion_coefficient(12.0e-6).unwrap(), 36.0e-6);
    }

    #[test]
    fn allows_negative_coefficients_and_temperature_differences() {
        assert!((linear_expansion(1.0, -1.0e-6, 10.0).unwrap() + 0.00001).abs() < 1.0e-12);
        assert!((linear_expansion(1.0, 1.0e-6, -10.0).unwrap() + 0.00001).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_invalid_thermal_expansion_inputs() {
        assert_eq!(
            LinearExpansionCoefficient::new(f64::NAN),
            Err(ThermalExpansionError::InvalidCoefficient)
        );
        assert_eq!(
            linear_expansion(0.0, 12.0e-6, 50.0),
            Err(ThermalExpansionError::InvalidInitialLength)
        );
        assert_eq!(
            coefficient_from_lengths(2.0, 2.0012, 0.0),
            Err(ThermalExpansionError::InvalidTemperatureChange)
        );
        assert_eq!(
            coefficient_from_lengths(2.0, f64::INFINITY, 50.0),
            Err(ThermalExpansionError::InvalidFinalLength)
        );
    }
}
