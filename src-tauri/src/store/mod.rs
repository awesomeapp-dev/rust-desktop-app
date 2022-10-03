//! Small store layer to talk to the SurrealDB.
//! This module is to narrow and normalize the lower level API surface
//! to the rest of the application code (.e.g, Backend Model Controllers)

use crate::prelude::*;
use crate::utils::{map, XTakeVal};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};

mod try_froms;
mod x_takes;

// --- Marker traits for types that can be used for query.
pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Filterable: Into<Value> {}

// --- Store definition and implementation
//     Note: This is used to normalize the store access for what is
//           needed for this application.

/// Store struct normalizing CRUD SurrealDB application calls
pub struct Store {
	ds: Datastore,
	ses: Session,
}

impl Store {
	pub async fn new() -> Result<Self> {
		let ds = Datastore::new("memory").await?;
		let ses = Session::for_db("appns", "appdb");
		Ok(Store { ds, ses })
	}

	pub async fn exec_get(&self, tid: &str) -> Result<Object> {
		let sql = "SELECT * FROM $th";

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		W(first_res.result?.first()).try_into()
	}

	pub async fn exec_create<T: Creatable>(&self, tb: &str, data: T) -> Result<String> {
		let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

		let mut data: Object = W(data.into()).try_into()?;
		let now = Datetime::default().timestamp_nanos();
		data.insert("ctime".into(), now.into());

		let vars = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
		let first_val = ress.into_iter().next().map(|r| r.result).expect("id not returned")?;

		if let Value::Object(mut val) = first_val.first() {
			val.x_take_val::<String>("id")
				.map_err(|ex| Error::StoreFailToCreate(f!("exec_create {tb} {ex}")))
		} else {
			Err(Error::StoreFailToCreate(f!("exec_create {tb}, nothing returned.")))
		}
	}

	pub async fn exec_merge<T: Patchable>(&self, tid: &str, data: T) -> Result<String> {
		let sql = "UPDATE $th MERGE $data RETURN id";

		let vars = map![
			"th".into() => thing(tid)?.into(),
			"data".into() => data.into()];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;

		let first_res = ress.into_iter().next().expect("id not returned");

		let result = first_res.result?;

		if let Value::Object(mut val) = result.first() {
			val.x_take_val("id")
		} else {
			Err(Error::StoreFailToCreate(f!("exec_merge {tid}, nothing returned.")))
		}
	}

	pub async fn exec_delete(&self, tid: &str) -> Result<String> {
		let sql = "DELETE $th";

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		// Return the error if result failed
		first_res.result?;

		// return success
		Ok(tid.to_string())
	}

	pub async fn exec_select(&self, tb: &str, filter: Option<Value>) -> Result<Vec<Object>> {
		let mut sql = String::from("SELECT * FROM type::table($tb)");

		let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

		// --- Apply the filter
		if let Some(filter) = filter {
			let obj: Object = W(filter).try_into()?;
			sql.push_str(" WHERE");
			for (idx, (k, v)) in obj.into_iter().enumerate() {
				let var = f!("w{idx}");
				sql.push_str(&f!(" {k} = ${var}"));
				vars.insert(var, v);
			}
		}

		// --- Apply the orderby
		sql.push_str(" ORDER ctime DESC");

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		// Get the result value as value array (fail if it is not)
		let array: Array = W(first_res.result?).try_into()?;

		// build the list of objects
		array.into_iter().map(|value| W(value).try_into()).collect()

		// Note: Above equivalent to
		// let mut objs: Vec<Object> = Vec::new();
		// for item in array.into_iter() {
		// 	objs.push(W(item).try_into()?);
		// }
		// Ok(objs)
	}
}
