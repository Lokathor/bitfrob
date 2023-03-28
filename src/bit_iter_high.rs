use crate::region_mask::*;

macro_rules! impl_bit_iter_high {
  ($name:ident, $elem:ty, $region_mask_fn:ident) => {
    /// Iterator for groups of bits in an integer (low to high).
    ///
    /// The number of bits per iteration does *not* have to be an even divisor.
    /// If it's not, the final output will simply have a partial number of
    /// significant bits.
    #[derive(Debug, Clone)]
    #[allow(missing_copy_implementations)]
    pub struct $name {
      bits: $elem,
      mask: $elem,
      bits_per_iter: u32,
      bits_remaining: i32,
    }
    impl $name {
      /// Constructs a new iterator for `bits_per_iter` at a time out of the
      /// `bits` value.
      ///
      /// If `bits_per_iter` doesn't divide evenly into the number of bits in
      /// the type then the final output of the iterator will be the partial
      /// chunk of bits aligned to the *high* end of the bit span.
      ///
      /// ## Panics
      /// * `bits_per_iter` must be greater than 0.
      /// * `bits_per_iter` must be less than or equal to the number of bits in
      ///   the type.
      #[inline]
      #[must_use]
      #[cfg_attr(feature = "track_caller", track_caller)]
      pub const fn from_count_and_bits(
        bits_per_iter: u32, bits: $elem,
      ) -> Self {
        assert!(bits_per_iter > 0);
        assert!(bits_per_iter <= <$elem>::BITS);
        Self {
          bits,
          mask: $region_mask_fn(0, bits_per_iter - 1)
            << (<$elem>::BITS - bits_per_iter),
          bits_per_iter,
          bits_remaining: <$elem>::BITS as i32,
        }
      }
    }
    impl Iterator for $name {
      type Item = $elem;
      #[inline]
      fn next(&mut self) -> Option<$elem> {
        if self.bits_remaining < 1 {
          None
        } else {
          let out: $elem =
            (self.bits & self.mask) >> (<$elem>::BITS - self.bits_per_iter);
          self.bits = self.bits.wrapping_shl(self.bits_per_iter);
          self.bits_remaining -= self.bits_per_iter as i32;
          Some(out)
        }
      }
    }
  };
}

impl_bit_iter_high!(U8BitIterHigh, u8, u8_region_mask);
impl_bit_iter_high!(U16BitIterHigh, u16, u16_region_mask);
impl_bit_iter_high!(U32BitIterHigh, u32, u32_region_mask);
impl_bit_iter_high!(U64BitIterHigh, u64, u64_region_mask);
impl_bit_iter_high!(U128BitIterHigh, u128, u128_region_mask);

#[test]
fn test_U8BitIterHigh() {
  let mut iter = U8BitIterHigh::from_count_and_bits(1, 0b1011_0111_u8);
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), Some(0b0_u8));
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), Some(0b0_u8));
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), Some(0b1_u8));
  assert_eq!(iter.next(), None);

  let mut iter = U8BitIterHigh::from_count_and_bits(2, 0b1011_0111_u8);
  assert_eq!(iter.next(), Some(0b10_u8));
  assert_eq!(iter.next(), Some(0b11_u8));
  assert_eq!(iter.next(), Some(0b01_u8));
  assert_eq!(iter.next(), Some(0b11_u8));
  assert_eq!(iter.next(), None);

  let mut iter = U8BitIterHigh::from_count_and_bits(3, 0b1011_0111_u8);
  assert_eq!(iter.next(), Some(0b101_u8));
  assert_eq!(iter.next(), Some(0b101_u8));
  assert_eq!(iter.next(), Some(0b110_u8));
  assert_eq!(iter.next(), None);

  let mut iter = U8BitIterHigh::from_count_and_bits(8, 0b1011_0111_u8);
  assert_eq!(iter.next(), Some(0b1011_0111_u8));
  assert_eq!(iter.next(), None);
}
