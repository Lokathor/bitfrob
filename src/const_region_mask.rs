/// Generates a bit mask where all bits in the region are 1.
///
/// The `low` and `high` values form an *inclusive* bit range where the mask
/// bits are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u32_region_mask(0, 2), 0b0000_0111_u32);
/// assert_eq!(u32_region_mask(1, 3), 0b0000_1110_u32);
/// assert_eq!(u32_region_mask(4, 7), 0b1111_0000_u32);
/// ```
///
/// ## Panics
/// * Both `low` and `high` must be less than 32.
/// * `low` must be less than `high`.
#[inline]
#[must_use]
pub const fn u32_region_mask(low: u32, high: u32) -> u32 {
  assert!(low < u32::BITS);
  assert!(high < u32::BITS);
  assert!(low < high);
  (((-1_i32) as u32) >> ((u32::BITS - 1) - (high - low))) << low
}

/// Like [`u32_region_mask`], but forces the value into an associated constant.
///
/// * Advantage: the compiler is forced to compute the constant at compile time,
///   regardless of debug level.
/// * Disadvantage: the `L` and `H` must themselves be constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U32ConstRegionMask<const L: u32, const H: u32>;
impl<const L: u32, const H: u32> U32ConstRegionMask<L, H> {
  /// The computed bit mask.
  pub const OUT: u32 = u32_region_mask(L, H);
}

/// Generates a bit mask where all bits in the region are 1.
///
/// The `low` and `high` values form an *inclusive* bit range where the mask
/// bits are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u16_region_mask(0, 2), 0b0000_0111_u16);
/// assert_eq!(u16_region_mask(1, 3), 0b0000_1110_u16);
/// assert_eq!(u16_region_mask(4, 7), 0b1111_0000_u16);
/// ```
///
/// ## Panics
/// * Both `low` and `high` must be less than 16.
/// * `low` must be less than `high`.
#[inline]
#[must_use]
pub const fn u16_region_mask(low: u32, high: u32) -> u16 {
  assert!(low < u16::BITS);
  assert!(high < u16::BITS);
  assert!(low < high);
  (((-1_i16) as u16) >> ((u16::BITS - 1) - (high - low))) << low
}

/// Like [`u16_region_mask`], but forces the value into an associated constant.
///
/// * Advantage: the compiler is forced to compute the constant at compile time,
///   regardless of debug level.
/// * Disadvantage: the `L` and `H` must themselves be constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U16ConstRegionMask<const L: u32, const H: u32>;
impl<const L: u32, const H: u32> U16ConstRegionMask<L, H> {
  /// The computed bit mask.
  pub const OUT: u16 = u16_region_mask(L, H);
}

/// Generates a bit mask where all bits in the region are 1.
///
/// The `low` and `high` values form an *inclusive* bit range where the mask
/// bits are 1.
///
/// ```
/// # use bitfrob::*;
/// assert_eq!(u8_region_mask(0, 2), 0b0000_0111_u8);
/// assert_eq!(u8_region_mask(1, 3), 0b0000_1110_u8);
/// assert_eq!(u8_region_mask(4, 7), 0b1111_0000_u8);
/// ```
///
/// ## Panics
/// * Both `low` and `high` must be less than 32.
/// * `low` must be less than `high`.
#[inline]
#[must_use]
pub const fn u8_region_mask(low: u32, high: u32) -> u8 {
  assert!(low < u8::BITS);
  assert!(high < u8::BITS);
  assert!(low < high);
  (((-1_i8) as u8) >> ((u8::BITS - 1) - (high - low))) << low
}

/// Like [`u8_region_mask`], but forces the value into an associated constant.
///
/// * Advantage: the compiler is forced to compute the constant at compile time,
///   regardless of debug level.
/// * Disadvantage: the `L` and `H` must themselves be constants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U8ConstRegionMask<const L: u32, const H: u32>;
impl<const L: u32, const H: u32> U8ConstRegionMask<L, H> {
  /// The computed bit mask.
  pub const OUT: u8 = u8_region_mask(L, H);
}
