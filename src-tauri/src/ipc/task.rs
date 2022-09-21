use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::model::{
	ModelMutateResultData, Task, TaskBmc, TaskFilter, TaskForCreate, TaskForUpdate,
};
use crate::utils::XInto;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_task(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Task> {
	TaskBmc::get(app.x_into().unwrap(), &params.id).await.into()
}

#[command]
pub async fn create_task(
	app: AppHandle<Wry>,
	params: CreateParams<TaskForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::create(app.x_into().unwrap(), params.data).await.into()
}

#[command]
pub async fn update_task(
	app: AppHandle<Wry>,
	params: UpdateParams<TaskForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::update(app.x_into().unwrap(), &params.id, params.data).await.into()
}

#[command]
pub async fn delete_task(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	TaskBmc::delete(app.x_into().unwrap(), &params.id).await.into()
}

#[command]
pub async fn list_tasks(
	app: AppHandle<Wry>,
	params: ListParams<TaskFilter>,
) -> IpcResponse<Vec<Task>> {
	TaskBmc::list(app.x_into().unwrap(), params.filter).await.into()
}
