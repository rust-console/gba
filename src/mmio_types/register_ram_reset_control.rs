use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ResetFlags(pub(crate) u8);
impl ResetFlags {
  const_new!();
  //bitfield_bool!(u8; 0, ewram, with_ewram, set_ewram);
  //bitfield_bool!(u8; 1, iwram, with_iwram, set_iwram);
  bitfield_bool!(u8; 2, palram, with_palram, set_palram);
  bitfield_bool!(u8; 3, vram, with_vram, set_vram);
  bitfield_bool!(u8; 4, oam, with_oam, set_oam);
  bitfield_bool!(u8; 5, sio, with_sio, set_sio);
  bitfield_bool!(u8; 6, sound, with_sound, set_sound);
  bitfield_bool!(u8; 7, all_other_io, with_all_other_io, set_all_other_io);
}
