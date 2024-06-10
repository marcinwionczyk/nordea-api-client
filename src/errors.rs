use thiserror::Error;
#[derive(Error, Debug)]
#[error("{msg}")]
pub(crate) struct AutomatonError {
    source: Option<anyhow::Error>,
    msg: String
}