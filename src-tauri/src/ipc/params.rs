//! Params used in the IPC methods.
//!
//! The current best practice is to follow a single argument type, called "params" for all types.
//!
//! This approach has the following benefits:
//!
//! 	- Simplify the IPC method signature, as we just need to know method name and the "param" type.
//! 	- Inline with `JSON-RPC 2.0` spec which eventually could be implemented on a server with the same frontend.
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
