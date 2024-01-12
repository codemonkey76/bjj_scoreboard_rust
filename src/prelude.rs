pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

//Personal preferences.
pub use std::format as f;