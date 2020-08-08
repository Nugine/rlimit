//! Error handling.

use thiserror::Error;

/// Library errors
#[derive(Error, Debug)]
#[error("rlimts parse error: {0}")]
pub struct RlimitsResourceError(pub(crate) String);
