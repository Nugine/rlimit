use thiserror::Error;

#[derive(Error, Debug)]
#[error("rlimit error: {0}")]
pub struct RlimitsError(pub(crate) String);

impl From<&str> for RlimitsError {
    fn from(arg: &str) -> Self {
        Self(arg.to_string())
    }
}

impl From<String> for RlimitsError {
    fn from(arg: String) -> Self {
        Self(arg)
    }
}
