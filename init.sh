#!/bin/sh

APP_NAME=$1
if [ -z "$APP_NAME"]; then APP_NAME="rust-console-hello"; fi

TARGET="thumbv4-none-agb"
CRT_LOCAL="crt0.s"

echo "Initializing rust-console gba at: $1"

# initialize cargo
cargo init $APP_NAME --bin

# remove old
rm -rf $APP_NAME/thumbv4-none-agb.json*
rm -rf $APP_NAME/$CRT_LOCAL*
rm -rf $APP_NAME/linker.ld*
rm -rf $APP_NAME/Makefile*
rm -rf $APP_NAME/README.md*

# download dependencies
wget https://raw.githubusercontent.com/rust-console/gba/master/thumbv4-none-agb.json
mv $TARGET.json $APP_NAME/$TARGET.json

wget https://raw.githubusercontent.com/rust-console/gba/master/crt0.s
mv crt0.s $APP_NAME/$CRT_LOCAL

wget https://raw.githubusercontent.com/rust-console/gba/master/linker.ld;
mv linker.ld $APP_NAME/linker.ld

# substitute cargo main file with a new basic one
rm -rf $APP_NAME/src/main.rs
wget https://raw.githubusercontent.com/rust-console/gba/master/examples/hello_world.rs
mv hello_world.rs $APP_NAME/src/main.rs

# precreate target directory for crt0.o file
mkdir $APP_NAME/target

# setup make file
echo -e "CRT_FILE=$(echo $CRT_LOCAL)
CRT_OUTPUT=target/crt0.o
PROJECT_NAME=$(echo $APP_NAME)
TARGET=$(echo $TARGET)
THUMB_TARGET=$(echo $TARGET).json

all: build

build:
\tarm-none-eabi-as \$(CRT_FILE) -o \$(CRT_OUTPUT)
\tcargo xbuild --target \$(THUMB_TARGET)

build-prod:
\tarm-none-eabi-as \$(CRT_FILE) -o \$(CRT_OUTPUT)
\tcargo xbuild --target \$(THUMB_TARGET) --release
\tarm-none-eabi-objcopy -O binary target/\$(TARGET)/release/\$(PROJECT_NAME) target/\$(PROJECT_NAME).gba
\tgbafix target/\$(PROJECT_NAME).gba
" > $APP_NAME/Makefile

# setup the readme file
echo -e "Rust console project
----

## Development

\`\`\`sh
make
\`\`\`
" > $APP_NAME/README.md
