# `gba`

# [Docs.rs Documentation](https://docs.rs/gba)

This crate is intended for working with the GBA.

To build for the GBA you'll need to use `build-std` (on Nightly) with the
`thumbv4t-none-eabi` or `armv4t-none-eabi` targets. The two targets are
identical except for which instruction set (thumb or arm) is used by default. It
is suggested that you use the thumb target, unless you know what you're doing and
have a very good reason to use the arm target.

Your `.cargo/config.toml` should look something like this:

```toml
[build]
target = "thumbv4t-none-eabi"

[unstable]
build-std = ["core"]
```

If you don't use this crate on the GBA, you **MUST** disable default features and
then not use the `on_gba` cargo feature.
