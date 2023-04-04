//! Tauri IPC commands to bridge Task Frontend Model Controller to Backend Model Controller
//!

use crate::ctx::Ctx;
use crate::ipc::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::model::{ModelMutateResultData, Task, TaskBmc, TaskForCreate, TaskForUpdate};
use crate::Error;
use serde_json::Value;
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
pub async fn list_tasks(app: AppHandle<Wry>, params: ListParams<Value>) -> IpcResponse<Vec<Task>> {
	// TODO: Needs to make error handling simpler (use ? rather than all into())
	match Ctx::from_app(app) {
		Ok(ctx) => match params.filter.map(serde_json::from_value).transpose() {
			Ok(filter) => TaskBmc::list(ctx, filter).await.into(),
			Err(err) => Err(Error::JsonSerde(err)).into(),
		},
		Err(_) => Err(Error::CtxFail).into(),
	}
}
