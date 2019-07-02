use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};

pub struct Config {
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
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

#[derive(Debug)]
pub struct BotBuilder {
    addr: SocketAddr,
}

impl BotBuilder {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self { addr: addr.into() }
    }

    pub fn build(self) -> Result<Bot, &'static str> {
        Ok(Bot {
            config: Config { addr: self.addr },
        })
    }
}
