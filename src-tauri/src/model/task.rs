//! All model and controller for the Item type
//!

use super::bmc_base::{bmc_create, bmc_delete, bmc_get, bmc_list, bmc_update};
use super::store::{Creatable, Filterable, Patchable};
use super::ModelMutateResultData;
use crate::ctx::Ctx;
use crate::utils::{map, XTake};
use crate::{Error, Result};
use modql::filter::{FilterNodes, OpValsString};
use modql::ListOptions;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use ts_rs::TS;

// region:    --- Task

#[skip_serializing_none]
#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct Task {
	pub id: String,
	pub ctime: String,
	pub project_id: String,

	pub done: bool,
	pub title: String,
	pub desc: Option<String>,
}

impl TryFrom<Object> for Task {
	type Error = Error;
	fn try_from(mut val: Object) -> Result<Task> {
		let task = Task {
			id: val.x_take_val("id")?,
			ctime: val.x_take_val::<i64>("ctime")?.to_string(),
			project_id: val.x_take_val("project_id")?,
			done: val.x_take_val("done")?,
			title: val.x_take_val("title")?,
			desc: val.x_take("desc")?,
		};

		Ok(task)
	}
}

// endregion: --- Task

// region:    --- TaskForCreate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct TaskForCreate {
	pub project_id: String,
	pub title: String,
	pub done: Option<bool>,
	pub desc: Option<String>,
}

impl From<TaskForCreate> for Value {
	fn from(val: TaskForCreate) -> Self {
		let mut data = map![
			"project_id".into() => val.project_id.into(),
			"title".into() => val.title.into(),
		];

		// default for done is false
		data.insert("done".into(), val.done.unwrap_or(false).into());

		if let Some(desc) = val.desc {
			data.insert("desc".into(), desc.into());
		}
		Value::Object(data.into())
	}
}

impl Creatable for TaskForCreate {}

// endregion: --- TaskForCreate

// region:    --- TaskForUpdate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct TaskForUpdate {
	pub title: Option<String>,
	pub done: Option<bool>,
	pub desc: Option<String>,
}

impl From<TaskForUpdate> for Value {
	fn from(val: TaskForUpdate) -> Self {
		let mut data = BTreeMap::new();
		if let Some(title) = val.title {
			data.insert("title".into(), title.into());
		}
		if let Some(done) = val.done {
			data.insert("done".into(), done.into());
		}
		if let Some(desc) = val.desc {
			data.insert("desc".into(), desc.into());
		}
		Value::Object(data.into())
	}
}

impl Patchable for TaskForUpdate {}

// endregion: --- TaskForUpdate

// region:    --- TaskFilter

#[derive(FilterNodes, Deserialize, Debug)]
pub struct TaskFilter {
	pub project_id: Option<OpValsString>,
	pub title: Option<OpValsString>,
}

impl Filterable for TaskFilter {}

// endregion: --- TaskFilter

// region:    --- TaskBmc

pub struct TaskBmc;

impl TaskBmc {
	const ENTITY: &'static str = "task";

	pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Task> {
		bmc_get::<Task>(ctx, Self::ENTITY, id).await
	}

	pub async fn create(ctx: Arc<Ctx>, data: TaskForCreate) -> Result<ModelMutateResultData> {
		bmc_create(ctx, Self::ENTITY, data).await
	}

	pub async fn update(
		ctx: Arc<Ctx>,
		id: &str,
		data: TaskForUpdate,
	) -> Result<ModelMutateResultData> {
		bmc_update(ctx, Self::ENTITY, id, data).await
	}

	pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
		bmc_delete(ctx, Self::ENTITY, id).await
	}

	pub async fn list(ctx: Arc<Ctx>, filter: Option<TaskFilter>) -> Result<Vec<Task>> {
		let opts = ListOptions {
			limit: None,
			offset: None,
			order_bys: Some("!ctime".into()),
		};
		bmc_list(ctx, Self::ENTITY, filter, opts).await
	}
}

// endregion: --- TaskBmc
