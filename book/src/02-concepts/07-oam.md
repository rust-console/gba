# Object Attribute Memory (OAM)

* **Address Span:** `0x700_0000` to `0x700_03FF` (1k)

The Object Attribute Memory has a 32-bit bus and no default wait, but suffers
from the "you might have to wait if the display controller is looking at it"
rule. You cannot write individual bytes to OAM at all, but that's not really a
problem because all the fields of the data types within OAM are either `i16` or
`u16` anyway.

Object attribute memory is the wildest yet: it conceptually contains two types
of things, but they're _interlaced_ with each other all the way through.

Now, [GBATEK](http://problemkaputt.de/gbatek.htm#lcdobjoamattributes) and
[CowByte](https://www.cs.rit.edu/~tjh8300/CowBite/CowBiteSpec.htm#OAM%20(sprites))
doesn't quite give names to the two data types here.
[TONC](https://www.coranac.com/tonc/text/regobj.htm#sec-oam) calls them
`OBJ_ATTR` and `OBJ_AFFINE`, but we'll be giving them names fitting with the
Rust naming convention. Just know that if you try to talk about it with others
they might not be using the same names. In Rust terms their layout would look
like this:

```rust
#[repr(C)]
pub struct ObjectAttributes {
  attr0: u16,
  attr1: u16,
  attr2: u16,
  filler: i16,
}

#[repr(C)]
pub struct AffineMatrix {
  filler0: [u16; 3],
  pa: i16,
  filler1: [u16; 3],
  pb: i16,
  filler2: [u16; 3],
  pc: i16,
  filler3: [u16; 3],
  pd: i16,
}
```

(Note: the `#[repr(C)]` part just means that Rust must lay out the data exactly
in the order we specify, which otherwise it is not required to do).

So, we've got 1024 bytes in OAM and each `ObjectAttributes` value is 8 bytes, so
naturally we can support up to 128 objects.

_At the same time_, we've got 1024 bytes in OAM and each `AffineMatrix` is 32
bytes, so we can have 32 of them.

But, as I said, these things are all _interlaced_ with each other. See how
there's "filler" fields in each struct? If we imagine the OAM as being just an
array of one type or the other, indexes 0/1/2/3 of the `ObjectAttributes` array
would line up with index 0 of the `AffineMatrix` array. It's kinda weird, but
that's just how it works. When we setup functions to read and write these values
we'll have to be careful with how we do it. We probably _won't_ want to use
those representations above, at least not with the `AffineMatrix` type, because
they're quite wasteful if you want to store just object attributes or just
affine matrices.
