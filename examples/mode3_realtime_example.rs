/* 
* Made by Evan Goemer (@evangoemer)
* Licenced under MIT
*/

#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn main() {
    DISPCNT.write(
        DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true),
    );

    let red = 0;
    let green = 255;
    let blue = 0;

    let color = Color::from_rgb(red, green, blue);

    for y in 0..160 {
        for x in 0..240 {
            VIDEO3_VRAM.index(x, y).write(color);
        }
    }
}
