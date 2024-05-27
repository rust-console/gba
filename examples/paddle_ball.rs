/*
 * Made by Evan Goemer
 * Discord: @evangoemer
 */

#![no_std]
#![no_main]
#![allow(unused)]

use core::ptr::addr_of;

use gba::{
  asm_runtime::USER_IRQ_HANDLER,
  bios::VBlankIntrWait,
  dma::{DmaControl, DmaSrcAddr},
  gba_cell::GbaCell,
  mmio::{
    DISPCNT, DISPSTAT, DMA3_CONTROL, DMA3_DESTINATION, DMA3_SOURCE,
    DMA3_TRANSFER_COUNT, IE, IME, KEYINPUT, MODE3_VRAM, OBJ_PALRAM,
  },
  video::{Color, DisplayControl, DisplayStatus},
  IrqBits, KeyInput,
};

const SCREEN_WIDTH: u16 = 240;
const SCREEN_HEIGHT: u16 = 160;

const PADDLE_WIDTH: u16 = 4;
const PADDLE_HEIGHT: u16 = 20;
const BALL_SIZE: u16 = 2;

struct Paddle {
  x: u16,
  y: u16,
}

struct Ball {
  x: u16,
  y: u16,
  dx: i16,
  dy: i16,
}

impl Paddle {
  fn new(x: u16, y: u16) -> Self {
    Self { x, y }
  }

  fn update(&mut self, keys: KeyInput) {
    if keys.up() && self.y > 1 {
      self.y -= 1;
    }

    if keys.down() && self.y + PADDLE_HEIGHT + 1 < SCREEN_HEIGHT {
      self.y += 1;
    }
  }
}

impl Ball {
  fn new(x: u16, y: u16) -> Self {
    Self { x, y, dx: 1, dy: 1 }
  }

  fn update(&mut self, paddle1: &Paddle, paddle2: &Paddle) {
    if self.y <= 0 || self.y + BALL_SIZE >= SCREEN_HEIGHT {
      self.dy = -self.dy;
    }

    if self.x + BALL_SIZE >= paddle1.x
      && self.x <= paddle1.x + PADDLE_WIDTH
      && self.y + BALL_SIZE >= paddle1.y
      && self.y <= paddle1.y + PADDLE_HEIGHT
    {
      self.dx = -self.dx;
      self.dy = -self.dy;
    }

    if self.x + BALL_SIZE >= paddle2.x
      && self.x <= paddle2.x + PADDLE_WIDTH
      && self.y + BALL_SIZE >= paddle2.y
      && self.y <= paddle2.y + PADDLE_HEIGHT
    {
      self.dx = -self.dx;
      self.dy = -self.dy;
    }

    if self.x + BALL_SIZE <= 1 + BALL_SIZE {
      self.x = SCREEN_WIDTH / 2 - BALL_SIZE / 2;
      self.y = SCREEN_HEIGHT / 2 - BALL_SIZE / 2;
      self.dx = 1;
      self.dy = 1;
    }

    if self.x >= SCREEN_WIDTH - BALL_SIZE - 1 {
      self.x = SCREEN_WIDTH / 2 - BALL_SIZE / 2;
      self.y = SCREEN_HEIGHT / 2 - BALL_SIZE / 2;
      self.dx = -1;
      self.dy = 1;
    }
    self.x = (self.x as i16 + self.dx) as u16;
    self.y = (self.y as i16 + self.dy) as u16;
  }
}

static SPRITE_POSITIONS: [GbaCell<u16>; 6] = [
  GbaCell::new(0),
  GbaCell::new(0),
  GbaCell::new(0),
  GbaCell::new(0),
  GbaCell::new(0),
  GbaCell::new(0),
];

gba::panic_handler!(empty_loop);

#[no_mangle]
fn main() -> ! {
  DISPCNT.write(DisplayControl::new().with_bg_mode(3).with_bg2(true));

  USER_IRQ_HANDLER.write(Some(draw_sprites));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::VBLANK);
  IME.write(true);

  let mut left_paddle =
    Paddle::new(10, SCREEN_HEIGHT as u16 / 2 - PADDLE_HEIGHT / 2);
  let mut right_paddle = Paddle::new(
    SCREEN_WIDTH as u16 - 10 - PADDLE_WIDTH,
    SCREEN_HEIGHT as u16 / 2 - PADDLE_HEIGHT / 2,
  );
  let mut ball = Ball::new(SCREEN_WIDTH as u16 / 2, SCREEN_HEIGHT as u16 / 2);

  loop {
    let keys = KEYINPUT.read();
    left_paddle.update(keys);
    right_paddle.update(keys);
    ball.update(&left_paddle, &right_paddle);

    SPRITE_POSITIONS[0].write(left_paddle.x);
    SPRITE_POSITIONS[1].write(left_paddle.y);
    SPRITE_POSITIONS[2].write(right_paddle.x);
    SPRITE_POSITIONS[3].write(right_paddle.y);
    SPRITE_POSITIONS[4].write(ball.x);
    SPRITE_POSITIONS[5].write(ball.y);

    VBlankIntrWait();
  }
}

extern "C" fn draw_sprites(_bits: IrqBits) {
  unsafe {
    let color = OBJ_PALRAM.index(0).read();
    let c: u32 = color.0 as u32 | (color.0 as u32) << 16;
    DMA3_SOURCE.write(addr_of!(c).cast());
    DMA3_DESTINATION.write(MODE3_VRAM.as_usize() as _);
    DMA3_TRANSFER_COUNT.write(SCREEN_WIDTH * SCREEN_HEIGHT / 2);
    DMA3_CONTROL.write(
      DmaControl::new()
        .with_src_addr(DmaSrcAddr::Fixed)
        .with_u32_transfer(true)
        .with_enabled(true),
    );
  }

  draw_rect(
    SPRITE_POSITIONS[0].read(),
    SPRITE_POSITIONS[1].read(),
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    Color::RED,
  );
  draw_rect(
    SPRITE_POSITIONS[2].read(),
    SPRITE_POSITIONS[3].read(),
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    Color::GREEN,
  );
  draw_rect(
    SPRITE_POSITIONS[4].read(),
    SPRITE_POSITIONS[5].read(),
    BALL_SIZE,
    BALL_SIZE,
    Color::CYAN,
  );
}

fn draw_rect(x: u16, y: u16, width: u16, height: u16, color: Color) {
  for i in 0..width {
    for j in 0..height {
      MODE3_VRAM.index((x + i) as usize, (y + j) as usize).write(color);
    }
  }
}
