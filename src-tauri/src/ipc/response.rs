//! Here we are following a "JSON-RPC 2.0" styleesponse with error or result.
//!
//! Notes:
//!     - For now, we do not handle the "request.id" of "JSON-RPC 2.0", and request batching
//!       but this could be added later.
//!     - The benefit of following the "JSON-RPC 2.0" scheme is that the frontend could be adapted to talk to a
//!       web server with minimum effort, and the JSON-RPC data format for request/response is simple, clean, and well thought out.

use crate::Result;
use serde::Serialize;

#[derive(Serialize)]
struct IpcError {
	message: String,
}

#[derive(Serialize)]
pub struct IpcSimpleResult<D>
where
	D: Serialize,
{
	pub data: D,
}

#[derive(Serialize)]
pub struct IpcResponse<D>
where
	D: Serialize,
{
	error: Option<IpcError>,
	result: Option<IpcSimpleResult<D>>,
}

impl<D> From<Result<D>> for IpcResponse<D>
where
	D: Serialize,
{
	fn from(res: Result<D>) -> Self {
		match res {
			Ok(data) => IpcResponse {
				error: None,
				result: Some(IpcSimpleResult { data }),
			},
			Err(err) => IpcResponse {
				error: Some(IpcError {
					message: format!("{err}"),
				}),
				result: None,
			},
		}
	}
}
