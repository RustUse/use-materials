#![forbid(unsafe_code)]
//! Primitive conductivity helpers.
//!
//! Initial calculations assume SI units unless otherwise documented.
//!
//! # Examples
//!
//! ```rust
//! use use_conductivity::{
//!     ElectricalConductivity, ThermalConductivity, conductivity_from_resistivity,
//!     heat_flow_rate, resistivity_from_conductivity, thermal_resistance,
//! };
//!
//! let thermal = ThermalConductivity::new(50.0).unwrap();
//! let electrical = ElectricalConductivity::new(5.8e7).unwrap();
//!
//! assert_eq!(thermal.watts_per_meter_kelvin(), 50.0);
//! assert_eq!(electrical.siemens_per_meter(), 5.8e7);
//! assert_eq!(thermal_resistance(0.1, 2.0, 50.0).unwrap(), 0.001);
//! assert_eq!(heat_flow_rate(50.0, 2.0, 10.0, 0.1).unwrap(), 10_000.0);
//! assert_eq!(resistivity_from_conductivity(2.0).unwrap(), 0.5);
//! assert_eq!(conductivity_from_resistivity(0.5).unwrap(), 2.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalConductivity {
    watts_per_meter_kelvin: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElectricalConductivity {
    siemens_per_meter: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductivityError {
    InvalidConductivity,
    InvalidArea,
    InvalidThickness,
    InvalidTemperatureDifference,
    InvalidResistivity,
}

fn validate_positive(value: f64, error: ConductivityError) -> Result<f64, ConductivityError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

fn validate_finite(value: f64, error: ConductivityError) -> Result<f64, ConductivityError> {
    if !value.is_finite() {
        Err(error)
    } else {
        Ok(value)
    }
}

impl ThermalConductivity {
    pub fn new(watts_per_meter_kelvin: f64) -> Result<Self, ConductivityError> {
        Ok(Self {
            watts_per_meter_kelvin: validate_positive(
                watts_per_meter_kelvin,
                ConductivityError::InvalidConductivity,
            )?,
        })
    }

    #[must_use]
    pub fn watts_per_meter_kelvin(&self) -> f64 {
        self.watts_per_meter_kelvin
    }
}

impl ElectricalConductivity {
    pub fn new(siemens_per_meter: f64) -> Result<Self, ConductivityError> {
        Ok(Self {
            siemens_per_meter: validate_positive(
                siemens_per_meter,
                ConductivityError::InvalidConductivity,
            )?,
        })
    }

    #[must_use]
    pub fn siemens_per_meter(&self) -> f64 {
        self.siemens_per_meter
    }
}

pub fn thermal_resistance(
    thickness_m: f64,
    area_m2: f64,
    conductivity_w_per_mk: f64,
) -> Result<f64, ConductivityError> {
    Ok(
        validate_positive(thickness_m, ConductivityError::InvalidThickness)?
            / (validate_positive(area_m2, ConductivityError::InvalidArea)?
                * validate_positive(
                    conductivity_w_per_mk,
                    ConductivityError::InvalidConductivity,
                )?),
    )
}

pub fn heat_flow_rate(
    conductivity_w_per_mk: f64,
    area_m2: f64,
    delta_temp_k: f64,
    thickness_m: f64,
) -> Result<f64, ConductivityError> {
    Ok(validate_positive(
        conductivity_w_per_mk,
        ConductivityError::InvalidConductivity,
    )? * validate_positive(area_m2, ConductivityError::InvalidArea)?
        * validate_finite(
            delta_temp_k,
            ConductivityError::InvalidTemperatureDifference,
        )?
        / validate_positive(thickness_m, ConductivityError::InvalidThickness)?)
}

pub fn resistivity_from_conductivity(conductivity_s_per_m: f64) -> Result<f64, ConductivityError> {
    Ok(1.0 / validate_positive(conductivity_s_per_m, ConductivityError::InvalidConductivity)?)
}

pub fn conductivity_from_resistivity(resistivity_ohm_m: f64) -> Result<f64, ConductivityError> {
    Ok(1.0 / validate_positive(resistivity_ohm_m, ConductivityError::InvalidResistivity)?)
}

#[cfg(test)]
mod tests {
    use super::{
        ConductivityError, ElectricalConductivity, ThermalConductivity,
        conductivity_from_resistivity, heat_flow_rate, resistivity_from_conductivity,
        thermal_resistance,
    };

    #[test]
    fn computes_conductivity_and_resistivity_values() {
        let thermal = ThermalConductivity::new(50.0).unwrap();
        let electrical = ElectricalConductivity::new(5.8e7).unwrap();

        assert_eq!(thermal.watts_per_meter_kelvin(), 50.0);
        assert_eq!(electrical.siemens_per_meter(), 5.8e7);
        assert_eq!(thermal_resistance(0.1, 2.0, 50.0).unwrap(), 0.001);
        assert_eq!(heat_flow_rate(50.0, 2.0, 10.0, 0.1).unwrap(), 10_000.0);
        assert_eq!(resistivity_from_conductivity(2.0).unwrap(), 0.5);
        assert_eq!(conductivity_from_resistivity(0.5).unwrap(), 2.0);
    }

    #[test]
    fn allows_negative_temperature_differences_for_heat_flow_direction() {
        assert_eq!(heat_flow_rate(50.0, 2.0, -10.0, 0.1).unwrap(), -10_000.0);
    }

    #[test]
    fn rejects_invalid_conductivity_inputs() {
        assert_eq!(
            ThermalConductivity::new(0.0),
            Err(ConductivityError::InvalidConductivity)
        );
        assert_eq!(
            ElectricalConductivity::new(f64::NAN),
            Err(ConductivityError::InvalidConductivity)
        );
        assert_eq!(
            thermal_resistance(0.1, 0.0, 50.0),
            Err(ConductivityError::InvalidArea)
        );
        assert_eq!(
            heat_flow_rate(50.0, 2.0, f64::NAN, 0.1),
            Err(ConductivityError::InvalidTemperatureDifference)
        );
        assert_eq!(
            conductivity_from_resistivity(-1.0),
            Err(ConductivityError::InvalidResistivity)
        );
    }
}
