pub mod error;

pub type VspResult<T, E = VspError> = Result<T, E>;

pub type VspError = crate::error::Error;
