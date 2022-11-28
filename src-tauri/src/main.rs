// #![allow(unused)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use crate::ipc::{
	create_project, create_task, delete_project, delete_task, get_project, get_task, list_projects,
	list_tasks, update_project, update_task,
};
use crate::prelude::*;
use model::{seed_store_for_dev, ModelStore};
use std::sync::Arc;
mod ctx;
mod error;
mod event;
mod ipc;
mod model;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	let model_manager = ModelStore::new().await?;
	let model_manager = Arc::new(model_manager);

	// for dev only
	seed_store_for_dev(model_manager.clone()).await?;

	tauri::Builder::default()
		.manage(model_manager)
		.invoke_handler(tauri::generate_handler![
			// Project
			get_project,
			create_project,
			update_project,
			delete_project,
			list_projects,
			// Task
			get_task,
			create_task,
			update_task,
			delete_task,
			list_tasks,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	Ok(())
}
