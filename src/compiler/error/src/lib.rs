pub mod error;

/// Default result for the project.
///
/// Usually used as a default type for return values.
pub type VspResult<T, E = VspError> = Result<T, E>;

/// Default error type for the project.
pub type VspError = crate::error::Error;
