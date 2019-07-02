use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};

pub struct Bot {}

impl Bot {
    pub fn start(self) {
        let addr = ([127, 0, 0, 1], 3000).into();

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

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|err| println!("Error: {}", err));

        hyper::rt::run(server)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
