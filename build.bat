@echo off
REM It could work to only rebuild the `crt0.o` file when `crt0.s` actually
REM changes, but it's actually a super cheap operation so we'll just do it
REM every single time to avoid any mix ups.
@echo on

arm-none-eabi-as crt0.s -o crt0.o

@echo off
REM This builds our program for the GBA. Note that the extension here is
REM important, because it causes all crates that we might import to also
REM use the correct target.
@echo on

cargo xbuild --target thumbv4-none-eabi.json

@echo off
REM Some emulators can use cargo's output directly (which is cool, because then
REM you can keep debug symbols and stuff), but to make a "real" ROM we have to
REM also use the devkitpro tools to patch up the file a bit.
@echo on

arm-none-eabi-objcopy -O binary target/thumbv4-none-eabi/debug/main target/output.gba
gbafix target/output.gba

@echo off
REM Now all the same for release mode too!
@echo on

cargo xbuild --target thumbv4-none-eabi.json --release
arm-none-eabi-objcopy -O binary target/thumbv4-none-eabi/release/main target/output-release.gba
gbafix target/output-release.gba
