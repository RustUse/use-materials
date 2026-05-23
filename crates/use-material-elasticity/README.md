# use-material-elasticity

Primitive elasticity and Hooke's-law-style helpers for material-property calculations.

## Install

```toml
[dependencies]
use-material-elasticity = "0.0.1"
```

## Usage

```rust
use use_material_elasticity::{
    ElasticModulus, elastic_deformation, strain_from_modulus, stress_from_modulus, youngs_modulus,
};

let modulus = ElasticModulus::new(200_000_000_000.0).unwrap();

assert_eq!(modulus.gigapascals(), 200.0);
assert_eq!(youngs_modulus(400_000_000.0, 0.002).unwrap(), 200_000_000_000.0);
assert_eq!(stress_from_modulus(200_000_000_000.0, 0.002).unwrap(), 400_000_000.0);
assert_eq!(strain_from_modulus(400_000_000.0, 200_000_000_000.0).unwrap(), 0.002);
assert_eq!(elastic_deformation(1_000.0, 2.0, 0.01, 200_000_000_000.0).unwrap(), 0.000_001);
```

## Scope

This crate covers primitive material elasticity helpers. The physics-owned `use-elasticity` crate covers broader scalar elasticity and mechanics-of-materials relations.
