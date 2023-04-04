//! TryFrom implementations for store related types

use crate::prelude::*;
use crate::{Error, Result};
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<W<Value>> for Object {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Object> {
		match val.0 {
			Value::Object(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Object")),
		}
	}
}

impl TryFrom<W<Value>> for Array {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<Array> {
		match val.0 {
			Value::Array(obj) => Ok(obj),
			_ => Err(Error::XValueNotOfType("Array")),
		}
	}
}

impl TryFrom<W<Value>> for i64 {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<i64> {
		match val.0 {
			Value::Number(obj) => Ok(obj.as_int()),
			_ => Err(Error::XValueNotOfType("i64")),
		}
	}
}

impl TryFrom<W<Value>> for bool {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<bool> {
		match val.0 {
			Value::False => Ok(false),
			Value::True => Ok(true),
			_ => Err(Error::XValueNotOfType("bool")),
		}
	}
}

impl TryFrom<W<Value>> for String {
	type Error = Error;
	fn try_from(val: W<Value>) -> Result<String> {
		match val.0 {
			Value::Strand(strand) => Ok(strand.as_string()),
			Value::Thing(thing) => Ok(thing.to_string()),
			_ => Err(Error::XValueNotOfType("String")),
		}
	}
}
