[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)

# gba

A crate that helps you make GBA games

# First Time Setup

[ketsuban](https://github.com/ketsuban) is the wizard who explained to me how to
do this stuff.

1) Install `devkitpro`. They have a graphical installer for Windows, or you can
   use pacman or whatever for linux things I guess. The goal here, among other
   things, is to have a `binutils` setup that's targeting `arm-none-eabi`. We'll
   also use some of their tools that are specific to GBA development so if you
   for some reason already have the appropriate `binutils` then you probably
   still want devkitpro.
   * On Windows you'll want something like `C:\devkitpro\devkitARM\bin` and
     `C:\devkitpro\tools\bin` to be added to your PATH. I'm not sure on the
     directories for other systems. If you know then file a PR with the info.

2) Next we use `cargo install cargo-xbuild` to get that all setup.

3) Create a binary project. We're going to want nightly rust for this, so if you
   don't already have it set to default to nightly you should [set that
   up](https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file) for
   this project.

4) Clone this repo. It has an appropriate `main.rs` that will draw three test
   dots as well as other support files:
  * crt0.s
  * linker.ld
  * thumbv4-none-eabi.json
  * build.rs

5) Run `arm-none-eabi-as crt0.s -o crt0.o` to build the `crt0.s` into a `crt0.o`
   file. You could theoretically to it only when `crt0.s` changes, but in out
   `build.bat` file it's set to simply run every single time because it's a
   cheap enough operation.

6) Build with `cargo xbuild --target thumbv4-none-eabi.json`
  * The file extension is significant, and `cargo xbuild` takes it as a flag to
    compile dependencies with the same sysroot, so you can include crates
    normally. Well, crates that can run inside a GBA at least (Which means they
    have to be `no_std`, and even `no_alloc`).
  * This generates an ELF binary that some emulators can run directly (which is
    helpful because it has debug symbols).

7) Also you can patch up the output to be a "real" ROM file:
  * `arm-none-eabi-objcopy -O binary target/thumbv4-none-eabi/debug/gbatest target/output.gba`
  * `gbafix target/output.gba`

8) Alternately, you can use the provided `build.bat` file (or write a similar
   `build.sh` file of course), which does all four steps above.

9) Time to read the [Tonc](https://www.coranac.com/tonc/text/toc.htm) tutorial
   and convert all the C code you see into rust code.
