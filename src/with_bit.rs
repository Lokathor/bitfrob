macro_rules! impl_with_bit {
  ($fn_name:ident, $t:ty) => {
    /// Replaces the `b` bit in `u`, returning the new value.
    ///
    /// ## Panics
    /// * `b` can't exceed the number of bits in the type.
    #[inline]
    #[must_use]
    pub const fn $fn_name(b: u32, x: $t, val: bool) -> $t {
      assert!(b < <$t>::BITS);
      let mask = 1 << b;
      (x & !mask) | ((val as $t) << b)
    }
  };
}

impl_with_bit!(u8_with_bit, u8);
impl_with_bit!(u16_with_bit, u16);
impl_with_bit!(u32_with_bit, u32);
impl_with_bit!(u64_with_bit, u64);
impl_with_bit!(u128_with_bit, u128);
