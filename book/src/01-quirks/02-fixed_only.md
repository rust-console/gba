# Fixed Only

In addition to not having much of the standard library available, we don't even
have a floating point unit available! We can't do floating point math in
hardware! We _could_ still do floating point math as pure software computations
if we wanted, but that's a slow, slow thing to do.

Are there faster ways? It's the same answer as always: "Yes, but not without a
tradeoff."

The faster way is to represent fractional values using a system called a [Fixed
Point Representation](https://en.wikipedia.org/wiki/Fixed-point_arithmetic).
What do we trade away? Numeric range.

* Floating point math stores bits for base value and for exponent all according
  to a single [well defined](https://en.wikipedia.org/wiki/IEEE_754) standard
  for how such a complicated thing works.
* Fixed point math takes a normal integer (either signed or unsigned) and then
  just "mentally associates" it (so to speak) with a fractional value for its
  "units". If you have 3 and it's in units of 1/2, then you have 3/2, or 1.5
  using decimal notation. If your number is 256 and it's in units of 1/256th
  then the value is 1.0 in decimal notation.

Floating point math requires dedicated hardware to perform quickly, but it can
"trade" precision when it needs to represent extremely large or small values.

Fixed point math is just integral math, which our GBA is reasonably good at, but
because your number is associated with a fixed fraction your results can get out
of range very easily.

## Representing A Fixed Point Value

So we want to associate our numbers with a mental note of what units they're in:

* [PhantomData](https://doc.rust-lang.org/core/marker/struct.PhantomData.html)
  is a type that tells the compiler "please remember this extra type info" when
  you add it as a field to a struct. It goes away at compile time, so it's
  perfect for us to use as space for a note to ourselves without causing runtime
  overhead.
* The [typenum](https://crates.io/crates/typenum) crate is the best way to
  represent a number within a type in Rust. Since our values on the GBA are
  always specified as a number of fractional bits to count the number as, we can
  put `typenum` types such as `U8` or `U14` into our `PhantomData` to keep track
  of what's going on.

Now, those of you who know me, or perhaps just know my reputation, will of
course _immediately_ question what happened to the real Lokathor. I do not care
for most crates, and I particularly don't care for using a crate in teaching
situations. However, `typenum` has a number of factors on its side that let me
suggest it in this situation:

* It's version 1.10 with a total of 21 versions and nearly 700k downloads, so we
  can expect that the major troubles have been shaken out and that it will remain
  fairly stable for quite some time to come.
* It has no further dependencies that it's going to drag into the compilation.
* It happens all at compile time, so it's not clogging up our actual game with
  any nonsense.
* The (interesting) subject of "how do you do math inside Rust's trait system?" is
  totally separate from the concern that we're trying to focus on here.

Therefore, we will consider it acceptable to use this crate.

Now the `typenum` crate defines a whole lot, but we'll focus down to just a
single type at the moment:
[UInt](https://docs.rs/typenum/1.10.0/typenum/uint/struct.UInt.html) is a
type-level unsigned value. It's like `u8` or `u16`, but while they're types that
then have values, each `UInt` construction statically equates to a specific
value. Like how the `()` type only has one value, which is also called `()`. In
this case, you wrap up `UInt` around smaller `UInt` values and a `B1` or `B0`
value to build up the binary number that you want at the type level.

In other words, instead of writing

```rust
let six = 0b110;
```

We write

```rust
type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
```

Wild, I know. If you look into the `typenum` crate you can do math and stuff
with these type level numbers, and we will a little bit below, but to start off
we _just_ need to store one in some `PhantomData`.

### A struct For Fixed Point

Our actual type for a fixed point value looks like this:

```rust
use core::marker::PhantomData;
use typenum::marker_traits::Unsigned;

/// Fixed point `T` value with `F` fractional bits.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fx<T, F: Unsigned> {
  bits: T,
  _phantom: PhantomData<F>,
}
```

This says that `Fx<T,F>` is a generic type that holds some base number type `T`
and a `F` type that's marking off how many fractional bits we're using. We only
want people giving unsigned type-level values for the `PhantomData` type, so we
use the trait bound `F: Unsigned`.

We use
[repr(transparent)](https://github.com/rust-lang/rfcs/blob/master/text/1758-repr-transparent.md)
here to ensure that `Fx` will always be treated just like the base type in the
final program (in terms of bit pattern and ABI).

If you go and check, this is _basically_ how the existing general purpose crates
for fixed point math represent their numbers. They're a little fancier about it
because they have to cover every case, and we only have to cover our GBA case.

That's quite a bit to type though. We probably want to make a few type aliases
for things to be easier to look at. Unfortunately there's [no standard
notation](https://en.wikipedia.org/wiki/Fixed-point_arithmetic#Notation) for how
you write a fixed point type. We also have to limit ourselves to what's valid
for use in a Rust type too. I like the `fx` thing, so we'll use that for signed
and then `fxu` if we need an unsigned value.

```rust
/// Alias for an `i16` fixed point value with 8 fractional bits.
pub type fx8_8 = Fx<i16,U8>;
```

Rust will complain about having `non_camel_case_types`, and you can shut that
warning up by putting an `#[allow(non_camel_case_types)]` attribute on the type
alias directly, or you can use `#![allow(non_camel_case_types)]` at the very top
of the module to shut up that warning for the whole module (which is what I
did).

## Constructing A Fixed Point Value

So how do we actually _make_ one of these values? Well, we can always just wrap or unwrap any value in our `Fx` type:

```rust
impl<T, F: Unsigned> Fx<T, F> {
  /// Uses the provided value directly.
  pub fn from_raw(r: T) -> Self {
    Fx {
      num: r,
      phantom: PhantomData,
    }
  }
  /// Unwraps the inner value.
  pub fn into_raw(self) -> T {
    self.num
  }
}
```

I'd like to use the `From` trait of course, but it was giving me some trouble, i
think because of the orphan rule. Oh well.

If we want to be particular to the fact that these are supposed to be
_numbers_... that gets tricky. Rust is actually quite bad at being generic about
number types. You can use the [num](https://crates.io/crates/num) crate, or you
can just use a macro and invoke it once per type. Guess what we're gonna do.

```rust
macro_rules! fixed_point_methods {
  ($t:ident) => {
    impl<F: Unsigned> Fx<$t, F> {
      /// Gives 0 for this type.
      pub fn zero() -> Self {
        Fx {
          num: 0,
          phantom: PhantomData,
        }
      }

      /// Gives the smallest positive non-zero value.
      pub fn precision() -> Self {
        Fx {
          num: 1,
          phantom: PhantomData,
        }
      }

      /// Makes a value with the integer part shifted into place.
      pub fn from_int_part(i: $t) -> Self {
        Fx {
          num: i << F::to_u8(),
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_methods! {u8}
fixed_point_methods! {i8}
fixed_point_methods! {i16}
fixed_point_methods! {u16}
fixed_point_methods! {i32}
fixed_point_methods! {u32}
```

Now _you'd think_ that those can all be `const`, but at the moment you can't
have a `const` function with a bound on any trait other than `Sized`, so they
have to be normal functions.

Also, we're doing something a little interesting there with `from_int_part`. We
can take our `F` type and get it as a value instead of a type using `to_u8`.

## Casting Values

Next, once we have a value in one type, we need to be able to move it into
another type. A particular `Fx` type is a base number type and a fractional
count, so there's two ways we might want to move it.

For casting the base type it's a little weird. Because there's so many number
types, and we can't be generic about them when using `as`, we'd have to make
like 30 functions (6 base number types we're using, times 5 target number types
you could cast to). Instead, we'll write it just once, and let the user pass a
closure that does the cast.

We can put this as part of the basic impl block that `from_raw` and `into_raw`
are part of. If can avoid having code inside a macro we'll do it just because
macros are messy.

```rust
  /// Casts the base type, keeping the fractional bit quantity the same.
  pub fn cast_inner<Z, C: Fn(T) -> Z>(self, op: C) -> Fx<Z, F> {
    Fx {
      num: op(self.num),
      phantom: PhantomData,
    }
  }
```

It's... not the best to have to pass in the casting operation like that.
Hopefully we won't have to use it much.

Also we might want to change the amount of fractional bits in a number. Oh,
gosh, this one is kinda complicated.

## Addition / Subtraction

## Multiplication / Division

## Trigonometry

## Just Using A Crate

If you feel too intimidated by all of this then I'll suggest to you that the
[fixed](https://crates.io/crates/fixed) crate seems to be the best crate
available for fixed point math.

_I have not tested its use on the GBA myself_.

It's just my recommendation from looking at the docs of the various options
available.
