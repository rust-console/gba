/// Gets the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u16_get_bit<const B: u32>(x: u16) -> bool {
  assert!(B < 16);
  let mask = 1 << B;
  (x & mask) != 0
}

/// Replaces the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u16_with_bit<const B: u32>(x: u16, val: bool) -> u16 {
  assert!(B < 16);
  let mask = 1 << B;
  (x & !mask) | ((val as u16) << B)
}

/// Gets a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u16_get_region<const L: u32, const H: u32>(x: u16) -> u16 {
  assert!(L < 16);
  assert!(H < 16);
  assert!(L < H);
  let mask = (((1_u64 << (H - L + 1)) - 1) << L) as u16;
  assert!(mask.count_ones() == (H - L + 1));
  x & mask
}

/// Replaces a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u16_with_region<const L: u32, const H: u32>(
  x: u16, val: u16,
) -> u16 {
  assert!(L < 16);
  assert!(H < 16);
  assert!(L < H);
  let mask = (((1_u64 << (H - L + 1)) - 1) << L) as u16;
  assert!(mask.count_ones() == (H - L + 1));
  (x & !mask) | val
}

/// Like [`u16_get_region`] but the output is shifted down appropriately.
///
/// ## Panics
/// * As `u16_get_region`
#[inline]
#[must_use]
pub const fn u16_get_value<const L: u32, const H: u32>(x: u16) -> u16 {
  u16_get_region::<L, H>(x) >> L
}

/// Like [`u16_with_region`] but the value is shifted up appropriately.
///
/// ## Panics
/// * As `u16_with_region`
#[inline]
#[must_use]
pub const fn u16_with_value<const L: u32, const H: u32>(
  x: u16, val: u16,
) -> u16 {
  u16_with_region::<L, H>(x, val << L)
}
