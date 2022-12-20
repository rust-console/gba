#![no_std]
#![no_main]

use core::num::Wrapping;

use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  use core::fmt::Write;
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
  // game simulation
  let mut player_x = Wrapping(13);
  let mut player_y = Wrapping(37);
  let mut world = [[0_u8; 32]; 32];
  world[0][0] = b'B';
  world[1][0] = b'G';
  world[2][0] = b'0';

  // hardware configuration
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  TIMER0_CONTROL.write(TimerControl::new().with_enabled(true));

  BG_PALETTE.index(1).write(Color::MAGENTA);
  OBJ_PALETTE.index(1).write(Color::CYAN);

  Cga8x8Thick.bitunpack_4bpp(CHARBLOCK0_4BPP.as_region(), 0);
  Cga8x8Thick.bitunpack_4bpp(OBJ_TILES.as_region(), 0);

  BG0CNT.write(BackgroundControl::new().with_screenblock(8));
  let screenblock_addr = TextScreenblockAddress::new(8);
  for row in 0..32 {
    for col in 0..32 {
      let te = TextEntry::new().with_tile(world[row][col] as u16);
      screenblock_addr.row_col(row, col).write(te);
    }
  }

  let mut obj = ObjAttr::new();
  obj.set_x(player_x.0);
  obj.set_y(player_y.0);
  obj.set_tile_id(1);
  OBJ_ATTR_ALL.index(0).write(obj);

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(1).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_show_obj(true).with_show_bg0(true));

  loop {
    // wait for vblank
    VBlankIntrWait();

    // update graphics
    OBJ_ATTR_ALL.index(0).write(obj);

    // get input and prepare next frame
    let keys = KEYINPUT.read();
    if keys.up() {
      player_y -= 1;
    }
    if keys.down() {
      player_y += 1;
    }
    if keys.left() {
      player_x -= 1;
    }
    if keys.right() {
      player_x += 1;
    }
    obj.set_x(player_x.0);
    obj.set_y(player_y.0);
  }
}

const fn allows_movement(u: u8) -> bool {
  u == 0 || u == b' ' || u == u8::MAX
}

fn iter_tiles_of_area(
  (x, y): (u16, u16), (width, height): (u16, u16),
) -> impl Iterator<Item = (u16, u16)> {
  let y_range_incl = (y / 8)..=((y + height - 1) / 8);
  let x_range_incl = (x / 8)..=((x + width - 1) / 8);
  y_range_incl
    .map(move |y_index| {
      x_range_incl.clone().map(move |x_index| (x_index, y_index))
    })
    .flatten()
}
