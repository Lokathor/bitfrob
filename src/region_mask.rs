macro_rules! impl_region_mask {
  ($fn_name:ident, $t:ty) => {
    /// Generates a bit mask where all bits in the region are 1.
    ///
    /// The `low` and `high` values form an *inclusive* bit range where the mask
    /// bits are 1.
    ///
    /// This is largely a helper function for other functions in this crate, but
    /// you can use it yourself if you think it's useful.
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

/*

backup: we might want to do the ZST associated const thing?

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

*/
