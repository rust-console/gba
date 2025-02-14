use crate::macros::{pub_const_fn_new_zeroed, u16_bool_field, u16_enum_field};

/// A function you want called during an interrupt.
pub type IrqFn = unsafe extern "C" fn(IrqBits);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(u16);
impl IrqBits {
  pub_const_fn_new_zeroed!();
  u16_bool_field!(0, vblank, with_vblank);
  u16_bool_field!(1, hblank, with_hblank);
  u16_bool_field!(2, vcounter, with_vcounter);
  u16_bool_field!(3, timer0, with_timer0);
  u16_bool_field!(4, timer1, with_timer1);
  u16_bool_field!(5, timer2, with_timer2);
  u16_bool_field!(6, timer3, with_timer3);
  u16_bool_field!(7, serial, with_serial);
  u16_bool_field!(8, dma0, with_dma0);
  u16_bool_field!(9, dma1, with_dma1);
  u16_bool_field!(10, dma2, with_dma2);
  u16_bool_field!(11, dma3, with_dma3);
  u16_bool_field!(12, keypad, with_keypad);
  u16_bool_field!(13, gamepak, with_gamepak);

  pub const VBLANK: Self = Self::new().with_vblank(true);
  pub const HBLANK: Self = Self::new().with_hblank(true);
  pub const VCOUNTER: Self = Self::new().with_vcounter(true);
  pub const TIMER0: Self = Self::new().with_timer0(true);
  pub const TIMER1: Self = Self::new().with_timer1(true);
  pub const TIMER2: Self = Self::new().with_timer2(true);
  pub const TIMER3: Self = Self::new().with_timer3(true);
  pub const SERIAL: Self = Self::new().with_serial(true);
  pub const DMA0: Self = Self::new().with_dma0(true);
  pub const DMA1: Self = Self::new().with_dma1(true);
  pub const DMA2: Self = Self::new().with_dma2(true);
  pub const DMA3: Self = Self::new().with_dma3(true);
  pub const KEYPAD: Self = Self::new().with_keypad(true);
  pub const GAMEPAK: Self = Self::new().with_gamepak(true);

  #[inline]
  #[must_use]
  pub const fn to_u16(self) -> u16 {
    self.0
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct WaitstateControl(pub u16);
impl WaitstateControl {
  pub_const_fn_new_zeroed!();
  u16_enum_field!(0 - 1: SramFirstAccess, sram, with_sram);
  u16_enum_field!(
    2 - 3:
    Waitstate0FirstAccess,
    ws0_first_access,
    with_ws0_first_access
  );
  // true = 2, false = 1
  u16_bool_field!(4, ws0_second_access, with_ws0_second_access);
  u16_enum_field!(
    5 - 6:
    Waitstate1FirstAccess,
    ws1_first_access,
    with_ws1_first_access
  );
  // true = 4, false = 1
  u16_bool_field!(7, ws1_second_access, with_ws1_second_access);
  u16_enum_field!(
    8 - 9:
    Waitstate2FirstAccess,
    ws2_first_access,
    with_ws2_first_access
  );
  // true = 8, false = 1
  u16_bool_field!(10, ws2_second_access, with_ws2_second_access);
  u16_enum_field!(
    11 - 12:
    PhiTerminalOutput,
    phi_terminal_output,
    with_phi_terminal_output
  );
  u16_bool_field!(14, game_pak_prefetch_buffer, with_game_pak_prefetch_buffer);
  u16_bool_field!(15, game_pak_is_cgb, with_game_pak_is_cgb);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SramFirstAccess {
  Cycles4 = 0,
  Cycles3 = 1,
  Cycles2 = 2,
  Cycles8 = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Waitstate0FirstAccess {
  Cycles4 = 0 << 2,
  Cycles3 = 1 << 2,
  Cycles2 = 2 << 2,
  Cycles8 = 3 << 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Waitstate1FirstAccess {
  Cycles4 = 0 << 5,
  Cycles3 = 1 << 5,
  Cycles2 = 2 << 5,
  Cycles8 = 3 << 5,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Waitstate2FirstAccess {
  Cycles4 = 0 << 8,
  Cycles3 = 1 << 8,
  Cycles2 = 2 << 8,
  Cycles8 = 3 << 8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum PhiTerminalOutput {
  Disabled = 0 << 11,
  Freq4MHz = 1 << 11,
  Freq8MHz = 2 << 11,
  Freq16MHz = 3 << 11,
}

// TODO: might want to support bit ops. But it's not super important right now
// since they can't be implented as const traits yet anyway.
