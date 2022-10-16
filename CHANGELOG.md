# Changelog

* **0.9.0:** (unreleased)
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
