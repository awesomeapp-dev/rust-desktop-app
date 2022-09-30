//! Key default crate types for this application.
//!
//! Notes:
//! 	- The best practice is to have a narrow crate prelude to normalize the key types throughout the application code.
//! 	- We keep this as small as possible, and try to limit generic name beside Result and Error (which is re-exported from this module)
//! 	- The `f!` macro alias of `format!`  and `s!` "to string" macro are just personal preferences, and relatively uncommon, remove as you see fit.
//!

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper class for newtype pattern, mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// from: https://github.com/surrealdb/surrealdb.wasm/blob/main/src/mac/mod.rs
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
pub(crate) use map; // export macro for crate

// Personal preference.
macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}
pub(crate) use s; // export macro for crate

// Performance preference.
pub use std::format as f;
