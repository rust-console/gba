//! Things that I wish were in core, but aren't.

//TODO(Lokathor): reorganize as gba::core_extras::fixed_point and gba::core_extras::volatile ?

use core::{cmp::Ordering, iter::FusedIterator, marker::PhantomData, num::NonZeroUsize};

/// Abstracts the use of a volatile hardware address.
///
/// If you're trying to do anything other than abstract a volatile hardware
/// device then you _do not want to use this type_. Use one of the many other
/// smart pointer types.
///
/// A volatile address doesn't store a value in the normal way: It maps to some
/// real hardware _other than_ RAM, and that hardware might have any sort of
/// strange rules. The specifics of reading and writing depend on the hardware
/// being mapped. For example, a particular address might be read only (ignoring
/// writes), write only (returning some arbitrary value if you read it),
/// "normal" read write (where you read back what you wrote), or some complex
/// read-write situation where writes have an effect but you _don't_ read back
/// what you wrote.
///
/// As you imagine it can be very unsafe. The design of this type is set up so
/// that _creation_ is unsafe, and _use_ is safe. This gives an optimal
/// experience, since you'll use memory locations a lot more often than you try
/// to name them, on average.
///
/// `VolAddress` is _not_ a thread safe type. If your device is multi-threaded
/// then you must arrange for synchronization in some other way. A `VolAddress`
/// _can_ be used to share data between an interrupt running on a core and a
/// thread running on that core as long as all access of that location is
/// volatile (if you're using the `asm!` macro add the "volatile" option, if
/// you're linking in ASM with the linker that's effectively volatile since the
/// compiler doesn't get a chance to mess with it).
///
/// # Safety
///
/// In order for values of this type to operate correctly they must follow quite
/// a few safety limits:
///
/// * The declared address must be non-null (it uses the `NonNull` optimization
///   for better iteration results). This shouldn't be a big problem, since
///   hardware can't really live at the null address.
/// * The declared address must be aligned for the declared type of `T`.
/// * The declared address must _always_ read as something that's a valid bit
///   pattern for `T`. Don't pick any enums or things like that if your hardware
///   doesn't back it up. If there's _any_ doubt at all, you must instead read
///   or write an unsigned int of the correct bit size and then parse the bits
///   by hand.
/// * The declared address must be a part of the address space that Rust's
///   allocator and/or stack frames will never use. If you're not sure, please
///   re-read the hardware specs of your device and its memory map until you
///   know.
///
/// The exact points of UB are if the address is ever 0, or if you ever `read`
/// or `write` with the invalid pointer. For example, if you offset to some
/// crazy (non-zero) value and then never use it that won't be an immediate
/// trigger of UB.
#[derive(Debug)]
#[repr(transparent)]
pub struct VolAddress<T> {
  address: NonZeroUsize,
  marker: PhantomData<*mut T>,
}
// Note(Lokathor): We have to hand implement all these traits because if we use
// `derive` then they only get derived if the inner `T` has the trait. However,
// since we're acting like a pointer to `T`, the capability we offer isn't
// affected by whatever type `T` ends up being.
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
  /// Constructs a new address.
  ///
  /// # Safety
  ///
  /// You must follow the standard safety rules as outlined in the type docs.
  pub const unsafe fn new_unchecked(address: usize) -> Self {
    VolAddress {
      address: NonZeroUsize::new_unchecked(address),
      marker: PhantomData,
    }
  }

  /// Casts the type of `T` into type `Z`.
  ///
  /// # Safety
  ///
  /// You must follow the standard safety rules as outlined in the type docs.
  pub const unsafe fn cast<Z>(self) -> VolAddress<Z> {
    VolAddress {
      address: self.address,
      marker: PhantomData,
    }
  }

  /// Offsets the address by `offset` slots (like `pointer::wrapping_offset`).
  ///
  /// # Safety
  ///
  /// You must follow the standard safety rules as outlined in the type docs.
  pub unsafe fn offset(self, offset: isize) -> Self {
    // TODO: const this
    VolAddress {
      address: NonZeroUsize::new_unchecked(self.address.get().wrapping_add(offset as usize * core::mem::size_of::<T>())),
      marker: PhantomData,
    }
  }

  /// Checks that the current target type of this address is aligned at this
  /// address value.
  ///
  /// Technically it's a safety violation to even make a `VolAddress` that isn't
  /// aligned. However, I know you're gonna try doing the bad thing, and it's
  /// better to give you a chance to call `is_aligned` and potentially back off
  /// from the operation or throw a `debug_assert!` or something instead of
  /// triggering UB. Eventually this will be `const fn`, which will potentially
  /// let you spot errors without even having to run your program.
  pub fn is_aligned(self) -> bool {
    // TODO: const this
    self.address.get() % core::mem::align_of::<T>() == 0
  }

  /// Makes an iterator starting here across the given number of slots.
  ///
  /// # Safety
  ///
  /// The normal safety rules must be correct for each address iterated over.
  pub const unsafe fn iter_slots(self, slots: usize) -> VolAddressIter<T> {
    VolAddressIter { vol_address: self, slots }
  }

  // non-const and never can be.

  /// Reads a `Copy` value out of the address.
  ///
  /// The `Copy` bound is actually supposed to be `!Drop`, but rust doesn't
  /// allow negative trait bounds. If your type isn't `Copy` you can use the
  /// `read_non_copy` fallback to do an unsafe read.
  ///
  /// That said, I don't think that you legitimately have hardware that maps to
  /// a Rust type with a `Drop` impl. If you do please tell me, I'm interested
  /// to hear about it.
  pub fn read(self) -> T
  where
    T: Copy,
  {
    unsafe { (self.address.get() as *mut T).read_volatile() }
  }

  /// Reads a value out of the address with no trait bound.
  ///
  /// # Safety
  ///
  /// This is _not_ a move, it forms a bit duplicate of the current address
  /// value. If `T` has a `Drop` trait that does anything it is up to you to
  /// ensure that repeated drops do not cause UB (such as a double free).
  pub unsafe fn read_non_copy(self) -> T {
    (self.address.get() as *mut T).read_volatile()
  }

  /// Writes a value to the address.
  ///
  /// Semantically, the value is moved into the `VolAddress` and then forgotten,
  /// so if `T` has a `Drop` impl then that will never get executed. This is
  /// "safe" under Rust's safety rules, but could cause something unintended
  /// (eg: a memory leak).
  pub fn write(self, val: T) {
    unsafe { (self.address.get() as *mut T).write_volatile(val) }
  }
}

/// An iterator that produces a series of `VolAddress` values.
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

/// This type is like `VolAddress`, but for when you have a block of values all
/// in a row.
///
/// This is similar to the idea of an array or a slice, but called a "block"
/// because you could _also_ construct a `[VolAddress]`, and we want to avoid
/// any accidental confusion.
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
  /// Constructs a new `VolAddressBlock`.
  ///
  /// # Safety
  ///
  /// The given `VolAddress` must be valid when offset by each of `0 .. slots`
  pub const unsafe fn new_unchecked(vol_address: VolAddress<T>, slots: usize) -> Self {
    VolAddressBlock { vol_address, slots }
  }

  /// Gives an iterator over this block's slots.
  pub const fn iter(self) -> VolAddressIter<T> {
    VolAddressIter {
      vol_address: self.vol_address,
      slots: self.slots,
    }
  }

  /// Unchecked indexing into the block.
  ///
  /// # Safety
  ///
  /// The slot given must be in bounds.
  pub unsafe fn index_unchecked(self, slot: usize) -> VolAddress<T> {
    // TODO: const this
    self.vol_address.offset(slot as isize)
  }

  /// Checked "indexing" style access of the block, giving either a `VolAddress` or a panic.
  pub fn index(self, slot: usize) -> VolAddress<T> {
    if slot < self.slots {
      unsafe { self.vol_address.offset(slot as isize) }
    } else {
      panic!("Index Requested: {} >= Bound: {}", slot, self.slots)
    }
  }

  /// Checked "getting" style access of the block, giving an Option value.
  pub fn get(self, slot: usize) -> Option<VolAddress<T>> {
    if slot < self.slots {
      unsafe { Some(self.vol_address.offset(slot as isize)) }
    } else {
      None
    }
  }
}
