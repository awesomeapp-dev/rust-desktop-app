//! Implementations of the crate::utils::x_utils traits for the store (i.e., surrealdb) value type.

use crate::prelude::*;
use crate::utils::XTakeInto;
use crate::utils::{XAs, XInto};
use surrealdb::sql::{Array, Object, Value};

// region:    --- XInto Value Implementations
impl XInto<Object> for Value {
	fn x_into(self) -> Result<Object> {
		match self {
			Value::Object(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Object")),
		}
	}
}

impl XInto<Array> for Value {
	fn x_into(self) -> Result<Array> {
		match self {
			Value::Array(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Array")),
		}
	}
}

impl XInto<i64> for Value {
	fn x_into(self) -> Result<i64> {
		match self {
			Value::Number(obj) => Ok(obj.as_int()),
			_ => Err(Error::XValueNotOfType("i64")),
		}
	}
}

impl XInto<bool> for Value {
	fn x_into(self) -> Result<bool> {
		match self {
			Value::False => Ok(false),
			Value::True => Ok(true),
			_ => Err(Error::XValueNotOfType("bool")),
		}
	}
}

impl XInto<String> for Value {
	fn x_into(self) -> Result<String> {
		match self {
			Value::Strand(strand) => Ok(strand.as_string()),
			Value::Thing(thing) => Ok(thing.to_string()),
			_ => Err(Error::XValueNotOfType("String")),
		}
	}
}
// endregion: --- XInto Value Implementations

// region:    --- XTake Object Implementations
impl XTakeInto<String> for Object {
	fn x_take_into(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.remove(k).map(|v| XAs::x_as::<String>(v));
		match v {
			None => Ok(None),
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(ex)) => Err(ex),
		}
	}
}

impl XTakeInto<i64> for Object {
	fn x_take_into(&mut self, k: &str) -> Result<Option<i64>> {
		let v = self.remove(k).map(|v| XAs::x_as::<i64>(v));
		match v {
			None => Ok(None),
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(ex)) => Err(ex),
		}
	}
}

impl XTakeInto<bool> for Object {
	fn x_take_into(&mut self, k: &str) -> Result<Option<bool>> {
		Ok(self.remove(k).map(|v| v.is_true()))
	}
}
// endregion: --- XTake Object Implementations
