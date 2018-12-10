# Newtype

There's one thing I want to get out of the way near the start of the book and it
didn't really have a good fit anywhere else in the book so it goes right here.

We're talking about the "Newtype Pattern"!

Now, I told you to read the Rust Book before you read this book, and I'm sure
you're all good students who wouldn't sneak into this book without doing the
required reading, so I'm sure you all remember exactly what I'm talking about,
because they touch on the newtype concept in the book twice, in two _very_ long
named sections:

* [Using the Newtype Pattern to Implement External Traits on External
  Types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types)
* [Using the Newtype Pattern for Type Safety and
  Abstraction](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction)

...Yeah... The Rust Book doesn't know how to make a short sub-section name to
save its life. Shame.

## Newtype Basics

So, we have all these pieces of data, and we want to keep them separated, and we
don't wanna pay the cost for it at runtime. Well, we're in luck, we can pay the
cost at compile time.

```rust
pub struct PixelColor(u16);
```

Ah, except that, as I'm sure you remember from [The
Rustonomicon](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent)
(and from [the
RFC](https://github.com/rust-lang/rfcs/blob/master/text/1758-repr-transparent.md)
too, of course), if we have a single field struct that's sometimes different
from having just the bare value, so we should be using `#[repr(transparent)]`
with our newtypes.

```rust
#[repr(transparent)]
pub struct PixelColor(u16);
```

Ah, and of course we'll need to make it so you can unwrap the value:

```rust
#[repr(transparent)]
pub struct PixelColor(u16);

impl From<PixelColor> for u16 {
  fn from(color: PixelColor) -> u16 {
    color.0
  }
}
```

And then we'll need to do that same thing for _every other newtype we want_.

Except there's only two tiny parts that actually differ between newtype
declarations: the new name and the base type. All the rest is just the same rote
code over and over. Generating piles and piles of boilerplate code? Sounds like
a job for a macro to me!

## Making It A Macro

The most basic version of the macro we want goes like this:

```rust
#[macro_export]
macro_rules! newtype {
  ($new_name:ident, $old_name:ident) => {
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}
```

Except we also want to be able to add attributes (which includes doc comments),
so we upgrade our macro a bit:

```rust
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}
```

And we want to automatically add the ability to turn the wrapper type back into
the wrapped type.

```rust
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
    
    impl From<$new_name> for $old_name {
      fn from(x: $new_name) -> $old_name {
        x.0
      }
    }
  };
}
```

That seems like enough for all of our examples, so we'll stop there. We could
add more things, such as making the `From` impl optional (because what if you
shouldn't unwrap it for some weird reason?), allowing for more precise
visibility controls (on both the newtype overall and the inner field), and maybe
even other things I can't think of right now. We won't really need those in our
example code for this book, so it's probably nicer to just keep the macro
simpler and quit while we're ahead.

**As a reminder:** remember that macros have to appear _before_ they're invoked in
your source, so the `newtype` macro will always have to be at the very top of
your file, or in a module that's declared before other modules and code.
