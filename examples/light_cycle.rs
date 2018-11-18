#![feature(start)]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPCNT.write(MODE3 | BG2);
  }

  let mut px = SCREEN_WIDTH / 2;
  let mut py = SCREEN_HEIGHT / 2;
  let mut color = rgb16(31, 0, 0);

  loop {
    // read the input for this frame
    let this_frame_keys = key_input();

    // adjust game state and wait for vblank
    px += 2 * this_frame_keys.column_direction() as isize;
    py += 2 * this_frame_keys.row_direction() as isize;
    wait_until_vblank();

    // draw the new game and wait until the next frame starts.
    unsafe {
      if px < 0 || py < 0 || px == SCREEN_WIDTH || py == SCREEN_HEIGHT {
        // out of bounds, reset the screen and position.
        mode3_clear_screen(0);
        color = color.rotate_left(5);
        px = SCREEN_WIDTH / 2;
        py = SCREEN_HEIGHT / 2;
      } else {
        let color_here = mode3_read_pixel(px, py);
        if color_here != 0 {
          // crashed into our own line, reset the screen
          mode3_clear_screen(0);
          color = color.rotate_left(5);
        } else {
          // draw the new part of the line
          mode3_draw_pixel(px, py, color);
          mode3_draw_pixel(px, py + 1, color);
          mode3_draw_pixel(px + 1, py, color);
          mode3_draw_pixel(px + 1, py + 1, color);
        }
      }
    }
    wait_until_vdraw();
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VolatilePtr<T>(pub *mut T);
impl<T> VolatilePtr<T> {
  pub unsafe fn read(&self) -> T {
    core::ptr::read_volatile(self.0)
  }
  pub unsafe fn write(&self, data: T) {
    core::ptr::write_volatile(self.0, data);
  }
  pub unsafe fn offset(self, count: isize) -> Self {
    VolatilePtr(self.0.wrapping_offset(count))
  }
}

pub const DISPCNT: VolatilePtr<u16> = VolatilePtr(0x04000000 as *mut u16);
pub const MODE3: u16 = 3;
pub const BG2: u16 = 0b100_0000_0000;

pub const VRAM: usize = 0x06000000;
pub const SCREEN_WIDTH: isize = 240;
pub const SCREEN_HEIGHT: isize = 160;

pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

pub unsafe fn mode3_clear_screen(color: u16) {
  let color = color as u32;
  let bulk_color = color << 16 | color;
  let mut ptr = VolatilePtr(VRAM as *mut u32);
  for _ in 0..SCREEN_HEIGHT {
    for _ in 0..(SCREEN_WIDTH / 2) {
      ptr.write(bulk_color);
      ptr = ptr.offset(1);
    }
  }
}

pub unsafe fn mode3_draw_pixel(col: isize, row: isize, color: u16) {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).write(color);
}

pub unsafe fn mode3_read_pixel(col: isize, row: isize) -> u16 {
  VolatilePtr(VRAM as *mut u16).offset(col + row * SCREEN_WIDTH).read()
}

pub const KEYINPUT: VolatilePtr<u16> = VolatilePtr(0x400_0130 as *mut u16);

/// A newtype over the key input state of the GBA.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct KeyInputSetting(u16);

/// A "tribool" value helps us interpret the arrow pad.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TriBool {
  Minus = -1,
  Neutral = 0,
  Plus = 1,
}

pub fn key_input() -> KeyInputSetting {
  unsafe { KeyInputSetting(KEYINPUT.read() ^ 0b0000_0011_1111_1111) }
}

pub const KEY_A: u16 = 1 << 0;
pub const KEY_B: u16 = 1 << 1;
pub const KEY_SELECT: u16 = 1 << 2;
pub const KEY_START: u16 = 1 << 3;
pub const KEY_RIGHT: u16 = 1 << 4;
pub const KEY_LEFT: u16 = 1 << 5;
pub const KEY_UP: u16 = 1 << 6;
pub const KEY_DOWN: u16 = 1 << 7;
pub const KEY_R: u16 = 1 << 8;
pub const KEY_L: u16 = 1 << 9;

impl KeyInputSetting {
  pub fn contains(&self, key: u16) -> bool {
    (self.0 & key) != 0
  }

  pub fn difference(&self, other: KeyInputSetting) -> KeyInputSetting {
    KeyInputSetting(self.0 ^ other.0)
  }

  pub fn column_direction(&self) -> TriBool {
    if self.contains(KEY_RIGHT) {
      TriBool::Plus
    } else if self.contains(KEY_LEFT) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }

  pub fn row_direction(&self) -> TriBool {
    if self.contains(KEY_DOWN) {
      TriBool::Plus
    } else if self.contains(KEY_UP) {
      TriBool::Minus
    } else {
      TriBool::Neutral
    }
  }
}

pub const VCOUNT: VolatilePtr<u16> = VolatilePtr(0x0400_0006 as *mut u16);

pub fn vcount() -> u16 {
  unsafe { VCOUNT.read() }
}

pub fn wait_until_vblank() {
  while vcount() < SCREEN_HEIGHT as u16 {}
}

pub fn wait_until_vdraw() {
  while vcount() >= SCREEN_HEIGHT as u16 {}
}
