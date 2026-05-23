# use-materials

Composable facade crate for RustUse material-property primitives.

## Install

```toml
[dependencies]
use-materials = "0.0.2"
```

## Usage

```rust
use use_materials::*;

let density = Density::new(7_850.0).unwrap();
let stress = Stress::new(250_000_000.0).unwrap();
let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();

assert_eq!(density.kg_per_m3(), 7_850.0);
assert_eq!(stress.megapascals(), 250.0);
assert_eq!(expansion, 0.0012);
```

This facade reexports the focused material-property crates, including `use-material-elasticity` as `use_material_elasticity`.
