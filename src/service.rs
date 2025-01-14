use crate::message::PotatoMessage;

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};
use std::{fs::File, future::Future, io::Read, pin::Pin};

pub struct PotatoService {
    sender: std::sync::mpsc::Sender<PotatoMessage>,
}

impl PotatoService {
    pub fn with(sender: std::sync::mpsc::Sender<PotatoMessage>) -> Self {
        Self { sender }
    }
}

impl Service<Request<body::Incoming>> for PotatoService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder();

        let res = match *req.method() {
            Method::GET => match req.uri().path() {
                "/" => {
                    let mut buf = vec![];
                    let mut page = File::open("frontend/index.html").expect("Failed to find file");
                    page.read_to_end(&mut buf)
                        .expect("Failed to read to buffer");
                    response
                        .status(StatusCode::OK)
                        .body(Full::new(Bytes::copy_from_slice(&buf)))
                }

                "/boom" => {
                    self.sender
                        .send(PotatoMessage::Shoot)
                        .expect("Failed to send shoot command");
                    let mut buf = vec![];
                    let mut page = File::open("frontend/kaboom.html").expect("Failed to find file");
                    page.read_to_end(&mut buf)
                        .expect("Failed to read to buffer");
                    response
                        .status(StatusCode::OK)
                        .body(Full::new(Bytes::copy_from_slice(&buf)))
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };

        Box::pin(async { res })
    }
}
