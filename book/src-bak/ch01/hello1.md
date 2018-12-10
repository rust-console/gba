

## A basic hello1 explanation

So, what just happened? Even if you're used to Rust that might look pretty
strange. We'll go over most of the little parts right here, and then bigger
parts will get their own sections.

```rust
#![feature(start)]
```

This enables the [start
feature](https://doc.rust-lang.org/beta/unstable-book/language-features/start.html),
which you would normally be able to read about in the unstable book, except that
the book tells you nothing at all except to look at the [tracking
issue](https://github.com/rust-lang/rust/issues/29633).

Basically, a GBA game is even more low-level than the _normal_ amount of
low-level that you get from Rust, so we have to tell the compiler to account for
that by specifying a `#[start]`, and we need this feature on to do that.

```rust
#![no_std]
```

There's no standard library available on the GBA, so we'll have to live a core
only life.

```rust
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
```

This sets our [panic
handler](https://doc.rust-lang.org/nightly/nomicon/panic-handler.html).
Basically, if we somehow trigger a panic, this is where the program goes.
However, right now we don't know how to get any sort of message out to the user
so... we do nothing at all. We _can't even return_ from here, so we just sit in
an infinite loop. The player will have to reset the universe from the outside.

```rust
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
```

This is our `#[start]`. We call it `main`, but it's not like a `main` that you'd
see in a Rust program. It's _more like_ the sort of `main` that you'd see in a C
program, but it's still **not** that either. If you compile a `#[start]` program
for a target with an OS such as `arm-none-eabi-nm` you can open up the debug
info and see that your result will have the symbol for the C `main` along side
the symbol for the start `main` that we write here. Our start `main` is just its
own unique thing, and the inputs and outputs have to be like that because that's
how `#[start]` is specified to work in Rust.

If you think about it for a moment you'll probably realize that, those inputs
and outputs are totally useless to us on a GBA. There's no OS on the GBA to call
our program, and there's no place for our program to "return to" when it's done.

Side note: if you want to learn more about stuff "before main gets called" you
can watch a great [CppCon talk](https://www.youtube.com/watch?v=dOfucXtyEsU) by
Matt Godbolt (yes, that Godbolt) where he delves into quite a bit of it. The
talk doesn't really apply to the GBA, but it's pretty good.

```rust
  unsafe {
```

I hope you're all set for some `unsafe`, because there's a lot of it to be had.

```rust
    (0x04000000 as *mut u16).write_volatile(0x0403);
```

Sure!

```rust
    (0x06000000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x06000000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x06000000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
```

Ah, of course.

```rust
    loop {}
  }
}
```

And, as mentioned above, there's no place for a GBA program to "return to", so
we can't ever let `main` try to return there. Instead, we go into an infinite
`loop` that does nothing. The fact that this doesn't ever return an `isize`
value doesn't seem to bother Rust, because I guess we're at least not returning
any other type of thing instead.

Fun fact: unlike in C++, an infinite loop with no side effects isn't Undefined
Behavior for us rustaceans... _semantically_. In truth LLVM has a [known
bug](https://github.com/rust-lang/rust/issues/28728) in this area, so we won't
actually be relying on empty loops in any future programs.

## All Those Magic Numbers

Alright, I cheated quite a bit in the middle there. The program works, but I
didn't really tell you why because I didn't really tell you what any of those
magic numbers mean or do.

* `0x04000000` is the address of an IO Register called the Display Control.
* `0x06000000` is the start of Video RAM.

So we write some magic to the display control register once, then we write some
other magic to three magic locations in the Video RAM. Somehow that shows three
dots. Gotta read on to find out why!
