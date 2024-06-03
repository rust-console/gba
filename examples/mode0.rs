#![no_std]
#![no_main]

use gba::{
  bios::VBlankIntrWait,
  mmio::{BG_PALRAM, DISPCNT, DISPSTAT, IE, IME, VRAM_BG_TILE4},
  sample_art::decompress_cga_face_to_vram_4bpp,
  video::{Color, DisplayControl, DisplayStatus},
  IrqBits,
};

gba::panic_handler!(empty_loop);

#[no_mangle]
pub extern "C" fn main() -> ! {
  decompress_cga_face_to_vram_4bpp(VRAM_BG_TILE4.as_region());

  BG_PALRAM.index(1).write(Color::WHITE);
  // TODO: set up the tilemap to look like something interesting

  IME.write(true);
  IE.write(IrqBits::new().with_vblank(true));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));

  DISPCNT.write(DisplayControl::new().with_bg_mode(0).with_bg0(true));
  loop {
    VBlankIntrWait();
  }
}