cargo build --examples

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/basic_keyinput >target/ex-basic_keyinput.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/do_nothing >target/ex-do_nothing.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/mode0 >target/ex-mode0.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/mode3 >target/ex-mode3.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/mode4 >target/ex-mode4.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/mode5 >target/ex-mode5.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/paddle_ball >target/ex-paddle_ball.txt

arm-none-eabi-objdump --headers --disassemble --demangle --architecture=armv4t --no-show-raw-insn -Mreg-names-std --visualize-jumps target/thumbv4t-none-eabi/debug/examples/timer >target/ex-timer.txt
