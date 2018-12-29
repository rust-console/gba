# No Std

First up, as you already saw in the `hello_magic` code, we have to use the
`#![no_std]` outer attribute on our program when we target the GBA. You can find
some info about `no_std` in two official sources:

* [unstable
  book section](https://doc.rust-lang.org/unstable-book/language-features/lang-items.html#writing-an-executable-without-stdlib)
* [embedded
  book section](https://rust-embedded.github.io/book/intro/no-std.html?highlight=no_std#a--no_std--rust-environment)

The unstable book is borderline useless here because it's describing too many
things in too many words. The embedded book is much better, but still fairly
terse.

## Bare Metal

The GBA falls under what the Embedded Book calls "Bare Metal Environments".
Basically, the machine powers on and immediately begins executing some ASM code.
Our ASM startup was provided by `Ketsuban` (check the `crt0.s` file). We'll go
over _how_ it works much later on, for now it's enough to know that it does
work, and eventually control passes into Rust code.

On the rust code side of things, we determine our starting point with the
`#[start]` attribute on our `main` function. The `main` function also has a
specific type signature that's different from the usual `main` that you'd see in
Rust. I'd tell you to read the unstable-book entry on `#[start]` but they
[literally](https://doc.rust-lang.org/unstable-book/language-features/start.html)
just tell you to look at the [tracking issue for
it](https://github.com/rust-lang/rust/issues/29633) instead, and that's not very
helpful either. Basically it just _has_ to be declared the way it is, even
though there's nothing passing in the arguments and there's no place that the
return value will go. The compiler won't accept it any other way.

## No Standard Library

The Embedded Book tells us that we can't use the standard library, but we get
access to something called "libcore", which sounds kinda funny. What they're
talking about is just [the core
crate](https://doc.rust-lang.org/core/index.html), which is called `libcore`
within the rust repository for historical reasons.

The `core` crate is actually still a really big portion of Rust. The standard
library doesn't actually hold too much code (relatively speaking), instead it
just takes code form other crates and then re-exports it in an organized way. So
with just `core` instead of `std`, what are we missing?

In no particular order:

* Allocation
* Clock
* Network
* File System

The allocation system and all the types that you can use if you have a global
allocator are neatly packaged up in the
[alloc](https://doc.rust-lang.org/alloc/index.html) crate. The rest isn't as
nicely organized.

It's _possible_ to implement a fair portion of the entire standard library
within a GBA context and make the rest just panic if you try to use it. However,
do you really need all that? Eh... probably not?

* We don't need a file system, because all of our data is just sitting there in
  the ROM for us to use. When programming we can organize our `const` data into
  modules and such to keep it organized, but once the game is compiled it's just
  one huge flat address space. TODO: Parasyte says that a FS can be handy even
  if it's all just ReadOnly, so we'll eventually talk about how you might set up
  such a thing I guess, since we'll already be talking about replacements for
  three of the other four things we "lost". Maybe we'll make Parasyte write that
  section.
* Networking, well, the GBA has a Link Cable you can use to communicate with
  another GBA, but it's not really like a unix socket with TCP, so the standard
  Rust networking isn't a very good match.
* Clock is actually two different things at once. One is the ability to store
  the time long term, which is a bit of hardware that some gamepaks have in them
  (eg: pokemon ruby/sapphire/emerald). The GBA itself can't keep time while
  power is off. However, the second part is just tracking time moment to moment,
  which the GBA can totally do. We'll see how to access the timers soon enough.

Which just leaves us with allocation. Do we need an allocator? Depends on your
game. For demos and small games you probably don't need one. For bigger games
you'll maybe want to get an allocator going eventually. It's in some sense a
crutch, but it's a very useful one.

So I promise that at some point we'll cover how to get an allocator going.
Either a Rust Global Allocator (if practical), which would allow for a lot of
the standard library types to be used "for free" once it was set up, or just a
custom allocator that's GBA specific if Rust's global allocator style isn't a
good fit for the GBA (I honestly haven't looked into it).

## Bare Metal Panic

If our code panics, we usually want to see that panic message. Unfortunately,
without a way to access something like `stdout` or `stderr` we've gotta do
something a little weirder.

If our program is running within the `mGBA` emulator, version 0.7 or later, we
can access a special set of addresses that allow us to send out `CString`
values, which then appear within a message log that you can check.

We can capture this behavior by making an `MGBADebug` type, and then implement
`core::fmt::Write` for that type. Once done, the `write!` macro will let us
target the mGBA debug output channel.

When used, it looks like this:

```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  use core::fmt::Write;
  use gba::mgba::{MGBADebug, MGBADebugLevel};

  if let Some(mut mgba) = MGBADebug::new() {
    let _ = write!(mgba, "{}", info);
    mgba.send(MGBADebugLevel::Fatal);
  }
  loop {}
}
```

If you want to follow the particulars you can check the `MGBADebug` source in
the `gba` crate. Basically, there's one address you can use to try and activate
the debug output, and if it works you write your message into the "array" at
another address, and then finally write a send value to a third address. You'll
need to have read the [volatile](03-volatile_destination.md) section for the
details to make sense.

## LLVM Intrinsics

The above code will make your program fail to build in debug mode, saying that
`__clzsi2` can't be found. This is a special builtin function that LLVM attempts
to use when there's no hardware version of an operation it wants to do (in this
case, counting the leading zeros). It's not _actually_ necessary in this case,
which is why you only need it in debug mode. The higher optimization level of
release mode makes LLVM pre-compute more and fold more constants or whatever and
then it stops trying to call `__clzsi2`.

Unfortunately, sometimes a build will fail with a missing intrinsic even in
release mode.

If LLVM wants _core_ to have that intrinsic then you're in
trouble, you'll have to send a PR to the
[compiler-builtins](https://github.com/rust-lang-nursery/compiler-builtins)
repository and hope to get it into rust itself.

If LLVM wants _your code_ to have the intrinsic then you're in less trouble. You
can look up the details and then implement it yourself. It can go anywhere in
your program, as long as it has the right ABI and name. In the case of
`__clzsi2` it takes a `usize` and returns a `usize`, so you'd write something
like:

```rust
#[no_mangle]
pub extern "C" fn __clzsi2(mut x: usize) -> usize {
  //
}
```

And so on for whatever other missing intrinsic.
