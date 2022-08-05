use crate::region_mask::*;

macro_rules! impl_with_region {
  ($fn_name:ident, $t:ty, $mask_get_fn:ident) => {
    /// Replaces the `low` to `high` bit region of `old`, returning the new value.
    ///
    /// The `low` and `high` values form an *inclusive* bit range.
    ///
    /// Bits in `replacement` outside of the region have no effect.
    ///
    /// ## Panics
    /// * `low` and `high` can't exceed the number of bits in the type.
    /// * `low` must be less than `high`.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 2, 0b11111111_", stringify!($t), ", 0), 0b1111_1000_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 3, 0b11111111_", stringify!($t), ", 0), 0b1111_0001_", stringify!($t),");")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(4, 7, 0b11111111_", stringify!($t), ", 0), 0b0000_1111_", stringify!($t),");")]
    /// ```
    #[inline]
    #[must_use]
    pub const fn $fn_name(low: u32, high: u32, old: $t, replacement: $t) -> $t {
      let mask = $mask_get_fn(low, high);
      (old & !mask) | (replacement & mask)
    }
  };
}

impl_with_region!(u8_with_region, u8, u8_region_mask);
impl_with_region!(u16_with_region, u16, u16_region_mask);
impl_with_region!(u32_with_region, u32, u32_region_mask);
impl_with_region!(u64_with_region, u64, u64_region_mask);
impl_with_region!(u128_with_region, u128, u128_region_mask);
