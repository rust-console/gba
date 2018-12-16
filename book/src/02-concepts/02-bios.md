# BIOS

* **Address Span:** `0x0` to `0x3FFF` (16k)

The [BIOS](https://en.wikipedia.org/wiki/BIOS) of the GBA is a small read-only
portion of memory at the very base of the address space. However, it is also
hardware protected against reading, so if you try to read from BIOS memory when
the program counter isn't pointed into the BIOS (eg: any time code _you_ write
is executing) then you get [basically garbage
data](https://problemkaputt.de/gbatek.htm#gbaunpredictablethings) back.

So we're not going to spend time here talking about what bits to read or write
within BIOS memory like we do with the other sections. Instead we're going to
spend time talking about [inline
assembly](https://doc.rust-lang.org/unstable-book/language-features/asm.html)
([tracking issue](https://github.com/rust-lang/rust/issues/29722)) and then use
it to call the [GBA BIOS
Functions](https://problemkaputt.de/gbatek.htm#biosfunctions).

Note that BIOS calls have _more overhead than normal function calls_, so don't
go using them all over the place if you don't have to. They're also usually
written more to be compact in terms of code than for raw speed, so you actually
can out speed them in some cases. Between the increased overhead and not being
as speed optimized, you can sometimes do a faster job without calling the BIOS
at all. (TODO: investigate more about  what parts of the BIOS we could
potentially offer faster alternatives for.)

I'd like to take a moment to thank [Marc Brinkmann](https://github.com/mbr)
(with contributions from [Oliver Schneider](https://github.com/oli-obk) and
[Philipp Oppermann](https://github.com/phil-opp)) for writing [this blog
post](http://embed.rs/articles/2016/arm-inline-assembly-rust/). It's at least
ten times the tutorial quality as the `asm` entry in the Unstable Book has. In
fairness to the Unstable Book, the actual spec of how inline ASM works in rust
is "basically what clang does", and that's specified as "basically what GCC
does", and that's basically/shockingly not specified much at all despite GCC
being like 30 years old.

So let's be slow and pedantic about this process.

## Inline ASM

**Fair Warning:** Inline asm is one of the least stable parts of Rust overall,
and if you write bad things you can trigger internal compiler errors and panics
and crashes and make LLVM choke and die without explanation. If you write some
inline asm and then suddenly your program suddenly stops compiling without
explanation, try commenting out that whole inline asm use and see if it's
causing the problem. Double check that you've written every single part of the
asm call absolutely correctly, etc, etc.

**Bonus Warning:** The general information that follows regarding the asm macro
is consistent from system to system, but specific information about register
names, register quantities, asm instruction argument ordering, and so on is
specific to ARM on the GBA. If you're programming for any other device you'll
need to carefully investigate that before you begin.

Now then, with those out of the way, the inline asm docs describe an asm call as
looking like this:

```rust
asm!(assembly template
   : output operands
   : input operands
   : clobbers
   : options
   );
```

And once you stick a lot of stuff in there it can _absolutely_ be hard to
remember the ordering of the elements. So we'll start with a code block that
has some comments thrown in on each line:

```rust
asm!(/* ASM */ TODO
    :/* OUT */ TODO
    :/* INP */ TODO
    :/* CLO */ TODO
    :/* OPT */
);
```

Now we have to decide what we're gonna write. Obviously we're going to do some
instructions, but those instructions use registers, and how are we gonna talk
about them? We've got two choices.

1) We can pick each and every register used by specifying exact register names.
   In THUMB mode we have 8 registers available, named `r0` through `r7`. If you
   switch into 32-bit mode there's additional registers that are also available.

2) We can specify slots for registers we need and let LLVM decide. In this style
   you name your slots `$0`, `$1` and so on. Slot numbers are assigned first to
   all specified outputs, then to all specified inputs, in the order that you
   list them.

In the case of the GBA BIOS, each BIOS function has pre-designated input and
output registers, so we will use the first style. If you use inline ASM in other
parts of your code you're free to use the second style.

### Assembly

This is just one big string literal. You write out one instruction per line, and
excess whitespace is ignored. You can also do comments within your assembly
using `;` to start a comment that goes until the end of the line.

Assembly convention doesn't consider it unreasonable to comment potentially as
much as _every single line_ of asm that you write when you're getting used to
things. Or even if you are used to things. This is cryptic stuff, there's a
reason we avoid writing in it as much as possible.

Remember that our Rust code is in 16-bit mode. You _can_ switch to 32-bit mode
within your asm as long as you switch back by the time the block ends. Otherwise
you'll have a bad time.

### Outputs

A comma separated list. Each entry looks like

* `"constraint" (binding)`

An output constraint starts with a symbol:

* `=` for write only
* `+` for reads and writes
* `&` for for "early clobber", meaning that you'll write to this at some point
  before all input values have been read. It prevents this register from being
  assigned to an input register.

Followed by _either_ the letter `r` (if you want LLVM to pick the register to
use) or curly braces around a specific register (if you want to pick).

* The binding can be any 32-bit sized binding in scope (`i32`, `u32`, `isize`,
  `usize`, etc).
* If your binding has bit pattern requirements ("must be non-zero", etc) you are
  responsible for upholding that.
* If your binding type will try to `Drop` later then you are responsible for it
  being in a fit state to do that.
* The binding must be either a mutable binding or a binding that was
  pre-declared but not yet assigned.

Anything else is UB.

### Inputs

This is a similar comma separated list.

* `"constraint" (binding)`

An input constraint doesn't have the symbol prefix, you just pick either `r` or
a named register with curly braces around it.

* An input binding must be 32-bit sized.
* An input binding _should_ be a type that is `Copy` but this is not an absolute
  requirement. Having the input be read is semantically similar to using
  `core::ptr::read(&binding)` and forgetting the value when you're done.

### Clobbers

Sometimes your asm will touch registers other than the ones declared for input
and output. 

Clobbers are declared as a comma separated list of string literals naming
specific registers. You don't use curly braces with clobbers.

LLVM _needs_ to know this information. It can move things around to keep your
data safe, but only if you tell it what's about to happen.

Failure to define all of your clobbers can cause UB.

### Options

There's only one option we'd care to specify, and we don't even always need it.
That option is "volatile".

Just like with a function call, LLVM will skip a block of asm if it doesn't see
that any outputs from the asm were used later on. A lot of our BIOS calls will
need to be declared "volatile" because to LLVM they don't seem to do anything.

### BIOS ASM

* Inputs are always `r0`, `r1`, `r2`, and/or `r3`, depending on function.
* Outputs are always zero or more of `r0`, `r1`, and `r3`.
* Any of the output registers that aren't actually used should be marked as
  clobbered.
* All other registers are unaffected.

All of the GBA BIOS calls are performed using the
[swi](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0068b/BABFCEEG.html)
instruction, combined with a value depending on what BIOS function you're trying
to invoke. If you're in 16-bit code you use the value directly, and if you're in
32-bit mode you shift the value up by 16 bits first.

### Example BIOS Function: Division

The GBA doesn't have hardware division. You have to do it in software.

We could potentially implement this in Rust (we might get around to trying that,
I was even sent [a link to a
paper](https://www.microsoft.com/en-us/research/wp-content/uploads/2008/08/tr-2008-141.pdf)
that I promptly did not actually read right away), or you can call the BIOS to
do it for you and trust that big N did a good enough job.

GBATEK gives a fairly clear explanation of our inputs and outputs:

```txt
Signed Division, r0/r1.
  r0  signed 32bit Number
  r1  signed 32bit Denom
Return:
  r0  Number DIV Denom ;signed
  r1  Number MOD Denom ;signed
  r3  ABS (Number DIV Denom) ;unsigned
For example, incoming -1234, 10 should return -123, -4, +123.
The function usually gets caught in an endless loop upon division by zero.
```

The math folks tell me that the `r1` value should be properly called the
"remainder" not the "modulus". We'll go with that for our function, doesn't hurt
to use the correct names. The function itself is an assert against dividing by
`0`, then we name some bindings _without_ giving them a value, we make the asm
call, and then return what we got.

```rust
pub fn div_rem(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  let div_out: i32;
  let rem_out: i32;
  unsafe {
    asm!(/* ASM */ "swi 0x06"
        :/* OUT */ "={r0}"(div_out), "={r1}"(rem_out)
        :/* INP */ "{r0}"(numerator), "{r1}"(denominator)
        :/* CLO */ "r3"
        :/* OPT */
    );
  }
  (div_out, rem_out)
}
```

I _hope_ this makes sense by now.
