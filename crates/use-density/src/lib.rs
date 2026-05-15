#![forbid(unsafe_code)]
//! Primitive material density helpers.
//!
//! Initial calculations assume SI units unless otherwise documented.
//!
//! # Examples
//!
//! ```rust
//! use use_density::{Density, density, mass_from_density, volume_from_density};
//!
//! let density_value = Density::new(1_000.0).unwrap();
//!
//! assert_eq!(density_value.kg_per_m3(), 1_000.0);
//! assert_eq!(density(10.0, 0.5).unwrap(), 20.0);
//! assert_eq!(mass_from_density(1_000.0, 0.01).unwrap(), 10.0);
//! assert_eq!(volume_from_density(10.0, 1_000.0).unwrap(), 0.01);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Density {
    kg_per_m3: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensityError {
    InvalidDensity,
    InvalidMass,
    InvalidVolume,
}

fn validate_density(value: f64) -> Result<f64, DensityError> {
    if !value.is_finite() || value <= 0.0 {
        Err(DensityError::InvalidDensity)
    } else {
        Ok(value)
    }
}

fn validate_mass(mass_kg: f64) -> Result<f64, DensityError> {
    if !mass_kg.is_finite() || mass_kg < 0.0 {
        Err(DensityError::InvalidMass)
    } else {
        Ok(mass_kg)
    }
}

fn validate_volume(volume_m3: f64) -> Result<f64, DensityError> {
    if !volume_m3.is_finite() || volume_m3 <= 0.0 {
        Err(DensityError::InvalidVolume)
    } else {
        Ok(volume_m3)
    }
}

impl Density {
    pub fn new(kg_per_m3: f64) -> Result<Self, DensityError> {
        Ok(Self {
            kg_per_m3: validate_density(kg_per_m3)?,
        })
    }

    #[must_use]
    pub fn kg_per_m3(&self) -> f64 {
        self.kg_per_m3
    }
}

pub fn density(mass_kg: f64, volume_m3: f64) -> Result<f64, DensityError> {
    Ok(validate_mass(mass_kg)? / validate_volume(volume_m3)?)
}

pub fn mass_from_density(density_kg_per_m3: f64, volume_m3: f64) -> Result<f64, DensityError> {
    Ok(validate_density(density_kg_per_m3)? * validate_volume(volume_m3)?)
}

pub fn volume_from_density(mass_kg: f64, density_kg_per_m3: f64) -> Result<f64, DensityError> {
    Ok(validate_mass(mass_kg)? / validate_density(density_kg_per_m3)?)
}

#[cfg(test)]
mod tests {
    use super::{Density, DensityError, density, mass_from_density, volume_from_density};

    #[test]
    fn computes_density_related_values() {
        let density_value = Density::new(7_850.0).unwrap();
        let computed_density = density(15.7, 0.002).unwrap();
        let computed_mass = mass_from_density(7_850.0, 0.002).unwrap();
        let computed_volume = volume_from_density(15.7, 7_850.0).unwrap();

        assert_eq!(density_value.kg_per_m3(), 7_850.0);
        assert!((computed_density - 7_850.0).abs() < 1.0e-12);
        assert!((computed_mass - 15.7).abs() < 1.0e-12);
        assert!((computed_volume - 0.002).abs() < 1.0e-12);
    }

    #[test]
    fn allows_zero_mass_where_valid() {
        assert_eq!(density(0.0, 1.0).unwrap(), 0.0);
        assert_eq!(volume_from_density(0.0, 1_000.0).unwrap(), 0.0);
    }

    #[test]
    fn rejects_invalid_density_inputs() {
        assert_eq!(Density::new(0.0), Err(DensityError::InvalidDensity));
        assert_eq!(
            mass_from_density(f64::NAN, 1.0),
            Err(DensityError::InvalidDensity)
        );
        assert_eq!(
            volume_from_density(1.0, -5.0),
            Err(DensityError::InvalidDensity)
        );
    }

    #[test]
    fn rejects_invalid_mass_and_volume_inputs() {
        assert_eq!(density(-1.0, 1.0), Err(DensityError::InvalidMass));
        assert_eq!(density(1.0, 0.0), Err(DensityError::InvalidVolume));
        assert_eq!(
            mass_from_density(1_000.0, f64::INFINITY),
            Err(DensityError::InvalidVolume)
        );
    }
}
