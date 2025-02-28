use std::{collections::HashMap, io};

pub struct HttpResponse<W> {
    status: u32,
    header: HashMap<String, String>,
    writer: W,
}

impl<W> HttpResponse<W> {
    pub fn new(w: W) -> Self {
        Self {
            status: 200,
            header: HashMap::new(),
            writer: w,
        }
    }
}
