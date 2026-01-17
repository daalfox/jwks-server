use std::convert::Infallible;
use std::io::Read;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn handler(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut file = std::fs::File::open("/jwks/jwks.json").unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/.well-known/jwks.json") => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Full::new(content.into()))
            .unwrap()),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from_static(b"")))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();

    let svc = service_fn(handler);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
