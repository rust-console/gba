# Development Setup

Before you can build a GBA game you'll have to follow some special steps to
setup the development environment.

Once again, extra special thanks to **Ketsuban**, who first dove into how to
make this all work with rust and then shared it with the world.

## Per System Setup

Obviously you need your computer to have a [working rust
installation](https://rustup.rs/). However, you'll also need to ensure that
you're using a nightly toolchain (we will need it for inline assembly, among
other potential useful features). You can run `rustup default nightly` to set
nightly as the system wide default toolchain, or you can use a [toolchain
file](https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file) to use
nightly just on a specific project, but either way we'll be assuming the use of
nightly from now on. You'll also need the `rust-src` component so that
`cargo-xbuild` will be able to compile the core crate for us in a bit, so run
`rustup component add rust-src`.

Next, you need [devkitpro](https://devkitpro.org/wiki/Getting_Started). They've
got a graphical installer for Windows that runs nicely, and I guess `pacman`
support on Linux (I'm on Windows so I haven't tried the Linux install myself).
We'll be using a few of their general binutils for the `arm-none-eabi` target,
and we'll also be using some of their tools that are specific to GBA
development, so _even if_ you already have the right binutils for whatever
reason, you'll still want devkitpro for the `gbafix` utility.

* On Windows you'll want something like `C:\devkitpro\devkitARM\bin` and
  `C:\devkitpro\tools\bin` to be [added to your
  PATH](https://stackoverflow.com/q/44272416/455232), depending on where you
  installed it to and such.
* On Linux you can use pacman to get it, and the default install puts the stuff
  in `/opt/devkitpro/devkitARM/bin` and `/opt/devkitpro/tools/bin`. If you need
  help you can look in our repository's
  [.travis.yml](https://github.com/rust-console/gba/blob/master/.travis.yml)
  file to see exactly what our CI does.

Finally, you'll need `cargo-xbuild`. Just run `cargo install cargo-xbuild` and
cargo will figure it all out for you.

## Per Project Setup

Once the system wide tools are ready, you'll need some particular files each
time you want to start a new project. You can find them in the root of the
[rust-console/gba repo](https://github.com/rust-console/gba).

* `thumbv4-none-agb.json` describes the overall GBA to cargo-xbuild (and LLVM)
  so it knows what to do. Technically the GBA is `thumbv4-none-eabi`, but we
  change the `eabi` to `agb` so that we can distinguish it from other `eabi`
  devices when using `cfg` flags.
* `crt0.s` describes some ASM startup stuff. If you have more ASM to place here
  later on this is where you can put it. You also need to build it into a
  `crt0.o` file before it can actually be used, but we'll cover that below.
* `linker.ld` tells the linker all the critical info about the layout
  expectations that the GBA has about our program, and that it should also
  include the `crt0.o` file with our compiled rust code.

## Compiling

Once all the tools are in place, there's particular steps that you need to
compile the project. For these to work you'll need some source code to compile.
Unlike with other things, an empty main file and/or an empty lib file will cause
a total build failure, because we'll need a
[no_std](https://rust-embedded.github.io/book/intro/no-std.html) build, and rust
defaults to builds that use the standard library. The next section has a minimal
example file you can use (along with explanation), but we'll describe the build
steps here.

* `arm-none-eabi-as crt0.s -o target/crt0.o`
  * This builds your text format `crt0.s` file into object format `crt0.o`
    that's placed in the `target/` directory. Note that if the `target/`
    directory doesn't exist yet it will fail, so you have to make the directory
    if it's not there. You don't need to rebuild `crt0.s` every single time,
    only when it changes, but you might as well throw a line to do it every time
    into your build script so that you never forget because it's a practically
    instant operation anyway.

* `cargo xbuild --target thumbv4-none-agb.json`
  * This builds your Rust source. It accepts _most of_ the normal options, such
    as `--release`, and options, such as `--bin foo` or `--examples`, that you'd
    expect `cargo` to accept.
  * You **can not** build and run tests this way, because they require `std`,
    which the GBA doesn't have. If you want you can still run some of your
    project's tests with `cargo test --lib` or similar, but that builds for your
    local machine, so anything specific to the GBA (such as reading and writing
    registers) won't be testable that way. If you want to isolate and try out
    some piece code running on the GBA you'll unfortunately have to make a demo
    for it in your `examples/` directory and then run the demo in an emulator
    and see if it does what you expect.
  * The file extension is important! It will work if you forget it, but `cargo
    xbuild` takes the inclusion of the extension as a flag to also compile
    dependencies with the same sysroot, so you can include other crates in your
    build. Well, crates that work in the GBA's limited environment, but you get
    the idea.

At this point you have an ELF binary that some emulators can execute directly
(more on that later). However, if you want a "real" ROM that works in all
emulators and that you could transfer to a flash cart to play on real hardware
there's a little more to do.

* `arm-none-eabi-objcopy -O binary target/thumbv4-none-agb/MODE/BIN_NAME target/ROM_NAME.gba`
  * This will perform an [objcopy](https://linux.die.net/man/1/objcopy) on our
    program. Here I've named the program `arm-none-eabi-objcopy`, which is what
    devkitpro calls their version of `objcopy` that's specific to the GBA in the
    Windows install. If the program isn't found under that name, have a look in
    your installation directory to see if it's under a slightly different name
    or something.
  * As you can see from reading the man page, the `-O binary` option takes our
    lovely ELF file with symbols and all that and strips it down to basically a
    bare memory dump of the program.
  * The next argument is the input file. You might not be familiar with how
    `cargo` arranges stuff in the `target/` directory, and between RLS and
    `cargo doc` and stuff it gets kinda crowded, so it goes like this:
    * Since our program was built for a non-local target, first we've got a
      directory named for that target, `thumbv4-none-agb/`
    * Next, the "MODE" is either `debug/` or `release/`, depending on if we had
      the `--release` flag included. You'll probably only be packing release
      mode programs all the way into GBA roms, but it works with either mode.
    * Finally, the name of the program. If your program is something out of the
      project's `src/bin/` then it'll be that file's name, or whatever name you
      configured for the bin in the `Cargo.toml` file. If your program is
      something out of the project's `examples/` directory there will be a
      similar `examples/` sub-directory first, and then the example's name.
  * The final argument is the output of the `objcopy`, which I suggest putting
    at just the top level of the `target/` directory. Really it could go
    anywhere, but if you're using git then it's likely that your `.gitignore`
    file is already setup to exclude everything in `target/`, so this makes sure
    that your intermediate game builds don't get checked into your git.

* `gbafix target/ROM_NAME.gba`
  * The `gbafix` tool also comes from devkitpro. The GBA is very picky about a
    ROM's format, and `gbafix` patches the ROM's header and such so that it'll
    work right. Unlike `objcopy`, this tool is custom built for GBA development,
    so it works just perfectly without any arguments beyond the file name. The
    ROM is patched in place, so we don't even need to specify a new destination.

And you're _finally_ done!

Of course, you probably want to make a script for all that, but it's up to you.
On our own project we have it mostly set up within a `Makefile.toml` which runs
using the [cargo-make](https://github.com/sagiegurari/cargo-make) plugin.

## Checking Your Setup

As I said, you need some source code to compile just to check that your
compilation pipeline is working. Here's a sample file that just puts three dots
on the screen without depending on any crates or anything at all.

`hello_magic.rs`:

```rust
#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x400_0000 as *mut u16).write_volatile(0x0403);
    (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x600_0000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}
```

Throw that into your project skeleton, build the program, and give it a run in
an emulator. I suggest [mgba](https://mgba.io/2019/01/26/mgba-0.7.0/), it has
some developer tools we'll use later on. You should see a red, green, and blue
dot close-ish to the middle of the screen. If you don't, something _already_
went wrong. Double check things, phone a friend, write your senators, try asking
`Lokathor` or `Ketsuban` on the [Rust Community
Discord](https://discordapp.com/invite/aVESxV8), until you're eventually able to
get your three dots going.

Of course, I'm sure you want to know why those particular numbers are the
numbers to use. Well that's what the whole rest of the book is about!
