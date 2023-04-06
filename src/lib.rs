#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::identity_op)]
#![cfg_attr(test, allow(bad_style))]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::must_use_candidate)]
#![warn(missing_copy_implementations)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_inline_in_public_items)]

//! A crate to help with bit manipulation of integers.
//!
//! ## Features
//! * `track_caller` adds the [`#[track_caller]`][ref-track_caller] attribute on
//!   all the functions that assert stuff.
//!
//! [ref-track_caller]:
//!     https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute

mod bit_iter_high;
mod bit_iter_low;
mod bit_split;
mod get_bit;
mod get_region;
mod get_value;
mod region_mask;
mod replicate_bits;
mod u8x2_;
mod with_bit;
mod with_region;
mod with_value;

pub use self::{
  bit_iter_high::*, bit_iter_low::*, bit_split::*, get_bit::*, get_region::*,
  get_value::*, region_mask::*, replicate_bits::*, u8x2_::*, with_bit::*,
  with_region::*, with_value::*,
};
