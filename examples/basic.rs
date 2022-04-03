#![no_std]
#![no_main]

use gba::prelude::*;

static PEN_COLOR: GbaCell<Color> =
  GbaCell::<Color>::new(Color::from_rgb(31, 0, 0));

#[no_mangle]
extern "C" fn main() -> ! {
  // Set up for VBlank interrupts
  IME.write(true);
  IE.write(IrqBits::default().with_vblank(true));
  DISPSTAT.write(DisplayStatus::default().with_vblank_irq(true));

  set_irq_handler(Some(irq_handler));

  // Turn on video mode 3
  DISPCNT.write(
    DisplayControl::default()
      .with_video_mode(VideoMode3)
      .with_display_bg2(true),
  );

  let mut y = 15_usize;
  let mut x = 20_usize;

  loop {
    // Wait for vblank
    VBlankIntrWait();

    // Draw
    unsafe {
      ((0x0600_0000 + ((y * 240 + x) * 2)) as *mut Color)
        .write(PEN_COLOR.read())
    };

    // Gather new input
    let keys = get_keys();

    // Update the "world state".
    if keys.left() {
      x = x.saturating_sub(1);
    } else if keys.right() {
      x = (x + 1).min(240 - 1);
    }
    if keys.up() {
      y = y.saturating_sub(1);
    } else if keys.down() {
      y = (y + 1).min(160 - 1);
    }
  }
}

#[link_section = ".iwram"]
extern "C" fn irq_handler(_: IrqBits) {
  let p = PEN_COLOR.read();
  let p2 = Color(p.0.rotate_left(1));
  PEN_COLOR.write(p2);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
  // TODO: log the info to debug.
  loop {}
}
