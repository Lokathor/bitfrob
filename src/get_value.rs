use crate::get_region::*;

macro_rules! impl_get_value {
  ($fn_name:ident, $t:ty, $region_get_fn:ident) => {
    /// Get the `low` to `high` bit region of `u` shifted to be a normal value.
    ///
    /// The `low` and `high` values form an *inclusive* bit range.
    ///
    /// The output is down shifted by `low` bits so that it will be based at 0.
    ///
    /// ## Panics
    /// * `low` and `high` can't exceed the number of bits in the type.
    /// * `low` must be less than `high`.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 2, ", stringify!($t), "::MAX), 0b0000_0111_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 3, ", stringify!($t), "::MAX), 0b0000_0111_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(4, 7, ", stringify!($t), "::MAX), 0b0000_1111_", stringify!($t),");")]
    /// ```
    #[inline]
    #[must_use]
    pub const fn $fn_name(low: u32, high: u32, u: $t) -> $t {
      $region_get_fn(low, high, u) >> low
    }
  };
}

impl_get_value!(u8_get_value, u8, u8_get_region);
impl_get_value!(u16_get_value, u16, u16_get_region);
impl_get_value!(u32_get_value, u32, u32_get_region);
impl_get_value!(u64_get_value, u64, u64_get_region);
impl_get_value!(u128_get_value, u128, u128_get_region);
