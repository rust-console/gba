use super::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptFlags(pub(crate) u16);
impl InterruptFlags {
  const_new!();
  bitfield_bool!(u16; 0, vblank, with_vblank, set_vblank);
  bitfield_bool!(u16; 1, hblank, with_hblank, set_hblank);
  bitfield_bool!(u16; 2, vcount, with_vcount, set_vcount);
  bitfield_bool!(u16; 3, timer0, with_timer0, set_timer0);
  bitfield_bool!(u16; 4, timer1, with_timer1, set_timer1);
  bitfield_bool!(u16; 5, timer2, with_timer2, set_timer2);
  bitfield_bool!(u16; 6, timer3, with_timer3, set_timer3);
  bitfield_bool!(u16; 7, serial, with_serial, set_serial);
  bitfield_bool!(u16; 8, dma0, with_dma0, set_dma0);
  bitfield_bool!(u16; 9, dma1, with_dma1, set_dma1);
  bitfield_bool!(u16; 10, dma2, with_dma2, set_dma2);
  bitfield_bool!(u16; 11, dma3, with_dma3, set_dma3);
  bitfield_bool!(u16; 12, keypad, with_keypad, set_keypad);
  bitfield_bool!(u16; 13, gamepak, with_gamepak, set_gamepak);
}
impl_bitwise_ops!(InterruptFlags);
impl core::fmt::Debug for InterruptFlags {
  #[inline(never)]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "InterruptFlags {{")?;
    if self.vblank() {
      write!(f, "vblank,")?;
    }
    if self.hblank() {
      write!(f, "hblank,")?;
    }
    if self.vcount() {
      write!(f, "vcount,")?;
    }
    if self.timer0() {
      write!(f, "timer0,")?;
    }
    if self.timer1() {
      write!(f, "timer1,")?;
    }
    if self.timer2() {
      write!(f, "timer2,")?;
    }
    if self.timer3() {
      write!(f, "timer3,")?;
    }
    if self.serial() {
      write!(f, "serial,")?;
    }
    if self.dma0() {
      write!(f, "dma0,")?;
    }
    if self.dma1() {
      write!(f, "dma1,")?;
    }
    if self.dma2() {
      write!(f, "dma2,")?;
    }
    if self.dma3() {
      write!(f, "dma3,")?;
    }
    if self.keypad() {
      write!(f, "keypad,")?;
    }
    if self.gamepak() {
      write!(f, "gamepak,")?;
    }
    write!(f, "}}")
  }
}
