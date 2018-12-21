#![no_std]
#![feature(start)]
#![feature(underscore_const_names)]

#[macro_export]
macro_rules! newtype {
  ($(#[$attr:meta])* $new_name:ident, $old_name:ident) => {
    $(#[$attr])*
    #[repr(transparent)]
    pub struct $new_name($old_name);
  };
}

#[macro_export]
macro_rules! const_assert {
  ($condition:expr) => {
    #[deny(const_err)]
    #[allow(dead_code)]
    const _: usize = 0 - !$condition as usize;
  };
}

/// Constructs an RGB value with a `const_assert!` that the input is in range.
#[macro_export]
macro_rules! const_rgb {
  ($r:expr, $g:expr, $b:expr) => {{
    const_assert!($r <= 31);
    const_assert!($g <= 31);
    const_assert!($b <= 31);
    Color::new($r, $g, $b)
  }};
}

mod vol_address {
  #![allow(unused)]
  use core::{cmp::Ordering, iter::FusedIterator, marker::PhantomData, num::NonZeroUsize};
  /// VolAddress
  #[derive(Debug)]
  #[repr(transparent)]
  pub struct VolAddress<T> {
    address: NonZeroUsize,
    marker: PhantomData<*mut T>,
  }
  impl<T> Clone for VolAddress<T> {
    fn clone(&self) -> Self {
      *self
    }
  }
  impl<T> Copy for VolAddress<T> {}
  impl<T> PartialEq for VolAddress<T> {
    fn eq(&self, other: &Self) -> bool {
      self.address == other.address
    }
  }
  impl<T> Eq for VolAddress<T> {}
  impl<T> PartialOrd for VolAddress<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.address.cmp(&other.address))
    }
  }
  impl<T> Ord for VolAddress<T> {
    fn cmp(&self, other: &Self) -> Ordering {
      self.address.cmp(&other.address)
    }
  }
  impl<T> VolAddress<T> {
    pub const unsafe fn new_unchecked(address: usize) -> Self {
      VolAddress {
        address: NonZeroUsize::new_unchecked(address),
        marker: PhantomData,
      }
    }
    pub const unsafe fn cast<Z>(self) -> VolAddress<Z> {
      VolAddress {
        address: self.address,
        marker: PhantomData,
      }
    }
    pub unsafe fn offset(self, offset: isize) -> Self {
      // TODO: const this
      VolAddress {
        address: NonZeroUsize::new_unchecked(self.address.get().wrapping_add(offset as usize * core::mem::size_of::<T>())),
        marker: PhantomData,
      }
    }
    pub fn is_aligned(self) -> bool {
      // TODO: const this
      self.address.get() % core::mem::align_of::<T>() == 0
    }
    pub const unsafe fn iter_slots(self, slots: usize) -> VolAddressIter<T> {
      VolAddressIter { vol_address: self, slots }
    }
    pub fn read(self) -> T
    where
      T: Copy,
    {
      unsafe { (self.address.get() as *mut T).read_volatile() }
    }
    pub unsafe fn read_non_copy(self) -> T {
      (self.address.get() as *mut T).read_volatile()
    }
    pub fn write(self, val: T) {
      unsafe { (self.address.get() as *mut T).write_volatile(val) }
    }
  }
  /// VolAddressIter
  #[derive(Debug)]
  pub struct VolAddressIter<T> {
    vol_address: VolAddress<T>,
    slots: usize,
  }
  impl<T> Clone for VolAddressIter<T> {
    fn clone(&self) -> Self {
      VolAddressIter {
        vol_address: self.vol_address,
        slots: self.slots,
      }
    }
  }
  impl<T> PartialEq for VolAddressIter<T> {
    fn eq(&self, other: &Self) -> bool {
      self.vol_address == other.vol_address && self.slots == other.slots
    }
  }
  impl<T> Eq for VolAddressIter<T> {}
  impl<T> Iterator for VolAddressIter<T> {
    type Item = VolAddress<T>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.slots > 0 {
        let out = self.vol_address;
        unsafe {
          self.slots -= 1;
          self.vol_address = self.vol_address.offset(1);
        }
        Some(out)
      } else {
        None
      }
    }
  }
  impl<T> FusedIterator for VolAddressIter<T> {}
  /// VolAddressBlock
  #[derive(Debug)]
  pub struct VolAddressBlock<T> {
    vol_address: VolAddress<T>,
    slots: usize,
  }
  impl<T> Clone for VolAddressBlock<T> {
    fn clone(&self) -> Self {
      VolAddressBlock {
        vol_address: self.vol_address,
        slots: self.slots,
      }
    }
  }
  impl<T> PartialEq for VolAddressBlock<T> {
    fn eq(&self, other: &Self) -> bool {
      self.vol_address == other.vol_address && self.slots == other.slots
    }
  }
  impl<T> Eq for VolAddressBlock<T> {}
  impl<T> VolAddressBlock<T> {
    pub const unsafe fn new_unchecked(vol_address: VolAddress<T>, slots: usize) -> Self {
      VolAddressBlock { vol_address, slots }
    }
    pub const fn iter(self) -> VolAddressIter<T> {
      VolAddressIter {
        vol_address: self.vol_address,
        slots: self.slots,
      }
    }
    pub unsafe fn index_unchecked(self, slot: usize) -> VolAddress<T> {
      // TODO: const this
      self.vol_address.offset(slot as isize)
    }
    pub fn index(self, slot: usize) -> VolAddress<T> {
      if slot < self.slots {
        unsafe { self.vol_address.offset(slot as isize) }
      } else {
        panic!("Index Requested: {} >= Bound: {}", slot, self.slots)
      }
    }
    pub fn get(self, slot: usize) -> Option<VolAddress<T>> {
      if slot < self.slots {
        unsafe { Some(self.vol_address.offset(slot as isize)) }
      } else {
        None
      }
    }
  }
}
use self::vol_address::*;

newtype! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  Color, u16
}

impl Color {
  /// Combines the Red, Blue, and Green provided into a single color value.
  pub const fn new(red: u16, green: u16, blue: u16) -> Color {
    Color(blue << 10 | green << 5 | red)
  }
}

newtype! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  DisplayControlSetting, u16
}

pub const DISPLAY_CONTROL: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new_unchecked(0x0400_0000) };
pub const JUST_MODE3: DisplayControlSetting = DisplayControlSetting(3);
pub const JUST_BG2: DisplayControlSetting = DisplayControlSetting(0b100_0000_0000);
pub const JUST_MODE3_AND_BG2: DisplayControlSetting = DisplayControlSetting(JUST_MODE3.0 | JUST_BG2.0);

pub struct Mode3;
impl Mode3 {
  const SCREEN_WIDTH: isize = 240;
  const SCREEN_HEIGHT: isize = 160;
  const PIXELS: VolAddressBlock<Color> =
    unsafe { VolAddressBlock::new_unchecked(VolAddress::new_unchecked(0x600_0000), (Self::SCREEN_WIDTH * Self::SCREEN_HEIGHT) as usize) };

  pub unsafe fn draw_pixel(col: usize, row: usize, color: Color) {
    Self::PIXELS.index(col + row * Self::SCREEN_WIDTH as usize).write(color);
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    DISPLAY_CONTROL.write(JUST_MODE3_AND_BG2);
    Mode3::draw_pixel(120, 80, const_rgb!(31, 0, 0));
    Mode3::draw_pixel(136, 80, const_rgb!(0, 31, 0));
    Mode3::draw_pixel(120, 96, const_rgb!(0, 0, 31));
    loop {}
  }
}
