//! Explanation of required setup per development machine.
//!
//! ## GNU Binutils
//!
//! The linker that comes with Rust (part of the LLVM utils) is capable of
//! linking a GBA rom with no special steps. However, the *other* LLVM binutils
//! do not all understand the "interworking" code that exists on the GBA and
//! similar old ARM targets.
//!
//! If you want to be able to dump the assembly of a compiled file and have it
//! all be readable, you will need to get the GNU objdump that's available as
//! part of the GNU binutils. If you just use the LLVM objdump then any
//! functions using `a32` code will get disassembled as complete nonsense.
//!
//! The GNU binutils are configured for each target family separately, and in
//! this case we want the binutils for `arm-none-eabi`. To get an appropriate
//! set of utilities you can go to the [ARM GNU Toolchain] website (Windows,
//! Mac, and Linux downloads), or if you're on Linux you can probably find it in
//! your package manager under a name like `binutils-arm-none-eabi` or something
//! like that.
//!
//! [ARM GNU Toolchain]:
//!     https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain
//!
//! ## Nightly Rust Toolchain
//!
//! You'll need a [Nightly channel][channels] of Rust available, because we'll
//! need to use the `build-std` nightly ability of `cargo`.
//!
//! [channels]: https://rust-lang.github.io/rustup/concepts/channels.html
//!
//! ## Rust Source
//!
//! You'll need to have the `rust-src` component from `rustup` installed. This
//! is also used by `build-std`.
//!
//! ```sh
//! rustup component add rust-src
//! ```
