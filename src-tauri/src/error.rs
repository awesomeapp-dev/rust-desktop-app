//! This is the main (and only for now) application Error type.
//! It's using 'thiserror' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!     - By best practices, `anyhow` is not used in application code, but can be used in unit or integration test (will be in dev_dependencies when used)
//!

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	CtxFail,

	XValueNotOfType(&'static str),

	XPropertyNotFound(String),

	StoreFailToCreate(String),

	Modql(modql::Error),

	JsonSerde(serde_json::Error),

	ModqlOperatorNotSupported(String),

	Surreal(surrealdb::err::Error),

	IO(std::io::Error),
}

// region:    --- Froms
impl From<modql::Error> for Error {
	fn from(val: modql::Error) -> Self {
		Error::Modql(val)
	}
}
impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::JsonSerde(val)
	}
}
impl From<surrealdb::err::Error> for Error {
	fn from(val: surrealdb::err::Error) -> Self {
		Error::Surreal(val)
	}
}
impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Error::IO(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boiler
impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boiler
