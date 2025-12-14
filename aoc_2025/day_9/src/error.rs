pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    FailReadFile(std::io::Error),
    PlotterError(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::FailReadFile(e) => write!(f, "{:?}", e),
            Error::PlotterError(e) => write!(f, "Plotter error: {:?}", e),
        }
    }
}

impl std::error::Error for Error {}
