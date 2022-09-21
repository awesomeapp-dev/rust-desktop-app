pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// from: https://github.com/surrealdb/surrealdb.wasm/blob/main/src/mac/mod.rs
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }
pub(crate) use map; // export macro for crate

// Don't blame the messenger.
// It's my brain, it's not me.
macro_rules! s {
	() => {
		String::new()
	};
	($x:expr $(,)?) => {
		ToString::to_string(&$x)
	};
}
pub(crate) use s; // export macro for crate

// Again, not me.
pub use std::format as f;
