#![feature(start)]
#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPCNT.write_volatile(MODE3 | BG2);
    mode3_pixel(120, 80, rgb16(31, 0, 0));
    mode3_pixel(136, 80, rgb16(0, 31, 0));
    mode3_pixel(120, 96, rgb16(0, 0, 31));
    loop {}
  }
}

pub const DISPCNT: *mut u16 = 0x04000000 as *mut u16;
pub const MODE3: u16 = 3;
pub const BG2: u16 = 0b100_0000_0000;

pub const VRAM: usize = 0x06000000;
pub const SCREEN_WIDTH: isize = 240;

pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

pub unsafe fn mode3_pixel(col: isize, row: isize, color: u16) {
  (VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write_volatile(color);
}
