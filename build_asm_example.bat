arm-none-eabi-as -o target/asm_only.o --warn --fatal-warnings -mthumb-interwork -mcpu=arm7tdmi examples/asm_only.s

arm-none-eabi-ld --print-map -o target/asm_only.elf --script gba_link_script.ld target/asm_only.o

arm-none-eabi-objcopy --output-target binary target/asm_only.elf target/asm_only.gba

arm-none-eabi-objdump --demangle --headers --no-show-raw-insn -M reg-names-std -d target/asm_only.elf
