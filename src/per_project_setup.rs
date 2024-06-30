//! Explanation of suggested setup per GBA project.
//!
//! ## Cargo Configuration
//!
//! We need to use cargo's unstable `build-std` ability to build the
//! `thumbv4t-none-eabi` target, and we'll want to have that be the default
//! target for basically everything we do.
//!
//! You'll want a GBA emulator for testing your GBA rom. I use
//! [mGBA](https://mgba.io/), but others also have success with
//! [No$GBA](https://www.nogba.com/). Whatever you pick, you should set it as
//! the `runner` under `[target.thumbv4t-none-eabi]`
//!
//! As discussed in the [Per System Setup][crate::per_system_setup], only the
//! GNU `objdump` will be able to correctly dump a GBA program. It's possible to
//! use the LLVM linker and only GNU for `objdump`, but the GNU `objdump` won't
//! be able to show all info if you mix utilities like this, because (i guess?)
//! the GNU `objdump` doesn't understand LLVM linker info format. Using the GNU
//!  linker will make sure you can see all the debug info. I've never noticed a
//! linking speed difference between LLVM and GNU on the GBA, programs just
//! don't get that big. Put `-Clinker=arm-none-eabi-ld` into the `rustflags` for
//! our target and we'll get the GNU linker.
//!
//! Regardless of if you're using the LLVM or GNU linker, you'll need a linker
//! script. The [Github Repo][github_script] for this project has a usable
//! linker script. You should save this file into your own project, such as in a
//! folder called `linker_scripts/` under the name `mono_boot.ld`. Then we add a
//! `rustflags` argument with `-Clink-arg=-T<path>`, using the script's path
//! relative to our project root.
//!
//! You can also set `-Ctarget-cpu=arm7tdmi`, because that is the specific CPU
//! of the GBA, though I'm not sure that LLVM really does much with the
//! information. In my very limited testing, I didn't see any different code
//! generated when setting the CPU compared to not doing it, but I do it just in
//! case it helps somehow, because it doesn't hurt.
//!
//! [github_script]:
//!     https://github.com/rust-console/gba/blob/main/linker_scripts/mono_boot.ld
//!
//! Your `.cargo/config.toml` should probably look like the following:
//! ```toml
//! [build]
//! target = "thumbv4t-none-eabi"
//!
//! [unstable]
//! build-std = ["core"]
//!
//! [target.thumbv4t-none-eabi]
//! runner = ["mgba-qt", "-C", "logToStdout=1", "-C", "logLevel.gba.debug=127"]
//! rustflags = [
//!   "-Zub-checks=no",
//!   "-Clinker=arm-none-eabi-ld",
//!   "-Clink-arg=-Tlinker_scripts/mono_boot.ld",
//!   "-Ctarget-cpu=arm7tdmi",
//! ]
//! ```
//!
//! * If you want the `compiler_builtins` crate to provide weak intrinsics so
//!   that you can override them yourself, you can set `build-std-features =
//!   ["compiler-builtins-weak-intrinsics"]` in the `[unstable]` section. This
//!   is not normally needed, but if you some day try to write your own
//!   intrinsics impls you'll want to know it's available.
//!
//! ## Rust Analyzer
//!
//! The `test` crate won't be available when using `build-std` like this. To
//! make Rust Analyzer work without constantly complaining about a missing
//! `test` crate, add this to your `.vscode/settings.json` (or whatever similar
//! file in your editor of choice)
//!
//! ```json
//! {
//!   "rust-analyzer.cargo.allTargets": false,
//!   "rust-analyzer.check.command": "build",
//!   "rust-analyzer.check.extraArgs": [
//!     "--lib",
//!     "--bins",
//!     "--examples"
//!   ]
//! }
//! ```
//!
//! ## `no_std` and `no_main`
//!
//! The full standard library isn't available on the GBA, it's a [bare metal]
//! environment, so we'll only have access to the [`core`] crate.
//!
//! * You'll need to use the [`#![no_std]`][no_std] and [`#![no_main]`][no_main]
//!   attributes on your binary.
//! * You'll also need to declare a [panic handler][panic_handler]. You can do
//!   this yourself or you can use the [`panic_handler!`][crate::panic_handler]
//!   macro from this crate to type a little less.
//! * You need to provide a correct `main` function for the assembly runtime to
//!   call once it has made the GBA ready for your Rust program. It needs to be
//!   an `extern "C" fn` named `main` (using [`#![no_mangle]`][no_mangle]), and
//!   which takes no arguments and never returns (`-> !`).
//!
//! Here's a minimal example:
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! gba::panic_handler!(empty_loop);
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!   loop {}
//! }
//! ```
//!
//! [bare metal]:
//!     https://docs.rust-embedded.org/book/intro/no-std.html#bare-metal-environments
//! [no_std]:
//!     https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
//! [no_main]:
//!     https://doc.rust-lang.org/reference/crates-and-source-files.html#the-no_main-attribute
//! [no_mangle]: https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute
//! [panic_handler]: https://doc.rust-lang.org/nomicon/panic-handler.html
