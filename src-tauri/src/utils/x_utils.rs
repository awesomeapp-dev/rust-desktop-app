//! This is just some convenient traits for type conversion.
//!
//! Goals:
//!   - consistent methods to translate type to type (xinto base trait and xas blanket turbofishable blanket implementation)
//!   - consistent method to get property values from dictionary like structure (x_take_into, and x_take_as, and the x_take_val as strict version)
//!   
//! Notes:
//!   - While the standard `From` and `TryFrom` traits are used when possible, those traits are used when the from/to type are both externals.
//!   - The `x_` prefix, while not really elegant, is a pratical way to make sure we know those methods are "application utilities."
//!   - The pattern still needs some tuneup, but as of now, it is already practical and simplify downstream code.
//!   - One catch is that the blanket implementations seem
//!     to confuse rust-analyser as it suggest the methods even if the trait bounds are not met (the blanket turbofishable ones)

use crate::prelude::{Error, Result};

// region:    --- XInto/XAs

/// Application trait for type translation with error handling.
pub trait XInto<O> {
	fn x_into(self) -> Result<O>;
}

/// Turbofishable method
pub trait XAs {
	fn x_as<O>(self) -> Result<O>
	where
		Self: XInto<O>;
}

// Blanket implementation (allows turbofish)
impl<T> XAs for T {
	fn x_as<O>(self) -> Result<O>
	where
		T: XInto<O>,
	{
		XInto::x_into(self)
	}
}

// endregion: --- XInto/XAs

// region:    --- XTake

/// Remove and return the Option<value> for a given type and key.
/// If no value for this key, return Result<None>.
/// If type missmatch, return a Error.
pub trait XTakeInto<T> {
	fn x_take_into(&mut self, k: &str) -> Result<Option<T>>;
}

/// For turbofish friendly version of XTakeInto with blanket implementation.
/// Note: Has a blanket implementation. Not to be implemented directly.
///       XTakeInto is the to be implemented trait
pub trait XTake {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		Self: XTakeInto<T>;
}

/// Blanket implementation
impl<S> XTake for S {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		Self: XTakeInto<T>,
	{
		XTakeInto::x_take_into(self, k)
	}
}

/// Take the value and return Error if None.
/// Note: Has a blanket implementation. Not to be implemented directly.
///       XTakeInto is the to be implemented trait
pub trait XTakeVal {
	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		Self: XTakeInto<T>;
}

/// Blanket implementation
impl<S> XTakeVal for S {
	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		Self: XTakeInto<T>,
	{
		let val: Option<T> = XTakeInto::x_take_into(self, k)?;
		val.ok_or_else(|| Error::XPropertyNotFound(k.to_string()))
	}
}

// endregion: --- XTake
