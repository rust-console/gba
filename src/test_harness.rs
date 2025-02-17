use crate::{bios, mem, mgba, prelude::*};
use core::fmt::Write;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  BG_PALETTE.index(0).write(Color::from_rgb(25, 10, 5));
  IE.write(IrqBits::VBLANK);
  IME.write(true);
  VBlankIntrWait();
  VBlankIntrWait();
  VBlankIntrWait();

  // the Fatal one kills emulation after one line / 256 bytes
  // so emit all the information as Error first
  if let Ok(mut log) =
    mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Error)
  {
    writeln!(log, "[failed]").ok();
    write!(log, "{}", info).ok();
  }

  if let Ok(mut log) =
    mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Fatal)
  {
    if let Some(loc) = info.location() {
      write!(log, "panic at {loc}! see mgba error log for details.").ok();
    } else {
      write!(log, "panic! see mgba error log for details.").ok();
    }
  }

  IE.write(IrqBits::new());
  bios::IntrWait(true, IrqBits::new());
  loop {}
}

pub(crate) trait UnitTest {
  fn run(&self);
}

impl<T: Fn()> UnitTest for T {
  fn run(&self) {
    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
    {
      write!(log, "{}...", core::any::type_name::<T>()).ok();
    }

    self();

    if let Ok(mut log) =
      mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
    {
      writeln!(log, "[ok]").ok();
    }
  }
}

pub(crate) fn test_runner(tests: &[&dyn UnitTest]) {
  if let Ok(mut log) =
    mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
  {
    write!(log, "Running {} tests", tests.len()).ok();
  }

  for test in tests {
    test.run();
  }
  if let Ok(mut log) =
    mgba::MgbaBufferedLogger::try_new(mgba::MgbaMessageLevel::Info)
  {
    write!(log, "Tests finished successfully").ok();
  }
}

#[no_mangle]
extern "C" fn main() {
  DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_0));
  BG_PALETTE.index(0).write(Color::new());

  crate::test_main();

  BG_PALETTE.index(0).write(Color::from_rgb(5, 15, 25));
  BG_PALETTE.index(1).write(Color::new());
  BG0CNT.write(BackgroundControl::new().with_charblock(0).with_screenblock(31));
  DISPCNT.write(
    DisplayControl::new().with_video_mode(VideoMode::_0).with_show_bg0(true),
  );

  // some niceties for people without mgba-test-runner
  let tsb = TEXT_SCREENBLOCKS.get_frame(31).unwrap();
  unsafe {
    mem::set_u32x80_unchecked(
      tsb.into_block::<1024>().as_mut_ptr().cast(),
      0,
      12,
    );
  }
  Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);

  let row = tsb.get_row(9).unwrap().iter().skip(6);
  for (addr, ch) in row.zip(b"all tests passed!") {
    addr.write(TextEntry::new().with_tile(*ch as u16));
  }

  DISPSTAT.write(DisplayStatus::new());
  bios::IntrWait(true, IrqBits::new());
}
