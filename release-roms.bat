cargo build --examples --release

arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/examples/hello target/hello.gba
gbafix -p -thello -cHELO -mRS target/hello.gba

arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/examples/game target/game.gba
gbafix -p -tgame -cGAME -mRS target/game.gba
