cargo build --examples
arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std target/thumbv4t-none-eabi/debug/examples/hello >target/dump-hello.txt
