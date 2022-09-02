macro_rules! impl_get_bit {
  ($fn_name:ident, $t:ty) => {
    /// Determines if the `b` bit is set in `u`.
    ///
    /// ## Panics
    /// * `b` can't exceed the number of bits in the type.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(0, 0b0000_1110_", stringify!($t), "), false);")]
    #[doc = concat!("assert_eq!(",stringify!($fn_name), "(1, 0b0000_1110_", stringify!($t), "), true);")]
    /// ```
    #[inline]
    #[must_use]
    #[cfg_attr(feature = "track_caller", track_caller)]
    pub const fn $fn_name(b: u32, u: $t) -> bool {
      assert!(b < <$t>::BITS);
      let mask = 1 << b;
      (u & mask) != 0
    }
  };
}

impl_get_bit!(u8_get_bit, u8);
impl_get_bit!(u16_get_bit, u16);
impl_get_bit!(u32_get_bit, u32);
impl_get_bit!(u64_get_bit, u64);
impl_get_bit!(u128_get_bit, u128);
