//! All model and controller for the Project type
//!
use super::bmc_base::{bmc_create, bmc_delete, bmc_get, bmc_list, bmc_update};
use super::store::{Creatable, Filterable, Patchable};
use super::ModelMutateResultData;
use crate::ctx::Ctx;
use crate::utils::XTake;
use crate::{Error, Result};
use modql::filter::{FilterNodes, OpValsString};
use modql::ListOptions;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use ts_rs::TS;

// region:    --- Project

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct Project {
	pub id: String,
	pub name: String,
	pub ctime: String,
}

impl TryFrom<Object> for Project {
	type Error = Error;
	fn try_from(mut val: Object) -> Result<Project> {
		let project = Project {
			id: val.x_take_val("id")?,
			name: val.x_take_val("name")?,
			ctime: val.x_take_val::<i64>("ctime")?.to_string(),
		};

		Ok(project)
	}
}

// endregion: --- Project

// region:    --- ProjectForCreate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ProjectForCreate {
	pub name: String,
}

impl From<ProjectForCreate> for Value {
	fn from(val: ProjectForCreate) -> Self {
		BTreeMap::from([
			// Note: could have used map![.. => ..] as well
			("name".into(), val.name.into()),
		])
		.into()
	}
}

impl Creatable for ProjectForCreate {}

// endregion: --- ProjectForCreate

// region:    --- ProjectForUpdate

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ProjectForUpdate {
	pub name: Option<String>,
}

impl From<ProjectForUpdate> for Value {
	fn from(val: ProjectForUpdate) -> Self {
		let mut data = BTreeMap::new();
		if let Some(name) = val.name {
			data.insert("name".into(), name.into());
		}
		data.into()
	}
}

impl Patchable for ProjectForUpdate {}

// endregion: --- ProjectForUpdate

// region:    --- ProjectFilter

#[derive(FilterNodes, Deserialize, Debug)]
pub struct ProjectFilter {
	pub id: Option<OpValsString>,
	pub name: Option<OpValsString>,
}

impl Filterable for ProjectFilter {}

// endregion: --- ProjectFilter

// region:    --- ProjectBmc

pub struct ProjectBmc;

impl ProjectBmc {
	const ENTITY: &'static str = "project";

	pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Project> {
		bmc_get(ctx, Self::ENTITY, id).await
	}

	pub async fn create(ctx: Arc<Ctx>, data: ProjectForCreate) -> Result<ModelMutateResultData> {
		bmc_create(ctx, Self::ENTITY, data).await
	}

	pub async fn update(
		ctx: Arc<Ctx>,
		id: &str,
		data: ProjectForUpdate,
	) -> Result<ModelMutateResultData> {
		bmc_update(ctx, Self::ENTITY, id, data).await
	}

	pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
		bmc_delete(ctx, Self::ENTITY, id).await
	}

	pub async fn list(ctx: Arc<Ctx>, filter: Option<ProjectFilter>) -> Result<Vec<Project>> {
		bmc_list(ctx, Self::ENTITY, filter, ListOptions::default()).await
	}
}

// endregion: --- ProjectBmc
