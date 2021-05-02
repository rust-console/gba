#![no_std]
#![no_main]

// Note(Lokathor): this demo must be run in release mode or it'll just be way
// too slow. When testing is running the screen fills from yellow to green. If
// there's a panic you should see red. If all tests complete the screen turns
// blue at the end.

use core::cmp;
use gba::prelude::*;

fn set_screen_color(r: u8, g: u8, b: u8) {
  const SETTING: DisplayControl = DisplayControl::new().with_display_mode(3).with_display_bg2(true);
  DISPCNT.write(SETTING);
  mode3::dma3_clear_to(Color::from_rgb(r, g, b));
}
fn set_screen_progress(cur: usize, max: usize) {
  let lines = cur * (mode3::WIDTH / max);
  let color = Color::from_rgb(0, 31, 0);
  for x in 0..lines {
    for y in 0..mode3::HEIGHT {
      mode3::bitmap_xy(x, y).write(color);
    }
  }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  set_screen_color(31, 0, 0);
  fatal!("{}", info)
}

#[derive(Clone)]
struct Rng(u32);
impl Rng {
  fn iter(&mut self) {
    self.0 = self.0.wrapping_mul(2891336453).wrapping_add(100001);
  }
  fn next_u8(&mut self) -> u8 {
    self.iter();
    (self.0 >> 22) as u8 ^ self.0 as u8
  }
  fn next_under(&mut self, under: u32) -> u32 {
    self.iter();
    let scale = 31 - under.leading_zeros();
    ((self.0 >> scale) ^ self.0) % under
  }
}

const MAX_BLOCK_SIZE: usize = 4 * 1024;

fn check_status<T>(r: Result<T, Error>) -> T {
  match r {
    Ok(v) => v,
    Err(e) => panic!("Error encountered: {:?}", e),
  }
}

fn setup_timers() {
  TIMER0_RELOAD.write(0);
  TIMER1_RELOAD.write(0);

  let ctl = TimerControl::new().with_prescaler_selection(3).with_enabled(true);
  TIMER0_CONTROL.write(ctl);
  let ctl = TimerControl::new().with_chained_counting(true).with_enabled(true);
  TIMER1_CONTROL.write(ctl);
}

// I'm fully aware how slow this is. But this is just example code, so, eh.
fn get_timer_secs() -> f32 {
  let raw_timer = (TIMER1_COUNTER.read() as u32) << 16 | TIMER0_COUNTER.read() as u32;
  (raw_timer as f32 * 1024.0) / ((1 << 24) as f32)
}
macro_rules! output {
  ($($args:tt)*) => {
    // we use warn so it shows by default on mGBA, nothing more.
    warning!("{:7.3}\t{}", get_timer_secs(), format_args!($($args)*))
  };
}

fn do_test(seed: Rng, offset: usize, len: usize, block_size: usize) -> Result<(), Error> {
  let access = SaveAccess::new()?;
  let mut buffer = [0; MAX_BLOCK_SIZE];

  output!(" - Clearing media...");
  access.prepare_write(offset..offset + len)?;

  output!(" - Writing media...");
  let mut rng = seed.clone();
  let mut current = offset;
  let end = offset + len;
  while current != end {
    let cur_len = cmp::min(end - current, block_size);
    for i in 0..cur_len {
      buffer[i] = rng.next_u8();
    }
    access.write(current, &buffer[..cur_len])?;
    current += cur_len;
  }

  output!(" - Validating media...");
  rng = seed.clone();
  current = offset;
  while current != end {
    let cur_len = cmp::min(end - current, block_size);
    access.read(current, &mut buffer[..cur_len])?;
    for i in 0..cur_len {
      let cur_byte = rng.next_u8();
      assert!(
        buffer[i] == cur_byte,
        "Read does not match earlier write: {} != {} @ 0x{:05x}",
        buffer[i],
        cur_byte,
        current + i,
      );
    }
    current += cur_len;
  }

  output!(" - Done!");

  Ok(())
}

#[no_mangle]
fn main() -> ! {
  // set a pattern to show that the ROM is working at all.
  set_screen_color(31, 31, 0);

  // sets up the timers so we can print time with our outputs.
  setup_timers();

  // set the save type
  use_flash_128k();
  set_timer_for_timeout(3);

  // check some metainfo on the save type
  let access = check_status(SaveAccess::new());
  output!("Media info: {:#?}", access.media_info());
  output!("Media size: {} bytes", access.len());
  output!("");

  // actually test the save implementation
  if access.len() >= (1 << 12) {
    output!("[ Full write, 4KiB blocks ]");
    check_status(do_test(Rng(2000), 0, access.len(), 4 * 1024));
    set_screen_progress(1, 10);
  }

  output!("[ Full write, 0.5KiB blocks ]");
  check_status(do_test(Rng(1000), 0, access.len(), 512));
  set_screen_progress(2, 10);

  // test with random segments now.
  let mut rng = Rng(12345);
  for i in 0..8 {
    let rand_length = rng.next_under((access.len() >> 1) as u32) as usize + 50;
    let rand_offset = rng.next_under(access.len() as u32 - rand_length as u32) as usize;
    let block_size = cmp::min(rand_length >> 2, MAX_BLOCK_SIZE - 100);
    let block_size = rng.next_under(block_size as u32) as usize + 50;

    output!(
      "[ Partial, offset = 0x{:06x}, len = {}, bs = {}]",
      rand_offset,
      rand_length,
      block_size,
    );
    check_status(do_test(Rng(i * 10000), rand_offset, rand_length, block_size));
    set_screen_progress(3 + i as usize, 10);
  }

  // show a pattern so we know it worked
  set_screen_color(0, 29, 29);
  output!("All tests complete!");
  loop {}
}
