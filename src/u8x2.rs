use crate::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct u8x2(u16);
impl u8x2 {
  #[inline]
  #[must_use]
  pub const fn low(self) -> u8 {
    u16_get_value::<0,7>(self.0) as u8
  }
  #[inline]
  #[must_use]
  pub const fn with_low(self, low: u8) -> Self {
    Self(u16_with_value::<0,7>(self.0, low as u16))
  }
  #[inline]
  #[must_use]
  pub const fn high(self) -> u8 {
    u16_get_value::<8,15>(self.0) as u8
  }
  #[inline]
  #[must_use]
  pub const fn with_high(self, high: u8) -> Self {
    Self(u16_with_value::<8,15>(self.0, high as u16))
  }
}
