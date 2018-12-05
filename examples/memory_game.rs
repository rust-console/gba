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
    init_palette();
    init_background();
    clear_objects_starting_with(13);
    arrange_cards();
    init_selector();
    loop {
      // TODO the game
    }
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
  pub data: [u32; 8],
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Tile8bpp {
  pub data: [u32; 16],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock4bpp {
  pub data: [Tile4bpp; 512],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock8bpp {
  pub data: [Tile8bpp; 256],
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

pub unsafe fn init_palette() {
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

pub fn bg_tile_4bpp(base_block: usize, tile_index: usize) -> Tile4bpp {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).read() }
}

pub fn set_bg_tile_4bpp(base_block: usize, tile_index: usize, tile: Tile4bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * base_block + size_of::<Tile4bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).write(tile) }
}

pub fn bg_tile_8bpp(base_block: usize, tile_index: usize) -> Tile8bpp {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile8bpp).read() }
}

pub fn set_bg_tile_8bpp(base_block: usize, tile_index: usize, tile: Tile8bpp) {
  assert!(base_block < 4);
  assert!(tile_index < 256);
  let address = VRAM + size_of::<Charblock8bpp>() * base_block + size_of::<Tile8bpp>() * tile_index;
  unsafe { VolatilePtr(address as *mut Tile8bpp).write(tile) }
}

//

pub fn obj_tile_4bpp(tile_index: usize) -> Tile4bpp {
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * 4 + 32 * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).read() }
}

pub fn set_obj_tile_4bpp(tile_index: usize, tile: Tile4bpp) {
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock4bpp>() * 4 + 32 * tile_index;
  unsafe { VolatilePtr(address as *mut Tile4bpp).write(tile) }
}

pub fn obj_tile_8bpp(tile_index: usize) -> Tile8bpp {
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock8bpp>() * 4 + 32 * tile_index;
  unsafe { VolatilePtr(address as *mut Tile8bpp).read() }
}

pub fn set_obj_tile_8bpp(tile_index: usize, tile: Tile8bpp) {
  assert!(tile_index < 512);
  let address = VRAM + size_of::<Charblock8bpp>() * 4 + 32 * tile_index;
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
    self.0 |= palbank_index << 12;
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

pub fn clear_objects_starting_with(base_slot: usize) {
  let mut obj = ObjectAttributes::default();
  obj.set_rendering(ObjectRenderMode::Disabled);
  for s in base_slot..128 {
    set_object_attributes(s, obj);
  }
}

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

/// BG2 Control
pub const BG2CNT: VolatilePtr<u16> = VolatilePtr(0x400_000C as *mut u16);

pub unsafe fn init_background() {
  // put the bg tiles in charblock 0
  set_bg_tile_4bpp(0, 0, FULL_ONE);
  set_bg_tile_4bpp(0, 1, FULL_THREE);
  // make a checker pattern, place at screenblock 8 (aka the start of charblock 1)
  let entry_black = RegularScreenblockEntry::default();
  let mut entry_gray = RegularScreenblockEntry::default();
  entry_gray.set_tile_id(1);
  let mut using_black = true;
  let mut screenblock: RegularScreenblock = core::mem::zeroed();
  for entry_mut in screenblock.data.iter_mut() {
    *entry_mut = if using_black { entry_black } else { entry_gray };
    using_black = !using_black;
  }
  let p: VolatilePtr<RegularScreenblock> = VolatilePtr((VRAM + size_of::<Charblock8bpp>()) as *mut RegularScreenblock);
  p.write(screenblock);
  // turn on bg2 and configure it
  let display_control_value = DISPCNT.read();
  DISPCNT.write(display_control_value | BG2);
  const SCREEN_BASE_BLOCK_FIRST_BIT: u32 = 8;
  BG2CNT.write(8 << SCREEN_BASE_BLOCK_FIRST_BIT);
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

pub const TM0D: VolatilePtr<u16> = VolatilePtr(0x400_0100 as *mut u16);
pub const TM0CNT: VolatilePtr<u16> = VolatilePtr(0x400_0102 as *mut u16);

pub const TM1D: VolatilePtr<u16> = VolatilePtr(0x400_0104 as *mut u16);
pub const TM1CNT: VolatilePtr<u16> = VolatilePtr(0x400_0106 as *mut u16);

pub const TM2D: VolatilePtr<u16> = VolatilePtr(0x400_0108 as *mut u16);
pub const TM2CNT: VolatilePtr<u16> = VolatilePtr(0x400_010A as *mut u16);

pub const TM3D: VolatilePtr<u16> = VolatilePtr(0x400_010C as *mut u16);
pub const TM3CNT: VolatilePtr<u16> = VolatilePtr(0x400_010E as *mut u16);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerControl(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerFrequency {
  One = 0,
  SixFour = 1,
  TwoFiveSix = 2,
  OneZeroTwoFour = 3,
}

impl TimerControl {
  pub fn frequency(self) -> TimerFrequency {
    match self.0 & 0b11 {
      0 => TimerFrequency::One,
      1 => TimerFrequency::SixFour,
      2 => TimerFrequency::TwoFiveSix,
      3 => TimerFrequency::OneZeroTwoFour,
      _ => unreachable!(),
    }
  }
  pub fn cascading(self) -> bool {
    self.0 & 0b100 > 0
  }
  pub fn interrupt(self) -> bool {
    self.0 & 0b100_0000 > 0
  }
  pub fn enabled(self) -> bool {
    self.0 & 0b1000_0000 > 0
  }
  //
  pub fn set_frequency(&mut self, frequency: TimerFrequency) {
    self.0 &= !0b11;
    self.0 |= frequency as u16;
  }
  pub fn set_cascading(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b100;
    } else {
      self.0 &= !0b100;
    }
  }
  pub fn set_interrupt(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b100_0000;
    } else {
      self.0 &= !0b100_0000;
    }
  }
  pub fn set_enabled(&mut self, bit: bool) {
    if bit {
      self.0 |= 0b1000_0000;
    } else {
      self.0 &= !0b1000_0000;
    }
  }
}

/// Mucks with the settings of Timers 0 and 1.
unsafe fn u32_from_user_wait() -> u32 {
  let mut t = TimerControl::default();
  t.set_enabled(true);
  t.set_cascading(true);
  TM1CNT.write(t.0);
  t.set_cascading(false);
  TM0CNT.write(t.0);
  while key_input().0 == 0 {}
  t.set_enabled(false);
  TM0CNT.write(t.0);
  TM1CNT.write(t.0);
  let low = TM0D.read() as u32;
  let high = TM1D.read() as u32;
  (high << 32) | low
}

/// For the user's "cursor" to select a card
#[rustfmt::skip]
pub const CARD_SELECTOR: Tile4bpp = Tile4bpp {
  data : [
    0x11100111,
    0x11000011,
    0x10000001,
    0x00000000,
    0x00000000,
    0x10000001,
    0x11000011,
    0x11100111
  ]
};

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
