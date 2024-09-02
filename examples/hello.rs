#![no_std]
#![no_main]

use core::fmt::Write;
use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  #[cfg(debug_assertions)]
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[allow(dead_code)]
const FOO: Align4<[u8; 14]> = include_aligned_bytes!("foo.txt");

#[link_section = ".ewram"]
static FRAME_KEYS: GbaCell<KeyInput> = GbaCell::new(KeyInput::new());

#[link_section = ".iwram"]
extern "C" fn irq_handler(_: IrqBits) {
  // We'll read the keys during vblank and store it for later.
  FRAME_KEYS.write(KEYINPUT.read());
}

#[no_mangle]
extern "C" fn main() -> ! {
  RUST_IRQ_HANDLER.write(Some(irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
    writeln!(logger, "hello!").ok();

    let fx_u: Fixed<u32, 8> =
      Fixed::<u32, 8>::wrapping_from(7) + Fixed::<u32, 8>::from_bits(12);
    writeln!(logger, "fixed unsigned: {fx_u:?}").ok();

    let fx_i1: Fixed<i32, 8> =
      Fixed::<i32, 8>::wrapping_from(8) + Fixed::<i32, 8>::from_bits(15);
    writeln!(logger, "fixed signed positive: {fx_i1:?}").ok();

    let fx_i2: Fixed<i32, 8> = Fixed::<i32, 8>::wrapping_from(0)
      - Fixed::<i32, 8>::wrapping_from(3)
      - Fixed::<i32, 8>::from_bits(17);
    writeln!(logger, "fixed signed negative: {fx_i2:?}").ok();
  }

  {
    // get our tile data into memory.
    Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  }

  {
    // set up the tilemap
    let tsb = TEXT_SCREENBLOCKS.get_frame(31).unwrap();
    for y in 0..16 {
      let row = tsb.get_row(y).unwrap();
      for (x, addr) in row.iter().enumerate().take(16) {
        let te = TextEntry::new().with_tile((y * 16 + x) as u16);
        addr.write(te);
      }
    }
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
