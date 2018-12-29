# Newtype

TODO: we've already used newtype twice by now (fixed point values and volatile
addresses), so we need to adjust how we start this section.

There's a great Zero Cost abstraction that we'll be using a lot that you might
not already be familiar with: we're talking about the "Newtype Pattern"!

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

TODO: we've already talked about repr(transparent) by now

Ah, except that, as I'm sure you remember from [The
Rustonomicon](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent)
(and from the RFC too, of course), if we have a single field struct that's
sometimes different from having just the bare value, so we should be using
`#[repr(transparent)]` with our newtypes.

```rust
#[repr(transparent)]
pub struct PixelColor(u16);
```

And then we'll need to do that same thing for _every other newtype we want_.

Except there's only two tiny parts that actually differ between newtype
declarations: the new name and the base type. All the rest is just the same rote
code over and over. Generating piles and piles of boilerplate code? Sounds like
a job for a macro to me!

## Making It A Macro

If you're going to do much with macros you should definitely read through [The
Little Book of Rust
Macros](https://danielkeep.github.io/tlborm/book/index.html), but we won't be
doing too much so you can just follow along here a bit if you like.

The most basic version of a newtype macro starts like this:

```rust
#[macro_export]
macro_rules! newtype {
  ($new_name:ident, $old_name:ident) => {
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}
```

The `#[macro_export]` makes it exported by the current module (like `pub`
kinda), and then we have one expansion option that takes an identifier, a `,`,
and then a second identifier. The new name is the outer type we'll be using, and
the old name is the inner type that's being wrapped. You'd use our new macro
something like this:

```rust
newtype! {PixelColorCurly, u16}

newtype!(PixelColorParens, u16);

newtype![PixelColorBrackets, u16];
```

Note that you can invoke the macro with the outermost grouping as any of `()`,
`[]`, or `{}`.  It makes no particular difference to the macro. Also, that space
in the first version is kinda to show off that you can put white space in
between the macro name and the grouping if you want. The difference is mostly
style, but there are some rules and considerations here:

* If you use curly braces then you _must not_ put a `;` after the invocation.
* If you use parentheses or brackets then you _must_ put the `;` at the end.
* Rustfmt cares which you use and formats accordingly:
  * Curly brace macro use mostly gets treated like a code block.
  * Parentheses macro use mostly gets treated like a function call.
  * Bracket macro use mostly gets treated like an array declaration.

**As a reminder:** remember that `macro_rules` macros have to appear _before_
they're invoked in your source, so the `newtype` macro will always have to be at
the very top of your file, or if you put it in a module within your project
you'll need to declare the module before anything that uses it.

## Upgrade That Macro!

We also want to be able to add `derive` stuff and doc comments to our newtype.
Within the context of `macro_rules!` definitions these are called "meta". Since
we can have any number of them we wrap it all up in a "zero or more" matcher.
Then our macro looks like this:

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

So now we can write

```rust
newtype! {
  /// Color on the GBA gives 5 bits for each channel, the highest bit is ignored.
  #[derive(Debug, Clone, Copy)]
  PixelColor, u16
}
```

Next, we can allow for the wrapping of types that aren't just a single
identifier by changing `$old_name` from `:ident` to `:ty`. We can't _also_ do
this for the `$new_type` part because declaring a new struct expects a valid
identifier that's _not_ already declared (obviously), and `:ty` is intended for
capturing types that already exist.

```rust
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ty) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}
```

Next of course we'll want to usually have a `new` method that's const and just
gives a 0 value. We won't always be making a newtype over a number value, but we
often will. It's usually silly to have a `new` method with no arguments since we
might as well just impl `Default`, but `Default::default` isn't `const`, so
having `pub const fn new() -> Self` is justified here.

Here, the token `0` is given the `{integer}` type, which can be converted into
any of the integer types as needed, but it still can't be converted into an
array type or a pointer or things like that. Accordingly we've added the "no
frills" option which declares the struct and no `new` method.

```rust
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ty) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $old_name:ty, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}
```

Finally, we usually want to have the wrapped value be totally private, but there
_are_ occasions where that's not the case. For this, we can allow the wrapped
field to accept a visibility modifier.

```rust
#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
    impl $new_name {
      /// A `const` "zero value" constructor
      pub const fn new() -> Self {
        $new_name(0)
      }
    }
  };
  ($(#[$attr:meta])* $new_name:ident, $v:vis $old_name:ty, no frills) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($v $old_name);
  };
}
```
