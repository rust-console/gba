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

We will need to use the `const_generics` rustc feature. In short, it's a way to
use constants in types we define the same way we can use them in array
definitions (`[u16; 8]`).

Our actual type for a fixed point value looks like this:

```rust
#![features(const_generics)]

/// Fixed point `T` value with `F` fractional bits.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Fx<T, const F: usize> {
  bits: T
}
```

This says that `Fx<T,F>` is a generic type that holds some base number type `T`
and a `F` type that's marking off how many fractional bits we're using.

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
pub type fx8_8 = Fx<i16,8>;
```

Rust will complain about having `non_camel_case_types`, and you can shut that
warning up by putting an `#[allow(non_camel_case_types)]` attribute on the type
alias directly, or you can use `#![allow(non_camel_case_types)]` at the very top
of the module to shut up that warning for the whole module (which is what I
did).

## Constructing A Fixed Point Value

So how do we actually _make_ one of these values? Well, we can always just wrap or unwrap any value in our `Fx` type:

```rust
impl<T, const F: usize> Fx<T, F> {
  /// Uses the provided value directly.
  pub fn from_raw(r: T) -> Self {
    Fx { bits: r }
  }
  /// Unwraps the inner value.
  pub fn into_raw(self) -> T {
    self.bits
  }
}
```

I'd like to use the `From` trait of course, but it was giving me some trouble, i
think because of the orphan rule. Oh well.

```rust
macro_rules! fixed_point_methods {
  ($t:ident) => {
    impl<const F: usize> Fx<$t, F> {
      /// Gives the smallest positive non-zero value.
      pub fn precision() -> Self {
        Fx { bits: 1 }
      }

      /// Makes a value with the integer part shifted into place.
      pub fn from_int_part(i: $t) -> Self {
        Fx { bits: i << F as u8 }
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

Now _you'd think_ that those can be `const`, but at the moment you can't have a
`const` function with a bound on any trait other than `Sized`, so they have to
be normal functions.

Also, we're doing something a little interesting there with `from_int_part`. We
can take our `F` type and get its constant value. There's other associated
constants if we want it in other types, and also non-const methods if you wanted
that for some reason (maybe passing it as a closure function? dunno).

## Casting Base Values

Next, once we have a value in one base type we will need to be able to move it
into another base type. Unfortunately this means we gotta use the `as` operator,
which requires a concrete source type and a concrete destination type. There's
no easy way for us to make it generic here.

We could let the user use `into_raw`, cast, and then do `from_raw`, but that's
error prone because they might change the fractional bit count accidentally.
This means that we have to write a function that does the casting while
perfectly preserving the fractional bit quantity. If we wrote one function for
each conversion it'd be like 30 different possible casts (6 base types that we
support, and then 5 possible target types). Instead, we'll write it just once in
a way that takes a closure, and let the user pass a closure that does the cast.
The compiler should merge it all together quite nicely for us once optimizations
kick in.

This code goes outside the macro. I want to avoid too much code in the macro if
we can, it's a little easier to cope with I think.

```rust
  /// Casts the base type, keeping the fractional bit quantity the same.
  pub fn cast_inner<Z, C: Fn(T) -> Z>(self, op: C) -> Fx<Z, F> {
    Fx { bits: op(self.num) }
  }
```

It's horrible and ugly, but Rust is just bad at numbers sometimes.

## Adjusting Fractional Part

In addition to the base value we might want to change our fractional bit
quantity. This is actually easier that it sounds, but it also requires us to be
tricky with the generics. We can actually use some typenum type level operators
here.

This code goes inside the macro: we need to be able to use the left shift and
right shift, which is easiest when we just use the macro's `$t` as our type. We
could alternately put a similar function outside the macro and be generic on `T`
having the left and right shift operators by using a `where` clause. As much as
I'd like to avoid too much code being generated by macro, I'd _even more_ like
to avoid generic code with huge and complicated trait bounds. It comes down to
style, and you gotta decide for yourself.

```rust
      /// Changes the fractional bit quantity, keeping the base type the same.
      pub fn adjust_fractional_bits<Y: Unsigned + IsEqual<F, Output = False>>(self) -> Fx<$t, Y> {
        let leftward_movement: i32 = Y::to_i32() - F::to_i32();
        Fx {
          num: if leftward_movement > 0 {
            self.num << leftward_movement
          } else {
            self.num >> (-leftward_movement)
          },
          phantom: PhantomData,
        }
      }
```

There's a few things at work. First, we introduce `Y` as the target number of
fractional bits, and we _also_ limit it that the target bits quantity can't be
the same as we already have using a type-level operator. If it's the same as we
started with, why are you doing the cast at all?

Now, once we're sure that the current bits and target bits aren't the same, we
compute `target - start`, and call this our "leftward movement". Example: if
we're targeting 8 bits and we're at 4 bits, we do 8-4 and get +4 as our leftward
movement. If the leftward_movement is positive we naturally shift our current
value to the left. If it's not positive then it _must_ be negative because we
eliminated 0 as a possibility using the type-level operator, so we shift to the
right by the negative value.

## Addition, Subtraction, Shifting, Negative, Comparisons

From here on we're getting help from [this blog
post](https://spin.atomicobject.com/2012/03/15/simple-fixed-point-math/) by [Job
Vranish](https://spin.atomicobject.com/author/vranish/), so thank them if you
learn something.

I might have given away the game a bit with those `derive` traits on our fixed
point type. For a fair number of operations you can use the normal form of the
op on the inner bits as long as the fractional parts have the same quantity.
This includes equality and ordering (which we derived) as well as addition,
subtraction, and bit shifting (which we need to do ourselves).

This code can go outside the macro, with sufficient trait bounds.

```rust
impl<T: Add<Output = T>, const F: usize> Add for Fx<T, F> {
  type Output = Self;
  fn add(self, rhs: Fx<T, F>) -> Self::Output {
    Fx {
      num: self.num + rhs.num,
      phantom: PhantomData,
    }
  }
}
```

The bound on `T` makes it so that `Fx<T, F>` can be added any time that `T` can
be added to its own type with itself as the output. We can use the exact same
pattern for `Sub`, `Shl`, `Shr`, and `Neg`. With enough trait bounds, we can do
anything!

```rust
impl<T: Sub<Output = T>, const F: usize> Sub for Fx<T, F> {
  type Output = Self;
  fn sub(self, rhs: Fx<T, F>) -> Self::Output {
    Fx {
      num: self.num - rhs.num,
      phantom: PhantomData,
    }
  }
}

impl<T: Shl<u32, Output = T>, const F: usize> Shl<u32> for Fx<T, F> {
  type Output = Self;
  fn shl(self, rhs: u32) -> Self::Output {
    Fx {
      num: self.num << rhs,
      phantom: PhantomData,
    }
  }
}

impl<T: Shr<u32, Output = T>, const F: usize> Shr<u32> for Fx<T, F> {
  type Output = Self;
  fn shr(self, rhs: u32) -> Self::Output {
    Fx {
      num: self.num >> rhs,
      phantom: PhantomData,
    }
  }
}

impl<T: Neg<Output = T>, const F: usize> Neg for Fx<T, F> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Fx {
      num: -self.num,
      phantom: PhantomData,
    }
  }
}
```

Unfortunately, for `Shl` and `Shr` to have as much coverage on our type as it
does on the base type (allowing just about any right hand side) we'd have to do
another macro, but I think just `u32` is fine. We can always add more later if
we need.

We could also implement `BitAnd`, `BitOr`, `BitXor`, and `Not`, but they don't
seem relevent to our fixed point math use, and this section is getting long
already. Just use the same general patterns if you want to add it in your own
programs. Shockingly, `Rem` also works directly if you want it, though I don't
forsee us needing floating point remainder. Also, the GBA can't do hardware
division or remainder, and we'll have to work around that below when we
implement `Div` (which maybe we don't need, but it's complex enough I should
show it instead of letting people guess).

**Note:** In addition to the various `Op` traits, there's also `OpAssign`
variants. Each `OpAssign` is the same as `Op`, but takes `&mut self` instead of
`self` and then modifies in place instead of producing a fresh value. In other
words, if you want both `+` and `+=` you'll need to do the `AddAssign` trait
too. It's not the worst thing to just write `a = a+b`, so I won't bother with
showing all that here. It's pretty easy to figure out for yourself if you want.

## Multiplication

This is where things get more interesting. When we have two numbers `A` and `B`
they really stand for `(a*f)` and `(b*f)`. If we write `A*B` then we're really
writing `(a*f)*(b*f)`, which can be rewritten as `(a*b)*2f`, and now it's
obvious that we have one more `f` than we wanted to have. We have to do the
multiply of the inner value and then divide out the `f`. We divide by `1 <<
bit_count`, so if we have 8 fractional bits we'll divide by 256.

The catch is that, when we do the multiply we're _extremely_ likely to overflow
our base type with that multiplication step. Then we do that divide, and now our
result is basically nonsense. We can avoid this to some extent by casting up to
a higher bit type, doing the multiplication and division at higher precision,
and then casting back down. We want as much precision as possible without being
too inefficient, so we'll always cast up to 32-bit (on a 64-bit machine you'd
cast up to 64-bit instead).

Naturally, any signed value has to be cast up to `i32` and any unsigned value
has to be cast up to `u32`, so we'll have to handle those separately.

Also, instead of doing an _actual_ divide we can right-shift by the correct
number of bits to achieve the same effect. _Except_ when we have a signed value
that's negative, because actual division truncates towards zero and
right-shifting truncates towards negative infinity. We can get around _this_ by
flipping the sign, doing the shift, and flipping the sign again (which sounds
silly but it's so much faster than doing an actual division).

Also, again signed values can be annoying, because if the value _just happens_
to be `i32::MIN` then when you negate it you'll have... _still_ a negative
value. I'm not 100% on this, but I think the correct thing to do at that point
is to give `$t::MIN` as the output num value.

Did you get all that? Good, because this involves casting, so we will need to
implement it three times, which calls for another macro.

```rust
macro_rules! fixed_point_signed_multiply {
  ($t:ident) => {
    impl<const F: usize> Mul for Fx<$t, F> {
      type Output = Self;
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        let pre_shift = (self.num as i32).wrapping_mul(rhs.num as i32);
        if pre_shift < 0 {
          if pre_shift == core::i32::MIN {
            Fx {
              num: core::$t::MIN,
              phantom: PhantomData,
            }
          } else {
            Fx {
              num: (-((-pre_shift) >> F::U8)) as $t,
              phantom: PhantomData,
            }
          }
        } else {
          Fx {
            num: (pre_shift >> F::U8) as $t,
            phantom: PhantomData,
          }
        }
      }
    }
  };
}

fixed_point_signed_multiply! {i8}
fixed_point_signed_multiply! {i16}
fixed_point_signed_multiply! {i32}

macro_rules! fixed_point_unsigned_multiply {
  ($t:ident) => {
    impl<const F: usize> Mul for Fx<$t, F> {
      type Output = Self;
      fn mul(self, rhs: Fx<$t, F>) -> Self::Output {
        Fx {
          num: ((self.num as u32).wrapping_mul(rhs.num as u32) >> F::U8) as $t,
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_unsigned_multiply! {u8}
fixed_point_unsigned_multiply! {u16}
fixed_point_unsigned_multiply! {u32}
```

## Division

Division is similar to multiplication, but reversed. Which makes sense. This
time `A/B` gives `(a*f)/(b*f)` which is `a/b`, one _less_ `f` than we were
after.

As with the multiplication version of things, we have to up-cast our inner value
as much a we can before doing the math, to allow for the most precision
possible.

The snag here is that the GBA has no division or remainder. Instead, the GBA has
a BIOS function you can call to do `i32/i32` division.

This is a potential problem for us though. If we have some unsigned value, we
need it to fit within the positive space of an `i32` _after the multiply_ so
that we can cast it to `i32`, call the BIOS function that only works on `i32`
values, and cast it back to its actual type.

* If you have a u8 you're always okay, even with 8 floating bits.
* If you have a u16 you're okay even with a maximum value up to 15 floating
  bits, but having a maximum value and 16 floating bits makes it break.
* If you have a u32 you're probably going to be in trouble all the time.

So... ugh, there's not much we can do about this. For now we'll just have to
suffer some.

// TODO: find a numerics book that tells us how to do `u32/u32` divisions.

```rust
macro_rules! fixed_point_signed_division {
  ($t:ident) => {
    impl<const F: usize> Div for Fx<$t, F> {
      type Output = Self;
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F::U8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx {
          num: divide_result as $t,
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_signed_division! {i8}
fixed_point_signed_division! {i16}
fixed_point_signed_division! {i32}

macro_rules! fixed_point_unsigned_division {
  ($t:ident) => {
    impl<const F: usize> Div for Fx<$t, F> {
      type Output = Self;
      fn div(self, rhs: Fx<$t, F>) -> Self::Output {
        let mul_output: i32 = (self.num as i32).wrapping_mul(1 << F::U8);
        let divide_result: i32 = crate::bios::div(mul_output, rhs.num as i32);
        Fx {
          num: divide_result as $t,
          phantom: PhantomData,
        }
      }
    }
  };
}

fixed_point_unsigned_division! {u8}
fixed_point_unsigned_division! {u16}
fixed_point_unsigned_division! {u32}
```

## Trigonometry

TODO: look up tables! arcbits!

## Just Using A Crate

If, after seeing all that, and seeing that I still didn't even cover every
possible trait impl that you might want for all the possible types... if after
all that you feel too intimidated, then I'll cave a bit on your behalf and
suggest to you that the [fixed](https://crates.io/crates/fixed) crate seems to
be the best crate available for fixed point math.

_I have not tested its use on the GBA myself_.

It's just my recommendation from looking at the docs of the various options
available, if you really wanted to just have a crate for it.
