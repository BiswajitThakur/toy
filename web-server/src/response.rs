use std::{
    collections::HashMap,
    fmt, fs,
    io::{self, BufReader},
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{MimeType, Status};

#[allow(unused)]
pub struct HttpResponse<W> {
    status: Mutex<Status>,
    content_type: Mutex<MimeType>,
    header: Mutex<HashMap<String, String>>,
    writer: Arc<Mutex<W>>,
}

impl<W> From<Arc<Mutex<W>>> for HttpResponse<W> {
    fn from(value: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::default()),
            content_type: Mutex::new(MimeType::default()),
            header: Mutex::new(HashMap::new()),
            writer: value,
        }
    }
}

impl<W: io::Write> HttpResponse<W> {
    pub fn new(w: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::default()),
            content_type: Mutex::new(MimeType::default()),
            header: Mutex::new(HashMap::new()),
            writer: w,
        }
    }
    #[allow(unused)]
    pub(crate) fn get_inner(self) -> Arc<Mutex<W>> {
        self.writer
    }
    pub fn status(&self, status: Status) -> &Self {
        *self.status.lock().unwrap() = status;
        self
    }
    pub fn content_type(&self, t: MimeType) -> &Self {
        *self.content_type.lock().unwrap() = t;
        self
    }
    pub fn insert_header(&self, key: String, value: String) -> &Self {
        self.header.lock().unwrap().insert(key, value);
        self
    }
    fn send_res_head(&self, len: usize) -> io::Result<()> {
        let http_version = "HTTP/1.1";
        let mut v = self.writer.lock().unwrap();
        // Ex: HTTP/1.1 200 OK
        writeln!(v, "{} {}\r", http_version, self.status.lock().unwrap())?;
        writeln!(v, "Content-Length: {}\r", len)?;
        writeln!(v, "Content-Type: {}\r", self.content_type.lock().unwrap())?;
        for (key, value) in self.header.lock().unwrap().iter() {
            writeln!(v, "{}: {}\r", key, value)?;
        }
        writeln!(v, "\r")?;
        Ok(())
    }
    pub fn send<T: fmt::Display>(self, value: T) -> io::Result<()> {
        let val = value.to_string();
        self.send_res_head(val.len())?;
        writeln!(self.writer.lock().unwrap(), "{}", val)?;
        Ok(())
    }
    pub fn send_file<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        let file = fs::File::open(path)?;
        let file_len = file.metadata()?.len();
        self.send_res_head(file_len as usize)?;
        let mut file_reader = BufReader::new(file);
        let mut v = self.writer.lock().unwrap();
        io::copy(&mut file_reader, &mut *v)?;
        Ok(())
    }
}

impl<W: io::Write> HttpResponse<W> {}
