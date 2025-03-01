#![allow(clippy::type_complexity)]

use std::{
    collections::HashMap,
    io::{self, BufReader, BufWriter},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, RwLock},
};

use crate::{request::HttpRequest, response::HttpResponse, server::HttpServer};

macro_rules! insert_handler {
    ($name:ident) => {
        #[inline]
        pub fn $name<F>(&self, path: &'static str, f: F)
        where
            F: Fn(
                    HttpRequest<BufReader<TcpStream>>,
                    HttpResponse<BufWriter<TcpStream>>,
                ) -> io::Result<()>
                + Send
                + Sync
                + 'static,
        {
            self.$name.write().unwrap().insert(path, Box::new(f));
        }
    };
}

pub struct App<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub(crate) middleware: RwLock<
        Vec<
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
    >,
    pub(crate) connect: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) get: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) post: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) delete: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) head: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) put: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) patch: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) trace: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) options: RwLock<
        HashMap<
            &'static str,
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
    pub(crate) unknown: RwLock<
        Option<
            Box<dyn Fn(HttpRequest<R>, HttpResponse<W>) -> io::Result<()> + Send + Sync + 'static>,
        >,
    >,
}

impl Default for App<BufReader<TcpStream>, BufWriter<TcpStream>> {
    fn default() -> Self {
        Self::new()
    }
}

impl App<BufReader<TcpStream>, BufWriter<TcpStream>> {
    pub fn new() -> Self {
        Self {
            middleware: RwLock::new(Vec::new()),
            connect: RwLock::new(HashMap::new()),
            get: RwLock::new(HashMap::new()),
            post: RwLock::new(HashMap::new()),
            delete: RwLock::new(HashMap::new()),
            head: RwLock::new(HashMap::new()),
            put: RwLock::new(HashMap::new()),
            patch: RwLock::new(HashMap::new()),
            trace: RwLock::new(HashMap::new()),
            options: RwLock::new(HashMap::new()),
            unknown: RwLock::new(None),
        }
    }
    pub fn use_middleware<F>(&self, f: F)
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
        self.middleware.write().unwrap().push(Box::new(f));
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
    pub fn listen<A: ToSocketAddrs, F: Fn(SocketAddr)>(
        self,
        addr: A,
        callback: F,
    ) -> io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        callback(listener.local_addr().unwrap());
        let server = HttpServer {
            listener,
            handler: Arc::new(self),
        };
        server.run();
        Ok(())
    }
}
