use crate::*;

/// Gets the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u32_get_bit<const B: u32>(x: u32) -> bool {
  assert!(B < 32);
  let mask = 1 << B;
  (x & mask) != 0
}

/// Replaces the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u32_with_bit<const B: u32>(x: u32, val: bool) -> u32 {
  assert!(B < 32);
  let mask = 1 << B;
  (x & !mask) | ((val as u32) << B)
}

/// Gets a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u32_get_region<const L: u32, const H: u32>(x: u32) -> u32 {
  let mask = U32ConstRegionMask::<L, H>::OUT;
  x & mask
}

/// Replaces a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u32_with_region<const L: u32, const H: u32>(
  x: u32, val: u32,
) -> u32 {
  let mask = U32ConstRegionMask::<L, H>::OUT;
  (x & !mask) | val
}

/// Like [`u32_get_region`] but the output is shifted down appropriately.
///
/// ## Panics
/// * As `u32_get_region`
#[inline]
#[must_use]
pub const fn u32_get_value<const L: u32, const H: u32>(x: u32) -> u32 {
  u32_get_region::<L, H>(x) >> L
}

/// Like [`u32_with_region`] but the value is shifted up appropriately.
///
/// ## Panics
/// * As `u32_with_region`
#[inline]
#[must_use]
pub const fn u32_with_value<const L: u32, const H: u32>(
  x: u32, val: u32,
) -> u32 {
  u32_with_region::<L, H>(x, val << L)
}

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
  let mask = U16ConstRegionMask::<L, H>::OUT;
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
  let mask = U16ConstRegionMask::<L, H>::OUT;
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

/// Gets the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u8_get_bit<const B: u32>(x: u8) -> bool {
  assert!(B < 8);
  let mask = 1 << B;
  (x & mask) != 0
}

/// Replaces the `B` bit.
///
/// ## Panics
/// * If the bit requested is out of range.
#[inline]
#[must_use]
pub const fn u8_with_bit<const B: u32>(x: u8, val: bool) -> u8 {
  assert!(B < 8);
  let mask = 1 << B;
  (x & !mask) | ((val as u8) << B)
}

/// Like [`u8_get_region`] but the output is shifted down appropriately.
///
/// ## Panics
/// * As `u8_get_region`
#[inline]
#[must_use]
pub const fn u8_get_value<const L: u32, const H: u32>(x: u8) -> u8 {
  u8_get_region::<L, H>(x) >> L
}

/// Like [`u8_with_region`] but the value is shifted up appropriately.
///
/// ## Panics
/// * As `u8_with_region`
#[inline]
#[must_use]
pub const fn u8_with_value<const L: u32, const H: u32>(x: u8, val: u8) -> u8 {
  u8_with_region::<L, H>(x, val << L)
}

/// Gets a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u8_get_region<const L: u32, const H: u32>(x: u8) -> u8 {
  let mask = U8ConstRegionMask::<L, H>::OUT;
  x & mask
}

/// Replaces a `L` to `H` (inclusive) bit region of the value.
///
/// ## Panics
/// * If `L` or `H` are out of range.
/// * If `L` >= `H`
#[inline]
#[must_use]
pub const fn u8_with_region<const L: u32, const H: u32>(x: u8, val: u8) -> u8 {
  let mask = U8ConstRegionMask::<L, H>::OUT;
  (x & !mask) | val
}
