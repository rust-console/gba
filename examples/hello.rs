#![no_std]
#![no_main]

use core::fmt::Write;
use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[allow(dead_code)]
const FOO_: Align4<[u8; 4]> = include_aligned_bytes!("foo.txt");

#[link_section = ".ewram"]
static FRAME_KEYS: GbaCell<KeyInput> = GbaCell::new(KeyInput::new());

#[link_section = ".iwram"]
extern "C" fn irq_handler(_: IrqBits) {
  // We'll read the keys during vblank and store it for later.
  FRAME_KEYS.write(KEYINPUT.read());
}

const TILE_LAYOUT: [u32; 512] = {
  // Rust's const eval is limited at the moment, but with a bit of careful math
  // we can set up `u32` values that store the right data. Tile maps are 32x32
  // `u16` values, so when packing it as `u32` instead we have to throw in some
  // `/2` stuff in a few places. Seperately, the tiles that we're using come
  // from an image that was drawn as a 16 by 16 tile sheet, so most of the
  // layout's area will be left as zero. Thankfully, tile index 0 is a blank
  // tile in this tileset, so it all works out.
  let mut data = [0; 512];
  let mut r = 0;
  while r < 16 {
    let mut c = 0;
    while c < 16 {
      let index = r * (32 / 2) + (c / 2);
      let a = r * 16 + c;
      let b = r * 16 + c + 1;
      data[index] = (a as u32) | ((b as u32) << 16);
      //
      c += 2;
    }
    //
    r += 1;
  }
  data
};

#[no_mangle]
extern "C" fn main() -> ! {
  RUST_IRQ_HANDLER.write(Some(irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
    writeln!(logger, "hello!").ok();

    let fx_u: Fixed<u32, 8> =
      Fixed::<u32, 8>::wrapping_from(7) + Fixed::<u32, 8>::from_raw(12);
    writeln!(logger, "fixed unsigned: {fx_u:?}").ok();

    let fx_i1: Fixed<i32, 8> =
      Fixed::<i32, 8>::wrapping_from(8) + Fixed::<i32, 8>::from_raw(15);
    writeln!(logger, "fixed signed positive: {fx_i1:?}").ok();

    let fx_i2: Fixed<i32, 8> = Fixed::<i32, 8>::wrapping_from(0)
      - Fixed::<i32, 8>::wrapping_from(3)
      - Fixed::<i32, 8>::from_raw(17);
    writeln!(logger, "fixed signed negative: {fx_i2:?}").ok();
  }

  {
    // get our tile data into memory.
    Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  }

  {
    // get the the tilemap copied into place
    let tsb = TextScreenblockAddress::new(31);
    tsb.write_word_array(&TILE_LAYOUT);
  }

  {
    // Set BG0 to use the tilemap we just made, and set it to be shown.
    BG0CNT.write(BackgroundControl::new().with_screenblock(31));
    DISPCNT.write(DisplayControl::new().with_show_bg0(true));
  }

  let mut x_off = 0_u32;
  let mut y_off = 0_u32;
  let mut backdrop_color = Color(0);
  loop {
    VBlankIntrWait();
    // show current frame
    BACKDROP_COLOR.write(backdrop_color);
    BG0HOFS.write(x_off as u16);
    BG0VOFS.write(y_off as u16);

    // prep next frame
    let k = FRAME_KEYS.read();
    backdrop_color = Color(k.to_u16());
    if k.up() {
      y_off = y_off.wrapping_add(1);
    }
    if k.down() {
      y_off = y_off.wrapping_sub(1);
    }
    if k.left() {
      x_off = x_off.wrapping_add(1);
    }
    if k.right() {
      x_off = x_off.wrapping_sub(1);
    }
  }
}
