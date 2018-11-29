# GBA PRNG

You often hear of the "Random Number Generator" in video games. First of all,
usually a game doesn't have access to any source of "true randomness". On a PC
you can send out a web request to [random.org](https://www.random.org/) which
uses atmospheric data, or even just [point a camera at some lava
lamps](https://blog.cloudflare.com/randomness-101-lavarand-in-production/). Even
then, the rate at which you'll want random numbers far exceeds the rate at which
those services can offer them up. So instead you'll get a pseudo-random number
generator and "seed" it with the true random data and then use that.

However, we don't even have that! On the GBA, we can't ask any external anything
what we should do for our initial seed. So we will not only need to come up with
a few PRNG options, but we'll also need to come up with some seed source
options. More than with other options within the book, I think this is an area
where you can tailor what you do to your specific game.

## What is a Pseudo-random Number Generator?

For those of you who somehow read The Rust Book, plus possibly The Rustonomicon,
and then found this book, but somehow _still_ don't know what a PRNG is... Well,
I don't think there are many such people. Still, we'll define it anyway I
suppose.

> A PRNG is any mathematical process that takes an initial input (of some fixed
> size) and then produces a series of outputs (of a possibly different size).

So, if you seed your PRNG with a 32-bit value you might get 32-bit values out or
you might get 16-bit values out, or something like that.

We measure the quality of a PRNG based upon:

1) **Is the output range easy to work with?** Most PRNG techniques that you'll
   find these days are already hip to the idea that we'll have the fastest
   operations with numbers that match our register width and all that, so
   they're usually designed around power of two inputs and power of two outputs.
   Still, every once in a while you might find some page old page intended for
   compatibility with the `rand()` function in the C standard library that'll
   talk about something _crazy_ like having 15-bit PRNG outputs. Stupid as it
   sounds, that's real. Avoid those. Whenever possible we want generators that
   give us uniformly distributed `u8`, `u16`, `u32`, or whatever size value
   we're producing. From there we can mold our random bits into whatever else we
   need (eg: turning a `u8` into a "1d6" roll).
2) **How long does each generation cycle take?** This can be tricky for us. A
   lot of the top quality PRNGs you'll find these days are oriented towards
   64-bit machines so they do a bunch of 64-bit operations. You _can_ do that on
   a 32-bit machine if you have to, and the compiler will automatically "lower"
   the 64-bit operation into a series of 32-bit operations. What we'd really
   like to pick is something that sticks to just 32-bit operations though, since
   those will be our best candidates for fast results. We can use [Compiler
   Explorer](https://rust.godbolt.org/z/JyX7z-) and tell it to build for the
   `thumbv6m-none-eabi` target to get a basic idea of what the ASM for a
   generator looks like. That's not our exact target, but it's the closest
   target that's shipped with the standard rust distribution.
3) **What is the statistical quality of the output?** This involves heavy
   amounts of math. Since computers are quite good a large amounts of repeated
   math you might wonder if there's programs for this already, and there are.
   Many in fact. They take a generator and then run it over and over and perform
   the necessary tests and report the results. I won't be explaining how to hook
   our generators up to those tools, they each have their own user manuals.
   However, if someone says that a generator "passes BigCrush" (the biggest
   suite in TestU01) or "fails PractRand" or anything similar it's useful to
   know what they're referring to. Example test suites include:
   * [TestU01](https://en.wikipedia.org/wiki/TestU01)
   * [PractRand](http://pracrand.sourceforge.net/)
   * [Dieharder](https://webhome.phy.duke.edu/~rgb/General/dieharder.php)
   * [NIST Statistical Test
     Suite](https://csrc.nist.gov/projects/random-bit-generation/documentation-and-software)

Note that if a generator is called upon to produce enough output relative to its
state size it will basically always end up failing statistical tests. This means
that any generator with 32-bit state will always fail in any of those test sets.
The theoretical _minimum_ state size for any generator at all to pass the
standard suites is 36 bits, but most generators need many more than that.

### Generator Size

I've mostly chosen to discuss generators that are towards the smaller end of the
state size scale. In fact we'll be going over many generators that are below the
36-bit theoretical minimum to pass all those fancy statistical tests. Why so?
Well, we don't always need the highest possible quality generators.

"But Lokathor!", I can already hear you shouting. "I want the highest quality
randomness at all times! The game depends on it!", you cry out.

Well... does it? Like, _really_?

The [GBA
Pokemon](https://bulbapedia.bulbagarden.net/wiki/Pseudorandom_number_generation_in_Pok%C3%A9mon)
games use a _dead simple_ 32-bit LCG (we'll see it below). Then starting with
the DS they moved to also using Mersenne Twister, which also fails several
statistical tests and is one of the most predictable PRNGs around. [Metroid
Fusion](http://wiki.metroidconstruction.com/doku.php?id=fusion:technical:rng)
has a 100% goofy PRNG system for enemies that would definitely never pass any
sort of statistics tests at all. But like, those games were still awesome. Since
we're never going to be keeping secrets safe with our PRNG, it's okay if we
trade in some quality for something else in return (we obviously don't want to
trade quality for nothing).

And you have to ask yourself: Where's the space used for the Metroid Fusion
PRNG? No where at all. They were already using everything involved for other
things too, so they're paying no extra cost to have the randomization they do.
How much does it cost Pokemon to throw in a 32-bit LCG? Just 4 bytes, might as
well. How much does it cost to add in a Mersenne Twister? ~2,500 bytes ya say?
I'm sorry _what on Earth_? Yeah, that sounds crazy, we're probably not doing
that one.

### k-Dimensional Equidistribution

So, wait, why did the Pokemon developers add in the Mersenne Twister generator?
They're smart people, surely they had a reason. Obviously we can't know for
sure, but Mersenne Twister is terrible in a lot of ways, so what's its single
best feature? Well, that gets us to a funky thing called **k-dimensional
equidistribution**. Basically, if you take a generator's output and chop it down
to get some value you want, with uniform generator output you can always get a
smaller ranged uniform result (though sometimes you will have to reject a result
and run the generator again). Imagine you have a `u32` output from your
generator. If you want a `u16` value from that you can just pick either half. If
you want a `[bool; 4]` from that you can just pick four bits. However you wanna
do it, as long as the final form of random thing we're getting needs a number of
bits _equal to or less than_ the number of bits that come out of a single
generator use, we're totally fine.

What happens if the thing you want to make requires _more_ bits than a single
generator's output? You obviously have to run the generator more than once and
then stick two or more outputs together, duh. Except, that doesn't always work.
What I mean is that obviously you can always put two `u8` side by side to get a
`u16`, but if you start with a uniform `u8` generator and then you run it twice
and stick the results together you _don't_ always get a uniform `u16` generator.
Imagine a byte generator that just does `state+=1` and then outputs the state.
It's not good by almost any standard, but it _does give uniform output_. Then we
run it twice in a row, put the two bytes together, and suddenly a whole ton of
potential `u16` values can never be generated. That's what k-dimensional
equidistribution is all about. Every uniform output generator is 1-dimensional
equidistributed, but if you need to combine outputs and still have uniform
results then you need a higher `k` value. So why does Pokemon have Mersenne
Twister in it? Because it's got 623-dimensional equidistribution. That means
when you're combining PRNG calls for all those little IVs and Pokemon Abilities
and other things you're sure to have every potential pokemon actually be a
pokemon that the game can generate. Do you need that for most situations?
Absolutely not. Do you need it for pokemon? No, not even then, but a lot of the
hot new PRNGs have come out just within the past 10 years, so we can't fault
them too much for it.

TLDR: 1-dimensional equidistribution just means "a normal uniform generator",
and higher k values mean "you can actually combine up to k output chains and
maintain uniformity". Generators that aren't uniform to begin with effectively
have a k value of 0.

### Other Tricks

Finally, some generators have other features that aren't strictly quantifiable.
Two tricks of note are "jump ahead" or "multiple streams":

* Jump ahead lets you advance the generator's state by some enormous number of
  outputs in a relatively small number of operations.
* Multi-stream generators have more than one output sequence, and then some part
  of their total state space picks a "stream" rather than being part of the
  actual seed, with each possible stream causing the potential output sequence
  to be in a different order.

They're normally used as a way to do multi-threaded stuff (we don't care about
that on GBA), but another interesting potential is to take one world seed and
then split off a generator for each "type" of thing you'd use PRNG for (combat,
world events, etc). This can become quite useful, where you can do things like
procedurally generate a world region, and then when they leave the region you
only need to store a single generator seed and a small amount of "delta"
information for what the player changed there that you want to save, and then
when they come back you can regenerate the region without having stored much at
all. This is the basis for how old games with limited memory like
[Starflight](https://en.wikipedia.org/wiki/Starflight) did their whole thing
(800 planets to explore on just to 5.25" floppy disks!).

## How To Seed

Oh I bet you thought we could somehow get through a section without learning
about yet another IO register. Ha, wishful thinking.

There's actually not much involved. Starting at `0x400_0100` there's an array of
registers that go "data", "control", "data", "control", etc. TONC and GBATEK use
different names here, and we'll go by the TONC names because they're much
clearer:

```rust
pub const TM0D: VolatilePtr<u16> = VolatilePtr(0x400_0100 as *mut u16);
pub const TM0CNT: VolatilePtr<u16> = VolatilePtr(0x400_0102 as *mut u16);

pub const TM1D: VolatilePtr<u16> = VolatilePtr(0x400_0104 as *mut u16);
pub const TM1CNT: VolatilePtr<u16> = VolatilePtr(0x400_0106 as *mut u16);

pub const TM2D: VolatilePtr<u16> = VolatilePtr(0x400_0108 as *mut u16);
pub const TM2CNT: VolatilePtr<u16> = VolatilePtr(0x400_010A as *mut u16);

pub const TM3D: VolatilePtr<u16> = VolatilePtr(0x400_010C as *mut u16);
pub const TM3CNT: VolatilePtr<u16> = VolatilePtr(0x400_010E as *mut u16);
```

Basically there's 4 timers, numbered 0 to 3. Each one has a Data register and a
Control register. They're all `u16` and you can definitely _read_ from all of
them normally, but then it gets a little weird. You can also _write_ to the
Control portions normally, when you write to the Data portion of a timer that
writes the value that the timer resets to, _without changing_ its current Data
value. So if `TM0D` is paused on some value other than `5` and you write `5` to
it, when you read it back you won't get a `5`. When the next timer run starts
it'll begin counting at `5` instead of whatever value it currently reads as.

The Data registers are just a `u16` number, no special bits to know about.

The Control registers are also pretty simple compared to most IO registers:

* 2 bits for the **Frequency:** 1, 64, 256, 1024. While active, the timer's
  value will tick up once every `frequency` CPU cycles. On the GBA, 1 CPU cycle
  is about 59.59ns (2^(-24) seconds). One display controller cycle is 280,896
  CPU cycles.
* 1 bit for **Cascade Mode:** If this is on the timer doesn't count on its own,
  instead it ticks up whenever the _preceding_ timer overflows its counter (eg:
  if t0 overflows, t1 will tick up if it's in cascade mode). You still have to
  also enable this timer for it to do that (below). This naturally doesn't have
  an effect when used with timer 0.
* 3 bits that do nothing
* 1 bit for **Interrupt:** Whenever this timer overflows it will signal an
  interrupt. We still haven't gotten into interrupts yet (since you have to hand
  write some ASM for that, it's annoying), but when we cover them this is how
  you do them with timers.
* 1 bit to **Enable** the timer. When you disable a timer it retains the current
  value, but when you enable it again the value jumps to whatever its currently
  assigned default value is.

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerControl(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerFrequency {
  One = 0,
  SixFour = 1,
  TwoFiveSix = 2,
  OneZeroTwoFour = 3,
}

impl TimerControl {
  pub fn frequency(self) -> TimerFrequency {
    match self.0 & 0b11 {
      0 => TimerFrequency::One,
      1 => TimerFrequency::SixFour,
      2 => TimerFrequency::TwoFiveSix,
      3 => TimerFrequency::OneZeroTwoFour,
      _ => unreachable!(),
    }
  }
  pub fn cascade_mode(self) -> bool {
    self.0 & 0b100 > 0
  }
  pub fn interrupt(self) -> bool {
    self.0 & 0b100_0000 > 0
  }
  pub fn enabled(self) -> bool {
    self.0 & 0b1000_0000 > 0
  }
  //
  pub fn set_frequency(&mut self, frequency: TimerFrequency) {
    self.0 &= !0b11;
    self.0 |= frequency as u16;
  }
  pub fn set_cascade_mode(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b100;
    } else {
      self.0 &= !0b100;
    }
  }
  pub fn set_interrupt(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b100_0000;
    } else {
      self.0 &= !0b100_0000;
    }
  }
  pub fn set_enabled(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b1000_0000;
    } else {
      self.0 &= !0b1000_0000;
    }
  }
}
```

### A Timer Based Seed

Okay so how do we turns some timers into a PRNG seed? Well, usually our seed is
a `u32`. So we'll take two timers, string them together with that cascade deal,
and then set them off. Then we wait until the user presses any key. We probably
do this as our first thing at startup, but we might show the title and like a
"press any key to continue" message, or something.

```rust
/// Mucks with the settings of Timers 0 and 1.
fn u32_from_user_wait() -> u32 {
  let mut t = TimerControl::default();
  t.set_enabled(true);
  t.set_cascading(true);
  TM1CNT.write(t.0);
  t.set_cascading(false);
  TM0CNT.write(t.0);
  while key_input().0 == 0 {}
  t.set_enabled(false);
  TM0CNT.write(t.0);
  TM1CNT.write(t.0);
  let low = TM0D.read() as u32;
  let high = TM1D.read() as u32;
  (high << 32) | low
}
```

## Various Generators

### SM64 (16-bit state, 16-bit output, non-uniform, bonkers)

Our first PRNG to mention isn't one that's at all good, but it sure might be
cute to use. It's the PRNG that Super Mario 64 had ([video explanation,
long](https://www.youtube.com/watch?v=MiuLeTE2MeQ)).

With a PRNG this simple the output of one call is _also_ the seed to the next
call, so we don't need to make a struct for it or anything. You're also assumed
to just seed with a plain 0 value at startup. The generator has a painfully
small period, and you're assumed to be looping through the state space
constantly while the RNG goes.

```rust
pub fn sm64(mut input: u16) -> u16 {
  if input == 0x560A {
    input = 0;
  }
  let mut s0 = input << 8;
  s0 ^= input;
  input = s0.rotate_left(8);
  s0 = ((s0 as u8) << 1) as u16 ^ input;
  let s1 = (s0 >> 1) ^ 0xFF80;
  if (s0 & 1) == 0 {
    if s1 == 0xAA55 {
        input = 0;
    } else {
        input = s1 ^ 0x1FF4;
    }
  } else {
    input = s1 ^ 0x8180;
  }
  input
}
```

[Compiler Explorer](https://rust.godbolt.org/z/1F6P8L)

If you watch the video explanation about this generator you'll note that the
first `if` checking for `0x560A` prevents you from being locked into a 2-step
cycle, but it's only important if you want to feed bad seeds to the generator. A
bad seed is unhelpfully defined defined as "any value that the generator can't
output". The second `if` that checks for `0xAA55` doesn't seem to be important
at all from a mathematical perspective. It cuts the generator's period shorter
by an arbitrary amount for no known reason. It's left in there only for
authenticity.

### LCG32 (32-bit state, 32-bit output, uniform)

The [Linear Congruential
Generator](https://en.wikipedia.org/wiki/Linear_congruential_generator) is a
well known PRNG family. You pick a multiplier and an additive and you're done.
Right? Well, not exactly, because (as the wikipedia article explains) the values
that you pick can easily make your LCG better or worse all on its own. You want
a good multiplier, and you want your additive to be odd. In our example here
we've got the values that
[Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Pseudorandom_number_generation_in_Pok%C3%A9mon)
says were used in the actual GBA Pokemon games, though Bulbapedia also lists
values for a few other other games as well.

I don't actually know if _any_ of the constants used in the official games are
particularly good from a statistical viewpoint, though with only 32 bits an LCG
isn't gonna be passing any of the major statistical tests anyway (you need way
more bits in your LCG for that to happen). In my mind the main reason to use a
plain LCG like this is just for the fun of using the same PRNG that an official
Pokemon game did.

You should _not_ use this as your default generator if you care about quality.

It is _very_ fast though... if you want to set everything else on fire for
speed. If you do, please _at least_ remember that the highest bits are the best
ones, so if you're after less than 32 bits you should shift the high ones down
and keep those, or if you want to turn it into a `bool` cast to `i32` and then
check if it's negative, etc.

```rust
pub fn lcg32(seed: u32) -> u32 {
  seed.wrapping_mul(0x41C6_4E6D).wrapping_add(0x6073)
}
```

[Compiler Explorer](https://rust.godbolt.org/z/k5n_jJ)

#### Multi-stream Generators

Note that you don't have to add a compile time constant, you could add a runtime
value instead. Doing so allows the generator to be "multi-stream", with each
different additive value being its own unique output stream. This true of LCGs
as well as all the PCGs below (since they're LCG based). The examples here just
use a fixed stream for simplicity and to save space, but if you want streams you
can add that in for only a small amount of extra space used:

```rust
pub fn lcg_streaming(seed: u32, stream: u32) -> u32 {
  seed.wrapping_mul(0x41C6_4E6D).wrapping_add(stream)
}
```

With a streaming LCG you should pass the same stream value every single time. If
you don't, then your generator will jump between streams in some crazy way and
you lose your nice uniformity properties.

There is the possibility of intentionally changing the stream value exactly when
the seed lands on a pre-determined value (after the multiply and add). This
_basically_ makes the stream selection value's bit size (minus one bit, because
it must be odd) count into the LCG's state bit size for calculating the overall
period of the generator. So an LCG32 with a 32-bit stream selection would have a
period of 2^32 * 2^31 = 2^63.

```rust
let next_seed = lcg_streaming(seed, stream);
// It's cheapest to test for 0, so we pick 0
if seed == 0 {
  stream = stream.wrapping_add(2)
}
```

However, this isn't a particularly effective way to extend the generator's
period, and we'll see a much better extension technique below.

### PCG16 XSH-RS (32-bit state, 16-bit output, uniform)

The [Permuted Congruential
Generator](https://en.wikipedia.org/wiki/Permuted_congruential_generator) family
is the next step in LCG technology. We start with LCG output, which is good but
not great, and then we apply one of several possible permutations to bump up the
quality. There's basically a bunch of permutation components that are each
defined in terms of the bit width that you're working with.

The "default" variant of PCG, PCG32, has 64 bits of state and 32 bits of output,
and it uses the "XSH-RR" permutation. Here we'll put together a 32 bit version
with 16-bit output, and using the "XSH-RS" permutation (but we'll show the other
one too for comparison).

Of course, since PCG is based on a LCG, we have to start with a good LCG base.
As I said above, a better or worse set of LCG constants can make your generator
better or worse. The Wikipedia example for PCG has a good 64-bit constant, but
not a 32-bit constant. So we gotta [ask an
expert](http://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf)
about what a good 32-bit constant would be. I'm definitely not the best at
reading math papers, but it seems that the general idea is that we want `m % 8
== 5` and `is_even(a)` to both hold for the values we pick. There are three
suggested LCG multipliers in a chart on page 10. A chart that's quite hard to
understand. Truth be told I asked several folks that are good at math papers and
even they couldn't make sense of the chart. Eventually `timutable` read the
whole paper in depth and concluded the same as I did: that we probably want to
pick the `32310901` option.

For an additive value, we can pick any odd value, so we might as well pick
something small so that we can do an immediate add. _Immediate_ add? That sounds
new. An immediate instruction is when one side of an operation is small enough
that you can encode the value directly into the space that'd normally be for the
register you want to use. It basically means one less load you have to do, if
you're working with small enough numbers. To see what I mean compare [loading
the add value](https://rust.godbolt.org/z/LKCFUS) and [immediate add
value](https://rust.godbolt.org/z/SnZW9a). It's something you might have seen
frequently in `x86` or `x86_64` ASM output, but because a thumb instruction is
only 16 bits total, we can only get immediate instructions if the target value
is 8 bits or less, so we haven't used them too much ourselves yet.

I guess we'll pick 5, because I happen to personally like the number.

```rust
// Demo only. The "default" PCG permutation, for use when rotate is cheaper
pub fn pcg16_xsh_rr(seed: &mut u32) -> u16 {
  *seed = seed.wrapping_mul(32310901).wrapping_add(5);
  const INPUT_SIZE: u32 = 32;
  const OUTPUT_SIZE: u32 = 16;
  const ROTATE_BITS: u32 = 4;
  let mut out32 = *seed;
  let rot = out32 >> (INPUT_SIZE - ROTATE_BITS);
  out32 ^= out32 >> ((OUTPUT_SIZE + ROTATE_BITS) / 2);
  ((out32 >> (OUTPUT_SIZE - ROTATE_BITS)) as u16).rotate_right(rot)
}

// This has slightly worse statistics but runs much better on the GBA
pub fn pcg16_xsh_rs(seed: &mut u32) -> u16 {
  *seed = seed.wrapping_mul(32310901).wrapping_add(5);
  const INPUT_SIZE: u32 = 32;
  const OUTPUT_SIZE: u32 = 16;
  const SHIFT_BITS: u32 = 2;
  const NEXT_MOST_BITS: u32 = 19;
  let mut out32 = *seed;
  let shift = out32 >> (INPUT_SIZE - SHIFT_BITS);
  out32 ^= out32 >> ((OUTPUT_SIZE + SHIFT_BITS) / 2);
  (out32 >> (NEXT_MOST_BITS + shift)) as u16
}
```

[Compiler Explorer](https://rust.godbolt.org/z/NtJAwS)

### PCG32 RXS-M-XS (32-bit state, 32-bit output, uniform)

Having the output be smaller than the input is great because you can keep just
the best quality bits that the LCG stage puts out, and you basically get 1 point
of dimensional equidistribution for each bit you discard as the size goes down
(so 32->16 gives 16). However, if your output size _has_ to the the same as your
input size, the PCG family is still up to the task.

```rust
pub fn pcg32_rxs_m_xs(seed: &mut u32) -> u32 {
  *seed = seed.wrapping_mul(32310901).wrapping_add(5);
  let mut out32 = *seed;
  let rxs = out32 >> 28;
  out32 ^= out32 >> (4 + rxs);
  const PURE_MAGIC: u32 = 277803737;
  out32 *= PURE_MAGIC;
  out32^ (out32 >> 22)
}
```

[Compiler Explorer](https://rust.godbolt.org/z/j3KPId)

This permutation is the slowest but gives the strongest statistical benefits. If
you're going to be keeping 100% of the output bits you want the added strength
obviously. However, the period isn't actually any longer, so each output will be
given only once within the full period (1-dimensional equidistribution).

### PCG Extension Array

As a general improvement to any PCG you can hook on an "extension array" to give
yourself a longer period. It's all described in the [PCG
Paper](http://www.pcg-random.org/paper.html), but here's the bullet points:

* In addition to your generator's state (and possible stream) you keep an array
  of "extension" values. The array _type_ is the same as your output type, and
  the array _count_ must be a power of two value that's less than the maximum
  value of your state size.
* When you run the generator, use the _lowest_ bits to select from your
  extension array according to the array's power of two. Eg: if the size is 2
  then use the single lowest bit, if it's 4 then use the lowest 2 bits, etc.
* Every time you run the generator, XOR the output with the selected value from
  the array.
* Every time the generator state lands on 0, cycle the array. We want to be
  careful with what we mean here by "cycle". We want the _entire_ pattern of
  possible array bits to occur eventually. However, we obviously can't do
  arbitrary adds for as many bits as we like, so we'll have to "carry the 1"
  between the portions of the array by hand.

Here's an example using an 8 slot array and `pcg16_xsh_rs`:

```rust
// uses pcg16_xsh_rs from above

pub struct PCG16Ext8 {
  state: u32,
  ext: [u16; 8],
}

impl PCG16Ext8 {
  pub fn next_u16(&mut self) -> u16 {
    // PCG as normal.
    let mut out = pcg16_xsh_rs(&mut self.state);
    // XOR with a selected extension array value
    out ^= unsafe { self.ext.get_unchecked((self.state & !0b111) as usize) };
    // if state == 0 we cycle the array with a series of overflowing adds
    if self.state == 0 {
      let mut carry = true;
      let mut index = 0;
      while carry && index < self.ext.len() {
        let (add_output, next_carry) = self.ext[index].overflowing_add(1);
        self.ext[index] = add_output;
        carry = next_carry;
        index += 1;
      }
    }
    out
  }
}
```

[Compiler Explorer](https://rust.godbolt.org/z/HTxoHY)

The period gained from using an extension array is quite impressive. For a b-bit
generator giving r-bit outputs, and k array slots, the period goes from 2^b to
2^(k*r+b). So our 2^32 period generator has been extended to 2^160.

Of course, we might care to seed the array itself so that it's not all 0 bits
all the way though, but that's not strictly necessary. All 0s is a legitimate
part of the extension cycle, so we have to pass through it at some point.

### Xoshiro128** (128-bit state, 32-bit output, non-uniform)

The [Xoshiro128**](http://xoshiro.di.unimi.it/xoshiro128starstar.c) generator is
an advancement of the [Xorshift family](https://en.wikipedia.org/wiki/Xorshift).
It was specifically requested, and I'm not aware of Xorshift specifically being
used in any of my favorite games, so instead of going over Xorshift and then
leading up to this, we'll just jump straight to this. Take care not to confuse
this generator with the very similarly named
[Xoroshiro128**](http://xoshiro.di.unimi.it/xoroshiro128starstar.c) generator,
which is the 64 bit variant. Note the extra "ro" hiding in the 64-bit version's
name near the start.

Anyway, weird names aside, it's fairly zippy. The biggest downside is that you
can't have a seed state that's all 0s, and as a result 0 will be produced one
less time than all other outputs within a full cycle, making it non-uniform by
just a little bit. You also can't do a simple stream selection like with the LCG
based generators, instead it has a fixed jump function that advances a seed as
if you'd done 2^64 normal generator advancements.

Note that `Xoshiro256**` is known to fail statistical tests, so the 128 version
is unlikely to pass them, though I admit that I didn't check myself.

```rust
pub fn xoshiro128_starstar(seed: &mut [u32; 4]) -> u32 {
  let output = seed[0].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
  let t = seed[1] << 9;

  seed[2] ^= seed[0];
  seed[3] ^= seed[1];
  seed[1] ^= seed[2];
  seed[0] ^= seed[3];

  seed[2] ^= t;

  seed[3] = seed[3].rotate_left(11);

  output
}

pub fn xoshiro128_starstar_jump(seed: &mut [u32; 4]) {
  const JUMP: [u32; 4] = [0x8764000b, 0xf542d2d3, 0x6fa035c3, 0x77f2db5b];
  let mut s0 = 0;
  let mut s1 = 0;
  let mut s2 = 0;
  let mut s3 = 0;
  for j in JUMP.iter() {
    for b in 0 .. 32 {
        if *j & (1 << b) > 0 {
            s0 ^= seed[0];
            s1 ^= seed[1];
            s2 ^= seed[2];
            s3 ^= seed[3];
        }
        xoshiro128_starstar(seed);
    }
  }
  seed[0] = s0;
  seed[1] = s1;
  seed[2] = s2;
  seed[3] = s3;
}
```

[Compiler Explorer](https://rust.godbolt.org/z/PGvwZw)

### jsf32 (128-bit state, 32-bit output, non-uniform)

This is Bob Jenkins's [Small/Fast PRNG](small noncryptographic PRNG). It's a
little faster than `Xoshiro128**` (no multiplication involved), and can pass any
statistical test that's been thrown at it.

Interestingly the generator's period is _not_ fixed based on the generator
overall. It's actually set by the exact internal generator state. There's even
six possible internal generator states where the generator becomes a fixed
point. Because of this, we should use the verified seeding method provided.
Using the provided seeding, the minimum period is expected to be 2^94, the
average is about 2^126, and no seed given to the generator is likely to overlap
with another seed's output for at least 2^64 uses.

```rust
pub struct JSF32 {
  a: u32,
  b: u32,
  c: u32,
  d: u32,
}

impl JSF32 {
  pub fn new(seed: u32) -> Self {
    let mut output = JSF32 {
      a: 0xf1ea5eed,
      b: seed,
      c: seed,
      d: seed
    };
    for _ in 0 .. 20 {
      output.next();
    }
    output
  }

  pub fn next(&mut self) -> u32 {
    let e = self.a - self.b.rotate_left(27);
    self.a = self.b ^ self.c.rotate_left(17);
    self.b = self.c + self.d;
    self.c = self.d + e;
    self.d = e + self.a;
    self.d
  }
}
```

[Compiler Explorer](https://rust.godbolt.org/z/qO3obQ)

Here it's presented with (27,17), but you can also use any of the following if
you want alternative generator flavors that use this same core technique:

* (9,16), (9,24), (10,16), (10,24), (11,16), (11,24), (25,8), (25,16), (26,8),
  (26,16), (26,17), or (27,16).

Note that these alternate flavors haven't had as much testing as the (27,17)
version, though they are likely to be just as good.

### Other Generators?

* [Mersenne Twister](https://en.wikipedia.org/wiki/Mersenne_Twister): Gosh, 2.5k
  is just way too many for me to ever want to use this thing. If you'd really
  like to use it, there is [a
  crate](https://docs.rs/mersenne_twister/1.1.1/mersenne_twister/) for it that
  already has it. Small catch, they use a ton of stuff from `std` that they
  could be importing from `core`, so you'll have to fork it and patch it
  yourself to get it working on the GBA. They also stupidly depend on an old
  version of `rand`, so you'll have to cut out that nonsense.

## Placing a Value In Range

I said earlier that you can always take a uniform output and then throw out some
bits, and possibly the whole result, to reduce it down into a smaller range. How
exactly does one do that? Well it turns out that it's [very
tricky](http://www.pcg-random.org/posts/bounded-rands.html) to get right, and we
could be losing as much as 60% of our execution time if we don't do it carefully.

The _best_ possible case is if you can cleanly take a specific number of bits
out of your result without even doing any branching. The rest can be discarded
or kept for another step as you choose. I know that I keep referencing Pokemon,
but it's a very good example for the use of randomization. Each pokemon has,
among many values, a thing called an "IV" for each of 6 stats. The IVs range
from 0 to 31, which is total nonsense to anyone not familiar with decimal/binary
conversions, but to us programmers that's clearly a 5 bit range. Rather than
making math that's better for people using decimal (such as a 1-20 range or
something like that) they went with what's easiest for the computer.

The _next_ best case is if you can have a designated range that you want to
generate within that's known at compile time. This at least gives us a chance to
write some bit of extremely specialized code that can take random bits and get
them into range. Hopefully your range can be "close enough" to a binary range
that you can get things into place. Example: if you want a "1d6" result then you
can generate a `u16`, look at just 3 bits (`0..8`), and if they're in the range
you're after you're good. If not you can discard those and look at the next 3
bits. We started with 16 of them, so you get five chances before you have to run
the generator again entirely.

The goal here is to avoid having to do one of the worst things possible in
computing: _divmod_. It's terribly expensive, even on a modern computer it's
about 10x as expensive as any other arithmetic, and on a GBA it's even worse for
us. We have to call into the BIOS to have it do a software division. Calling
into the BIOS at all is about a 60 cycle overhead (for comparison, a normal
function call is more like 30 cycles of overhead), _plus_ the time it takes to
do the math itself. Remember earlier how we were happy to have a savings of 5
instructions here or there? Compared to this, all our previous efforts are
basically useless if we can't evade having to do a divmod. You can do quite a
bit of `if` checking and potential additional generator calls before it exceeds
the cost of having to do even a single divmod.

### Calling The BIOS

How do we do the actual divmod when we're forced to? Easy: [inline
assembly](https://doc.rust-lang.org/unstable-book/language-features/asm.html) of
course (There's also an [ARM
oriented](http://embed.rs/articles/2016/arm-inline-assembly-rust/) blog post
about it that I found most helpful). The GBA has many [BIOS
Functions](http://problemkaputt.de/gbatek.htm#biosfunctions), each of which has
a designated number. We use the
[swi](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0068b/BABFCEEG.html)
op (short for "SoftWare Interrupt") combined with the BIOS function number that
we want performed. Our code halts, some setup happens (hence that 60 cycles of
overhead I mentioned), the BIOS does its thing, and then eventually control
returns to us.

The precise details of what the BIOS call does depends on the function number
that we call. We'd even have to potentially mark it as volatile asm if there's
no clear outputs, otherwise the compiler would "helpfully" eliminate it for us
during optimization. In our case there _are_ clear outputs. The numerator goes
into register 0, and the denominator goes into register 1, the divmod happens,
and then the division output is left in register 0 and the modulus output is
left in register 1. I keep calling it "divmod" because div and modulus are two
sides of the same coin. There's no way to do one of them faster by not doing the
other or anything like that, so we'll first define it as a unified function that
returns a tuple:

```rust
#![feature(asm)]
// put the above at the top of any program and/or library that uses inline asm

pub fn div_modulus(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  {
    let div_out: i32;
    let mod_out: i32;
    unsafe {
      asm!(/* assembly template */ "swi 0x06"
          :/* output operands */ "={r0}"(div_out), "={r1}"(mod_out)
          :/* input operands */ "{r0}"(numerator), "{r1}"(denominator)
          :/* clobbers */ "r3"
          :/* options */
    );
    }
    (div_out, mod_out)
  }
}
```

And next, since most of the time we really do want just the `div` or `modulus`
without having to explicitly throw out the other half, we also define
intermediary functions to unpack the correct values.

```rust
pub fn div(numerator: i32, denominator: i32) -> i32 {
  div_modulus(numerator, denominator).0
}

pub fn modulus(numerator: i32, denominator: i32) -> i32 {
  div_modulus(numerator, denominator).1
}
```

We can generally trust the compiler to inline single line functions correctly
even without an `#[inline]` directive when it's not going cross-crate or when
LTO is on. I'd point you to some exact output from the Compiler Explorer, but at
the time of writing their nightly compiler is broken, and you can only use
inline asm with a nightly compiler. Unfortunate. Hopefully they'll fix it soon
and I can come back to this section with some links.

### Finally Those Random Ranges We Mentioned

Of course, now that we can do divmod if we need to, let's get back to random
numbers in ranges that aren't exact powers of two.

yada yada yada, if you just use `x % n` to place `x` into the range `0..n` then
you'll turn an unbiased value into a biased value (or you'll turn a biased value
into an arbitrarily _more_ biased value). You should never do this, etc etc.

So what's a good way to get unbiased outputs? We're going to be adapting some
CPP code from that  that I first hinted at way up above. It's specifically all
about the various ways you can go about getting unbiased random results for
various bounds. There's actually many different methods offered, and for
specific situations there's sometimes different winners for speed. The best
overall performer looks like this:

```cpp
uint32_t bounded_rand(rng_t& rng, uint32_t range) {
    uint32_t x = rng();
    uint64_t m = uint64_t(x) * uint64_t(range);
    uint32_t l = uint32_t(m);
    if (l < range) {
        uint32_t t = -range;
        if (t >= range) {
            t -= range;
            if (t >= range) 
                t %= range;
        }
        while (l < t) {
            x = rng();
            m = uint64_t(x) * uint64_t(range);
            l = uint32_t(m);
        }
    }
    return m >> 32;
}
```

And, wow, I sure don't know what a lot of that means (well, I do, but let's
pretend I don't for dramatic effect, don't tell anyone). Let's try to pick it
apart some.

First, all the `uint32_t` and `uint64_t` are C nonsense names for what we just
call `u32` and `u64`. You probably guessed that on your own.

Next, `rng_t& rng` is more properly written as `rng: &rng_t`. Though, here
there's a catch: as you can see we're calling `rng` within the function, so in
rust we'd need to declare it as `rng: &mut rng_t`, because C++ doesn't track
mutability the same as we do (barbaric, I know).

Finally, what's `rng_t` actually defined as? Well, I sure don't know, but in our
context it's taking nothing and then spitting out a `u32`. We'll also presume
that it's a different `u32` each time (not a huge leap in this context). To us
rust programmers that means we'd want something like `impl FnMut() -> u32`.

```rust
pub fn bounded_rand(rng: &mut impl FnMut() -> u32, range: u32) -> u32 {
  let mut x: u32 = rng();
  let mut m: u64 = x as u64 * range as u64;
  let mut l: u32 = m as u32;
  if l < range {
    let mut t: u32 = range.wrapping_neg();
    if t >= range {
      t -= range;
      if t >= range {
        t = modulus(t, range);
      }
    }
    while l < t {
      x = rng();
      m = x as u64 * range as u64;
      l = m as u32;
    }
  }
  (m >> 32) as u32
}
```

So, now we can read it. Can we compile it? No, actually. Turns out we can't.
Remember how our `modulus` function is `(i32, i32) -> i32`? Here we're doing
`(u32, u32) -> u32`. You can't just cast, modulus, and cast back. You'll get
totally wrong results most of the time because of sign-bit stuff. Since it's
fairly probable that `range` fits in a positive `i32`, its negation must
necessarily be a negative value, which triggers exactly the bad situation where
casting around gives us the wrong results.

Well, that's not the worst thing in the world either, since we also didn't
really wanna be doing those 64-bit multiplies. Let's try again with everything
scaled down one stage:

```rust
pub fn bounded_rand16(rng: &mut impl FnMut() -> u16, range: u16) -> u16 {
  let mut x: u16 = rng();
  let mut m: u32 = x as u32 * range as u32;
  let mut l: u16 = m as u16;
  if l < range {
    let mut t: u16 = range.wrapping_neg();
    if t >= range {
      t -= range;
      if t >= range {
        t = modulus(t as i32, range as i32) as u16;
      }
    }
    while l < t {
      x = rng();
      m = x as u32 * range as u32;
      l = m as u16;
    }
  }
  (m >> 16) as u16
}
```

Okay, so the code compiles, _and_ it plays nicely what the known limits of the
various number types involved. We know that if we cast a `u16` up into `i32`
it's assured to fit properly and also be positive, and the output is assured to
be smaller than the input so it'll fit when we cast it back down to `u16`.
What's even happening though? Well, this is a variation on [Lemire's
method](https://arxiv.org/abs/1805.10941). One of the biggest attempts at a
speedup here is that when you have

```rust
a %= b;
```

You can translate that into 

```rust
if a >= b {
  a -= b;
  if a >= b {
    a %= b;
  }
}
```

Now... if we're being real with ourselves, let's just think about this for a
moment. How often will this help us? I genuinely don't know. But I do know how
to find out: we write a program to just [enumerate all possible
cases](https://play.rust-lang.org/?version=stable&mode=release&edition=2015&gist=48b36f8c9f6a3284c0bc65366a4fab47)
and run the code. You can't always do this, but there's not many possible `u16`
values. The output is this:

```
skip_all:32767
sub_worked:10923
had_to_modulus:21846
Some skips:
32769
32770
32771
32772
32773
Some subs:
21846
21847
21848
21849
21850
Some mods:
0
1
2
3
4
```

So, about half the time, we're able to skip all our work, and about a sixth of
the time we're able to solve it with just the subtract, with the other third of
the time we have to do the mod. However, what I personally care about the most
is smaller ranges, and we can see that we'll have to do the mod if our target
range size is in `0..21846`, and just the subtract if our target range size is
in `21846..32769`, and we can only skip all work if our range size is `32769`
and above. So that's not cool.

But what _is_ cool is that we're doing the modulus only once, and the rest of
the time we've just got the cheap operations. Sounds like we can maybe try to
cache that work and reuse a range of some particular size. We can also get that
going pretty easily.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandRangeU16 {
  range: u16,
  threshold: u16,
}

impl RandRangeU16 {
  pub fn new(mut range: u16) -> Self {
    let mut threshold = range.wrapping_neg();
    if threshold >= range {
      threshold -= range;
      if threshold >= range {
        threshold = modulus(threshold as i32, range as i32) as u16;
      }
    }
    RandRangeU16 { range, threshold }
  }

  pub fn roll_random(&self, rng: &mut impl FnMut() -> u16) -> u16 {
    let mut x: u16 = rng();
    let mut m: u32 = x as u32 * self.range as u32;
    let mut l: u16 = m as u16;
    if l < self.range {
      while l < self.threshold {
        x = rng();
        m = x as u32 * self.range as u32;
        l = m as u16;
      }
    }
    (m >> 16) as u16
  }
}
```

What if you really want to use ranges bigger than `u16`? Well, that's possible,
but we'd want a whole new technique. Preferably one that didn't do divmod at
all, to avoid any nastiness with sign bit nonsense. Thankfully there is one such
method listed in the blog post, "Bitmask with Rejection (Unbiased)"

```cpp
uint32_t bounded_rand(rng_t& rng, uint32_t range) {
    uint32_t mask = ~uint32_t(0);
    --range;
    mask >>= __builtin_clz(range|1);
    uint32_t x;
    do {
        x = rng() & mask;
    } while (x > range);
    return x;
}
```

And in Rust

```rust
pub fn bounded_rand32(rng: &mut impl FnMut() -> u32, mut range: u32) -> u32 {
  let mut mask: u32 = !0;
  range -= 1;
  mask >>= (range | 1).leading_zeros();
  let mut x = rng() & mask;
  while x > range {
    x = rng() & mask;
  }
  x
}
```

Wow, that's so much less code. What the heck? Less code is _supposed_ to be the
faster version, why is this rated slower? Basically, because of how the math
works out on how often you have to run the PRNG again and stuff, Lemire's method
_usually_ better with smaller ranges and the masking method _usually_ works
better with larger ranges. If your target range fits in a `u8`, probably use
Lemire's. If it's bigger than `u8`, or if you need to do it just once and can't
benefit from the cached modulus, you might want to start moving toward the
masking version at some point in there. Obviously if your target range is more
than a `u16` then you have to use the masking method. The fact that they're each
oriented towards different size generator outputs only makes things more
complicated.

Life just be that way, I guess.

## Summary Table

That was a whole lot. Let's put them in a table:

| Generator      | Bytes | Output | Period | k-Dim |
|:---------------|:-----:|:------:|:------:|:-----:|
| sm64           | 2     | u16    | 65,114 | 0     |
| lcg32          | 4     | u16    | 2^32   | 1     |
| pcg16_xsh_rs   | 4     | u16    | 2^32   | 1     |
| pcg32_rxs_m_xs | 4     | u32    | 2^32   | 1     |
| PCG16Ext8      | 20    | u16    | 2^160  | 8     |
| xoshiro128**   | 16    | u32    | 2^128-1| 0     |
| jsf32          | 16    | u32    | ~2^126 | 0     |
