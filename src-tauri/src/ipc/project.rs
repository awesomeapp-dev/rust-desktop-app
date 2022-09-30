//! Tauri IPCs for Project Model Controller (Frontend Controller to Backend Controller)
//!
//! TODO: Needs remove .unwrap() while still having control over the exception format.

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
	ModelMutateResultData, Project, ProjectBmc, ProjectFilter, ProjectForCreate,
	ProjectForUpdate,
};
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_project(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Project> {
	ProjectBmc::get(Ctx::from_app(app).unwrap(), &params.id).await.into()
}

#[command]
pub async fn create_project(
	app: AppHandle<Wry>,
	params: CreateParams<ProjectForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::create(Ctx::from_app(app).unwrap(), params.data).await.into()
}

#[command]
pub async fn update_project(
	app: AppHandle<Wry>,
	params: UpdateParams<ProjectForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::update(Ctx::from_app(app).unwrap(), &params.id, params.data)
		.await
		.into()
}

#[command]
pub async fn delete_project(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	ProjectBmc::delete(Ctx::from_app(app).unwrap(), &params.id).await.into()
}

#[command]
pub async fn list_projects(
	app: AppHandle<Wry>,
	params: ListParams<ProjectFilter>,
) -> IpcResponse<Vec<Project>> {
	ProjectBmc::list(Ctx::from_app(app).unwrap(), params.filter).await.into()
}
