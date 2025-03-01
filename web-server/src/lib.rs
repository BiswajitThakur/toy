mod error;
mod method;
mod request;
mod response;

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Read},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    str::FromStr,
    sync::Arc,
    thread,
};

use error::HttpError;
use method::Method;
use request::HttpRequest;
use response::HttpResponse;

pub struct App<R, W>
where
    R: io::Read,
    W: io::Write,
{
    /*
    middleware:
        Vec<Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>>,*/
    connect: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    get: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    post: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    delete: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    head: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    put: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    patch: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    trace: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    options: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    unknown: Option<
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
}

impl App<BufReader<TcpStream>, BufWriter<TcpStream>> {
    pub fn new() -> Self {
        Self {
            connect: HashMap::new(),
            get: HashMap::new(),
            post: HashMap::new(),
            delete: HashMap::new(),
            head: HashMap::new(),
            put: HashMap::new(),
            patch: HashMap::new(),
            trace: HashMap::new(),
            options: HashMap::new(),
            unknown: None,
        }
    }
    pub fn get<F>(&mut self, path: &'static str, f: F)
    where
        F: Fn(
                HttpRequest<BufReader<TcpStream>>,
                HttpResponse<BufWriter<TcpStream>>,
            ) -> io::Result<()>
            + Send
            + Sync
            + 'static,
    {
        self.get.insert(path, Box::new(f));
    }
    pub fn run<A: ToSocketAddrs>(self, addr: A) -> io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        let server = HttpServer {
            listener,
            handler: Arc::new(self),
        };
        server.run();
        Ok(())
    }
}

struct HttpServer<R, W>
where
    R: io::Read,
    W: io::Write,
{
    listener: TcpListener,
    handler: Arc<App<R, W>>,
}

impl HttpServer<BufReader<TcpStream>, BufWriter<TcpStream>> {
    fn run(self) {
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
    let writer = BufWriter::new(stream);
    let req = get_req(req_stream).unwrap();
    let res = HttpResponse::new(writer);
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
fn get_req<W: io::Read>(reader: W) -> Result<HttpRequest<BufReader<W>>, HttpError> {
    let mut reader = BufReader::new(reader);
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
    Ok(HttpRequest::new(method, path, version, header, reader))
}
