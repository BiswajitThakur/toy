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
        (method, path, version): (Method, String, String),
        mut r: R,
    ) -> Result<Self, HttpError> {
        let mut reader = BufReader::new(&mut r);
        let mut header = HashMap::new();
        for line in reader.by_ref().lines() {
            println!("mmmmmmmmmmm");
            let line = line?;
            println!(">>> {}", &line);
            if line.trim().is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                header.insert(key.trim().to_owned(), value.trim().to_owned());
            }
        }
        Ok(Self {
            method,
            path,
            version,
            header,
            reader: r,
        })
    }
}
