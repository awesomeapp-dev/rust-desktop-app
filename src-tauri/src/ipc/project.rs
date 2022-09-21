use crate::model::{
	ModelMutateResultData, Project, ProjectBmc, ProjectFilter, ProjectForCreate,
	ProjectForUpdate,
};
use crate::utils::XInto;
use tauri::{command, AppHandle, Wry};

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};

#[command]
pub async fn get_project(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Project> {
	ProjectBmc::get(app.x_into().unwrap(), &params.id).await.into()
}

#[command]
pub async fn create_project(
	app: AppHandle<Wry>,
	params: CreateParams<ProjectForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::create(app.x_into().unwrap(), params.data).await.into()
}

#[command]
pub async fn update_project(
	app: AppHandle<Wry>,
	params: UpdateParams<ProjectForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::update(app.x_into().unwrap(), &params.id, params.data).await.into()
}

#[command]
pub async fn delete_project(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::delete(app.x_into().unwrap(), &params.id).await.into()
}

#[command]
pub async fn list_projects(
	app: AppHandle<Wry>,
	params: ListParams<ProjectFilter>,
) -> IpcResponse<Vec<Project>> {
	ProjectBmc::list(app.x_into().unwrap(), params.filter).await.into()
}
