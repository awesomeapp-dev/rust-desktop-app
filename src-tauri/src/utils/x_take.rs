//! XTake trait is about taking a value from an object for a given key.
//!
//! The trait to implement for a type is the `XTakeImpl` which has only one function.
//!
//! `x_take_impl(&mut self, k: &str) -> Result<Option<T>>`
//!
//! Then, XTake is a blanket implementation (Do not implement it) with
//! - `x_take` that returns a `Result<Option<T>>`
//! - `x_take_val` that returns `Result<T>` (i.e. fails if no value for key)
//!

use crate::{Error, Result};

/// Remove and return the Option<value> for a given type and key.
/// If no value for this key, return Result<None>.
/// If type missmatch, return a Error.
pub trait XTakeImpl<T> {
	fn x_take_impl(&mut self, k: &str) -> Result<Option<T>>;
}

/// For turbofish friendly version of XTakeInto with blanket implementation.
/// Note: Has a blanket implementation. Not to be implemented directly.
///       XTakeInto is the to be implemented trait
pub trait XTake {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		Self: XTakeImpl<T>;

	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		Self: XTakeImpl<T>;
}

/// Blanket implementation
impl<O> XTake for O {
	fn x_take<T>(&mut self, k: &str) -> Result<Option<T>>
	where
		Self: XTakeImpl<T>,
	{
		XTakeImpl::x_take_impl(self, k)
	}

	fn x_take_val<T>(&mut self, k: &str) -> Result<T>
	where
		Self: XTakeImpl<T>,
	{
		let val: Option<T> = XTakeImpl::x_take_impl(self, k)?;
		val.ok_or_else(|| Error::XPropertyNotFound(k.to_string()))
	}
}
