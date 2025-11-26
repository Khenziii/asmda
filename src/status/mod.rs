use crate::environment;
use crate::logger::logger;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use hyper_util::rt::{TokioIo, TokioTimer};
use std::net::SocketAddr;
use std::convert::Infallible;

async fn status_handler(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

pub struct StatusServer {
    address: SocketAddr,
}

impl StatusServer {
    pub fn new() -> Self {
        let config = environment::environment();
        let address: SocketAddr = ([127, 0, 0, 1], config.status_server.port as u16).into();

        StatusServer { address }
    }

    pub async fn start_blocking(&self) {
        let listener = TcpListener::bind(self.address).await.expect(&format!("Failed to start the StatusServer on address {}", self.address));

        loop {
            let (tcp, _) = listener.accept().await.expect("Failed to accept a TCP connection!");
            let io = TokioIo::new(tcp);

            tokio::task::spawn(async move {
                if let Err(error) = http1::Builder::new()
                    .timer(TokioTimer::new())
                    .serve_connection(io, service_fn(status_handler))
                    .await
                {
                    let error_message = format!("Error serving status check! Details: {}", error);
                    logger().error(&error_message);
                }
            });
        }
    }

    pub fn start_non_blocking(self) {
        tokio::spawn(async move { self.start_blocking().await });
    }
}
