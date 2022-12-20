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
  // game simulation setup
  let mut player_x = 13_u16;
  let mut player_y = 37_u16;
  let mut world = [[0_u8; 32]; 32];
  for i in 0..32 {
    world[0][i] = b'z';
    world[31][i] = b'z';
    world[i][0] = b'z';
    world[i][31] = b'z';
  }
  world[1][1] = b'B';
  world[2][1] = b'G';
  world[3][1] = b'0';

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
  obj.set_x(player_x);
  obj.set_y(player_y);
  obj.set_tile_id(1);
  OBJ_ATTR_ALL.index(0).write(obj);

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(1).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_show_obj(true).with_show_bg0(true));

  loop {
    // wait for vblank
    VBlankIntrWait();

    // update graphics MMIO
    OBJ_ATTR_ALL.index(0).write(obj);

    // handle input
    let keys = KEYINPUT.read();
    // the way we handle movement here is per-direction. If you're against a
    // wall and you press a diagonal then one axis will progress while the other
    // will be halted by the wall. This makes the player slide along the wall
    // when bumping into walls.
    if keys.up() {
      let new_y = player_y.saturating_sub(1);
      if iter_tiles_of_area((player_x, new_y), (8, 8))
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]))
      {
        player_y = new_y;
      }
    }
    if keys.down() {
      let new_y = player_y.saturating_add(1);
      if iter_tiles_of_area((player_x, new_y), (8, 8))
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]))
      {
        player_y = new_y;
      }
    }
    if keys.left() {
      let new_x = player_x.saturating_sub(1);
      if iter_tiles_of_area((new_x, player_y), (8, 8))
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]))
      {
        player_x = new_x;
      }
    }
    if keys.right() {
      let new_x = player_x.saturating_add(1);
      if iter_tiles_of_area((new_x, player_y), (8, 8))
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]))
      {
        player_x = new_x;
      }
    }

    // ready our graphics for next frame
    obj.set_x(player_x);
    obj.set_y(player_y);
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
