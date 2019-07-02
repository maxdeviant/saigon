use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};

pub trait Plugin {
    fn version(&self) -> String;
}

#[derive(Debug)]
pub struct Command {
    pub value: String,
}

#[derive(Debug)]
pub struct Message {
    pub value: String,
}

pub trait Receive {
    fn receive(&mut self, command: Command);
}

pub struct Config {
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Bot {
    pub fn start(self) {
        let service = || {
            service_fn_ok(|req| match (req.method(), req.uri().path()) {
                (&Method::POST, "/") => {
                    let body = req.into_body();

                    println!("Body is {:?}", body);

                    Response::new(Body::from("Hello"))
                }
                _ => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap(),
            })
        };

        let server = Server::bind(&self.config.addr)
            .serve(service)
            .map_err(|err| println!("Error: {}", err));

        hyper::rt::run(server)
    }
}

pub struct BotBuilder {
    addr: SocketAddr,
    plugins: Vec<Box<dyn Plugin>>,
}

impl BotBuilder {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self {
            addr: addr.into(),
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn build(self) -> Result<Bot, &'static str> {
        Ok(Bot {
            config: Config { addr: self.addr },
            plugins: self.plugins,
        })
    }
}
