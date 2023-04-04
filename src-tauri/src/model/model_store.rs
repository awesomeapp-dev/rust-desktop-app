//! ModelStore is just a Store wrapper so that Store does not have to be exposed outside the model module tree.
//!
//! This pattern allows to:
//!   1) Expose only the "new" to outside the model module tree.
//!   2) Access to the underlying store is allowed only for the model module tree.

use super::SurrealStore;
use crate::Result;

pub struct ModelStore(SurrealStore);

impl ModelStore {
	/// Create a new ModelStore instance and its corresponding SurrealStore
	pub async fn new() -> Result<Self> {
		Ok(ModelStore(SurrealStore::new().await?))
	}

	pub(in crate::model) fn store(&self) -> &SurrealStore {
		&self.0
	}
}
