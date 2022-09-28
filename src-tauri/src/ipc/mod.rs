//! `ipc` module and its sub-modules are all of the Rust constructs necessary for the WebView to Rust Tauri IPC calls.
//!
//! Notes:
//!   - This module re-exports the appropriate sub-module constructs are their sub-module structure
//!     does not add any value to be known or specified (for now).

mod params;
mod project;
mod response;
mod task;

// --- re-exports
pub use params::*;
pub use project::*;
pub use response::*;
pub use task::*;
