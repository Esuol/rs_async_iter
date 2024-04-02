#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs)]

mod from_iterator;
mod into_iterator;
mod iter;

pub use crate::iter::{Iterator, Map};
pub use from_iterator::FromIterator;
pub use into_iterator::IntoIterator;
