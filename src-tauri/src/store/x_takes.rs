//! Implementations of the crate::utils::x_utils traits for the store (i.e., surrealdb) value type.
use crate::prelude::*;
use crate::utils::XTakeInto;
use surrealdb::sql::Object;

// region:    --- XTake Object Implementations
impl XTakeInto<String> for Object {
	fn x_take_into(&mut self, k: &str) -> Result<Option<String>> {
		let v = self.remove(k).map(|v| W(v).try_into());
		match v {
			None => Ok(None),
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(ex)) => Err(ex),
		}
	}
}

impl XTakeInto<i64> for Object {
	fn x_take_into(&mut self, k: &str) -> Result<Option<i64>> {
		let v = self.remove(k).map(|v| W(v).try_into());
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
