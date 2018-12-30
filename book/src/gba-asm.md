# GBA Assembly

On the GBA sometimes you just end up using assembly. Not a whole lot, but
sometimes. Accordingly, you should know how assembly works on the GBA.

* The [ARM Infocenter:
  ARM7TDMI](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0210c/index.html)
  is the basic authority for reference information. The GBA has a CPU with the
  `ARMv4` ISA, the `ARMv4T` variant, and specifically the `ARM7TDMI`
  microarchitecture. Someone at ARM decided that having both `ARM#` and `ARMv#`
  was a good way to [version things](https://en.wikichip.org/wiki/arm/versions),
  even when the numbers don't match, and the rest of us have been sad ever
  since. The link there will take you to the correct book within the big pile of
  ARM books available within the ARM Infocenter. Note that there is also a [PDF
  Version](http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf)
  of the documentation available, if you'd like that.

* The [GBATek: ARM CPU
  Overview](https://problemkaputt.de/gbatek.htm#armcpuoverview) also has quite a
  bit of info. Most of it is somewhat a duplication of what you'd find in the
  ARM Infocenter reference manual, but it's also somewhat specialized towards
  the GBA's specifics. It's in the usual, uh, "sparse" style that GBATEK is
  written in, so I wouldn't suggest that read it first.

* The [Compiler Explorer](https://rust.godbolt.org/z/ndCnk3) can be used to
  quickly look at assembly output of your Rust code. That link there will load
  up an essentially blank `no_std` file with `opt-level=3` set and targeting
  `thumbv6m-none-eabi`. That's _not_ the same as the GBA (it's two ISA revisions
  later, ARMv6 instead of ARMv4), but it's the closest CPU target that ships
  with rustc, so it's the closest you can get with the compiler explorer
  website. If you're very dedicated I suppose you could setup a [local
  instance](https://github.com/mattgodbolt/compiler-explorer#running-a-local-instance)
  of compiler explorer and then add the extra target definition and so on, but
  that's _probably_ overkill.

## ARM and THUMB

The "T" part in `ARMv4T` and `ARM7TDMI` means "Thumb". An ARM chip that supports
Thumb mode has two different instruction sets instead of just one. The chip can
run in ARM mode with 32-bit instructions, or it can run in THUMB mode with
16-bit instructions. Apparently these modes are sometimes called `a32` and `t32`
in a more modern context, but I will stick with ARM and THUMB because that's
what other GBA references use (particularly GBATEK), and it's probably best to
be more in agreement with them than with stuff for Raspberry Pi programming or
whatever other modern ARM thing.

On the GBA, the memory bus that physically transfers data from the game pak into
the device is a 16-bit memory bus. This means that if you need to transfer more
than 16 bits at a time you have to do more than one transfer. Since we'd like
our instructions to get to the CPU as fast as possible, we compile the majority
of our program with the THUMB instruction set. The ARM reference says that with
THUMB instructions on a 16-bit memory bus system you get about 160% performance
compared to using ARM instructions. That's absolutely something we want to take
advantage of. Also, your THUMB compiled code is about 65% of the same code
compiled with ARM. Since a game ROM can only be 32MB total, and we're trying to
fit in images and sound too, we want to get space savings where we can.

You may wonder, why is the THUMB code 65% as large if the instructions
themselves are 50% as large, and why have ARM mode at all if there's such a
benefit to be had with THUMB? Well, THUMB mode doesn't support as many different
instructions as ARM mode does. Some lines of source code that can compile to a
single ARM instruction might need to compile into more than one THUMB
instruction. THUMB still has most of the really good instructions available, so
it all averages out to about 65%.

That said, some parts of a GBA program _must_ be written in ARM mode. Also, ARM
mode does allow that increased instruction flexibility. So we _need_ to use ARM
some of the time, and we might just _want_ to use ARM even when we don't need
to. It is possible to switch modes on the fly, there's extremely minimal
overhead, even less than doing some function calls. The only problem is the
16-bit memory bus of the game pak giving us a needless speed penalty with our
ARM code. The CPU _executes_ the ARM instructions at full speed, but then it has
to wait while more instructions get sent in. What do we do? Well, code is
ultimately just a different kind of data. We can copy parts of our code off the
game pak ROM and place it into a part of the RAM that has a 32-bit memory bus.
Then the CPU can execute the code from there, going at full speed. Of course,
there's only a very small amount of RAM compared to the size of a game pak, so
we'll only do this with a few select functions. Exactly which functions will
probably depend on your game.

One problem with this process is that Rust doesn't currently offer a way to mark
individual functions for being ARM or THUMB. The whole program is compiled in a
single mode. That's not an automatic killer, since we can use the `asm!` macro
to write some inline assembly, then within our inline assembly we switch from
THUMB to ARM, do some ARM stuff, and switch back to THUMB mode before the inline
assembly is over. Rust is none the wiser to what happened. Yeah, it's clunky,
that's why [it's on the 2019
wishlist](https://github.com/rust-embedded/wg/issues/256#issuecomment-439677804)
to fix it (then LLVM can manage it automatically for you).

The bigger problem is that when we do that all of our functions still start off
in THUMB mode, even if they temporarily use ARM mode. For the few bits of code
that must start _already in_ ARM mode, we're stuck. Those parts have to be
written in external assembly files and then included with the linker. We were
already going to write some assembly, and we already use more than one file in
our project all the time, those parts aren't a big problem. The big problem is
that using custom linker scripts isn't transitive between crates.

What I mean is that once we have a file full of custom assembly that we're
linking in by hand, that's not "part of" the crate any more. At least not as
`cargo` see it. So we can't just upload it to `crates.io` and then depend on it
in other projects and have `cargo` download the right version and and include it
all automatically. We're back to fully manually copying files from the old
project into the new one, adding more lines to the linker script each time we
split up a new assembly file, all that stuff. Like the stone age. Sometimes ya
gotta suffer for your art.
