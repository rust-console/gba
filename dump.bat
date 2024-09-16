cargo build --examples
arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/mode3_pong_example_game >target/dump-mode3_pong_example_game.txt
