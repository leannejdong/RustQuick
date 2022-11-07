//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrap typle struct for newtype pattern, mostly from external type to type From/Try/From concersions

pub struct W<T>(pub T);

pub use std::format as f;