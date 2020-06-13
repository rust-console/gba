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
(with contributions from [Oliver Scherer](https://github.com/oli-obk) and
[Philipp Oppermann](https://github.com/phil-opp)) for writing [this blog
post](http://embed.rs/articles/2016/arm-inline-assembly-rust/). It's at least
ten times the tutorial quality as the `asm` entry in the Unstable Book has. In
fairness to the Unstable Book, the actual spec of how inline ASM works in rust
is "basically what clang does", and that's specified as "basically what GCC
does", and that's basically/shockingly not specified much at all despite GCC
being like 30 years old.

So let's be slow and pedantic about this process.

## Inline ASM

**Fair Warning:** The general information that follows regarding the asm macro
is consistent from system to system, but specific information about register
names, register quantities, asm instruction argument ordering, and so on is
specific to ARM on the GBA. If you're programming for any other device you'll
need to carefully investigate that before you begin.

Now then, with those out of the way, the inline asm docs describe an asm call as
looking like this:

```rust
let x = 10u32;
let y = 34u32;
let result: u32;
asm!(
  // assembly template
  "add {lhs}, {rhs}",
  lhs = inout(reg_thumb) x => result,
  rhs = in(reg_thumb) y,
  options(nostack, nomem),
);
// result == 44
```

The `asm` macro follows the [RFC
2873](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md)
syntax. The following is just a summary of the RFC.

Now we have to decide what we're gonna write. Obviously we're going to do some
instructions, but those instructions use registers, and how are we gonna talk
about them? We've got two choices.

1) We can pick each and every register used by specifying exact register names.
   In THUMB mode we have 8 registers available, named `r0` through `r7`. To use
   those registers you would write  `in("r0") x` instead of
   `rhs = in(reg_thumb) x`, and directly refer to `r0` in the assembly template.

2) We can specify slots for registers we need and let LLVM decide. This is what
   we do when we write `rhs = in(reg_thumb) y` and use `{rhs}` in the assembly
   template.

   The `reg_thumb` stands for the register class we are using. Since we are
   in THUMB mode, the set of registers we can use is limited. `reg_thumb` tells
   LLVM: "use only registers available in THUMB mode". In 32-bit mode, you have
   access to more register and you should use a different register class.

   The register classes [are described in the
   RFC](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md#register-operands).
   Look for "ARM" register classes.

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

### Register bindings

After the assembly string literal, you need to define your binding (which
rust variables are getting into your registers and which ones are going to refer
to their value afterward).

There are many operand types [as per the
RFC](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md#operand-type),
but you will most often use:

```
[alias =] in(<reg>) <binding> // input
[alias =] out(<reg>) <binding> // output
[alias =] inout(<reg>) <in binding> => <out binding> // both
out(<reg>) _ // Clobber
```

* The binding can be any single 32-bit or smaller value.
* If your binding has bit pattern requirements ("must be non-zero", etc) you are
  responsible for upholding that.
* If your binding type will try to `Drop` later then you are responsible for it
  being in a fit state to do that.
* The binding must be either a mutable binding or a binding that was
  pre-declared but not yet assigned.
* An input binding must be a single 32-bit or smaller value.
* An input binding _should_ be a type that is `Copy` but this is not an absolute
  requirement. Having the input be read is semantically similar to using
  `core::ptr::read(&binding)` and forgetting the value when you're done.

Anything else is UB.

### Clobbers

Sometimes your asm will touch registers other than the ones declared for input
and output. 

Clobbers are declared as a comma separated list of string literals naming
specific registers. You don't use curly braces with clobbers.

LLVM _needs_ to know this information. It can move things around to keep your
data safe, but only if you tell it what's about to happen.

Failure to define all of your clobbers can cause UB.

### Options

By default the compiler won't optimize the code you wrote in an `asm` block. You
will need to specify with the `options(..)` parameter that your code can be
optimized. The available options [are specified in the
RFC](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md#options-1).

An optimization might duplicate or remove your instructions from the final
code.

Typically when executing a BIOS call (such as `swi 0x01`, which resets the
console), it's important that the instruction is executed, and not optimized
away, even though it has no observable input and output to the compiler.

However some BIOS calls, such as _some_ math functions, have no observable
effects outside of the registers we specified, in this case, we instruct the
compiler to optimize them.

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

For our example we'll use the division function, because GBATEK gives very clear
instructions on how each register is used with that one:

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
to use the correct names. Our Rust function has an assert against dividing by
`0`, then we name some bindings _without_ giving them a value, we make the asm
call, and then return what we got.

```rust
pub fn div_rem(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  let div_out: i32;
  let rem_out: i32;
  unsafe {
    asm!(
      "swi 0x06",
      inout("r0") numerator => div_out,
      inout("r1") denominator => rem_out,
      out("r3") _,
      options(nostack, nomem),
    );
  }
  (div_out, rem_out)
}
```

I _hope_ this all makes sense by now.

## Specific BIOS Functions

For a full list of all the specific BIOS functions and their use you should
check the `gba::bios` module within the `gba` crate. There's just so many of
them that enumerating them all here wouldn't serve much purpose.

Which is not to say that we'll never cover any BIOS functions in this book!
Instead, we'll simply mention them when whenever they're relevent to the task at
hand (such as controlling sound or waiting for vblank).

//TODO: list/name all BIOS functions as well as what they relate to elsewhere.
