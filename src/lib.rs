#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::must_use_candidate)]

//! A crate to help with bit manipulation of integers.

mod const_region_mask;
pub use const_region_mask::*;

mod fns;
pub use fns::*;

// Note: module name must not match type name!
mod u8x2_;
pub use u8x2_::*;
