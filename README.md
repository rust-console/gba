# `gba`

# [Docs.rs Documentation](https://docs.rs/gba)

This crate is intended for working with the GBA.

To build for the GBA you'll need to use `build-std` and you'll also need to
activate the `compiler-builtins-weak-intrinsics` feature.

The following should be somewhere in your `.cargo/config.toml`:

```toml
[unstable]
build-std = ["core"]
build-std-features = ["compiler-builtins-weak-intrinsics"]
```
