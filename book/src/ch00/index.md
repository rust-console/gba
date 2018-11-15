# Chapter 0: Development Setup

Before you can build a GBA game you'll have to follow some special steps to
setup the development environment. Perhaps unfortunately, there's enough detail
here to warrant a mini-chapter all on its own.

Before we begin I'd like to give a special thanks to **Ketsuban**, who is the
wizard that arranged for all of this to be able to happen and laid out the
details of the plan to the rest of the world.

## Per System Setup

Obviously you need your computer to have a working rust installation. However,
you'll also need to ensure that you're using a nightly toolchain. You can run
`rustup default nightly` to set nightly as the system wide default toolchain, or
you can use a [toolchain
file](https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file) to use
nightly just on a specific project, but either way we'll be assuming nightly
from now on.

Next you need [devkitpro](https://devkitpro.org/wiki/Getting_Started). They've
got a graphical installer for Windows, and `pacman` support on Linux. We'll be
using a few of their binutils for the `arm-none-eabi` target, and we'll also be
using some of their tools that are specific to GBA development, so _even if_ you
already have the right binutils for whatever reason, you'll still want devkitpro
for the `gbafix` utility.

* On Windows you'll want something like `C:\devkitpro\devkitARM\bin` and
  `C:\devkitpro\tools\bin` to be [added to your
  PATH](https://stackoverflow.com/q/44272416/455232), depending on where you
  installed it to and such.
* On Linux you'll also want it to be added to your path, but if you're using
  Linux I'll just assume you know how to do all that.

Finally, you'll need `cargo-xbuild`. Just run `cargo install cargo-xbuild` and
cargo will figure it all out for you.

## Per Project Setup

Now you'll need some particular files each time you want to start a new project.
You can find them in the root of the [rust-console/gba
repo](https://github.com/rust-console/gba).

* `thumbv4-none-agb.json` describes the overall GBA to cargo-xbuild so it knows
  what to do. This is actually a somewhat made up target name since there's no
  official target name. The GBA is essentially the same as a normal
  `thumbv4-none-eabi` device, but we give it the "agb" signifier so that later
  on we'll be able to use rust's `cfg` ability to allow our code to know if it's
  specifically targeting a GBA or some other similar device (like an NDS).
* `crt0.s` describes some ASM startup stuff. If you have more ASM to place here
  later on this is where you can put it. You also need to build it into a
  `crt0.o` file before it can actually be used, but we'll cover that below.
* `linker.ld` tells the linker more critical info about the layout expectations
  that the GBA has about our program.

## Compiling

The next steps only work once you've got some source code to build. If you need
a quick test, copy the `hello1.rs` file from our examples directory in the
repository.

Once you've got something to build, you perform the following steps:

* `arm-none-eabi-as crt0.s -o crt0.o`
  * This builds your text format `crt0.s` file into object format `crt0.o`. You
    don't need to perform it every time, only when `crt0.s` changes, but you
    might as well do it every time so that you never forget to because it's a
    practically instant operation.

* `cargo xbuild --target thumbv4-none-agb.json`
  * This builds your Rust source. It accepts _most of_ the normal options, such
    as `--release`, and options, such as `--bin foo` or `--examples`, that you'd
    expect `cargo` to accept.
  * You **can not** build and run tests this way, because they require `std`,
    which the GBA doesn't have. You can still run some of your project's tests
    with `cargo test`, but that builds for your local machine, so anything
    specific to the GBA (such as reading and writing registers) won't be
    testable that way. If you want to isolate and try out some piece code
    running on the GBA you'll unfortunately have to make a demo for it in your
    `examples/` directory and then run the demo in an emulator and see if it
    does what you expect.
  * The file extension is important. `cargo xbuild` takes it as a flag to
    compile dependencies with the same sysroot, so you can include crates
    normally. Well, creates that work in the GBA's limited environment, but you
    get the idea.

At this point you have an ELF binary that some emulators can execute directly.
This is helpful because it'll have debug symbols and all that, assuming a debug
build. Specifically, [mgba 0.7 beta
1](https://mgba.io/2018/09/24/mgba-0.7-beta1/) can do it, and perhaps other
emulators can also do it.

However, if you want a "real" ROM that works in all emulators and that you could
transfer to a flash cart there's a little more to do.

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

And you're finally done!

Of course, you probably want to make a script for all that, but it's up to you.
