use std::{
    collections::HashMap,
    fmt, io,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{MimeType, Status};

#[allow(unused)]
pub struct HttpResponse<W> {
    status: Mutex<Status>,
    content_type: Mutex<MimeType>,
    header: HashMap<String, String>,
    writer: Arc<Mutex<W>>,
}

impl<W> From<Arc<Mutex<W>>> for HttpResponse<W> {
    fn from(value: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::default()),
            content_type: Mutex::new(MimeType::default()),
            header: HashMap::new(),
            writer: value,
        }
    }
}

impl<W> HttpResponse<W> {
    pub fn new(w: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::default()),
            content_type: Mutex::new(MimeType::default()),
            header: HashMap::new(),
            writer: w,
        }
    }
    #[allow(unused)]
    pub(crate) fn get_inner(self) -> Arc<Mutex<W>> {
        self.writer
    }
    pub fn status(self, status: Status) -> Self {
        *self.status.lock().unwrap() = status;
        self
    }
    pub fn content_type(&self, t: MimeType) {
        *self.content_type.lock().unwrap() = t;
    }
    //fn send_res_head()
    pub fn send<T: fmt::Display>(self, value: T) -> io::Result<()> {
        todo!()
    }
    pub fn send_file<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        todo!()
    }
}

impl<W: io::Write> HttpResponse<W> {}
