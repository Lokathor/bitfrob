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

mod get_bit;
pub use get_bit::*;

mod with_bit;
pub use with_bit::*;

mod region_mask;
pub use region_mask::*;

mod get_region;
pub use get_region::*;

mod with_region;
pub use with_region::*;

mod get_value;
pub use get_value::*;

mod with_value;
pub use with_value::*;

// Note: module name must not match type name!
mod u8x2_;
pub use u8x2_::*;

mod replicate_bits;
pub use replicate_bits::*;

mod bit_iter_low;
pub use bit_iter_low::*;
