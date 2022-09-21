use crate::ctx::Ctx;
use crate::event::HubEvent;
use serde::Serialize;
use ts_rs::TS;

mod bmc;
mod project;
mod task;

// --- Re-exports
pub use project::*;
pub use task::*;

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
