macro_rules! impl_region_mask {
  ($fn_name:ident, $t:ty) => {
    /// Generates a bit mask where all bits in the region are 1.
    ///
    /// The `low` and `high` values form an *inclusive* bit range where the mask
    /// bits are 1.
    ///
    /// ## Panics
    /// * `low` and `high` can't exceed the number of bits in the type.
    /// * `low` must be less than `high`.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 2), 0b0000_0111_", stringify!($t), ");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 3), 0b0000_1110_", stringify!($t), ");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(4, 7), 0b1111_0000_", stringify!($t), ");")]
    /// ```
    #[inline]
    #[must_use]
    pub const fn $fn_name(low: u32, high: u32) -> $t {
      assert!(low < <$t>::BITS);
      assert!(high < <$t>::BITS);
      assert!(low < high);
      (<$t>::MAX >> ((<$t>::BITS - 1) - (high - low))) << low
    }
  };
}

impl_region_mask!(u8_region_mask, u8);
impl_region_mask!(u16_region_mask, u16);
impl_region_mask!(u32_region_mask, u32);
impl_region_mask!(u64_region_mask, u64);
impl_region_mask!(u128_region_mask, u128);
