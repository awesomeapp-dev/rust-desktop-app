//! Tauri IPC commands to bridge Project Frontend Model Controller to Backend Model Controller
//!

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
	ModelMutateResultData, Project, ProjectBmc, ProjectFilter, ProjectForCreate,
	ProjectForUpdate,
};
use crate::prelude::*;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_project(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Project> {
	match Ctx::from_app(app) {
		Ok(ctx) => ProjectBmc::get(ctx, &params.id).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn create_project(
	app: AppHandle<Wry>,
	params: CreateParams<ProjectForCreate>,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => ProjectBmc::create(ctx, params.data).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn update_project(
	app: AppHandle<Wry>,
	params: UpdateParams<ProjectForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => ProjectBmc::update(ctx, &params.id, params.data).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn delete_project(
	app: AppHandle<Wry>,
	params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
	match Ctx::from_app(app) {
		Ok(ctx) => ProjectBmc::delete(ctx, &params.id).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}

#[command]
pub async fn list_projects(
	app: AppHandle<Wry>,
	params: ListParams<ProjectFilter>,
) -> IpcResponse<Vec<Project>> {
	match Ctx::from_app(app) {
		Ok(ctx) => ProjectBmc::list(ctx, params.filter).await.into(),
		Err(_) => Err(Error::CtxFail).into(),
	}
}
