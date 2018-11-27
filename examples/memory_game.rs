#![feature(start)]
#![feature(asm)]
#![no_std]

use core::mem::size_of;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPCNT.write(MODE3 | BG2);
  }

  let mut px = SCREEN_WIDTH / 2;
  let mut py = SCREEN_HEIGHT / 2;
  let mut color = rgb16(31, 0, 0);

  loop {
    // read the input for this frame
    let this_frame_keys = key_input();

    // adjust game state and wait for vblank
    px += 2 * this_frame_keys.column_direction() as isize;
    py += 2 * this_frame_keys.row_direction() as isize;
    wait_until_vblank();

    // draw the new game and wait until the next frame starts.
    unsafe {
      if px < 0 || py < 0 || px == SCREEN_WIDTH || py == SCREEN_HEIGHT {
        // out of bounds, reset the screen and position.
        mode3_clear_screen(0);
        color = color.rotate_left(5);
        px = SCREEN_WIDTH / 2;
        py = SCREEN_HEIGHT / 2;
      } else {
        let color_here = mode3_read_pixel(px, py);
        if color_here != 0 {
          // crashed into our own line, reset the screen
          mode3_clear_screen(0);
          color = color.rotate_left(5);
        } else {
          // draw the new part of the line
          mode3_draw_pixel(px, py, color);
          mode3_draw_pixel(px, py + 1, color);
          mode3_draw_pixel(px + 1, py, color);
          mode3_draw_pixel(px + 1, py + 1, color);
        }
      }
    }
    wait_until_vdraw();
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(&self) -> T {
    core::ptr::read_volatile(self.0)
  }
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
}

pub const DISPCNT: VolatilePtr<u16> = VolatilePtr(0x04000000 as *mut u16);
pub const MODE3: u16 = 3;
pub const BG2: u16 = 0b100_0000_0000;

pub const VRAM: usize = 0x600_0000;
pub const SCREEN_WIDTH: isize = 240;
pub const SCREEN_HEIGHT: isize = 160;

pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

pub unsafe fn mode3_clear_screen(color: u16) {
  let color = color as u32;
  let bulk_color = color << 16 | color;
  let mut ptr = VolatilePtr(VRAM as *mut u32);
  for _ in 0..SCREEN_HEIGHT {
    for _ in 0..(SCREEN_WIDTH / 2) {
      ptr.write(bulk_color);
      ptr = ptr.offset(1);
    }
  }
}

pub unsafe fn mode3_draw_pixel(col: isize, row: isize, color: u16) {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write(color);
}

pub unsafe fn mode3_read_pixel(col: isize, row: isize) -> u16 {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).read()
}

pub const KEYINPUT: VolatilePtr<u16> = VolatilePtr(0x400_0130 as *mut u16);

/// A newtype over the key input state of the GBA.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInputSetting(u16);

/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TriBool {
  Minus = -1,
  Neutral = 0,
  Plus = 1,
}

pub fn key_input() -> KeyInputSetting {
  unsafe { KeyInputSetting(KEYINPUT.read() ^ 0b0000_0011_1111_1111) }
}

pub const KEY_A: u16 = 1 << 0;
pub const KEY_B: u16 = 1 << 1;
pub const KEY_SELECT: u16 = 1 << 2;
pub const KEY_START: u16 = 1 << 3;
pub const KEY_RIGHT: u16 = 1 << 4;
pub const KEY_LEFT: u16 = 1 << 5;
pub const KEY_UP: u16 = 1 << 6;
pub const KEY_DOWN: u16 = 1 << 7;
pub const KEY_R: u16 = 1 << 8;
pub const KEY_L: u16 = 1 << 9;

impl KeyInputSetting {
  pub fn contains(&self, key: u16) -> bool {
    (self.0 & key) != 0
  }

  pub fn difference(&self, other: KeyInputSetting) -> KeyInputSetting {
    KeyInputSetting(self.0 ^ other.0)
  }

  pub fn column_direction(&self) -> TriBool {
    if self.contains(KEY_RIGHT) {
      TriBool::Plus
    } else if self.contains(KEY_LEFT) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  pub fn row_direction(&self) -> TriBool {
    if self.contains(KEY_DOWN) {
      TriBool::Plus
    } else if self.contains(KEY_UP) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
}

pub const VCOUNT: VolatilePtr<u16> = VolatilePtr(0x0400_0006 as *mut u16);

pub fn vcount() -> u16 {
  unsafe { VCOUNT.read() }
}

pub fn wait_until_vblank() {
  while vcount() < SCREEN_HEIGHT as u16 {}
}

pub fn wait_until_vdraw() {
  while vcount() >= SCREEN_HEIGHT as u16 {}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Tile4bpp {
  data: [u32; 8],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Tile8bpp {
  data: [u32; 16],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock4bpp {
  data: [Tile4bpp; 512],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock8bpp {
  data: [Tile8bpp; 256],
}

pub const PALRAM_BG_BASE: VolatilePtr<u16> = VolatilePtr(0x500_0000 as *mut u16);

pub fn bg_palette(slot: usize) -> u16 {
  assert!(slot < 256);
  unsafe { PALRAM_BG_BASE.offset(slot as isize).read() }
}

pub fn set_bg_palette(slot: usize, color: u16) {
  assert!(slot < 256);
  unsafe { PALRAM_BG_BASE.offset(slot as isize).write(color) }
}

pub fn bg_tile_4pp(base_block: usize, tile_index: usize) -> Tile4bpp {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).read() }
}

pub fn set_bg_tile_4pp(base_block: usize, tile_index: usize, tile: Tile4bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).write(tile) }
}

pub fn bg_tile_8pp(base_block: usize, tile_index: usize) -> Tile8bpp {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile8bpp).read() }
}

pub fn set_bg_tile_8pp(base_block: usize, tile_index: usize, tile: Tile8bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile8bpp).write(tile) }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RegularScreenblock {
  data: [RegularScreenblockEntry; 32 * 32],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct RegularScreenblockEntry(u16);

impl RegularScreenblockEntry {
  pub fn tile_id(self) -> u16 {
    self.0 & 0b11_1111_1111
  }
  pub fn set_tile_id(&mut self, id: u16) {
    self.0 &= !0b11_1111_1111;
    self.0 |= id;
  }
  pub fn horizontal_flip(self) -> bool {
    (self.0 & (1 << 0xA)) > 0
  }
  pub fn set_horizontal_flip(&mut self, bit: bool) {
    if bit {
      self.0 |= 1 << 0xA;
    } else {
      self.0 &= !(1 << 0xA);
    }
  }
  pub fn vertical_flip(self) -> bool {
    (self.0 & (1 << 0xB)) > 0
  }
  pub fn set_vertical_flip(&mut self, bit: bool) {
    if bit {
      self.0 |= 1 << 0xB;
    } else {
      self.0 &= !(1 << 0xB);
    }
  }
  pub fn palbank_index(self) -> u16 {
    self.0 >> 12
  }
  pub fn set_palbank_index(&mut self, palbank_index: u16) {
    self.0 &= 0b1111_1111_1111;
    self.0 |= palbank_index;
  }
}

pub const PALRAM_OBJECT_BASE: VolatilePtr<u16> = VolatilePtr(0x500_0200 as *mut u16);

pub fn object_palette(slot: usize) -> u16 {
  assert!(slot < 256);
  unsafe { PALRAM_OBJECT_BASE.offset(slot as isize).read() }
}

pub fn set_object_palette(slot: usize, color: u16) {
  assert!(slot < 256);
  unsafe { PALRAM_OBJECT_BASE.offset(slot as isize).write(color) }
}

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ObjectAttributes {
  attr0: u16,
  attr1: u16,
  attr2: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectRenderMode {
  Normal,
  Affine,
  Disabled,
  DoubleAreaAffine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectMode {
  Normal,
  AlphaBlending,
  ObjectWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectShape {
  Square,
  Horizontal,
  Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        _ => unimplemented!(),
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

pub fn div_modulus(numerator: i32, denominator: i32) -> (i32, i32) {
  assert!(denominator != 0);
  {
    let div_out: i32;
    let mod_out: i32;
    unsafe {
      asm!(/* assembly template */ "swi 0x06"
          :/* output operands */ "={r0}"(div_out), "={r1}"(mod_out)
          :/* input operands */ "{r0}"(numerator), "{r1}"(denominator)
          :/* clobbers */ "r3"
          :/* options */
    );
    }
    (div_out, mod_out)
  }
}
pub fn div(numerator: i32, denominator: i32) -> i32 {
  div_modulus(numerator, denominator).0
}

pub fn modulus(numerator: i32, denominator: i32) -> i32 {
  div_modulus(numerator, denominator).1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandRangeU16 {
  range: u16,
  threshold: u16,
}

impl RandRangeU16 {
  pub fn new(mut range: u16) -> Self {
    let mut threshold = range.wrapping_neg();
    if threshold >= range {
      threshold -= range;
      if threshold >= range {
        threshold = modulus(threshold as i32, range as i32) as u16;
      }
    }
    RandRangeU16 { range, threshold }
  }

  pub fn roll_random(&self, rng: &mut impl FnMut() -> u16) -> u16 {
    let mut x: u16 = rng();
    let mut m: u32 = x as u32 * self.range as u32;
    let mut l: u16 = m as u16;
    if l < self.range {
      while l < self.threshold {
        x = rng();
        m = x as u32 * self.range as u32;
        l = m as u16;
      }
    }
    (m >> 16) as u16
  }
}

pub fn bounded_rand32(rng: &mut impl FnMut() -> u32, mut range: u32) -> u32 {
  let mut mask: u32 = !0;
  range -= 1;
  mask >>= (range | 1).leading_zeros();
  let mut x = rng() & mask;
  while x > range {
    x = rng() & mask;
  }
  x
}
