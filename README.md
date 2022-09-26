# `gba`

## Status: Pending

This branch is a **pending** release for `0.7` of the crate.

## How To Make Your Own GBA Project Using This Crate

This will require the use of Nightly Rust. Any recent-ish version of Nightly should be fine.

### Get ARM Binutils

You'll need the ARM version of the GNU binutils in your path, specifically the linker (`arm-none-eabi-ld`).

You can get them from your linux package manager, or from the [ARM Website](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain)

### Run `rustup component add rust-src`

This makes rustup keep the standard library source code on hand, which is necessary for `build-std` to work.

### Create `.cargo/config.toml`

You should set up your project's cargo config like so:

```toml
[build]
target = "thumbv4t-none-eabi"

[unstable]
build-std = ["core"]

[target.thumbv4t-none-eabi]
runner = "mgba-qt"
rustflags = ["-Clink-arg=-Tlink_scripts/mono_boot.ld"]
```

This sets the default build target to be `thumbv4t-none-eabi` using the unstable `build-std` cargo feature.

Also, this sets `cargo run` to run the binary as an argument to `mgba-qt`.
If you're on windows then your copy of mGBA will be called "mgba.exe" instead.

Also, this sets [mono_boot.ld](link_scripts/mono_boot.ld) as the linker script.
You'll need to copy this into your project.
If you save it to another location, adjust the path accordingly.

### Make Your Executables

At this point you can make a `bin` or an `example`.

Every executable will need to be `no_std` and `no_main`.
Place these at the top of the file:

```rust
#![no_std]
#![no_main]
```

Every executable will need a panic handler defined, even if your code can't actually panic.
A minimal panic handler looks like this:

```rust
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

Every executable will need a `main` function defined.
We used the `no_main` attribute on the executable so that Rust will allow us to use a non-standard function signature:

```rust
#[no_mangle]
extern "C" fn main() -> ! {
  loop {}
}
```

### Optional: Use `objcopy` and `gbafix`

The `cargo build` will produce ELF files, which mGBA can run directly.

If you want to run your program on real hardware you'll need to:

1) `objcopy` the raw binary out of the ELF into its own file.
2) Use `gbafix` to give the file appropriate header data to that file.

You can get `gbafix` through cargo: `cargo install gbafix`.

## Other GBA Crates

This crate provides a largely "unmanaged" interaction with the GBA's hardware.
If you would like an API that use the borrow checker to guide you more,
the [agb](https://docs.rs/agb) crate might be what you want.
