#!/bin/sh
cargo build --examples --verbose
arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std target/thumbv4t-none-eabi/debug/examples/hello >target/dump-hello.txt
arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std target/thumbv4t-none-eabi/debug/examples/asm_scratch >target/dump-asm_scratch.txt
