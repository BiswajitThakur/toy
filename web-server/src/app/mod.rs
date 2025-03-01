use std::{
    collections::HashMap,
    io::{self, BufReader, BufWriter},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Arc,
};

use crate::{request::HttpRequest, response::HttpResponse, server::HttpServer};

macro_rules! insert_handler {
    ($name:ident) => {
        #[inline]
        pub fn $name<F>(&mut self, path: &'static str, f: F)
        where
            F: Fn(
                    HttpRequest<BufReader<TcpStream>>,
                    HttpResponse<BufWriter<TcpStream>>,
                ) -> io::Result<()>
                + Send
                + Sync
                + 'static,
        {
            self.$name.insert(path, Box::new(f));
        }
    };
}

pub struct App<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub(crate) middleware: Vec<
        Box<
            dyn Fn(
                    HttpRequest<R>,
                    HttpResponse<W>,
                ) -> io::Result<Option<(HttpRequest<R>, HttpResponse<W>)>>
                + Send
                + Sync
                + 'static,
        >,
    >,
    pub(crate) connect: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) get: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) post: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) delete: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) head: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) put: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) patch: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) trace: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) options: HashMap<
        &'static str,
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
    pub(crate) unknown: Option<
        Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
    >,
}

impl App<BufReader<TcpStream>, BufWriter<TcpStream>> {
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
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
    pub fn use_middleware<F>(&mut self, f: F)
    where
        F: Fn(
                HttpRequest<BufReader<TcpStream>>,
                HttpResponse<BufWriter<TcpStream>>,
            ) -> io::Result<
                Option<(
                    HttpRequest<BufReader<TcpStream>>,
                    HttpResponse<BufWriter<TcpStream>>,
                )>,
            > + Send
            + Sync
            + 'static,
    {
        self.middleware.push(Box::new(f));
    }
    insert_handler!(connect);
    insert_handler!(get);
    insert_handler!(post);
    insert_handler!(delete);
    insert_handler!(head);
    insert_handler!(put);
    insert_handler!(patch);
    insert_handler!(trace);
    insert_handler!(options);
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
