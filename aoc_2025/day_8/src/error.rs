pub type Result<T> = core::result::Result<T, Error>;
use derive_more::From;
use ndarray::ShapeError;

#[derive(Debug, From)]
pub enum Error {
    FailReadFile(std::io::Error),
    Shape(ShapeError),
    KMeans(String),
    Linfa(linfa::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::FailReadFile(e) => write!(f, "{:?}", e),
            Error::Shape(e) => write!(f, "{:?}", e),
            Error::KMeans(e) => write!(f, "KMeans error: {}", e),
            Error::Linfa(e) => write!(f, "Linfa error: {:?}", e),
        }
    }
}

impl std::error::Error for Error {}
