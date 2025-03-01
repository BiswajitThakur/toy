use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read},
    ops::Deref,
};

use crate::{error::HttpError, method::Method};

pub struct HttpRequest<R> {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub header: HashMap<String, String>,
    reader: R,
}

// unsafe impl<R> std::marker::Send for HttpRequest<R> {}

impl<R> Deref for HttpRequest<R> {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl<R: io::Read> HttpRequest<R> {
    pub fn new(
        method: Method,
        path: String,
        version: String,
        header: HashMap<String, String>,
        r: R,
    ) -> Self {
        Self {
            method,
            path,
            version,
            header,
            reader: r,
        }
    }
}
