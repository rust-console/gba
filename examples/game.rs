#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  #[cfg(debug_assertions)]
  if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
    use core::fmt::Write;
    writeln!(logger, "{info}").ok();
  }
  loop {}
}

#[derive(Debug, Clone, Copy, Default)]
struct Position {
  x: u16,
  y: u16,
}
#[derive(Debug, Clone, Copy, Default)]
struct Rect {
  x: u16,
  y: u16,
  w: u16,
  h: u16,
}
impl Rect {
  fn intersect(self, other: Self) -> bool {
    self.x < other.x + other.w
      && self.x + self.w > other.x
      && self.y < other.y + other.h
      && self.h + self.y > other.y
  }

  fn iter_tiles(self) -> impl Iterator<Item = (u16, u16)> {
    let y_range_incl = (self.y / 8)..=((self.y + self.h - 1) / 8);
    let x_range_incl = (self.x / 8)..=((self.x + self.w - 1) / 8);
    y_range_incl
      .map(move |y_index| {
        x_range_incl.clone().map(move |x_index| (x_index, y_index))
      })
      .flatten()
  }
}

#[no_mangle]
extern "C" fn main() -> ! {
  // game simulation setup
  let mut creatures = [Position::default(); 5];
  creatures[0].x = 11;
  creatures[0].y = 14;
  //
  creatures[1].x = 44;
  creatures[1].y = 38;
  creatures[2].x = 100;
  creatures[2].y = 23;
  creatures[3].x = 14;
  creatures[3].y = 101;
  creatures[4].x = 72;
  creatures[4].y = 59;

  let mut world = [[0_u8; 32]; 32];
  for i in 0..32 {
    world[0][i] = b'z';
    world[19][i] = b'z';
    world[i][0] = b'z';
    world[i][29] = b'z';
  }
  world[1][3] = b'B';
  world[2][3] = b'G';
  world[3][3] = b'0';

  // hardware configuration
  DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  TIMER0_CONTROL.write(TimerControl::new().with_enabled(true));

  // bg
  BG_PALETTE.index(1).write(Color::MAGENTA);
  // obj
  let colors =
    [Color::CYAN, Color::GREEN, Color::RED, Color::BLUE, Color::YELLOW];
  for (pal, color) in colors.iter().enumerate() {
    obj_palbank(pal).index(1).write(*color);
  }

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

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(creatures.len()).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_show_obj(true).with_show_bg0(true));

  let mut l_was_pressed = false;
  let mut r_was_pressed = false;

  loop {
    // wait for vblank
    VBlankIntrWait();

    // update graphics MMIO
    for (i, (creature_pos, attr_addr)) in
      creatures.iter().zip(OBJ_ATTR_ALL.iter()).enumerate()
    {
      let mut obj = ObjAttr::new();
      obj.set_x(creature_pos.x);
      obj.set_y(creature_pos.y);
      obj.set_tile_id(1);
      obj.set_palbank(i as u16);
      attr_addr.write(obj);
    }

    // handle input
    let keys = KEYINPUT.read();
    if keys.l() && !l_was_pressed {
      creatures.rotate_left(1);
    }
    if keys.r() && !r_was_pressed {
      creatures.rotate_right(1);
    }
    l_was_pressed = keys.l();
    r_was_pressed = keys.r();

    // the way we handle movement here is per-direction. If you're against a
    // wall and you press a diagonal then one axis will progress while the other
    // will be halted by the wall. This makes the player slide along the wall
    // when bumping into walls.
    let (player, enemies) = match &mut creatures {
      [player, enemies @ ..] => (player, enemies),
    };
    if keys.up() {
      let new_p = Position { x: player.x, y: player.y - 1 };
      let new_r = Rect { x: new_p.x, y: new_p.y, w: 8, h: 8 };
      let terrain_clear = new_r
        .iter_tiles()
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]));
      let enemy_clear = enemies.iter().all(|enemy| {
        let enemy_r = Rect { x: enemy.x, y: enemy.y, w: 8, h: 8 };
        !new_r.intersect(enemy_r)
      });
      if terrain_clear && enemy_clear {
        *player = new_p;
      }
    }
    if keys.down() {
      let new_p = Position { x: player.x, y: player.y + 1 };
      let new_r = Rect { x: new_p.x, y: new_p.y, w: 8, h: 8 };
      let terrain_clear = new_r
        .iter_tiles()
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]));
      let enemy_clear = enemies.iter().all(|enemy| {
        let enemy_r = Rect { x: enemy.x, y: enemy.y, w: 8, h: 8 };
        !new_r.intersect(enemy_r)
      });
      if terrain_clear && enemy_clear {
        *player = new_p;
      }
    }
    if keys.left() {
      let new_p = Position { x: player.x - 1, y: player.y };
      let new_r = Rect { x: new_p.x, y: new_p.y, w: 8, h: 8 };
      let terrain_clear = new_r
        .iter_tiles()
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]));
      let enemy_clear = enemies.iter().all(|enemy| {
        let enemy_r = Rect { x: enemy.x, y: enemy.y, w: 8, h: 8 };
        !new_r.intersect(enemy_r)
      });
      if terrain_clear && enemy_clear {
        *player = new_p;
      }
    }
    if keys.right() {
      let new_p = Position { x: player.x + 1, y: player.y };
      let new_r = Rect { x: new_p.x, y: new_p.y, w: 8, h: 8 };
      let terrain_clear = new_r
        .iter_tiles()
        .all(|(tx, ty)| allows_movement(world[ty as usize][tx as usize]));
      let enemy_clear = enemies.iter().all(|enemy| {
        let enemy_r = Rect { x: enemy.x, y: enemy.y, w: 8, h: 8 };
        !new_r.intersect(enemy_r)
      });
      if terrain_clear && enemy_clear {
        *player = new_p;
      }
    }
  }
}

const fn allows_movement(u: u8) -> bool {
  u == 0 || u == b' ' || u == u8::MAX
}
