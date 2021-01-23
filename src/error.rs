extern crate serde_json;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("JSON deserialization error")]
    Json(#[from] serde_json::Error),
    #[error("Set-env statement cannot have empty name or value")]
    InvalidSetEnv,
    #[error("Unknown error")]
    Unknown,
}
