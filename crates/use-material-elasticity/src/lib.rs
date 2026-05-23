#![forbid(unsafe_code)]
//! Primitive elasticity helpers.
//!
//! Initial calculations assume SI units unless otherwise documented.
//!
//! # Examples
//!
//! ```rust
//! use use_material_elasticity::{
//!     ElasticModulus, elastic_deformation, strain_from_modulus, stress_from_modulus,
//!     youngs_modulus,
//! };
//!
//! let modulus = ElasticModulus::new(200_000_000_000.0).unwrap();
//!
//! assert_eq!(modulus.gigapascals(), 200.0);
//! assert_eq!(youngs_modulus(400_000_000.0, 0.002).unwrap(), 200_000_000_000.0);
//! assert_eq!(stress_from_modulus(200_000_000_000.0, 0.002).unwrap(), 400_000_000.0);
//! assert_eq!(strain_from_modulus(400_000_000.0, 200_000_000_000.0).unwrap(), 0.002);
//! assert_eq!(elastic_deformation(1_000.0, 2.0, 0.01, 200_000_000_000.0).unwrap(), 0.000_001);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElasticModulus {
    pascals: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElasticityError {
    InvalidStress,
    InvalidStrain,
    InvalidModulus,
    InvalidForce,
    InvalidLength,
    InvalidArea,
}

const fn validate_positive(value: f64, error: ElasticityError) -> Result<f64, ElasticityError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(error)
    }
}

const fn validate_finite(value: f64, error: ElasticityError) -> Result<f64, ElasticityError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(error)
    }
}

impl ElasticModulus {
    /// Creates an elastic modulus from pascals.
    ///
    /// # Errors
    ///
    /// Returns [`ElasticityError::InvalidModulus`] when `pascals` is not finite or is less than
    /// or equal to zero.
    pub fn new(pascals: f64) -> Result<Self, ElasticityError> {
        Ok(Self {
            pascals: validate_positive(pascals, ElasticityError::InvalidModulus)?,
        })
    }

    #[must_use]
    pub const fn pascals(&self) -> f64 {
        self.pascals
    }

    #[must_use]
    pub const fn gigapascals(&self) -> f64 {
        self.pascals / 1_000_000_000.0
    }
}

/// Computes Young's modulus from stress and strain.
///
/// # Errors
///
/// Returns [`ElasticityError::InvalidStress`] when `stress_pa` is not finite,
/// [`ElasticityError::InvalidStrain`] when `strain` is not finite or is zero, and
/// [`ElasticityError::InvalidModulus`] when the computed modulus is not positive.
pub fn youngs_modulus(stress_pa: f64, strain: f64) -> Result<f64, ElasticityError> {
    let stress_pa = validate_finite(stress_pa, ElasticityError::InvalidStress)?;
    let strain = validate_finite(strain, ElasticityError::InvalidStrain)?;

    if strain == 0.0 {
        return Err(ElasticityError::InvalidStrain);
    }

    let modulus = stress_pa / strain;
    validate_positive(modulus, ElasticityError::InvalidModulus)
}

/// Computes stress from modulus and strain.
///
/// # Errors
///
/// Returns [`ElasticityError::InvalidModulus`] when `modulus_pa` is not finite or is less than or
/// equal to zero, and [`ElasticityError::InvalidStrain`] when `strain` is not finite.
pub fn stress_from_modulus(modulus_pa: f64, strain: f64) -> Result<f64, ElasticityError> {
    Ok(
        validate_positive(modulus_pa, ElasticityError::InvalidModulus)?
            * validate_finite(strain, ElasticityError::InvalidStrain)?,
    )
}

/// Computes strain from stress and modulus.
///
/// # Errors
///
/// Returns [`ElasticityError::InvalidStress`] when `stress_pa` is not finite, and
/// [`ElasticityError::InvalidModulus`] when `modulus_pa` is not finite or is less than or equal to
/// zero.
pub fn strain_from_modulus(stress_pa: f64, modulus_pa: f64) -> Result<f64, ElasticityError> {
    Ok(validate_finite(stress_pa, ElasticityError::InvalidStress)?
        / validate_positive(modulus_pa, ElasticityError::InvalidModulus)?)
}

/// Computes axial elastic deformation.
///
/// # Errors
///
/// Returns an [`ElasticityError`] when any input is not finite, or when length, area, or modulus is
/// less than or equal to zero.
pub fn elastic_deformation(
    force_newtons: f64,
    length_m: f64,
    area_m2: f64,
    modulus_pa: f64,
) -> Result<f64, ElasticityError> {
    Ok(
        validate_finite(force_newtons, ElasticityError::InvalidForce)?
            * validate_positive(length_m, ElasticityError::InvalidLength)?
            / (validate_positive(area_m2, ElasticityError::InvalidArea)?
                * validate_positive(modulus_pa, ElasticityError::InvalidModulus)?),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ElasticModulus, ElasticityError, elastic_deformation, strain_from_modulus,
        stress_from_modulus, youngs_modulus,
    };

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1.0e-12);
    }

    #[test]
    fn computes_modulus_and_hookes_law_values() {
        let modulus = ElasticModulus::new(200_000_000_000.0).unwrap();

        assert_close(modulus.pascals(), 200_000_000_000.0);
        assert_close(modulus.gigapascals(), 200.0);
        assert_close(
            youngs_modulus(400_000_000.0, 0.002).unwrap(),
            200_000_000_000.0,
        );
        assert_close(
            stress_from_modulus(200_000_000_000.0, 0.002).unwrap(),
            400_000_000.0,
        );
        assert_close(
            strain_from_modulus(400_000_000.0, 200_000_000_000.0).unwrap(),
            0.002,
        );
    }

    #[test]
    fn computes_elastic_deformation() {
        assert_close(
            elastic_deformation(1_000.0, 2.0, 0.01, 200_000_000_000.0).unwrap(),
            0.000_001,
        );
    }

    #[test]
    fn rejects_invalid_elasticity_inputs() {
        assert_eq!(
            ElasticModulus::new(0.0),
            Err(ElasticityError::InvalidModulus)
        );
        assert_eq!(
            youngs_modulus(400_000_000.0, 0.0),
            Err(ElasticityError::InvalidStrain)
        );
        assert_eq!(
            youngs_modulus(-1.0, 0.001),
            Err(ElasticityError::InvalidModulus)
        );
        assert_eq!(
            stress_from_modulus(f64::NAN, 0.001),
            Err(ElasticityError::InvalidModulus)
        );
        assert_eq!(
            strain_from_modulus(1.0, -1.0),
            Err(ElasticityError::InvalidModulus)
        );
        assert_eq!(
            elastic_deformation(1_000.0, 2.0, 0.0, 200_000_000_000.0),
            Err(ElasticityError::InvalidArea)
        );
    }
}
