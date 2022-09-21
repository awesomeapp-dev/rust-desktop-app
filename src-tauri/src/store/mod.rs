use crate::prelude::*;
use crate::utils::{XAs, XInto, XTakeVal};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};

mod utils;

// --- Marker traits for types that can be used for query.
pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Filterable: Into<Value> {}

// --- Store definition and implementation
//     Note: This is used to normalize the store access for what is
//           needed for this application.

/// Store struct normalizing application CRUD apis
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
		let sql = format!("SELECT * FROM $th");

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let res = ress.into_iter().next().expect("Did not get a response");
		let val = res.result?.first();
		match val {
			Value::Object(val) => Ok(val),
			_ => panic!("Error not object found for tid {tid}"),
		}
	}

	pub async fn exec_create<T: Creatable>(&self, tb: &str, data: T) -> Result<String> {
		let sql = s!("CREATE type::table($tb) CONTENT $data RETURN id");

		let mut data: Object = data.into().x_as()?;
		let now = Datetime::default().timestamp_nanos();
		data.insert("ctime".into(), now.into());

		let vars = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
		let val = ress.into_iter().next().map(|r| r.result).expect("id not returned")?;

		if let Value::Object(mut val) = val.first() {
			let val = val.remove("id").expect("id not found").as_string();
			Ok(format!("{val}"))
		} else {
			panic!("Object not created for {tb}")
		}
	}

	pub async fn exec_merge<T: Patchable>(&self, tid: &str, data: T) -> Result<String> {
		let sql = s!("UPDATE $th MERGE $data RETURN id");

		let vars = map![
			"th".into() => thing(tid)?.into(),
			"data".into() => data.into()];

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let res = ress.into_iter().next().expect("id not returned");

		let result = res.result?;

		if let Value::Object(mut val) = result.first() {
			Ok(val.x_take_val("id")?)
		} else {
			panic!("exec_merge panic - no value returned??? for tid: {tid}")
		}
	}

	pub async fn exec_delete(&self, tid: &str) -> Result<String> {
		let sql = format!("DELETE $th");

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let res = ress.into_iter().next().expect("Did not get a response");

		// Return the error if result failed
		res.result?;

		// return success
		Ok(tid.to_string())
	}

	pub async fn exec_select(&self, tb: &str, filter: Option<Value>) -> Result<Vec<Object>> {
		let mut sql = format!("SELECT * FROM type::table($tb)");

		let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

		// --- Apply the filter
		if let Some(filter) = filter {
			let obj: Object = filter.x_as()?;
			sql.push_str(" WHERE");
			for (idx, (k, v)) in obj.into_iter().enumerate() {
				let var = f!("w{idx}");
				sql.push_str(&f!(" {k} = ${var}"));
				vars.insert(var, v);
			}
		}

		// --- Apply the orderby
		sql.push_str(&f!(" ORDER ctime DESC"));

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let res = ress.into_iter().next().expect("Did not get a response");

		// Get the result value as value array (fail if it is not)
		let array: Array = res.result?.x_as()?;

		// build the list of object
		let mut objs: Vec<Object> = Vec::new();
		for item in array.into_iter() {
			objs.push(item.x_into()?);
		}

		Ok(objs)
	}
}
