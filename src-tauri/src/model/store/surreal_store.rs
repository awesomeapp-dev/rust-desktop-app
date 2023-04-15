//! Small store layer to talk to the SurrealDB.
//!
//! This module is to narrow and normalize the surrealdb API surface
//! to the rest of the application code (.e.g, Backend Model Controllers)

use crate::model::store::surreal_modql::build_select_query;
use crate::model::store::{Creatable, Patchable};
use crate::prelude::*;
use crate::utils::{map, XTake};
use crate::{Error, Result};
use modql::filter::FilterGroups;
use modql::ListOptions;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};

// --- Store definition and implementation
//     Note: This is used to normalize the store access for what is
//           needed for this application.

/// Store struct normalizing CRUD SurrealDB application calls
pub(in crate::model) struct SurrealStore {
	ds: Datastore,
	ses: Session,
}

impl SurrealStore {
	pub(in crate::model) async fn new() -> Result<Self> {
		let ds = Datastore::new("memory").await?;
		let ses = Session::for_db("appns", "appdb");
		Ok(SurrealStore { ds, ses })
	}

	pub(in crate::model) async fn exec_get(&self, tid: &str) -> Result<Object> {
		let sql = "SELECT * FROM $th";

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		W(first_res.result?.first()).try_into()
	}

	pub(in crate::model) async fn exec_create<T: Creatable>(
		&self,
		tb: &str,
		data: T,
	) -> Result<String> {
		let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

		let mut data: Object = W(data.into()).try_into()?;
		let now = Datetime::default().timestamp_nanos();
		data.insert("ctime".into(), now.into());

		let vars = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
		let first_val = ress
			.into_iter()
			.next()
			.map(|r| r.result)
			.expect("id not returned")?;

		if let Value::Object(mut val) = first_val.first() {
			val.x_take_val::<String>("id")
				.map_err(|ex| Error::StoreFailToCreate(f!("exec_create {tb} {ex}")))
		} else {
			Err(Error::StoreFailToCreate(f!(
				"exec_create {tb}, nothing returned."
			)))
		}
	}

	pub(in crate::model) async fn exec_merge<T: Patchable>(
		&self,
		tid: &str,
		data: T,
	) -> Result<String> {
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
			Err(Error::StoreFailToCreate(f!(
				"exec_merge {tid}, nothing returned."
			)))
		}
	}

	pub(in crate::model) async fn exec_delete(&self, tid: &str) -> Result<String> {
		let sql = "DELETE $th";

		let vars = map!["th".into() => thing(tid)?.into()];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		// Return the error if result failed
		first_res.result?;

		// return success
		Ok(tid.to_string())
	}

	pub(in crate::model) async fn exec_select<O: Into<FilterGroups>>(
		&self,
		tb: &str,
		filter_groups: Option<O>,
		list_options: ListOptions,
	) -> Result<Vec<Object>> {
		let filter_or_groups = filter_groups.map(|v| v.into());
		let (sql, vars) = build_select_query(tb, filter_or_groups, list_options)?;

		let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

		let first_res = ress.into_iter().next().expect("Did not get a response");

		// Get the result value as value array (fail if it is not)
		let array: Array = W(first_res.result?).try_into()?;

		// build the list of objects
		array.into_iter().map(|value| W(value).try_into()).collect()
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	use modql::filter::*;
	use std::sync::Arc;
	use tokio::sync::OnceCell;

	use crate::model::ModelStore;
	use crate::utils::XTake;
	use modql::ListOptions;

	static STORE_ONCE: OnceCell<Arc<ModelStore>> = OnceCell::const_new();

	/// Initialize store once for this unit test group.
	/// Will panic if can't create store.
	async fn get_shared_test_store() -> Arc<ModelStore> {
		STORE_ONCE
			.get_or_init(|| async {
				// create and seed the store
				let model_manager = ModelStore::new().await.unwrap();
				let model_manager = Arc::new(model_manager);

				crate::model::seed_store_for_dev(model_manager.clone())
					.await
					.unwrap();
				model_manager
			})
			.await
			.clone()
	}

	#[derive(Debug, FilterNodes)]
	struct ProjectFilter {
		pub id: Option<OpValsInt64>,
		pub name: Option<OpValsString>,
		pub some_other: Option<OpValsString>,
	}

	#[derive(Debug, FilterNodes)]
	struct TaskFilter {
		pub project_id: Option<OpValsString>,
		pub title: Option<OpValsString>,
		pub done: Option<OpValsBool>,
		pub desc: Option<OpValsString>,
	}

	#[test]
	fn test_surreal_build_select_query() -> anyhow::Result<()> {
		let filter = ProjectFilter {
			id: Some(OpValInt64::Lt(1).into()),
			name: Some(OpValString::Eq("Hello".to_string()).into()),
			some_other: None,
		};
		let filter_nodes: Vec<FilterNode> = filter.try_into()?;

		let (sql, vars) = super::build_select_query(
			"project",
			Some(filter_nodes.into()),
			ListOptions::default(),
		)?;

		assert!(sql.contains("id <"), "should contain id <");
		assert!(sql.contains("name ="), "should contain name =");
		assert!(sql.contains("$w1"), "should contain $w1");
		// should have 3 vars, one for the $tb, and one per var
		assert_eq!(vars.len(), 3, "should have e vars");

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_simple_project_select() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter = ProjectFilter {
			id: None,
			name: Some(OpValString::Eq("Project A".to_string()).into()),
			some_other: None,
		};

		// --- EXEC
		let mut rs = model_manager
			.store()
			.exec_select("project", Some(filter), ListOptions::default())
			.await?;

		// --- CHECKS
		assert_eq!(rs.len(), 1, "number of projects returned");
		let mut obj = rs.pop().unwrap();
		assert_eq!(obj.x_take::<String>("name")?.unwrap(), "Project A");

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_simple_task_select() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;

		// get the "Project A" project_id
		let project_filter_node = FilterNode::from(("name", "Project A"));
		let mut rs = model_manager
			.store()
			.exec_select("project", Some(project_filter_node), ListOptions::default())
			.await?;
		let project_id = rs.pop().unwrap().x_take_val::<String>("id")?;

		let filter = TaskFilter {
			project_id: Some(OpValString::from(project_id).into()),
			title: None,
			done: Some(OpValBool::Eq(true).into()),
			desc: None,
		};

		// --- EXEC
		let rs = model_manager
			.store()
			.exec_select("task", Some(filter), ListOptions::default())
			.await?;

		// --- CHECKS
		assert_eq!(
			rs.len(),
			100,
			"Result length (for Project A & done: true tasks"
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_contains() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_node = FilterNode::from(("title", OpValString::Contains("200".into())));

		// --- EXEC
		let mut rs = model_manager
			.store()
			.exec_select(
				"task",
				Some(filter_node),
				ListOptions {
					order_bys: Some("title".into()),
					..Default::default()
				},
			)
			.await?;

		// --- CHECK
		assert_eq!(
			"Task B.200",
			rs.pop().unwrap().x_take_val::<String>("title")?
		);
		assert_eq!(
			"Task A.200",
			rs.pop().unwrap().x_take_val::<String>("title")?
		);

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_starts_with() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_node = FilterNode::from(("title", OpValString::StartsWith("Task A.1".into())));

		// --- EXEC
		let rs = model_manager
			.store()
			.exec_select("task", Some(filter_node), ListOptions::default())
			.await?;

		// --- CHECK
		assert_eq!(rs.len(), 111, "Number of tasks starting with 'Task A.1'");

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_ends_with() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_node = FilterNode::from(("title", OpValString::EndsWith("11".into())));

		// --- EXEC
		let rs = model_manager
			.store()
			.exec_select("task", Some(filter_node), ListOptions::default())
			.await?;

		// --- CHECK
		assert_eq!(rs.len(), 4, "Number of tasks ending with '11'");

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_or() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_nodes_1: Vec<FilterNode> = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("11".into()),
		))];
		let filter_nodes_2: Vec<FilterNode> = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("22".into()),
		))];

		// --- EXEC
		let rs = model_manager
			.store()
			.exec_select(
				"task",
				Some(vec![filter_nodes_1, filter_nodes_2]),
				ListOptions::default(),
			)
			.await?;

		// --- CHECK
		assert_eq!(rs.len(), 8, "Number of tasks ending with '11' OR '22'");

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_order_bys() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_nodes_1 = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("11".into()),
		))];
		let filter_nodes_2 = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("22".into()),
		))];

		let list_options = ListOptions {
			order_bys: Some(vec!["done", "!title"].into()),
			..Default::default()
		};

		// --- EXEC
		let rs = model_manager
			.store()
			.exec_select(
				"task",
				Some(vec![filter_nodes_1, filter_nodes_2]),
				list_options,
			)
			.await?;

		// --- CHECK
		assert_eq!(rs.len(), 8, "Number of tasks ending with '11' OR '22'");
		// TODO: Need to check the order

		// for mut obj in rs.into_iter() {
		// 	println!(
		// 		"{:?} {:?}",
		// 		obj.x_take_val::<String>("title")?,
		// 		obj.x_take_val::<bool>("done")?
		// 	);
		// }

		Ok(())
	}

	#[tokio::test]
	async fn test_surreal_select_offset_limit() -> anyhow::Result<()> {
		// --- FIXTURE
		let model_manager = get_shared_test_store().await;
		let filter_nodes_1 = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("11".into()),
		))];
		let filter_nodes_2 = vec![FilterNode::from((
			"title",
			OpValString::EndsWith("22".into()),
		))];

		let list_options = ListOptions {
			order_bys: Some(vec!["done", "title"].into()),
			limit: Some(2),
			offset: Some(1),
		};

		// --- EXEC
		let mut rs = model_manager
			.store()
			.exec_select(
				"task",
				Some(vec![filter_nodes_1, filter_nodes_2]),
				list_options,
			)
			.await?;

		// --- CHECK
		assert_eq!(rs.len(), 2, "Number of tasks when Limit = 2");
		// Check tasks
		// Note: This will reverse order checked as we are usin pop.
		assert_eq!(
			"Task B.11",
			rs.pop().unwrap().x_take_val::<String>("title")?
		);
		assert_eq!(
			"Task A.111",
			rs.pop().unwrap().x_take_val::<String>("title")?
		);

		// --- Visualy check results
		// for mut obj in rs.into_iter() {
		// 	println!(
		// 		"{:?} {:?}",
		// 		obj.x_take_val::<String>("title")?,
		// 		obj.x_take_val::<bool>("done")?
		// 	);
		// }

		Ok(())
	}
}
// endregion: --- Tests
