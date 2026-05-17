# use-materials

Composable primitive material-property utilities for Rust.

`use-materials` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-signal`, `use-physics`, `use-chemistry`,
`use-measure`, `use-units`, and `use-color`. It groups small, focused
crates for density, stress, strain, elasticity, hardness, thermal expansion,
conductivity, and lightweight material-property metadata.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64`, `usize`, and small structs or enums
- dependencies stay minimal so each crate is easy to audit and adopt

Initial calculations assume SI units unless otherwise documented.

## Workspace crates

- `use-materials`: thin facade crate that reexports the full materials workspace
- `use-materials-density`: material density primitives and conversions, exposed in code as `use_density`
- `use-stress`: stress and pressure-like helpers in pascals
- `use-strain`: engineering strain and percent-strain helpers
- `use-elasticity`: Young's-modulus and elastic-deformation helpers
- `use-hardness`: hardness values and same-scale comparisons
- `use-thermal-expansion`: linear expansion and coefficient helpers
- `use-conductivity`: thermal and electrical conductivity helpers
- `use-material-property`: lightweight material-property metadata helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-materials`. It
reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_materials::*;

let density = Density::new(7_850.0).unwrap();
let stress = Stress::new(250_000_000.0).unwrap();
let expansion = linear_expansion(2.0, 12.0e-6, 50.0).unwrap();

assert_eq!(density.kg_per_m3(), 7_850.0);
assert_eq!(stress.megapascals(), 250.0);
assert_eq!(expansion, 0.0012);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
materials surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
