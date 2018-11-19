
@rem Build the crt0 file before we begin
@if not exist ".\target" mkdir target
arm-none-eabi-as crt0.s -o target/crt0.o

@rem Build all examples, both debug and release
cargo xbuild --examples --target thumbv4-none-agb.json
cargo xbuild --examples --target thumbv4-none-agb.json --release

@echo Packing examples into ROM files...
@for %%I in (.\examples\*.*) do @(
  echo %%~nI
  arm-none-eabi-objcopy -O binary target/thumbv4-none-agb/release/examples/%%~nI target/example-%%~nI.gba >nul
  gbafix target/example-%%~nI.gba >nul
)
