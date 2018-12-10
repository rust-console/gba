#![feature(start)]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  // bg palette
  set_bg_palette_4bpp(0, 1, WHITE);
  set_bg_palette_4bpp(0, 2, LIGHT_GRAY);
  set_bg_palette_4bpp(0, 3, DARK_GRAY);
  // bg tiles
  set_bg_tile_4bpp(0, 0, ALL_TWOS);
  set_bg_tile_4bpp(0, 1, ALL_THREES);
  // screenblock
  let light_entry = RegularScreenblockEntry::from_tile_id(0);
  let dark_entry = RegularScreenblockEntry::from_tile_id(1);
  checker_screenblock(8, light_entry, dark_entry);
  // bg0 control
  unsafe { BG0CNT.write(BackgroundControlSetting::from_base_block(8)) };
  // Display Control
  unsafe { DISPCNT.write(DisplayControlSetting::JUST_ENABLE_BG0) };
  loop {
    // TODO the whole thing
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(&self) -> T {
    core::ptr::read_volatile(self.0)
  }
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }
  pub fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
  pub fn cast<Z>(self) -> VolatilePtr<Z> {
    VolatilePtr(self.0 as *mut Z)
  }
}

pub const BACKGROUND_PALETTE: VolatilePtr<u16> = VolatilePtr(0x500_0000 as *mut u16);

pub fn set_bg_palette_4bpp(palbank: usize, slot: usize, color: u16) {
  assert!(palbank < 16);
  assert!(slot > 0 && slot < 16);
  unsafe {
    BACKGROUND_PALETTE
      .cast::<[u16; 16]>()
      .offset(palbank as isize)
      .cast::<u16>()
      .offset(slot as isize)
      .write(color);
  }
}

pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

pub const WHITE: u16 = rgb16(31, 31, 31);
pub const LIGHT_GRAY: u16 = rgb16(25, 25, 25);
pub const DARK_GRAY: u16 = rgb16(15, 15, 15);

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct Tile4bpp {
  pub data: [u32; 8],
}

pub const ALL_TWOS: Tile4bpp = Tile4bpp {
  data: [
    0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222,
  ],
};

pub const ALL_THREES: Tile4bpp = Tile4bpp {
  data: [
    0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333,
  ],
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Charblock4bpp {
  pub data: [Tile4bpp; 512],
}

pub const VRAM: VolatilePtr<Charblock4bpp> = VolatilePtr(0x0600_0000 as *mut Charblock4bpp);

pub fn set_bg_tile_4bpp(charblock: usize, index: usize, tile: Tile4bpp) {
  assert!(charblock < 4);
  assert!(index < 512);
  unsafe { VRAM.offset(charblock as isize).cast::<Tile4bpp>().offset(index as isize).write(tile) }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct RegularScreenblockEntry(u16);

impl RegularScreenblockEntry {
  pub const SCREENBLOCK_ENTRY_TILE_ID_MASK: u16 = 0b11_1111_1111;
  pub const fn from_tile_id(id: u16) -> Self {
    RegularScreenblockEntry(id & Self::SCREENBLOCK_ENTRY_TILE_ID_MASK)
  }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RegularScreenblock {
  pub data: [RegularScreenblockEntry; 32 * 32],
}

pub fn checker_screenblock(slot: usize, a_entry: RegularScreenblockEntry, b_entry: RegularScreenblockEntry) {
  let mut p = VRAM.cast::<RegularScreenblock>().offset(slot as isize).cast::<RegularScreenblockEntry>();
  let mut checker = true;
  for _row in 0..32 {
    for _col in 0..32 {
      unsafe { p.write(if checker { a_entry } else { b_entry }) };
      p = p.offset(1);
      checker = !checker;
    }
    checker = !checker;
  }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct BackgroundControlSetting(u16);

impl BackgroundControlSetting {
  pub const SCREEN_BASE_BLOCK_MASK: u16 = 0b1_1111;
  pub const fn from_base_block(sbb: u16) -> Self {
    BackgroundControlSetting((sbb & Self::SCREEN_BASE_BLOCK_MASK) << 8)
  }
}

pub const BG0CNT: VolatilePtr<BackgroundControlSetting> = VolatilePtr(0x400_0008 as *mut BackgroundControlSetting);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayControlSetting(u16);

impl DisplayControlSetting {
  pub const JUST_ENABLE_BG0: DisplayControlSetting = DisplayControlSetting(1 << 8);
}

pub const DISPCNT: VolatilePtr<DisplayControlSetting> = VolatilePtr(0x0400_0000 as *mut DisplayControlSetting);
