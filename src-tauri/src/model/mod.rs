//! model module and sub-modules contain all of the model types and
//! backend model controllers for the application.
//!
//! The application code call the model controllers, and the
//! model controller calls the store and fire model events as appropriate.
//!

use crate::ctx::Ctx;
use crate::event::HubEvent;
use serde::Serialize;
use ts_rs::TS;

mod bmc_base;
mod project;
mod task;

// --- Re-exports
pub use project::*;
pub use task::*;

// region:    --- Model Event

fn fire_model_event<D>(ctx: &Ctx, entity: &str, action: &str, data: D)
where
	D: Serialize + Clone,
{
	ctx.emit_hub_event(HubEvent {
		hub: "Model".to_string(),
		topic: entity.to_string(),
		label: Some(action.to_string()),
		data: Some(data),
	});
}

// endregion: --- Model Event

// region:    --- Common Model Result Data

/// For now, all mutation queries will return an {id} struct.
/// Note: Keep it light, and client can do a get if needed.
#[derive(TS, Serialize, Clone)]
#[ts(export, export_to = "../src-ui/src/bindings/")]
pub struct ModelMutateResultData {
	pub id: String,
}

impl From<String> for ModelMutateResultData {
	fn from(id: String) -> Self {
		Self { id }
	}
}

// endregion: --- Common Model Result Data
