use std::{
    collections::HashMap,
    io,
    sync::{Arc, Mutex},
};

use crate::Status;

#[allow(unused)]
pub struct HttpResponse<W> {
    status: Mutex<Status>,
    header: HashMap<String, String>,
    writer: Arc<Mutex<W>>,
}

impl<W> From<Arc<Mutex<W>>> for HttpResponse<W> {
    fn from(value: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::OK),
            header: HashMap::new(),
            writer: value,
        }
    }
}

impl<W> HttpResponse<W> {
    pub fn new(w: Arc<Mutex<W>>) -> Self {
        Self {
            status: Mutex::new(Status::OK),
            header: HashMap::new(),
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
}

impl<W: io::Write> HttpResponse<W> {}
