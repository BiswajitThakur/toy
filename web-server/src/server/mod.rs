use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Read},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use crate::{App, error::HttpError, method::Method, request::HttpRequest, response::HttpResponse};

pub(crate) struct HttpServer<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub(crate) listener: TcpListener,
    pub(crate) handler: Arc<App<R, W>>,
}

impl HttpServer<BufReader<TcpStream>, BufWriter<TcpStream>> {
    pub(crate) fn run(self) {
        let handler = self.handler;
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler = handler.clone();
                    thread::spawn(move || {
                        let handler = handler;
                        let _ = handle_stream(handler, stream);
                    });
                }
                Err(err) => eprintln!("{}\n", err),
            }
        }
    }
}

fn handle_stream(
    handler: Arc<App<BufReader<TcpStream>, BufWriter<TcpStream>>>,
    stream: TcpStream,
) -> io::Result<()> {
    let req_stream = stream.try_clone().unwrap();
    let res_strean = Arc::new(Mutex::new(BufWriter::new(stream)));
    let mut req = get_req(Arc::new(Mutex::new(BufReader::new(req_stream)))).unwrap();
    let mut res = HttpResponse::new(res_strean.clone());
    for f in handler.middleware.iter() {
        match f(req, res) {
            Ok(Some((r, s))) => {
                req = r;
                res = s;
            }
            Ok(None) => {
                return Ok(());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    match &req.method {
        Method::Connect => {
            if let Some(f) = handler.connect.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Get => {
            if let Some(f) = handler.get.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Post => {
            if let Some(f) = handler.post.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Delete => {
            if let Some(f) = handler.delete.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Head => {
            if let Some(f) = handler.head.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Put => {
            if let Some(f) = handler.put.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Patch => {
            if let Some(f) = handler.patch.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Trace => {
            if let Some(f) = handler.trace.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Options => {
            if let Some(f) = handler.options.get(req.path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
    }
    Ok(())
}
fn get_req<W: io::Read>(
    r: Arc<Mutex<BufReader<W>>>,
) -> Result<HttpRequest<BufReader<W>>, HttpError> {
    let cloned_r = r.clone();
    let mut reader = cloned_r.lock().unwrap();
    let first_line = match reader.by_ref().lines().next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => return Err(err.into()),
        None => return Err(HttpError::BadRequest),
    };
    let mut iter = first_line.split_whitespace();
    let method = iter.next().unwrap_or_default();
    let method = Method::from_str(method)?;
    let path = iter.next().unwrap_or_default().to_owned();
    let version = iter.next().unwrap_or_default().to_owned();
    let mut header = HashMap::new();
    for line in reader.by_ref().lines() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(":") {
            header.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
    Ok(HttpRequest::new(method, path, version, header, r))
}
