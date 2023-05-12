use crate::*;

/// Two `u8` values packed as a `u16`.
///
/// Compared to using `[u8; 2]`, this forces the data to have an alignment of 2,
/// and also keeps volatile reads and writes as a single access (when possible).
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

impl From<[u8; 2]> for u8x2 {
  #[inline]
  fn from(value: [u8; 2]) -> Self {
    Self(u16::from_ne_bytes(value))
  }
}

impl From<u8x2> for [u8; 2] {
  #[inline]
  fn from(value: u8x2) -> Self {
    value.0.to_ne_bytes()
  }
}
