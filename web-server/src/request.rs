use std::{
    collections::HashMap,
    io,
    ops::Deref,
    sync::{Arc, Mutex},
};

use crate::method::Method;

#[allow(unused)]
pub struct HttpRequest<R> {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) header: HashMap<String, String>,
    reader: Arc<Mutex<R>>,
}

impl<R> Deref for HttpRequest<R> {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl<R> HttpRequest<R> {
    #[allow(unused)]
    pub(crate) fn get_inner(self) -> Arc<Mutex<R>> {
        self.reader
    }
}

impl<R: io::Read> HttpRequest<R> {
    pub fn new(
        method: Method,
        path: String,
        version: String,
        header: HashMap<String, String>,
        r: Arc<Mutex<R>>,
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
    pub fn insert_header(&mut self, key: String, value: String) {
        self.header.insert(key, value);
    }
    #[inline]
    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.header
    }
}
