macro_rules! impl_replicate_bits {
  ($fn_name:ident, $t:ty) => {
    /// Replicates the low `count` bits across the entire value.
    ///
    /// If higher bits are set in the input it will not affect the output.
    ///
    /// This is a form of "sample depth scaling". If your source data's bit
    /// depth is *less* than the full range of a `uX` value (eg, only 5 bits out
    /// of a `u8`), this function can scale up the sample into the integer's
    /// full range.
    ///
    /// ## Panics
    /// * `count` can't exceed the number of bits in the type.
    /// * `count` can't be 0.
    ///
    /// ```
    /// # use bitfrob::*;
    #[doc = concat!("assert_eq!(u8_replicate_bits(5, 0b0001_0111_u8), 0b1011_1101_u8);")]
    #[doc = concat!("assert_eq!(u16_replicate_bits(5, 0b0001_0111_u16), 0b1011_1101_1110_1111_u16);")]
    #[doc = concat!("assert_eq!(u32_replicate_bits(5, 0b0001_0111_u32), 0xBDEF_7BDE_u32);")]
    #[doc = concat!("assert_eq!(u64_replicate_bits(5, 0b0001_0111_u64), 0xBDEF_7BDE_F7BD_EF7B_u64);")]
    #[doc = concat!("assert_eq!(u128_replicate_bits(5, 0b0001_0111_u128), 0xBDEF_7BDE_F7BD_EF7B_DEF7_BDEF_7BDE_F7BD_u128);")]
    /// ```
    #[inline]
    #[must_use]
    pub fn $fn_name(mut count: u32, u: $t) -> $t {
      assert!(count <= <$t>::BITS);
      assert!(count > 0);
      let mut out_count = count;
      let mut out = u << (<$t>::BITS - count);
      while out_count < <$t>::BITS {
        out |= (out >> count);
        out_count += count;
        // Note(Lokathor): As the output builds the number of "good" bits can
        // double each time, which helps when small starting bits are replicated
        // across the larger int types.
        count *= 2;
      }
      out
    }
  };
}

impl_replicate_bits!(u8_replicate_bits, u8);
impl_replicate_bits!(u16_replicate_bits, u16);
impl_replicate_bits!(u32_replicate_bits, u32);
impl_replicate_bits!(u64_replicate_bits, u64);
impl_replicate_bits!(u128_replicate_bits, u128);

#[test]
fn test_replicate_bits_allows_full_bits() {
  // test passes if this doesn't panic
  let _ = u8_replicate_bits(8, 0xFF);
}
