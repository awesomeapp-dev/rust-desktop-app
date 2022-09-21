// surrealdb::Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Property {0} not found")]
	ExtractPropNotFound(String),

	#[error(transparent)]
	SurrealError(#[from] surrealdb::Error),

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}
