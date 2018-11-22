# Introduction

Here's a book that'll help you program in Rust on the Game Boy Advance (GBA).

It's a work in progress of course, but so is most of everything in Rust.

## Style and Purpose

I'm out to teach you how to program in Rust on the GBA, obviously. However,
while there _is_ a [gba](https://github.com/rust-console/gba) crate, and while I
genuinely believe it to be a good and useful crate for GBA programming, we _will
not_ be using the `gba` crate within this book. In fact we won't be using any
crates at all. We can call it the [Handmade Hero](https://handmadehero.org/)
approach, if you like.

I don't want to just teach you how to use the `gba` crate, I want to teach you
what you'd need to know to write the crate from scratch if it wasn't there.

Each chapter of the book will focus on a few things you'll need to know about
GBA programming and then present a fully self-contained example that puts those
ideas into action. Just one file per example, no dependencies, no external
assets, no fuss. The examples will be in the text of the book within code
blocks, but also you can find them in the [examples
directory](https://github.com/rust-console/gba/tree/master/examples) of the repo
if you want to get them that way.

## Expected Knowledge

I will try not to ask too much of the reader ahead of time, but you are expected
to have already read [The Rust Book](https://doc.rust-lang.org/book/). Having
also read through the [Rustonomicon](https://doc.rust-lang.org/nomicon/) is
appreciated but not required.

It's very difficult to know when you've said something that someone else won't
already know about, or if you're presenting ideas out of order. If things aren't
clear please [file an issue](https://github.com/rust-console/gba/issues) and
we'll try to address it.

## Getting Help

If you want to contact us you should join the [Rust Community
Discord](https://discordapp.com/invite/aVESxV8) and ask in the `#gamedev`
channel.

* `Ketsuban` is the wizard who knows much more about how it all works
* `Lokathor` is the fool who decided to write a crate and book for it.

If it's _not_ a GBA specific question then you can probably ask any of the other
folks in the server as well (there's a few hundred folks).

## Further Reading

If you want to read more about developing on the GBA there are some other good
resources as well:

* [TONC](https://www.coranac.com/tonc/text/toc.htm), a tutorial series written
  for C, but it's what I based the ordering of this book's sections on.
* [GBATEK](http://problemkaputt.de/gbatek.htm), a homebrew tech manual for
  GBA/NDS/DSi. We will regularly link to parts of it when talking about various
  bits of the GBA.
