use std::{error::Error, fmt::Display};

/// Error type
#[derive(Debug)]
pub enum JfError {
    /// MediaType returned from jellyfin is of type None,
    /// this should be reported on github
    UnrecognizedMediaType,
    /// Content is in blacklist
    ContentBlacklist,
    MissingRequiredValues,
    NoImage,
    /// Network errors
    NetworkError(String),
    /// JSON parsing errors
    JsonParseError(String),
}

impl Error for JfError {}

impl Display for JfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JfError::MissingRequiredValues => write!(f, "missing required values to build client"),
            JfError::UnrecognizedMediaType => write!(f, "unrecognized media type"),
            JfError::ContentBlacklist => write!(f, "content is blacklisted"),
            JfError::NoImage => write!(f, "media does not have an image"),
            JfError::NetworkError(msg) => write!(f, "network error: {}", msg),
            JfError::JsonParseError(msg) => write!(f, "error parsing JSON: {}", msg),
        }
    }
}
