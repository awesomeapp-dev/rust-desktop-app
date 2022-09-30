//! Params types used in the IPC methods.
//!
//! The current best practice is to follow a single argument type, called "params" for all method (JSON-RPC's style).
//!

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateParams<D>
where
	D: Serialize,
{
	pub(crate) data: D,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateParams<D>
where
	D: Serialize,
{
	pub(crate) id: String,
	pub(crate) data: D,
}

#[derive(Serialize, Deserialize)]
pub struct ListParams<F>
where
	F: Serialize,
{
	pub(crate) filter: Option<F>,
}

#[derive(Serialize, Deserialize)]
pub struct GetParams {
	pub(crate) id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteParams {
	pub(crate) id: String,
}
