#!/bin/sh

if [ "$1" = "" ]; then
  echo "Usage: $0 [example to build]"
  exit 1
fi

cargo build --example $1 --release || exit 1
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/examples/$1 target/$1.gba || exit 1
gbafix target/$1.gba || exit 1
echo "ROM built successfully!"
