#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(bad_style))]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::must_use_candidate)]

//! A crate to help with bit manipulation of integers.
//!
//! ## Naming Conventions
//!
//! * The `bit` function work on a single bit.
//! * The `region` functions work on multiple adjacent bits.
//! * The `value` functions are like a `region` but the value is automatically
//!   shifted down on read and up on write so that you can input/output normal
//!   values and the function will put it in place for you.
//! * `get` reads a value.
//! * `with` returns a new value.
//!
//! ## Features
//! * `track_caller` adds the [`#[track_caller]`][ref-track_caller] attribute on
//!   all the functions that assert stuff. Note that using the `track_caller`
//!   attribute can give LLVM trouble with inlining, so it's suggested that you
//!   only use this feature as necessary during debugging.
//!
//! [ref-track_caller]:
//!     https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute

mod bit_iter_high;
mod bit_iter_low;
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
  bit_iter_high::*, bit_iter_low::*, get_bit::*, get_region::*, get_value::*,
  region_mask::*, replicate_bits::*, u8x2_::*, with_bit::*, with_region::*,
  with_value::*,
};
