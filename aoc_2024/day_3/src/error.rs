pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    FailReadFile(std::io::Error),
    FailedToParseRegex(regex::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::FailReadFile(e) => write!(f, "Failed to read file: {:?}", e),
            Error::FailedToParseRegex(e) => write!(f, "Failed to parse Regex{:?}", e),
        }
    }
}

impl std::error::Error for Error {}
