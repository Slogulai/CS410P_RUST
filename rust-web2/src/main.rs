fn main() {
// https://hyper.rs/guides/1/server/hello-world/

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let (parts, _) = req.into_parts();
    if parts.method == Method::GET && parts.uri.path() == "/" {
        let body = Full::new(Bytes::from("<p><em>hello world</em>&#x1F980;</p>"));
        Ok(Response::new(body))
    } else {
        let body = Full::new(Bytes::from("<html><body>not found</body></html>"));
        let response = Response::builder().status(StatusCode::NOT_FOUND).body(body).unwrap();
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    eprintln!("webhello: serving {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}}
