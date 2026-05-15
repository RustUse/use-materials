#![forbid(unsafe_code)]
//! Thin facade for the `use-materials` workspace.
//!
//! The crate reexports the focused material-property crates directly so
//! consumers can opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_materials::*;
//!
//! let density = Density::new(7_850.0).unwrap();
//! let stress = Stress::new(250_000_000.0).unwrap();
//! let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();
//!
//! assert_eq!(density.kg_per_m3(), 7_850.0);
//! assert_eq!(stress.megapascals(), 250.0);
//! assert!((expansion - 0.0012).abs() < 1.0e-12);
//! ```

pub use use_conductivity;
pub use use_conductivity::*;
pub use use_density;
pub use use_density::*;
pub use use_elasticity;
pub use use_elasticity::*;
pub use use_hardness;
pub use use_hardness::*;
pub use use_material_property;
pub use use_material_property::*;
pub use use_strain;
pub use use_strain::*;
pub use use_stress;
pub use use_stress::*;
pub use use_thermal_expansion;
pub use use_thermal_expansion::*;

#[cfg(test)]
mod tests {
    use super::{
        Density, Hardness, HardnessScale, MaterialProperty, MaterialPropertyKind, Stress,
        filter_by_kind, linear_expansion,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let density = Density::new(7_850.0).unwrap();
        assert_eq!(density.kg_per_m3(), 7_850.0);

        let stress = Stress::new(250_000_000.0).unwrap();
        assert_eq!(stress.megapascals(), 250.0);

        let hardness = Hardness::new(200.0, HardnessScale::Vickers).unwrap();
        assert_eq!(hardness.scale(), HardnessScale::Vickers);

        let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();
        assert!((expansion - 0.0012).abs() < 1.0e-12);

        let properties =
            [MaterialProperty::new(MaterialPropertyKind::Density, 7_850.0, "kg/m^3").unwrap()];
        assert_eq!(
            filter_by_kind(&properties, MaterialPropertyKind::Density).len(),
            1
        );
    }
}
