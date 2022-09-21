use super::{fire_model_event, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::prelude::*;
use crate::store::{Creatable, Filterable, Patchable};
use crate::utils::{XAs, XInto};
use std::sync::Arc;
use surrealdb::sql::Object;

pub async fn bmc_get<E>(ctx: Arc<Ctx>, _entity: &'static str, id: &str) -> Result<E>
where
	Object: XInto<E>,
{
	// TODO: Need to check entity & id
	ctx.get_store().exec_get(&id).await?.x_as()
}

pub async fn bmc_create<D>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	data: D,
) -> Result<ModelMutateResultData>
where
	D: Creatable,
{
	let id = ctx.get_store().exec_create(entity, data).await?;
	let result_data = ModelMutateResultData::from(id);

	fire_model_event(&ctx, entity, "create", result_data.clone());

	Ok(result_data)
}

pub async fn bmc_update<D>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	id: &str,
	data: D,
) -> Result<ModelMutateResultData>
where
	D: Patchable,
{
	// TODO: Check that id has matching entity
	let id = ctx.get_store().exec_merge(&id, data).await?;

	let result_data = ModelMutateResultData::from(id);
	fire_model_event(&ctx, entity, "update", result_data.clone());

	Ok(result_data)
}

pub async fn bmc_delete(
	ctx: Arc<Ctx>,
	entity: &'static str,
	id: &str,
) -> Result<ModelMutateResultData> {
	let id = ctx.get_store().exec_delete(&id).await?;
	let result_data = ModelMutateResultData::from(id);

	fire_model_event(&ctx, entity, "delete", result_data.clone());

	Ok(result_data)
}

pub async fn bmc_list<E, F>(
	ctx: Arc<Ctx>,
	entity: &'static str,
	filter: Option<F>,
) -> Result<Vec<E>>
where
	Object: XInto<E>,
	F: Filterable + std::fmt::Debug,
{
	// FIXME: Needs to pass filter
	// query for the Surreal Objects
	let objects = ctx.get_store().exec_select(entity, filter.map(|f| f.into())).await?;

	// then get the entities
	objects.into_iter().map(|o| o.x_into()).collect::<Result<_>>()
}
