# Changelog

* **0.9.3:**
  * Added `as_u32_slice` and `as_u16_slice` to `Align4`.
  * *Removed* the requirement for inputs to `include_aligned_bytes!` to be a
    multiple of 4 bytes.
  * Added `as_usize` to all the screeblock address types.
* **0.9.2:**
  * Adds support for more BIOS functions, though not all functions are as
    clearly documented as I'd like.
  * Made much more of the `Fixed` type const friendly. Most ops now have an
    inherent method that is `const fn` as well as implementing the `core::ops`
    trait (the trait fn just calls the inherent fn). This means that you can't
    do `x + y` in a const context, but you can do `x.add(y)`. This is not the
    best system, but until const trait impls are stable this is the best middle
    ground.
* **0.9.1:**
  * Adds some randomization support directly into the crate.
  * Added more methods to the `Fixed` type.
  * Adds an `include_aligned_bytes!` macro to pull in compile time data that's
    aligned to 4.
* **0.9.0:**
  * **MSRV:** The crate now requires `compiler_builtins-0.1.81` to build. You
    will need a Nightly from 2022-10-15 or later.
  * **Break:** Quite a bit of the video interface has been replaced, but it
    should be much easier to use now.
  * **Break:** The timer interface has been updated so that fields more closely
    match the mGBA names, for ease of debugging.
* **0.8.0:**
  * **Break:** Removed the macros for `GbaCell` access in favor of just methods.
    I had at first thought that they'd assign registers and then inline, but it
    turns out that the inline phase happens way before the register assignment
    phase, so the macros were unnecessary (and clunky).
  * **Break:** The `IrqFn` type is changed to pass the function an `IrqBits`
    instead of a bare `u16`.
  * Adds functions to pick a screenblock location (one for each screenblock
    type).
  * Add `BitUnPack` BIOS function.
  * Add the `CGA_8X8_THICK` art data.
  * Greatly improved documentation.
* **0.7.4:** Adds mGBA logging support.
* **0.7.3:** Fixes "multiple definition" errors with the AEABI division functions.
  Filed a PR to fix this soon:
  https://github.com/rust-lang/compiler-builtins/pull/495
* **0.7.2:** First version that configures docs.rs properly!
