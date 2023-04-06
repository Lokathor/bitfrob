/// Splits a byte into 1-bit chunks.
///
/// Bit 0 of the input will be in the **first** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_1x8(byte: u8) -> [u8; 8] {
  [
    (byte >> 0) & 0b1,
    (byte >> 1) & 0b1,
    (byte >> 2) & 0b1,
    (byte >> 3) & 0b1,
    (byte >> 4) & 0b1,
    (byte >> 5) & 0b1,
    (byte >> 6) & 0b1,
    (byte >> 7) & 0b1,
  ]
}

/// Splits a byte into 2-bit chunks.
///
/// Bit 0 of the input will be in the **first** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_2x4(byte: u8) -> [u8; 4] {
  [
    (byte >> 0) & 0b11,
    (byte >> 2) & 0b11,
    (byte >> 4) & 0b11,
    (byte >> 6) & 0b11,
  ]
}

/// Splits a byte into 4-bit chunks.
///
/// Bit 0 of the input will be in the **first** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_4x2(byte: u8) -> [u8; 2] {
  [(byte >> 0) & 0b1111, (byte >> 4) & 0b1111]
}

/// Splits a byte into 1-bit chunks (reversed).
///
/// Bit 0 of the input will be in the **last** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_1x8_rev(byte: u8) -> [u8; 8] {
  [
    (byte >> 7) & 0b1,
    (byte >> 6) & 0b1,
    (byte >> 5) & 0b1,
    (byte >> 4) & 0b1,
    (byte >> 3) & 0b1,
    (byte >> 2) & 0b1,
    (byte >> 1) & 0b1,
    (byte >> 0) & 0b1,
  ]
}

/// Splits a byte into 2-bit chunks (reversed).
///
/// Bit 0 of the input will be in the **last** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_2x4_rev(byte: u8) -> [u8; 4] {
  [
    (byte >> 6) & 0b11,
    (byte >> 4) & 0b11,
    (byte >> 2) & 0b11,
    (byte >> 0) & 0b11,
  ]
}

/// Splits a byte into 4-bit chunks (reversed).
///
/// Bit 0 of the input will be in the **last** element of the output.
#[inline]
#[must_use]
pub const fn u8_bit_split_4x2_rev(byte: u8) -> [u8; 2] {
  [(byte >> 4) & 0b1111, (byte >> 0) & 0b1111]
}

/// When used as a multiplier, scales a "1 bit" `u8` to spread the value across
/// all 8 bits.
pub const U8_SCALE_1_TO_8: u8 = 0b_11111111;

/// When used as a multiplier, scales a "2 bit" `u8` to spread the value across
/// all 8 bits.
pub const U8_SCALE_2_TO_8: u8 = 0b_01010101;

/// When used as a multiplier, scales a "4 bit" `u8` to spread the value across
/// all 8 bits.
pub const U8_SCALE_4_TO_8: u8 = 0b_00010001;

#[test]
#[allow(clippy::erasing_op)]
fn test_bit_split() {
  let x: [u8; 8] = u8_bit_split_1x8(0b11001010);
  assert_eq!(x, [0, 1, 0, 1, 0, 0, 1, 1]);

  let x: [u8; 4] = u8_bit_split_2x4(0b11001010);
  assert_eq!(x, [0b10, 0b10, 0b00, 0b11]);

  let x: [u8; 2] = u8_bit_split_4x2(0b11001010);
  assert_eq!(x, [0b1010, 0b1100]);

  let x: [u8; 8] = u8_bit_split_1x8_rev(0b11001010);
  assert_eq!(x, [1, 1, 0, 0, 1, 0, 1, 0]);

  let x: [u8; 4] = u8_bit_split_2x4_rev(0b11001010);
  assert_eq!(x, [0b11, 0b00, 0b10, 0b10]);

  let x: [u8; 2] = u8_bit_split_4x2_rev(0b11001010);
  assert_eq!(x, [0b1100, 0b1010]);

  assert_eq!(U8_SCALE_1_TO_8 * 0, 0);
  assert_eq!(U8_SCALE_1_TO_8 * 1, u8::MAX);
  //
  assert_eq!(U8_SCALE_2_TO_8 * 0, 0);
  assert_eq!(U8_SCALE_2_TO_8 * 1, 0b01010101);
  assert_eq!(U8_SCALE_2_TO_8 * 2, 0b10101010);
  assert_eq!(U8_SCALE_2_TO_8 * 3, u8::MAX);
  //
  assert_eq!(U8_SCALE_4_TO_8 * 0x0, 0x00);
  assert_eq!(U8_SCALE_4_TO_8 * 0x1, 0x11);
  assert_eq!(U8_SCALE_4_TO_8 * 0x2, 0x22);
  assert_eq!(U8_SCALE_4_TO_8 * 0x3, 0x33);
  assert_eq!(U8_SCALE_4_TO_8 * 0x4, 0x44);
  assert_eq!(U8_SCALE_4_TO_8 * 0x5, 0x55);
  assert_eq!(U8_SCALE_4_TO_8 * 0x6, 0x66);
  assert_eq!(U8_SCALE_4_TO_8 * 0x7, 0x77);
  assert_eq!(U8_SCALE_4_TO_8 * 0x8, 0x88);
  assert_eq!(U8_SCALE_4_TO_8 * 0x9, 0x99);
  assert_eq!(U8_SCALE_4_TO_8 * 0xA, 0xAA);
  assert_eq!(U8_SCALE_4_TO_8 * 0xB, 0xBB);
  assert_eq!(U8_SCALE_4_TO_8 * 0xC, 0xCC);
  assert_eq!(U8_SCALE_4_TO_8 * 0xD, 0xDD);
  assert_eq!(U8_SCALE_4_TO_8 * 0xE, 0xEE);
  assert_eq!(U8_SCALE_4_TO_8 * 0xF, 0xFF);
}
