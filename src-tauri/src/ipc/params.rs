//! Params types used in the IPC methods.
//!
//! The current best practice is to follow a single argument type, called "params" for all method (JSON-RPC's style).
//!

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateParams<D> {
	pub data: D,
}

#[derive(Deserialize)]
pub struct UpdateParams<D> {
	pub id: String,
	pub data: D,
}

#[derive(Deserialize)]
pub struct ListParams<F> {
	pub filter: Option<F>,
}

#[derive(Deserialize)]
pub struct GetParams {
	pub id: String,
}

#[derive(Deserialize)]
pub struct DeleteParams {
	pub id: String,
}
