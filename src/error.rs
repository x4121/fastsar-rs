use std::error::Error as StdError;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    // TomlDe(toml::de::Error),
    // GroupNotFound(String),
    InvalidSetEnv,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {:?}", error),
            Self::Json(error) => write!(f, "JSON deserialization error: {:?}", error),
            // Self::TomlDe(error) => write!(f, "Serialization error: {:?}", error),
            // Self::GroupNotFound(error) => write!(f, "Group could not be found: {}", error),
            Self::InvalidSetEnv => write!(f, "Set-env statement cannot have empty name or value"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            // Self::TomlDe(error) => Some(error),
            // Self::GroupNotFound(_) => None,
            Self::InvalidSetEnv => None,
        }
    }
}
