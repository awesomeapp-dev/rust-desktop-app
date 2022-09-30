//! Tauri IPC commands to bridge Task Frontend Model Controller to Backend Model Controller
//!

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
	ModelMutateResultData, Task, TaskBmc, TaskFilter, TaskForCreate, TaskForUpdate,
};
use crate::prelude::*;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_task(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Task> {
	match Ctx::from_app(app) {
		Ok(ctx) => TaskBmc::get(ctx, &params.id).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn create_task(
	app: AppHandle<Wry>,
	params: CreateParams<TaskForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => TaskBmc::create(ctx, params.data).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn update_task(
	app: AppHandle<Wry>,
	params: UpdateParams<TaskForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => TaskBmc::update(ctx, &params.id, params.data).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn delete_task(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => TaskBmc::delete(ctx, &params.id).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn list_tasks(
	app: AppHandle<Wry>,
	params: ListParams<TaskFilter>,
) -> IpcResponse<Vec<Task>> {
	match Ctx::from_app(app) {
		Ok(ctx) => TaskBmc::list(ctx, params.filter).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}
