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
go using them all over the place if you don't have to.

I'd like to take a moment to thank [Marc Brinkmann](https://github.com/mbr)
(with contributions from [Oliver Schneider](https://github.com/oli-obk) and
[Philipp Oppermann](https://github.com/phil-opp)) for writing [this blog
post](http://embed.rs/articles/2016/arm-inline-assembly-rust/). It's at least
ten times the tutorial quality as the `asm` entry in the Unstable Book has. In
their defense, the actual spec of how inline ASM works in rust is "basically
what clang does", and that's specified as "basically what GCC does", and that's
basically not specified at all despite GCC being like 30 years old.

So we're in for a very slow, careful, and pedantic ride on this one.

## Inline ASM

The inline asm docs describe an asm call as looking like this:

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
has some commends throw in on each line:

```rust
asm!(/* ASM */ TODO
    :/* OUT */ TODO
    :/* INP */ TODO
    :/* CLO */ TODO
    :/* OPT */
);
```

Note: it's possible to use an inline ASM style where you allow LLVM to determine
the exact register placement. We will _not_ do that in this section because each
BIOS call has specific input and output slots that we must follow. However, if
you want to use inline asm for other purposes elsewhere in your code you can use
it then.

* **ASM:** The actual asm instructions to use.
  * When writing inline asm, remember that we're writing for 16-bit THUMB mode
    because that's what all of our Rust code is compiled to. You can switch to
    32-bit ARM mode on the fly, but be sure to switch back before the inline ASM
    block ends or things will go _bad_.
  * You can write code for specific registers (`r0` through `r7` are available
    in THUMB mode) or you can write code for _register slots_ and let LLVM pick
    what actual registers to assign to what slots. In this case, you'd instead
    write `$0`, `$1` and so on (however many you need). Outputs take up one slot
    each, followed by inputs taking up one slot each.
* **OUT:** The output variables, if any. Comma separated list.
  * Output is specified as `"constraint" (binding)`
  * A constraint is either `=` (write), `+` (read and write), or `&` (early
    clobber) followed by either the name of a specific register in curly braces,
    such as `{r0}`, or simply `r` if you want to let LLVM assign it.
  * If you're writing to `r0` you'd use `={r0}`, if you're read writing from
    `r3` you'd use `+{r3}` and so on.
  * Bindings named in the outputs must be mutable bindings or bindings that
    are declared but not yet assigned to.
  * GBA registers are 32-bit, and you must always use an appropriately sized
    type for the binding.
  * LLVM assumes when selecting registers for you that no output is written to
    until all inputs are read. If this is not the case you need to use the `&`
    designation on your output to give LLVM the heads up so that LLVM doesn't
    assign it as an input register.
* **INP:** The inputs, if any. Comma separated list.
  * Similar to outputs, the input format is `"constraint" (binding)`
  * Inputs don't have a symbol prefix, you simply name the specific register in
    curly braces or use `r` to let LLVM pick.
  * Inputs should always be 32-bit types. (TODO: can you use smaller types and
    have it 'just work'?)
* **CLO:** This is possibly _the most important part to get right_. The
  "clobbers" part describes what registers are affected by this use of asm. The
  compiler will use this to make sure that you don't accidentally destroy any of
  your data.
  * The clobbers list is a comma separated series of string literals that each
    name one of the registers clobbered.
  * Example: "r0", "r1", "r3"
* **OPT:** This lets us specify any options. At the moment the only option we
  care about is that some asm calls will need to be "volatile". As with reads
  and writes, the compiler will attempt to eliminate asm that it thinks isn't
  necessary, so if there's no output from an asm block we'll need to mark it
  volatile to make sure that it gets done.

That seems like a whole lot, but since we're only handling BIOS calls in this
section we can tone it down quite a bit:

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

You can implement that yourself (we might get around to trying that, i was even
sent [a link to a
paper](https://www.microsoft.com/en-us/research/wp-content/uploads/2008/08/tr-2008-141.pdf)
that I promptly did not read), or you can call the BIOS to do it for you and
trust that it's being as efficient as possible.

GBATEK gives a very clear explanation of it:

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

Of course, the math folks tell me that the `r1` value should be properly called
the "remainder" not the "modulus". We'll go with that for our function, doesn't
hurt to use the correct names. The function itself is a single assert, then we
name some bindings without giving them a value, make the asm call, and then
return what we got.

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
