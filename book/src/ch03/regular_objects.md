# Regular Objects

As with backgrounds, objects can be used in both an affine and non-affine way.
For this section we'll focus on the non-affine elements, and then we'll do all
the affine stuff in a later chapter.

## Objects vs Sprites

As [TONC](https://www.coranac.com/tonc/text/regobj.htm) helpfully reminds us
(and then proceeds to not follow its own advice), we should always try to think
in terms of _objects_, not _sprites_. A sprite is a logical / software concern,
perhaps a player concern, whereas an object is a hardware concern.

What's more, a given sprite that the player sees might need more than one object
to display. Objects must be either square or rectangular (so sprite bits that
stick out probably call for a second object), and can only be from 8x8 to 64x64
(so anything bigger has to be two objects lined up to appear as one).

## General Object Info

Unlike with backgrounds, you can enable the object layer in any video mode.
There's space for 128 object definitions in OAM.

The display gets a number of cycles per scanline to process objects: 1210 by
default, but only 954 if you enable the "HBlank interval free" setting in the
display control register. The [cycle cost per
object](http://problemkaputt.de/gbatek.htm#lcdobjoverview) depends on the
object's size and if it's using affine or regular mode, so enabling the HBlank
interval free setting doesn't cut the number of objects displayable by an exact
number of objects. The objects are processed in order of their definitions and
if you run out of cycles then the rest just don't get shown. If there's a
concern that you might run out of cycles you can place important objects (such
as the player) at the start of the list and then less important animation
objects later on.

## Ready the Palette

Objects use the palette the same as the background does. The only difference is
that the palette data for objects starts at `0x500_0200`.

```rust
pub const PALRAM_OBJECT_BASE: VolatilePtr<u16> = VolatilePtr(0x500_0200 as *mut u16);

pub fn object_palette(slot: usize) -> u16 {
  assert!(slot < 256);
  unsafe { PALRAM_OBJECT_BASE.offset(slot as isize).read() }
}

pub fn set_object_palette(slot: usize, color: u16) {
  assert!(slot < 256);
  unsafe { PALRAM_OBJECT_BASE.offset(slot as isize).write(color) }
}
```

## Ready the Tiles

Objects, as with backgrounds, are composed of 8x8 tiles, and if you want
something bigger than 8x8 you have to use more than one tile put together.
Object tiles go into the final two charblocks of VRAM (indexes 4 and 5). Because
there's only two of them, they are sometimes called the lower block
(`0x601_0000`) and the higher/upper block (`0x601_4000`).

Tile indexes for sprites always offset from the base of the lower block, and
they always go 32 bytes at a time, regardless of if the object is set for 4bpp
or 8bpp. From this we can determine that there's 512 tile slots in each of the
two object charblocks. However, in video modes 3, 4, and 5 the space for the
background cuts into the lower charblock, so you can only safely use the upper
charblock.

With backgrounds you picked every single tile individually with a bunch of
screen entry values. Objects don't do that at all. Instead you pick a base tile,
size, and shape, then it figures out the rest from there. However, you may
recall back with the display control register something about an "object memory
1d" bit. This is where that comes into play.

* If object memory is set to be 2d (the default) then each charblock is treated
  as 32 tiles by 32 tiles square. Each object has a base tile and dimensions,
  and that just extracts directly from the charblock picture as if you were
  selecting an area. This mode probably makes for the easiest image editing.
* If object memory is set to be 1d then the tiles are loaded sequentially from
  the starting point, enough to fill in the object's dimensions. This most
  probably makes it the easiest to program with about things, since programming
  languages are pretty good at 1d things.

I'm not sure I explained that well, here's a picture:

![2d1d-diagram](obj_memory_2d1d.jpg)

In 2d mode, a new row of tiles starts every 32 tile indexes.

Of course, the mode that you actually end up using is not particularly
important, since it should be the job of your image conversion routine to get
everything all lined up and into place anyway.

## Set the Object Attributes

The final step is to assign the correct attributes to an object. Each object has
three `u16` values that make up its overall attributes.

Before we go into the details, I want to remind you that the hardware will
attempt to process every single object every single frame, and also that all of
the GBA's memory is cleared to 0 at startup. Why do these two things matter
right now? As you'll see in a second an "all zero" set of object attributes
causes an 8x8 object to appear at 0,0 using object tile index 0. This is usually
_not_ what you want your unused objects to do. When your game first starts you
should take a moment to mark any objects you won't be using as objects to not
render.

### ObjectAttributes.attr0

* 8 bits for row coordinate (marks the top of the sprite)
* 2 bits for object rendering: 0 = Normal, 1 = Affine, 2 = Disabled, 3 = Affine with double rendering area
* 2 bits for object mode: 0 = Normal, 1 = Alpha Blending, 2 = Object Window, 3 = Forbidden
* 1 bit for mosaic enabled
* 1 bit 8bpp color enabled
* 2 bits for shape: 0 = Square, 1 = Horizontal, 2 = Vertical, 3 = Forbidden

If an object is 128 pixels big at Y > 128 you'll get a strange looking result
where it acts like Y > -128 and then displays partly off screen to the top.

### ObjectAttributes.attr1

* 9 bit for column coordinate (marks the left of the sprite)
* Either:
  * 3 empty bits, 1 bit for horizontal flip, 1 bit for vertical flip (non-affine)
  * 5 bits for affine index (affine)
* 2 bits for size.

| Size | Square | Horizontal | Vertical|
|:----:|:------:|:----------:|:-------:|
| 0    | 8x8    | 16x8       | 8x16    |
| 1    | 16x16  | 32x8       | 8x32    |
| 2    | 32x32  | 32x16      | 16x32   |
| 3    | 64x64  | 64x32      | 32x64   |

### ObjectAttributes.attr2

* 10 bits for the base tile index
* 2 bits for priority
* 4 bits for the palbank index (4bpp mode only, ignored in 8bpp)

### ObjectAttributes summary

So I said in the GBA memory mapping section that C people would tell you that
the object attributes should look like this:

```rust
#[repr(C)]
pub struct ObjectAttributes {
  attr0: u16,
  attr1: u16,
  attr2: u16,
  filler: i16,
}
```

Except that:

1) It's wasteful when we store object attributes on their own outside of OAM
   (which we definitely might want to do).
2) In Rust we can't access just one field through a volatile pointer (our
   pointers aren't actually volatile to begin with, just the ops we do with them
   are). We have to read or write the whole pointer's value at a time.
   Similarly, we can't do things like `|=` and `&=` with volatile in Rust. So in
   rust we can't have a volatile pointer to an ObjectAttributes and then write
   to just the three "real" values and not touch the filler field. Having the
   filler value in there just means we have to dance around it more, not less.
3) We want to newtype this whole thing to prevent accidental invalid states from
   being written into memory.

So we will not be using that representation. At the same time we want to have no
overhead, so we will stick to three `u16` values. We could newtype each
individual field to be its own type (`ObjectAttributesAttr0` or something silly
like that), since there aren't actual dependencies between two different fields
such that a change in one can throw another into a forbidden state. The worst
that can happen is if we disable or enable affine mode (`attr0`) it can change
the meaning of `attr1`. The changed meaning isn't actually in invalid state
though, so we _could_ make each field its own type if we wanted.

However, when you think about it, I can't imagine a common situation where we do
something like make an `attr0` value that we then want to save on its own and
apply to several different `ObjectAttributes` that we make during a game. That
just doesn't sound likely to me. So, we'll go the route where `ObjectAttributes`
is just a big black box to the outside world and we don't need to think about
the three fields internally as being separate.

First we make it so that we can get and set object attributes from memory:

```rust
pub const OAM: usize = 0x700_0000;

pub fn object_attributes(slot: usize) -> ObjectAttributes {
  assert!(slot < 128);
  let ptr = VolatilePtr((OAM + slot * (size_of::<u16>() * 4)) as *mut u16);
  unsafe {
    ObjectAttributes {
      attr0: ptr.read(),
      attr1: ptr.offset(1).read(),
      attr2: ptr.offset(2).read(),
    }
  }
}

pub fn set_object_attributes(slot: usize, obj: ObjectAttributes) {
  assert!(slot < 128);
  let ptr = VolatilePtr((OAM + slot * (size_of::<u16>() * 4)) as *mut u16);
  unsafe {
    ptr.write(obj.attr0);
    ptr.offset(1).write(obj.attr1);
    ptr.offset(2).write(obj.attr2);
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectAttributes {
  attr0: u16,
  attr1: u16,
  attr2: u16,
}
```

Then we add a billion methods to the `ObjectAttributes` type so that we can
actually set all the different values that we want to set.

This code block is the last thing on this page so if you don't wanna scroll past
the whole thing you can just go to the next page.

```rust
#[derive(Debug, Clone, Copy)]
pub enum ObjectRenderMode {
  Normal,
  Affine,
  Disabled,
  DoubleAreaAffine,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectMode {
  Normal,
  AlphaBlending,
  ObjectWindow,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectShape {
  Square,
  Horizontal,
  Vertical,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectOrientation {
  Normal,
  HFlip,
  VFlip,
  BothFlip,
  Affine(u8),
}

impl ObjectAttributes {
  pub fn row(&self) -> u16 {
    self.attr0 & 0b1111_1111
  }
  pub fn column(&self) -> u16 {
    self.attr1 & 0b1_1111_1111
  }
  pub fn rendering(&self) -> ObjectRenderMode {
    match (self.attr0 >> 8) & 0b11 {
      0 => ObjectRenderMode::Normal,
      1 => ObjectRenderMode::Affine,
      2 => ObjectRenderMode::Disabled,
      3 => ObjectRenderMode::DoubleAreaAffine,
      _ => unimplemented!(),
    }
  }
  pub fn mode(&self) -> ObjectMode {
    match (self.attr0 >> 0xA) & 0b11 {
      0 => ObjectMode::Normal,
      1 => ObjectMode::AlphaBlending,
      2 => ObjectMode::ObjectWindow,
      _ => unimplemented!(),
    }
  }
  pub fn mosaic(&self) -> bool {
    ((self.attr0 << 3) as i16) < 0
  }
  pub fn two_fifty_six_colors(&self) -> bool {
    ((self.attr0 << 2) as i16) < 0
  }
  pub fn shape(&self) -> ObjectShape {
    match (self.attr0 >> 0xE) & 0b11 {
      0 => ObjectShape::Square,
      1 => ObjectShape::Horizontal,
      2 => ObjectShape::Vertical,
      _ => unimplemented!(),
    }
  }
  pub fn orientation(&self) -> ObjectOrientation {
    if (self.attr0 >> 8) & 1 > 0 {
      ObjectOrientation::Affine((self.attr1 >> 9) as u8 & 0b1_1111)
    } else {
      match (self.attr1 >> 0xC) & 0b11 {
        0 => ObjectOrientation::Normal,
        1 => ObjectOrientation::HFlip,
        2 => ObjectOrientation::VFlip,
        3 => ObjectOrientation::BothFlip,
      }
    }
  }
  pub fn size(&self) -> u16 {
    self.attr1 >> 0xE
  }
  pub fn tile_index(&self) -> u16 {
    self.attr2 & 0b11_1111_1111
  }
  pub fn priority(&self) -> u16 {
    self.attr2 >> 0xA
  }
  pub fn palbank(&self) -> u16 {
    self.attr2 >> 0xC
  }
  //
  pub fn set_row(&mut self, row: u16) {
    self.attr0 &= !0b1111_1111;
    self.attr0 |= row & 0b1111_1111;
  }
  pub fn set_column(&mut self, col: u16) {
    self.attr1 &= !0b1_1111_1111;
    self.attr2 |= col & 0b1_1111_1111;
  }
  pub fn set_rendering(&mut self, rendering: ObjectRenderMode) {
    const RENDERING_MASK: u16 = 0b11 << 8;
    self.attr0 &= !RENDERING_MASK;
    self.attr0 |= (rendering as u16) << 8;
  }
  pub fn set_mode(&mut self, mode: ObjectMode) {
    const MODE_MASK: u16 = 0b11 << 0xA;
    self.attr0 &= MODE_MASK;
    self.attr0 |= (mode as u16) << 0xA;
  }
  pub fn set_mosaic(&mut self, bit: bool) {
    const MOSAIC_BIT: u16 = 1 << 0xC;
    if bit {
      self.attr0 |= MOSAIC_BIT
    } else {
      self.attr0 &= !MOSAIC_BIT
    }
  }
  pub fn set_two_fifty_six_colors(&mut self, bit: bool) {
    const COLOR_MODE_BIT: u16 = 1 << 0xD;
    if bit {
      self.attr0 |= COLOR_MODE_BIT
    } else {
      self.attr0 &= !COLOR_MODE_BIT
    }
  }
  pub fn set_shape(&mut self, shape: ObjectShape) {
    self.attr0 &= 0b0011_1111_1111_1111;
    self.attr0 |= (shape as u16) << 0xE;
  }
  pub fn set_orientation(&mut self, orientation: ObjectOrientation) {
    const AFFINE_INDEX_MASK: u16 = 0b1_1111 << 9;
    self.attr1 &= !AFFINE_INDEX_MASK;
    let bits = match orientation {
      ObjectOrientation::Affine(index) => (index as u16) << 9,
      ObjectOrientation::Normal => 0,
      ObjectOrientation::HFlip => 1 << 0xC,
      ObjectOrientation::VFlip => 1 << 0xD,
      ObjectOrientation::BothFlip => 0b11 << 0xC,
    };
    self.attr1 |= bits;
  }
  pub fn set_size(&mut self, size: u16) {
    self.attr1 &= 0b0011_1111_1111_1111;
    self.attr1 |= size << 14;
  }
  pub fn set_tile_index(&mut self, index: u16) {
    self.attr2 &= !0b11_1111_1111;
    self.attr2 |= 0b11_1111_1111 & index;
  }
  pub fn set_priority(&mut self, priority: u16) {
    self.attr2 &= !0b0000_1100_0000_0000;
    self.attr2 |= (priority & 0b11) << 0xA;
  }
  pub fn set_palbank(&mut self, palbank: u16) {
    self.attr2 &= !0b1111_0000_0000_0000;
    self.attr2 |= (palbank & 0b1111) << 0xC;
  }
}
```
