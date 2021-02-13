#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{
  fatal, info,
  io::display::{DisplayControlSetting, DisplayMode, DISPCNT},
  save::*,
  time_this01,
  vram::bitmap::Mode3,
  Color,
};

fn set_screen_color(r: u16, g: u16, b: u16) {
  const SETTING: DisplayControlSetting =
    DisplayControlSetting::new().with_mode(DisplayMode::Mode3).with_bg2(true);
  DISPCNT.write(SETTING);
  Mode3::dma_clear_to(Color::from_rgb(r, g, b));
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  set_screen_color(31, 0, 0);
  fatal!("{}", info);
  loop {}
}

struct Pattern(u32, bool);
impl Pattern {
  fn next(&mut self) -> u8 {
    if self.1 {
      self.0 = self.0 * 2891336453 + 100001;
      (self.0 >> 22) as u8 ^ self.0 as u8
    } else {
      let r = self.0;
      self.0 += 1;
      r as u8
    }
  }
}

const MAX_BLOCK_SIZE: usize = 4 * 1024; // Flash sector size.
const GBA_CLOCKRATE: u32 = 16780000;

fn check_status<T>(r: Result<T, Error>) -> T {
  match r {
    Ok(v) => v,
    Err(e) => panic!("Error encountered: {:?}", e),
  }
}

fn do_write(mut pat: Pattern, shift: usize) -> Result<(), Error> {
  let access = SaveAccess::new()?;
  let block_ct = access.len() >> shift;
  let mut buffer = [0; MAX_BLOCK_SIZE];

  info!(" - Clearing media...");
  access.prepare_write(0..access.len())?;

  info!(" - Writing media...");
  for i in 0..block_ct {
    for j in 0..(1 << shift) {
      buffer[j] = pat.next();
    }
    access.write(i << shift, &buffer[0..(1 << shift)])?;
  }

  Ok(())
}
fn do_verify(mut pat: Pattern, shift: usize) -> Result<(), Error> {
  let access = SaveAccess::new()?;
  let block_ct = access.len() >> shift;
  let mut buffer = [0; MAX_BLOCK_SIZE];

  info!(" - Validating media...");
  for i in 0..block_ct {
    access.read(i << shift, &mut buffer[0..(1 << shift)])?;
    for j in 0..(1 << shift) {
      let cur = pat.next();
      assert!(
        buffer[j] == cur,
        "Read does not match earlier write: {} != {} @ 0x{:x}",
        buffer[j],
        cur,
        i * 512 + j
      );
    }
  }

  Ok(())
}

fn print_time(time: u32) {
  let seconds = time / GBA_CLOCKRATE;
  let fractional = (time / (GBA_CLOCKRATE / 1000)) % 1000;
  info!(" - Finished in {}.{:03} seconds.", seconds, fractional);
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  // set a pattern to show that the ROM is working at all.
  set_screen_color(31, 31, 0);

  // set the save type
  use_flash_64k();
  set_timer_for_timeout(3);

  // check some metainfo on the save type
  let access = check_status(SaveAccess::new());
  info!("Media info: {:?}", access.media_info());
  info!("Media size: {} bytes", access.len());
  info!("");

  // actually test the save implementation
  if access.len() >= (1 << 12) {
    info!("[ Incrementing, 4KiB blocks ]");
    print_time(time_this01!(check_status(do_write(Pattern(128, false), 12))));
    print_time(time_this01!(check_status(do_verify(Pattern(128, false), 12))));

    info!("[ Random, 4KiB blocks ]");
    print_time(time_this01!(check_status(do_write(Pattern(2000, true), 12))));
    print_time(time_this01!(check_status(do_verify(Pattern(2000, true), 12))));
  }

  info!("[ Incrementing, 0.5KiB blocks ]");
  print_time(time_this01!(check_status(do_write(Pattern(0, false), 9))));
  print_time(time_this01!(check_status(do_verify(Pattern(0, false), 9))));

  info!("[ Random, 0.5KiB blocks ]");
  print_time(time_this01!(check_status(do_write(Pattern(1000, true), 9))));
  print_time(time_this01!(check_status(do_verify(Pattern(1000, true), 9))));

  // show a pattern so we know it worked
  set_screen_color(0, 31, 0);

  0
}
