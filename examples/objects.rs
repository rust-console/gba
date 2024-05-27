#![no_std]
#![no_main]

use gba::{
  asm_runtime::USER_IRQ_HANDLER,
  bios::VBlankIntrWait,
  gba_cell::GbaCell,
  mmio::{
    obj_palbank, BG0CNT, BG_PALRAM, DISPCNT, DISPSTAT, IE, IME, KEYINPUT,
    OBJ_ATTR0, OBJ_ATTR_ALL, TEXT_SCREENBLOCKS, VRAM_BG_TILE4, VRAM_OBJ_TILE4,
  },
  obj::{ObjAttr, ObjAttr0, ObjDisplayStyle},
  sample_art::{decompress_cga_face_to_vram_4bpp, Cga},
  video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry},
  IrqBits,
};

gba::panic_handler!(mgba_log_err);

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

/// This data is shared between the vblank handler and the `main` fn.
static CREATURE_POSITIONS: [(GbaCell<u16>, GbaCell<u16>); 5] = [
  (GbaCell::new(0), GbaCell::new(0)),
  (GbaCell::new(0), GbaCell::new(0)),
  (GbaCell::new(0), GbaCell::new(0)),
  (GbaCell::new(0), GbaCell::new(0)),
  (GbaCell::new(0), GbaCell::new(0)),
];

/// This runs at the start of each vblank period. We just reformat the shared
/// creature position data into the OAM region.
extern "C" fn irq_handler(_bits: IrqBits) {
  // update graphics MMIO
  for (i, (creature_pos, attr_addr)) in
    CREATURE_POSITIONS.iter().zip(OBJ_ATTR_ALL.iter()).enumerate()
  {
    let mut obj = ObjAttr::new();
    obj.set_x(creature_pos.0.read());
    obj.set_y(creature_pos.1.read());
    obj.set_tile_id(1);
    obj.set_palbank(i as u16);
    attr_addr.write(obj);
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

  // indexing with `[y][x]`
  let mut world = [[0_u8; 32]; 32];
  for i in 0..32 {
    world[0][i] = Cga::LEFT_RIGHT;
    world[19][i] = Cga::LEFT_RIGHT;
    world[i][0] = Cga::UP_DOWN;
    world[i][29] = Cga::UP_DOWN;
  }
  world[0][0] = Cga::DOWN_RIGHT;
  world[0][29] = Cga::LEFT_DOWN;
  world[19][0] = Cga::UP_RIGHT;
  world[19][29] = Cga::UP_LEFT;
  world[1][3] = b'B';
  world[2][3] = b'G';
  world[3][3] = b'0';

  // interrupt configuration
  USER_IRQ_HANDLER.write(Some(irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  // bg
  BG_PALRAM.index(1).write(Color::MAGENTA);
  // obj
  let colors =
    [Color::CYAN, Color::GREEN, Color::RED, Color::BLUE, Color::YELLOW];
  for (pal, color) in colors.iter().enumerate() {
    obj_palbank(pal).index(1).write(*color);
  }

  decompress_cga_face_to_vram_4bpp(VRAM_BG_TILE4.as_region());
  decompress_cga_face_to_vram_4bpp(VRAM_OBJ_TILE4.as_region());

  BG0CNT.write(BackgroundControl::new().with_screenblock(8));
  let screenblock = TEXT_SCREENBLOCKS.get_frame(8).unwrap();
  for y in 0..32 {
    let row = screenblock.get_row(y).unwrap();
    for (x, addr) in row.iter().enumerate() {
      let te = TextEntry::new().with_tile(world[y][x] as u16);
      addr.write(te);
    }
  }

  let no_display = ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed);
  OBJ_ATTR0.iter().skip(creatures.len()).for_each(|va| va.write(no_display));

  DISPCNT.write(DisplayControl::new().with_objects(true).with_bg0(true));

  let mut l_was_pressed = false;
  let mut r_was_pressed = false;

  loop {
    // copy the current data into memory that the interrupt handler can see, so
    // that the handler can update all the graphics in proper timing with the
    // start of vblank.
    for (c, c_pos) in creatures.iter().zip(CREATURE_POSITIONS.iter()) {
      c_pos.0.write(c.x);
      c_pos.1.write(c.y);
    }
    // wait for vblank, graphics updates during the handler
    VBlankIntrWait();

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
