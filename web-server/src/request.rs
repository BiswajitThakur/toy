use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Read},
    ops::Deref,
};

use crate::{error::HttpError, method::Method};

pub struct HttpRequest<R> {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) header: HashMap<String, String>,
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
    #[inline]
    pub fn insert(&mut self, key: String, value: String) {
        self.header.insert(key, value);
    }
    #[inline]
    pub fn get_header(&self) -> &HashMap<String, String> {
        &self.header
    }
}
