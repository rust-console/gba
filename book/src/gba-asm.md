# GBA Assembly

On the GBA sometimes you just end up using assembly. Not a whole lot, but
sometimes. Accordingly, you should know how assembly works on the GBA.

* The [ARM Infocenter:
  ARM7TDMI](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0210c/index.html)
  is the basic authority for reference information. The GBA has a CPU with the
  `ARMv4` ISA, the `ARMv4T` variant, and specifically the `ARM7TDMI`
  microarchitecture. Someone at ARM decided that having both `ARM#` and `ARMv#`
  was a good way to [version things](https://en.wikichip.org/wiki/arm/versions),
  even when the numbers don't match. The rest of us have been sad ever since.
  The link there will take you to the correct book specific to the GBA's
  microarchitecture. There's a whole big pile of ARM books available within the
  ARM Infocenter, so if you just google it or whatever make sure you end up
  looking at the correct one. Note that there is also a [PDF
  Version](http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf)
  of the documentation available, if you'd like that.

* In addition to the `ARM7TDMI` book, which is specific to the GBA's CPU, you'll
  need to find a copy of the ARM Architecture Reference Manual if you want
  general ARM knowledge. The ARM Infocenter has the
  [ARMv5](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0100i/index.html)
  version of said manual hosted on their site. Unfortunately, they don't seem to
  host the `ARMv4T` version of the manual any more.

* The [GBATek: ARM CPU
  Overview](https://problemkaputt.de/gbatek.htm#armcpuoverview) also has quite a
  bit of info. Some of it is a duplication of what you'd find in the ARM
  Infocenter reference manuals. Some of it is information that's specific to the
  GBA's layout and how the CPU interacts with other parts (such as how its
  timings and the display adapter's timings line up). Some of it is specific to
  the ARM chips _within the DS and DSi_, so be careful to make sure that you
  don't wander into the wrong section. GBATEK is always a bit of a jumbled mess,
  and the explanations are often "sparse" (to put it nicely), so I'd advise that
  you also look at the official ARM manuals.

* The [Compiler Explorer](https://rust.godbolt.org/z/ndCnk3) can be used to
  quickly look at assembly versions of your Rust code. That link there will load
  up an essentially blank `no_std` file with `opt-level=3` set and targeting
  `thumbv6m-none-eabi`. That's _not_ the same target as the GBA (it's two ISA
  revisions later, `ARMv6` instead of `ARMv4`), but it's the closest CPU target
  that is bundled with `rustc`, so it's the closest you can get with the
  compiler explorer website. If you're very dedicated I suppose you could setup
  a [local
  instance](https://github.com/mattgodbolt/compiler-explorer#running-a-local-instance)
  of compiler explorer and then add the extra target definition and so on, but
  that's _probably_ overkill.

## ARM and Thumb

The "T" part in `ARMv4T` and `ARM7TDMI` means "Thumb". An ARM chip that supports
Thumb has two different instruction sets instead of just one. The chip can run
in ARM state with 32-bit instructions, or it can run in Thumb state with 16-bit
instructions. Note that the CPU _state_ (ARM or Thumb) is distinct from the
_mode_ (User, FIQ, IRQ, etc). Apparently these states are sometimes called
`a32` and `t32` in a more modern context, but I will stick with ARM and Thumb
because that's what the official ARM7TDMI manual and GBATEK both use.

On the GBA, the memory bus that physically transfers data from the cartridge into
the device is a 16-bit memory bus. This means that if you need to transfer more
than 16 bits at a time you have to do more than one transfer. Since we'd like
our instructions to get to the CPU as fast as possible, we compile the majority
of our program with the Thumb instruction set. The ARM reference says that with
Thumb instructions on a 16-bit memory bus system you get about 160% performance
compared to using ARM instructions. That's absolutely something we want to take
advantage of. Also, your Thumb compiled code is about 65% of the same code
compiled with ARM. Since a game ROM can only be 32MB total, and we're trying to
fit in images and sound too, we want to get space savings where we can.

You may wonder, why is the Thumb code 65% as large if the instructions
themselves are 50% as large, and why have ARM state at all if there's such a
benefit to be had with Thumb? Well, Thumb state doesn't support as many different
instructions as ARM state does. Some lines of source code that can compile to a
single ARM instruction might need to compile into more than one Thumb
instruction. Thumb still has most of the really good instructions available, so
it all averages out to about 65%.

That said, some parts of a GBA program _must_ be written for ARM state. Also,
ARM state does allow that increased instruction flexibility. So we _need_ to use
ARM some of the time, and we might just _want_ to use ARM even when we don't
need to at other times. It is possible to switch states on the fly, there's
extremely minimal overhead, even less than doing some function calls. The only
problem is the 16-bit memory bus of the cartridge giving us a needless speed
penalty with our ARM code. The CPU _executes_ the ARM instructions at full
speed, but then it has to wait while more instructions get sent in. What do we
do? Well, code is ultimately just a different kind of data. We can copy parts of
our code off the cartridge ROM and place it into a part of the RAM that has a
32-bit memory bus. Then the CPU can execute the code from there, going at full
speed. Of course, there's only a very small amount of RAM compared to the size
of a cartridge, so we'll only do this with a few select functions. Exactly which
functions will probably depend on your game.

There's two problems that we face as Rust programmers:

1) Rust offers no way to specify individual functions as being ARM or Thumb. The
   whole program is compiled for one state or the other. Obviously this is no
   good, so it's on the [2019 embedded
   wishlist](https://github.com/rust-embedded/wg/issues/256#issuecomment-439677804),
   and perhaps a fix will come.

2) Rust offers no way to get a pointer to a function as well as the length of
   the compiled function, so we can't copy a function from the ROM to some other
   location because we can't even express statements about the function's data.
   I also put this [on the
   wishlist](https://github.com/rust-embedded/wg/issues/256#issuecomment-450539836),
   but honestly I have much less hope that this becomes a part of rust.

What this ultimately means is that some parts of our program have to be written
in external assembly files and then added to the program with the linker. We
were already going to write some assembly, and we already use more than one file
in our project all the time, those parts aren't a big problem. The big problem
is that using custom linker scripts to get assembly code into our final program
isn't transitive between crates.

What I mean is that once we have a file full of custom assembly that we're
linking in by hand, that's not "part of" the crate any more. At least not as
`cargo` sees it. So we can't just upload it to `crates.io` and then depend on it
in other projects and have `cargo` download the right version and and include it
all automatically. We're back to fully manually copying files from the old
project into the new one, adding more lines to the linker script each time we
split up a new assembly file, all that stuff. Like the stone age. Sometimes ya
gotta suffer for your art.
