use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum MyError {
    OutOfBounds,
    InvalidDirection,
    InvalidCell,
    NotABomb,
    InvalidArguments,
    IoError(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "Out of bounds"),
            Self::InvalidDirection => write!(f, "Invalid direction"),
            Self::InvalidCell => write!(f, "Invalid cell"),
            Self::NotABomb => write!(f, "Not a bomb"),
            Self::InvalidArguments => write!(f, "Invalid arguments"),
            Self::IoError(ref error) => write!(f, "IO error: {}", error),
        }
    }
}

pub type MyResult<T> = Result<T, MyError>;
