//! # Error module
//! Error module of project common utilities.
//!
//! For most cases, use the common type `VspResult` and `VspError` to avoid the actual error, and it
//! is of great convenience to pass the error as the return value among the crates.
//!
//! ```rust
//! use vsp_error::VspError;
//! use vsp_error::VspResult;
//!
//! pub fn foo() -> VspResult<()> {
//!   Ok(())
//! }
//! ```
//!
//! For now, the basic error implementation is a decoration wrapper of `anyhow::Error`, which is the
//! most simple implementation at this stage. It is planned to be replaced with new implementations
//! without `anyhow` in the future.
//! ```rust
//! use anyhow;
//!
//! pub struct Error(anyhow::Error);
//! ```
extern crate anyhow;

pub mod error;

/// Default result for the project.
///
/// Usually used as a default type for return values.
pub type VspResult<T, E = VspError> = Result<T, E>;

/// Default error type for the project.
pub type VspError = crate::error::Error;
