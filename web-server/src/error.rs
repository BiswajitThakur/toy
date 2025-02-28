use std::{fmt, io};

#[derive(Debug)]
pub enum HttpError {
    InvalidMethod,
    InvalidHttpVersion,
    BadRequest,
    StdError(io::Error),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMethod => write!(f, "Invalid Method"),
            Self::InvalidHttpVersion => write!(f, "Invalid Http Version"),
            Self::BadRequest => write!(f, "Bad Request"),
            Self::StdError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for HttpError {
    fn from(value: io::Error) -> Self {
        Self::StdError(value)
    }
}

impl std::error::Error for HttpError {}
