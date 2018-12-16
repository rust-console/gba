#![no_std]
#![feature(start)]

#[no_mangle]
pub unsafe extern "C" fn __clzsi2(mut x: usize) -> usize {
  let mut y: usize;
  let mut n: usize = 32;
  y = x >> 16;
  if y != 0 {
    n = n - 16;
    x = y;
  }
  y = x >> 8;
  if y != 0 {
    n = n - 8;
    x = y;
  }
  y = x >> 4;
  if y != 0 {
    n = n - 4;
    x = y;
  }
  y = x >> 2;
  if y != 0 {
    n = n - 2;
    x = y;
  }
  y = x >> 1;
  if y != 0 {
    n - 2
  } else {
    n - x
  }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  unsafe {
    const DEBUG_ENABLE_MGBA: *mut u16 = 0x4fff780 as *mut u16;
    const DEBUG_OUTPUT_BASE: *mut u8 = 0x4fff600 as *mut u8;
    const DEBUG_SEND_MGBA: *mut u16 = 0x4fff700 as *mut u16;
    const DEBUG_SEND_FLAG: u16 = 0x100;
    const DEBUG_FATAL: u16 = 0;
    const DEBUG_ERROR: u16 = 1;
    DEBUG_ENABLE_MGBA.write_volatile(0xC0DE);
    if DEBUG_ENABLE_MGBA.read_volatile() == 0x1DEA {
      // Give the location
      if let Some(location) = info.location() {
        let mut out_ptr = DEBUG_OUTPUT_BASE;
        let line = location.line();
        let mut line_bytes = [
          (line / 10000) as u8,
          ((line / 1000) % 10) as u8,
          ((line / 1000) % 10) as u8,
          ((line / 10) % 10) as u8,
          (line % 10) as u8,
        ];
        for line_bytes_mut in line_bytes.iter_mut() {
          *line_bytes_mut += b'0';
        }
        for b in "Panic: "
          .bytes()
          .chain(location.file().bytes())
          .chain(", Line ".bytes())
          .chain(line_bytes.iter().cloned())
          .take(255)
        {
          out_ptr.write_volatile(b);
          out_ptr = out_ptr.offset(1);
        }
      } else {
        let mut out_ptr = DEBUG_OUTPUT_BASE;
        for b in "Panic with no location info:".bytes().take(255) {
          out_ptr.write_volatile(b);
          out_ptr = out_ptr.offset(1);
        }
      }
      DEBUG_SEND_MGBA.write_volatile(DEBUG_SEND_FLAG + DEBUG_ERROR);
      // Give the payload
      if let Some(payload) = info.payload().downcast_ref::<&str>() {
        let mut out_ptr = DEBUG_OUTPUT_BASE;
        for b in payload.bytes().take(255) {
          out_ptr.write_volatile(b);
          out_ptr = out_ptr.offset(1);
        }
      } else {
        let mut out_ptr = DEBUG_OUTPUT_BASE;
        for b in "no payload".bytes().take(255) {
          out_ptr.write_volatile(b);
          out_ptr = out_ptr.offset(1);
        }
      }
      DEBUG_SEND_MGBA.write_volatile(DEBUG_SEND_FLAG + DEBUG_ERROR);
      DEBUG_SEND_MGBA.write_volatile(DEBUG_SEND_FLAG + DEBUG_FATAL);
    }
  }
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x400_0000 as *mut u16).write_volatile(0x0403);
    (0x600_0000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x600_0000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x600_0000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    panic!("fumoffu!");
  }
}
