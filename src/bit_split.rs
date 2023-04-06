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
