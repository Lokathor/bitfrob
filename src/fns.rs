/// Generates a bit mask where all bits in the region are 1.
///
/// The `L` and `H` parameters form an *inclusive* bit range where the mask bits
/// are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u32_region_mask::<0, 2>(), 0b0000_0111_u32);
/// assert_eq!(u32_region_mask::<1, 3>(), 0b0000_1110_u32);
/// assert_eq!(u32_region_mask::<4, 7>(), 0b1111_0000_u32);
/// ```
///
/// ## Panics
/// * Both `L` and `H` must be less than 32.
/// * `L` must be less than `H`.
pub const fn u32_region_mask<const L: u32, const H: u32>() -> u32 {
  assert!(L < u32::BITS);
  assert!(H < u32::BITS);
  assert!(L < H);
  (((-1_i32) as u32) >> ((u32::BITS - 1) - (H - L))) << L
}

/// Generates a bit mask where all bits in the region are 1.
///
/// The `L` and `H` parameters form an *inclusive* bit range where the mask bits
/// are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u16_region_mask::<0, 2>(), 0b0000_0111_u16);
/// assert_eq!(u16_region_mask::<1, 3>(), 0b0000_1110_u16);
/// assert_eq!(u16_region_mask::<4, 7>(), 0b1111_0000_u16);
/// ```
///
/// ## Panics
/// * Both `L` and `H` must be less than 16.
/// * `L` must be less than `H`.
pub const fn u16_region_mask<const L: u32, const H: u32>() -> u16 {
  assert!(L < u16::BITS);
  assert!(H < u16::BITS);
  assert!(L < H);
  (((-1_i16) as u16) >> ((u16::BITS - 1) - (H - L))) << L
}

/// Generates a bit mask where all bits in the region are 1.
///
/// The `L` and `H` parameters form an *inclusive* bit range where the mask bits
/// are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u8_region_mask::<0, 2>(), 0b0000_0111_u8);
/// assert_eq!(u8_region_mask::<1, 3>(), 0b0000_1110_u8);
/// assert_eq!(u8_region_mask::<4, 7>(), 0b1111_0000_u8);
/// ```
///
/// ## Panics
/// * Both `L` and `H` must be less than 8.
/// * `L` must be less than `H`.
pub const fn u8_region_mask<const L: u32, const H: u32>() -> u8 {
  assert!(L < u8::BITS);
  assert!(H < u8::BITS);
  assert!(L < H);
  (((-1_i8) as u8) >> ((u8::BITS - 1) - (H - L))) << L
}

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
  assert!(L < 32);
  assert!(H < 32);
  assert!(L < H);
  let mask = u32_region_mask::<L, H>();
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
  assert!(L < 32);
  assert!(H < 32);
  assert!(L < H);
  let mask = u32_region_mask::<L, H>();
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
  assert!(L < 16);
  assert!(H < 16);
  assert!(L < H);
  let mask = u16_region_mask::<L, H>();
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
  let mask = u16_region_mask::<L, H>();
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
  assert!(L < 8);
  assert!(H < 8);
  assert!(L < H);
  let mask = u8_region_mask::<L, H>();
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
  assert!(L < 8);
  assert!(H < 8);
  assert!(L < H);
  let mask = u8_region_mask::<L, H>();
  (x & !mask) | val
}
