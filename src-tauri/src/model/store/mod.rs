use modql::IntoFilterNodes;
use surrealdb::sql::Value;

mod surreal_modql;
mod surreal_store;
mod try_froms;
mod x_takes;

// --- Re-export
pub(super) use surreal_store::SurrealStore;

// --- Marker traits for types that can be used for query.
pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Filterable: IntoFilterNodes {}
