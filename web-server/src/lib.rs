mod error;
mod method;
mod request;
mod response;

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    str::FromStr,
    sync::Arc,
    thread,
};

use error::HttpError;
use method::Method;
use request::HttpRequest;
use response::HttpResponse;

pub struct HandlerFn<R, W>
where
    R: io::Read,
    W: io::Write,
{
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

impl HandlerFn<BufReader<TcpStream>, BufWriter<TcpStream>> {
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

pub struct HttpServer<R, W>
where
    R: io::Read,
    W: io::Write,
{
    listener: TcpListener,
    handler: Arc<HandlerFn<R, W>>,
}

impl HttpServer<BufReader<TcpStream>, BufWriter<TcpStream>> {
    pub fn run(self) {
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
    handler: Arc<HandlerFn<BufReader<TcpStream>, BufWriter<TcpStream>>>,
    stream: TcpStream,
) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let writer = BufWriter::new(stream);
    let (method, path, version) = get_first_line(reader.get_mut()).unwrap();
    let req = HttpRequest::new((method.clone(), path.clone(), version), reader).unwrap();
    println!("++++++++++");
    let res = HttpResponse::new(writer);
    match method {
        Method::Connect => {
            if let Some(f) = handler.connect.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Get => {
            if let Some(f) = handler.get.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Post => {
            if let Some(f) = handler.post.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Delete => {
            if let Some(f) = handler.delete.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Head => {
            if let Some(f) = handler.head.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Put => {
            if let Some(f) = handler.put.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Patch => {
            if let Some(f) = handler.patch.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Trace => {
            if let Some(f) = handler.trace.get(path.as_str()) {
                f(req, res)?;
            } else {
                if let Some(not_found) = &handler.unknown {
                    not_found(req, res)?;
                }
            }
        }
        Method::Options => {
            if let Some(f) = handler.options.get(path.as_str()) {
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
fn get_first_line<W: io::Read>(reader: W) -> Result<(Method, String, String), HttpError> {
    let mut lines = BufReader::new(reader).lines();
    let first_line = match lines.next() {
        Some(Ok(line)) => line,
        Some(Err(err)) => return Err(err.into()),
        None => return Err(HttpError::BadRequest),
    };
    println!(">> {}", &first_line);
    let mut iter = first_line.split_whitespace();
    let method = iter.next().unwrap_or_default();
    let method = Method::from_str(method)?;
    let path = iter.next().unwrap_or_default().to_owned();
    let version = iter.next().unwrap_or_default().to_owned();
    Ok((method, path, version))
}
