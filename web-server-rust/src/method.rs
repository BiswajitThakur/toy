use std::str::FromStr;

use crate::error::HttpError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}

impl FromStr for Method {
    type Err = HttpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONNECT" => Ok(Self::Connect),
            "DELETE" => Ok(Self::Delete),
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "PATCH" => Ok(Self::Patch),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "TRACE" => Ok(Self::Trace),
            _ => Err(HttpError::InvalidMethod),
        }
    }
}
