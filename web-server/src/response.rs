use std::collections::HashMap;

#[allow(unused)]
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
