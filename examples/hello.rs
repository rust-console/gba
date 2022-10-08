#![no_std]
#![no_main]

use core::{fmt::Write, mem::size_of_val};
use gba::{
  mgba::{MgbaBufferedLogger, MgbaMessageLevel},
  prelude::*,
};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    write!(logger, "{info}").ok();
  }
  loop {}
}

static FRAME_KEYS: GbaCell<KeyInput> = GbaCell::new(KeyInput::new());

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
  }

  {
    // get our tile data into memory.
    let src = CGA_8X8_THICK.as_ptr().cast::<u8>();
    let dest = CHARBLOCK0_4BPP.index(0).as_usize() as *mut u32;
    let info = BitUnpackInfo {
      src_byte_len: size_of_val(&CGA_8X8_THICK) as u16,
      src_elem_width: 1,
      dest_elem_width: 4,
      offset_and_touch_zero: 0,
    };
    unsafe { BitUnPack(src, dest, &info) };
  }

  {
    // the the tilemap set up
    let tsb = TileScreenblock::new(31);
    for row in 0..16_usize {
      for col in 0..16_usize {
        let id = row * 16 + col;
        let entry = TileEntry::new().with_tile_id(id as u16);
        tsb.row_col(row, col).write(entry);
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
