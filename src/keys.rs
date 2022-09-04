use voladdress::{Safe, VolAddress};

pub const KEYINPUT: VolAddress<u16, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };
