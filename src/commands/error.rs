use std::fmt;

#[derive(Debug)]
pub(crate) enum CliError {
    Config(String),
    Api(String),
    Io(String),
    Http(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Config(msg) => write!(f, "configuration error: {}", msg),
            CliError::Api(msg) => write!(f, "API error: {}", msg),
            CliError::Io(msg) => write!(f, "IO error: {}", msg),
            CliError::Http(msg) => write!(f, "HTTP error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err.to_string())
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        CliError::Http(err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for CliError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        CliError::Config(format!("invalid access token format: {}", err))
    }
}
