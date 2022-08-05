use crate::with_region::*;

macro_rules! impl_with_value {
  ($fn_name:ident, $t:ty, $with_region_fn:ident) => {
    /// Replaces the `low` to `high` bit region of `u_old`, returning the new
    /// value.
    ///
    /// The `low` and `high` values form an *inclusive* bit range.
    ///
    /// Bits in `u_replace` outside of the region have no effect.
    ///
    /// ## Panics
    /// * `low` and `high` can't exceed the number of bits in the type.
    /// * `low` must be less than `high`.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 2, 0, 1), 1 << 0);")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 3, 0, 1), 1 << 1);")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(4, 7, 0, 1), 1 << 4);")]
    /// ```
    #[inline]
    #[must_use]
    pub const fn $fn_name(low: u32, high: u32, u_old: $t, u_replace: $t) -> $t {
      $with_region_fn(low, high, u_old, u_replace << low)
    }
  };
}

impl_with_value!(u8_with_value, u8, u8_with_region);
impl_with_value!(u16_with_value, u16, u16_with_region);
impl_with_value!(u32_with_value, u32, u32_with_region);
impl_with_value!(u64_with_value, u64, u64_with_region);
impl_with_value!(u128_with_value, u128, u128_with_region);
