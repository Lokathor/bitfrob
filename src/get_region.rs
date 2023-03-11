use crate::region_mask::*;

macro_rules! impl_get_region {
  ($fn_name:ident, $t:ty, $mask_get_fn:ident) => {
    /// Get the `low` to `high` bit region of `u`.
    ///
    /// The `low` and `high` values form an *inclusive* bit range.
    ///
    /// ## Panics
    /// * `low` and `high` can't exceed the number of bits in the type.
    /// * `low` must be less than `high`.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 2, ", stringify!($t), "::MAX), 0b0000_0111_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 3, ", stringify!($t), "::MAX), 0b0000_1110_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(4, 7, ", stringify!($t), "::MAX), 0b1111_0000_", stringify!($t),");")]
    /// ```
    #[inline]
    #[must_use]
    #[cfg_attr(feature = "track_caller", track_caller)]
    pub const fn $fn_name(low: u32, high: u32, u: $t) -> $t {
      let mask = $mask_get_fn(low, high);
      (u & mask)
    }
  };
}

impl_get_region!(u8_get_region, u8, u8_region_mask);
impl_get_region!(u16_get_region, u16, u16_region_mask);
impl_get_region!(u32_get_region, u32, u32_region_mask);
impl_get_region!(u64_get_region, u64, u64_region_mask);
impl_get_region!(u128_get_region, u128, u128_region_mask);
