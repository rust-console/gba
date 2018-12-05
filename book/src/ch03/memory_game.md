# memory_game

For this example to show off our new skills we'll make a "memory" game. The idea
is that there's some face down cards and you pick one, it flips, you pick a
second, if they match they both go away, if they don't match they both turn back
face down. The player keeps going until all the cards are gone, then we'll deal
the cards again.

For this example, I started with the `light_cycle.rs` example and then just
copied it into a new file, `memory_game.rs`. Then I added most all the code from
the previous sections right into that file, so we'll assume that all those
definitions are in scope.

## Getting Some Images

First we need some images to show! Let's have one for our little selector thingy
that we'll move around to pick cards with. How about some little triangles at
the corner of a square like on a picture frame.

```rust
#[rustfmt::skip]
pub const CARD_SELECTOR: Tile4bpp = Tile4bpp {
  data : [
    0x44400444,
    0x44000044,
    0x40000004,
    0x00000000,
    0x00000000,
    0x40000004,
    0x44000044,
    0x44400444
  ]
};
```

That weird looking attribute keeps rustfmt from spreading out the values, so
that we can see it as an ASCII art. Now that we understand what an individual
tile looks like, let's add some mono-color squares.

```rust
#[rustfmt::skip]
pub const FULL_ONE: Tile4bpp = Tile4bpp {
  data : [
    0x11111111,
    0x11111111,
    0x11111111,
    0x11111111,
    0x11111111,
    0x11111111,
    0x11111111,
    0x11111111,
  ]
};

#[rustfmt::skip]
pub const FULL_TWO: Tile4bpp = Tile4bpp {
  data : [
    0x22222222,
    0x22222222,
    0x22222222,
    0x22222222,
    0x22222222,
    0x22222222,
    0x22222222,
    0x22222222
  ]
};

#[rustfmt::skip]
pub const FULL_THREE: Tile4bpp = Tile4bpp {
  data : [
    0x33333333,
    0x33333333,
    0x33333333,
    0x33333333,
    0x33333333,
    0x33333333,
    0x33333333,
    0x33333333
  ]
};
```

We can control the rest with palbank selection. Since there's 16 palbanks,
that's 48 little colored squares we can make, and 16 different selector colors,
which should be plenty in both cases.

## Setup The Images

### Arrange the PALRAM

Alright, so, as we went over, the first step is to make sure that we've got our
palette data in order. We'll be using this to set our palette values.

```rust
pub fn set_bg_palette(slot: usize, color: u16) {
  assert!(slot < 256);
  unsafe { PALRAM_BG_BASE.offset(slot as isize).write(color) }
}
```

Should the type of `slot` be changed to `u8` instead of `usize`? Well, maybe.
Let's not worry about it at the moment.

Of course, we don't need to set the color black, all the values start as black.
We just need to set the other colors we'll be wanting. For this demo, we'll just
use the same basic colors for both the BG and Object stuff.

```rust
pub fn init_palette() {
  // palbank 0: black/white/gray
  set_bg_palette(2, rgb16(31, 31, 31));
  set_bg_palette(3, rgb16(15, 15, 15));
  // palbank 1 is reds
  set_bg_palette(1 * 16 + 1, rgb16(31, 0, 0));
  set_bg_palette(1 * 16 + 2, rgb16(22, 0, 0));
  set_bg_palette(1 * 16 + 3, rgb16(10, 0, 0));
  // palbank 2 is greens
  set_bg_palette(2 * 16 + 1, rgb16(0, 31, 0));
  set_bg_palette(2 * 16 + 2, rgb16(0, 22, 0));
  set_bg_palette(2 * 16 + 3, rgb16(0, 10, 0));
  // palbank 2 is blues
  set_bg_palette(3 * 16 + 1, rgb16(0, 0, 31));
  set_bg_palette(3 * 16 + 2, rgb16(0, 0, 22));
  set_bg_palette(3 * 16 + 3, rgb16(0, 0, 10));

  // Direct copy all BG selections into OBJ palette too
  let mut bgp = PALRAM_BG_BASE;
  let mut objp = PALRAM_OBJECT_BASE;
  for _ in 0..(4 * 16) {
    objp.write(bgp.read());
    bgp = bgp.offset(1);
    objp = objp.offset(1);
  }
}
```

### Arrange the Objects

So, time to think about objects. I'm thinking we'll have 13 objects in use. One
for the selector, and then 12 for the cards (a simple grid that's 4 wide and 3
tall).

We want a way to easily clear away all the objects that we're not using, which
is all the slots starting at some index and then going to the end.

```rust
pub fn clear_objects_starting_with(base_slot: usize) {
  let mut obj = ObjectAttributes::default();
  obj.set_rendering(ObjectRenderMode::Disabled);
  for s in base_slot..128 {
    set_object_attributes(s, obj);
  }
}
```

Next we set out the positions of our cards. We set the tile data we need, and
then assign the object attributes to go with it. For this, we'll make the
position finder function be its own thing since we'll also need it for the card
selector to move around. Finally, we set our selector as being at position 0,0
of the card grid.

```rust
pub fn position_of_card(card_col: usize, card_row: usize) -> (u16, u16) {
  (10 + card_col as u16 * 17, 5 + card_row as u16 * 15)
}

pub fn arrange_cards() {
  set_obj_tile_4bpp(1, FULL_ONE);
  set_obj_tile_4bpp(2, FULL_TWO);
  set_obj_tile_4bpp(3, FULL_THREE);
  let mut obj = ObjectAttributes::default();
  obj.set_tile_index(2); // along with palbank0, this is a white card
  for card_row in 0..3 {
    for card_col in 0..4 {
      let (col, row) = position_of_card(card_col, card_row);
      obj.set_column(col);
      obj.set_row(row);
      set_object_attributes(1 + card_col as usize + (card_row as usize * 3), obj);
    }
  }
}

pub fn init_selector() {
  set_obj_tile_4bpp(0, CARD_SELECTOR);
  let mut obj = ObjectAttributes::default();
  let (col, row) = position_of_card(0, 0);
  obj.set_column(col);
  obj.set_row(row);
  set_object_attributes(0, obj);
}
```

### Arrange the Background

TODO

## Shuffling The Cards

TODO

## Picking One Card

TODO

## Picking The Second Card

TODO

## Resetting The Game

TODO
