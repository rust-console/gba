# Help and Resources

## Help

So you're stuck on a problem and the book doesn't say what to do. Where can you
find out more?

The first place I would suggest is the [Rust Community
Discord](https://discordapp.com/invite/aVESxV8). If it's a general Rust question
then you can ask anyone in any channel you feel is appropriate. If it's GBA
specific then you can try asking me (`Lokathor`) or `Ketsuban` in the `#gamedev`
channel.

## Emulators

You certainly might want to eventually write a game that you can put on a flash
cart and play on real hardware, but for most of your development you'll probably
want to be using an emulator for testing, because you don't have to fiddle with
cables and all that.

In terms of emulators, you want to be using
[mGBA](https://github.com/mgba-emu/mgba), and you want to be using the [0.7 Beta
1](https://github.com/mgba-emu/mgba/releases/tag/0.7-b1) or later. This update
lets you run raw ELF files, which means that you can have full debug symbols
available while you're debugging problems.

## Information Resources

Ketsuban and I didn't magically learn this all from nowhere, we read various
technical manuals and guides ourselves and then distilled the knowledge (usually
oriented towards C and C++) into this book for Rust.

We have personally used some or all of the following:

* [GBATEK](http://problemkaputt.de/gbatek.htm): This is _the_ resource. It
  covers not only the GBA, but also the DS and DSi, and also a run down of ARM
  assembly (32-bit and 16-bit opcodes). The link there is to the 2.9b version on
  `problemkaputt.de` (the official home of the document), but if you just google
  for gbatek the top result is for the 2.5 version on `akkit.org`, so make sure
  you're looking at the newest version. Sometimes `problemkaputt.de` is a little
  sluggish so I've also [mirrored](https://lokathor.com/gbatek.html) the 2.9b
  version on my own site as well. GBATEK is rather large, over 2mb of text, so
  if you're on a phone or similar you might want to save an offline copy to go
  easy on your data usage.
* [TONC](https://www.coranac.com/tonc/text/): While GBATEK is basically just a
  huge tech specification, TONC is an actual _guide_ on how to make sense of the
  GBA's abilities and organize it into a game. It's written for C of course, but
  it's a large
* [CowBite](https://www.cs.rit.edu/~tjh8300/CowBite/CowBiteSpec.htm): This is
  more like GBATEK, and it's less complete, but it mixes in a little more
  friendly explanation of things in between the hardware spec parts.

And I haven't had time to look at it myself, [The Audio
Advance](http://belogic.com/gba/) seems to be very good. It explains in depth
how you can get audio working on the GBA. Note that the table of contents for
each page goes along the top instead of down the side.

## Non-Rust GBA Community

There's also the [GBADev.org](http://www.gbadev.org/) site, which has a forum
and everything. They're coding in C and C++, but you can probably overcome that
difference with a little work on your part.

I also found a place called
[GBATemp](https://gbatemp.net/categories/nintendo-gba-discussions.32/), which
seems to have a more active forum but less of a focus on actual coding.
