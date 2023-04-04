//! Ctx is the context object passed through any IPC calls.
//! It can be queried to get the necessary states/services to perform any steps of a request.
//!
//! Notes:
//!     - Simple implementation for now.
//!     - For cloud applications, this will be used for authorization.
//!     - Eventually, this will also be used for "full context" logging/tracing or even performance tracing.
//!     - For a single user, desktop application, this object is much simpler as authorization and logging requirements are much reduced.

use crate::event::HubEvent;
use crate::model::ModelStore;
use crate::Result;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

pub struct Ctx {
	model_manager: Arc<ModelStore>,
	app_handle: AppHandle<Wry>,
}

impl Ctx {
	pub fn from_app(app: AppHandle<Wry>) -> Result<Arc<Ctx>> {
		Ok(Arc::new(Ctx::new(app)))
	}
}

impl Ctx {
	pub fn new(app_handle: AppHandle<Wry>) -> Self {
		Ctx {
			model_manager: (*app_handle.state::<Arc<ModelStore>>()).clone(),
			app_handle,
		}
	}

	pub fn get_model_manager(&self) -> Arc<ModelStore> {
		self.model_manager.clone()
	}

	pub fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>) {
		let _ = self.app_handle.emit_all("HubEvent", hub_event);
	}
}
