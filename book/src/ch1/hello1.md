# hello1

Ready? Here goes:

`hello1.rs`

```rust
#![feature(start)]
#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x04000000 as *mut u16).write_volatile(0x0403);
    (0x06000000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x06000000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x06000000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}
```

Throw that into your project, build the program (as described back in Chapter
0), and give it a run. You should see a red, green, and blue dot close-ish to
the middle of the screen. If you don't, something already went wrong. Double
check things, phone a friend, write your senators, try asking Ketsuban on the
[Rust Community Discord](https://discordapp.com/invite/aVESxV8), until you're
able to get your three dots going.

## Explaining hello1

So, what just happened? Even if you're used to Rust that might look pretty
strange. We'll go over each part extra carefully.

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
#[cfg(not(test))]
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

The `#[cfg(not(test))]` part makes this item only exist in the program when
we're _not_ in a test build. This is so that `cargo test` and such work right as
much as possible.

```rust
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
```

This is our `#[start]`. We call it `main`, but the signature looks a lot more
like the main from C than it does the main from Rust. Actually, those inputs are
useless, because nothing will be calling our code from the outside. Similarly,
it's totally undefined to return anything, so the fact that we output an `isize`
is vacuously true at best. We just have to use this function signature because
that's how `#[start]` works, not because the inputs and outputs are meaningful.

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
other magic to three locations of magic to the Video RAM. We get three dots,
each in their own location... so that second part makes sense at least.

We'll get into the magic number details in the other sections of this chapter.

## Sidebar: Volatile

We'll get into what all that is in a moment, but first let's ask ourselves: Why
are we doing _volatile_ writes? You've probably never used it before at all.
What is volatile anyway?

Well, the optimizer is pretty aggressive some of the time, and so it'll skip
reads and writes when it thinks can. Like if you write to a pointer once, and
then again a moment later, and it didn't see any other reads in between, it'll
think that it can just skip doing that first write since it'll get overwritten
anyway. Sometimes that's right, but sometimes it's wrong.

Marking a read or write as _volatile_ tells the compiler that it really must do
that action, and in the exact order that we wrote it out. It says that there
might even be special hardware side effects going on that the compiler isn't
aware of. In this case, the Display Control write sets a video mode, and the
Video RAM writes set pixels that will show up on the screen.

Similar to "atomic" operations you might have heard about, all volatile
operations are enforced to happen in the exact order that you specify them, but
only relative to other volatile operations. So something like

```rust
c.volatile_write(5);
a += b;
d.volatile_write(7);
```

might end up changing `a` either before or after the change to `c`, but the
write to `d` will _always_ happen after the write to `c`.

If you ever use volatile stuff on other platforms it's important to note that
volatile doesn't make things thread-safe, you still need atomic for that.
However, the GBA doesn't have threads, so we don't have to worry about thread
safety concerns.
