# lean-sys
[![crates.io](https://img.shields.io/crates/v/lean-sys)](https://crates.io/crates/lean-sys)
[![docs.rs](https://img.shields.io/docsrs/lean-sys)](https://docs.rs/crate/lean-sys/latest)

Rust bindings to [Lean 4](https://github.com/leanprover/lean4)'s C API

Functions and comments manually translated from those in the [`lean.h` header](https://github.com/leanprover/lean4/blob/master/src/include/lean/lean.h) provided with Lean 4

## Dynamic Builds

If you are using this package as part of a dynamic build - in particular if your `Cargo.toml` is configured with

```toml
[lib]
crate-type = ["cdylib"]
```

then the `static` feature of this library (which is enabled by default) will need to be disabled. This can be done by configuring `lean-sys` as follows:

```toml
[dependencies]
lean-sys = { version = "0.0.5", features=["small_allocator"], default-features=false }
```
