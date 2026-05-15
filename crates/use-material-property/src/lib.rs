#![forbid(unsafe_code)]
//! Lightweight material-property metadata helpers.
//!
//! This crate intentionally stays small. It models a property kind, a numeric
//! value, and a display unit without becoming a full material database.
//!
//! # Examples
//!
//! ```rust
//! use use_material_property::{
//!     MaterialProperty, MaterialPropertyKind, filter_by_kind, property_values,
//! };
//!
//! let properties = [
//!     MaterialProperty::new(MaterialPropertyKind::Density, 7_850.0, "kg/m^3").unwrap(),
//!     MaterialProperty::new(MaterialPropertyKind::Stress, 250_000_000.0, "Pa").unwrap(),
//!     MaterialProperty::new(MaterialPropertyKind::Density, 1_000.0, "kg/m^3").unwrap(),
//! ];
//!
//! assert_eq!(filter_by_kind(&properties, MaterialPropertyKind::Density).len(), 2);
//! assert_eq!(property_values(&properties, MaterialPropertyKind::Density), vec![7_850.0, 1_000.0]);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPropertyKind {
    Density,
    Stress,
    Strain,
    ElasticModulus,
    Hardness,
    ThermalConductivity,
    ElectricalConductivity,
    ThermalExpansion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialProperty {
    kind: MaterialPropertyKind,
    value: f64,
    unit: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPropertyError {
    InvalidValue,
    InvalidUnit,
}

impl MaterialProperty {
    pub fn new(
        kind: MaterialPropertyKind,
        value: f64,
        unit: &'static str,
    ) -> Result<Self, MaterialPropertyError> {
        if !value.is_finite() {
            return Err(MaterialPropertyError::InvalidValue);
        }

        if unit.trim().is_empty() {
            return Err(MaterialPropertyError::InvalidUnit);
        }

        Ok(Self { kind, value, unit })
    }

    #[must_use]
    pub fn kind(&self) -> MaterialPropertyKind {
        self.kind
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn unit(&self) -> &'static str {
        self.unit
    }
}

#[must_use]
pub fn filter_by_kind(
    properties: &[MaterialProperty],
    kind: MaterialPropertyKind,
) -> Vec<MaterialProperty> {
    properties
        .iter()
        .copied()
        .filter(|property| property.kind == kind)
        .collect()
}

#[must_use]
pub fn property_values(properties: &[MaterialProperty], kind: MaterialPropertyKind) -> Vec<f64> {
    filter_by_kind(properties, kind)
        .into_iter()
        .map(|property| property.value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        MaterialProperty, MaterialPropertyError, MaterialPropertyKind, filter_by_kind,
        property_values,
    };

    #[test]
    fn filters_properties_by_kind() {
        let properties = [
            MaterialProperty::new(MaterialPropertyKind::Density, 7_850.0, "kg/m^3").unwrap(),
            MaterialProperty::new(MaterialPropertyKind::Stress, 250_000_000.0, "Pa").unwrap(),
            MaterialProperty::new(MaterialPropertyKind::Density, 1_000.0, "kg/m^3").unwrap(),
        ];

        assert_eq!(
            filter_by_kind(&properties, MaterialPropertyKind::Density).len(),
            2
        );
        assert_eq!(
            property_values(&properties, MaterialPropertyKind::Density),
            vec![7_850.0, 1_000.0]
        );
    }

    #[test]
    fn exposes_property_fields() {
        let property =
            MaterialProperty::new(MaterialPropertyKind::ElasticModulus, 200.0, "GPa").unwrap();

        assert_eq!(property.kind(), MaterialPropertyKind::ElasticModulus);
        assert_eq!(property.value(), 200.0);
        assert_eq!(property.unit(), "GPa");
    }

    #[test]
    fn rejects_invalid_material_property_inputs() {
        assert_eq!(
            MaterialProperty::new(MaterialPropertyKind::Density, f64::NAN, "kg/m^3"),
            Err(MaterialPropertyError::InvalidValue)
        );
        assert_eq!(
            MaterialProperty::new(MaterialPropertyKind::Density, 1.0, "   "),
            Err(MaterialPropertyError::InvalidUnit)
        );
    }
}
