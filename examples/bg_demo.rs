#![no_std]
#![feature(start)]

use gba::{
  io::{
    background::{BackgroundControlSetting, BG0},
    display::{DisplayControlSetting, DISPCNT},
  },
  palram::index_palram_bg_4bpp,
  video::tiled::{TextScreenblockEntry, Tile4bpp, VRAM_CHARBLOCKS, VRAM_TEXT_SCREENBLOCKS},
  Color,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  pub const WHITE: Color = Color::from_rgb(31, 31, 31);
  pub const LIGHT_GRAY: Color = Color::from_rgb(25, 25, 25);
  pub const DARK_GRAY: Color = Color::from_rgb(15, 15, 15);
  // bg palette
  index_palram_bg_4bpp(0, 1).write(WHITE);
  index_palram_bg_4bpp(0, 2).write(LIGHT_GRAY);
  index_palram_bg_4bpp(0, 3).write(DARK_GRAY);
  // bg tiles
  set_bg_tile_4bpp(0, 0, ALL_TWOS);
  set_bg_tile_4bpp(0, 1, ALL_THREES);
  // screenblock
  let light_entry = TextScreenblockEntry::from_tile_index(0);
  let dark_entry = TextScreenblockEntry::from_tile_index(1);
  checker_screenblock(8, light_entry, dark_entry);
  // bg0 control
  BG0::BG0CNT.write(BackgroundControlSetting::from_screen_base_block(8));
  // Display Control
  DISPCNT.write(DisplayControlSetting::new().with_display_bg0(true));
  loop {
    // TODO the whole thing
  }
}

pub const ALL_TWOS: Tile4bpp = Tile4bpp([
  0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222, 0x22222222,
]);

pub const ALL_THREES: Tile4bpp = Tile4bpp([
  0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333, 0x33333333,
]);

pub fn set_bg_tile_4bpp(charblock: usize, index: usize, tile: Tile4bpp) {
  assert!(charblock < 4);
  assert!(index < 512);
  unsafe { VRAM_CHARBLOCKS.index(charblock).cast::<Tile4bpp>().offset(index as isize).write(tile) }
}

pub fn checker_screenblock(slot: usize, a_entry: TextScreenblockEntry, b_entry: TextScreenblockEntry) {
  let mut p = unsafe { VRAM_TEXT_SCREENBLOCKS.index(slot).cast::<TextScreenblockEntry>() };
  let mut checker = true;
  for _row in 0..32 {
    for _col in 0..32 {
      unsafe {
        p.write(if checker { a_entry } else { b_entry });
        p = p.offset(1);
      }
      checker = !checker;
    }
    checker = !checker;
  }
}
