//! Tauri IPCs for Task Model Controller (Frontend Controller to Backend Controller)
//!
//! TODO: Needs remove .unwrap() while still having control over the exception format.

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
	ModelMutateResultData, Task, TaskBmc, TaskFilter, TaskForCreate, TaskForUpdate,
};
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_task(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Task> {
	TaskBmc::get(Ctx::from_app(app).unwrap(), &params.id).await.into()
}

#[command]
pub async fn create_task(
	app: AppHandle<Wry>,
	params: CreateParams<TaskForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::create(Ctx::from_app(app).unwrap(), params.data).await.into()
}

#[command]
pub async fn update_task(
	app: AppHandle<Wry>,
	params: UpdateParams<TaskForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::update(Ctx::from_app(app).unwrap(), &params.id, params.data)
		.await
		.into()
}

#[command]
pub async fn delete_task(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::delete(Ctx::from_app(app).unwrap(), &params.id).await.into()
}

#[command]
pub async fn list_tasks(
	app: AppHandle<Wry>,
	params: ListParams<TaskFilter>,
) -> IpcResponse<Vec<Task>> {
	TaskBmc::list(Ctx::from_app(app).unwrap(), params.filter).await.into()
}
