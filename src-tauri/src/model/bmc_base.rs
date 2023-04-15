//! Base and low level Backend Model Controller functions
//!

use super::store::{Creatable, Filterable, Patchable};
use super::{fire_model_event, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::{Error, Result};
use modql::ListOptions;
use std::sync::Arc;
use surrealdb::sql::Object;

pub(super) async fn bmc_get<E>(ctx: Arc<Ctx>, _entity: &'static str, id: &str) -> Result<E>
where
	E: TryFrom<Object, Error = Error>,
{
	ctx.get_model_manager()
		.store()
		.exec_get(id)
		.await?
		.try_into()
}

pub(super) async fn bmc_create<D>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	data: D,
) -> Result<ModelMutateResultData>
where
	D: Creatable,
{
	let id = ctx
		.get_model_manager()
		.store()
		.exec_create(entity, data)
		.await?;
	let result_data = ModelMutateResultData::from(id);

	fire_model_event(&ctx, entity, "create", result_data.clone());

	Ok(result_data)
}

pub(super) async fn bmc_update<D>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	id: &str,
	data: D,
) -> Result<ModelMutateResultData>
where
	D: Patchable,
{
	let id = ctx.get_model_manager().store().exec_merge(id, data).await?;

	let result_data = ModelMutateResultData::from(id);
	fire_model_event(&ctx, entity, "update", result_data.clone());

	Ok(result_data)
}

pub(super) async fn bmc_delete(
	ctx: Arc<Ctx>,
	entity: &'static str,
	id: &str,
) -> Result<ModelMutateResultData> {
	let id = ctx.get_model_manager().store().exec_delete(id).await?;
	let result_data = ModelMutateResultData::from(id);

	fire_model_event(&ctx, entity, "delete", result_data.clone());

	Ok(result_data)
}

pub(super) async fn bmc_list<E, F>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	filter: Option<F>,
	opts: ListOptions,
) -> Result<Vec<E>>
where
	E: TryFrom<Object, Error = Error>,
	F: Filterable + std::fmt::Debug,
{
	// query for the Surreal Objects
	let objects = ctx
		.get_model_manager()
		.store()
		.exec_select(entity, filter.map(|f| f.filter_nodes(None)), opts)
		.await?;

	// then get the entities
	objects
		.into_iter()
		.map(|o| o.try_into())
		.collect::<Result<_>>()
}
