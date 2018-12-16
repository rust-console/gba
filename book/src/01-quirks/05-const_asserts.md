# Constant Assertions

Have you ever wanted to assert things _even before runtime_? We all have, of
course. Particularly when the runtime machine is a poor little GBA, we'd like to
have the machine doing the compile handle as much checking as possible.

Enter the [static assertions](https://docs.rs/static_assertions/) crate, which
provides a way to let you assert on a `const` expression.

This is an amazing crate that you should definitely use when you can.

It's written by [Nikolai Vazquez](https://github.com/nvzqz), and they kindly
wrote up a [blog
post](https://nikolaivazquez.com/posts/programming/rust-static-assertions/) that
explains the thinking behind it.

However, I promised that each example would be single file, and I also promised
to explain what's going on as we go, so we'll briefly touch upon giving an
explanation here.

## How We Const Assert

Alright, as it stands (2018-12-15), we can't use `if` in a `const` context.

Since we can't use `if`, we can't use a normal `assert!`. Some day it will be
possible, and a failed assert at compile time will be a compile error and a
failed assert at run time will be a panic and we'll have a nice unified
programming experience. We can add runtime-only assertions by being a little
tricky with the compiler.

If we write

```rust
const ASSERT: usize = 0 - 1;
```

that gives a warning, since the math would underflow. We can upgrade that
warning to a hard error:

```rust
#[deny(const_err)]
const ASSERT: usize = 0 - 1;
```

And to make our construction reusable we can enable the
[underscore_const_names](https://github.com/rust-lang/rust/issues/54912) feature
in our program (or library) and then give each such const an underscore for a
name.

```rust
#![feature(underscore_const_names)]

#[deny(const_err)]
const _: usize = 0 - 1;
```

Now we wrap this in a macro where we give a `bool` expression as input. We
negate the bool then cast it to a `usize`, meaning that `true` negates into
`false`, which becomes `0usize`, and then there's no underflow error. Or if the
input was `false`, it negates into `true`, then becomes `1usize`, and then the
underflow error fires.

```rust
macro_rules! const_assert {
  ($condition:expr) => {
    #[deny(const_err)]
    #[allow(dead_code)]
    const ASSERT: usize = 0 - !$condition as usize;
  }
}
```

Technically, written like this, the expression can be anything with a
`core::ops::Not` implementation that can also be `as` cast into `usize`. That's
`bool`, but also basically all the other number types.

It doesn't really hurt if you want to `const_assert!` a number I guess. I mean,
any number other than the `MAX` value of an unsigned type or the `-1` value of
an unsigned type will fail such an assertion, but I bet you'll notice that you
did something wrong pretty quick. We could use the
[type_ascription](https://github.com/rust-lang/rust/issues/23416) feature to
really force a `bool`, but it's not that critical, so we'll avoid using a
feature that we don't need until it's stable.

## Asserting Something

As an example of how we might use a `const_assert`, we'll do a demo with colors.
There's a red, blue, and green channel. We store colors in a `u16` with 5 bits
for each channel.

```rust
newtype! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  Color, u16
}
```

And when we're building a color, we're passing in `u16` values, but they could
be using more than just 5 bits of space. We want to make sure that each channel
is 31 or less, so we can make a color builder that does a `const_assert!` on the
value of each channel.

```rust
macro_rules! rgb {
  ($r:expr, $g:expr, $b:expr) => {
    {
      const_assert!($r <= 31);
      const_assert!($g <= 31);
      const_assert!($b <= 31);
      Color($b << 10 | $g << 5 | $r)
    }
  }
}
```

And then we can declare some colors

```rust
const RED: Color = rgb!(31, 0, 0);

const BLUE: Color = rgb!(31, 500, 0);
```

The second one is clearly out of bounds and it fires an error just like we
wanted.
