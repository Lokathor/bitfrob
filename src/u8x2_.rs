use crate::*;

/// Two `u8` values packed as a `u16`.
///
/// This forces the data to have an alignment of 2, while keeping volatile reads
/// and writes as a single access (depending on CPU).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u8x2(u16);
impl u8x2 {
  /// The lower byte
  #[inline]
  #[must_use]
  pub const fn low(self) -> u8 {
    u16_get_value(0, 7, self.0) as u8
  }
  /// Updates the lower byte value, returning the new `u8x2`
  #[inline]
  #[must_use]
  pub const fn with_low(self, low: u8) -> Self {
    Self(u16_with_value(0, 7, self.0, low as u16))
  }
  /// The upper byte
  #[inline]
  #[must_use]
  pub const fn high(self) -> u8 {
    u16_get_value(8, 15, self.0) as u8
  }
  /// Updates the upper byte value, returning the new `u8x2`
  #[inline]
  #[must_use]
  pub const fn with_high(self, high: u8) -> Self {
    Self(u16_with_value(8, 15, self.0, high as u16))
  }
}
