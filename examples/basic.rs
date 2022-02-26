#![no_std]
#![no_main]

use gba::*;

#[no_mangle]
extern "C" fn main() -> ! {
  DISPCNT.write(
    DisplayControl::default()
      .with_display_bg2(true)
      .with_video_mode(VideoMode3),
  );

  let mut y = 15_usize;
  let mut x = 20_usize;

  loop {
    // Wait for vblank
    VBlankIntrWait();

    // Draw
    unsafe {
      ((0x0600_0000 + ((y * 240 + x) * 2)) as *mut Color)
        .write(Color::from_rgb(31, 0, 0))
    };

    // Gather new input
    let keys = get_keys();

    // Update the "world state".
    if keys.left() {
      x = (x + 1).min(240 - 1);
    } else if keys.right() {
      x = x.saturating_sub(1);
    }
    if keys.up() {
      y = y.saturating_sub(1);
    } else if keys.down() {
      y = (y + 1).min(160 - 1);
    }
  }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
  // TODO: log the info to debug.
  loop {}
}
