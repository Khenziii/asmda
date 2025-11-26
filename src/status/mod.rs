use crate::environment;
use crate::logger::logger;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use hyper_util::rt::{TokioIo, TokioTimer};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use std::convert::Infallible;

fn construct_response(status_code: StatusCode, text: String) -> Response<Full<Bytes>> {
    Response::builder().status(status_code).body(Full::new(Bytes::from(text))).unwrap()
}

async fn status_handler(server: Arc<StatusServer>, _: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let error_unlocked = server.error_message.lock().unwrap();
    let error = (*error_unlocked).clone();

    let response = if let Some(err) = error {
        construct_response(StatusCode::INTERNAL_SERVER_ERROR, err)
    } else {
        construct_response(StatusCode::OK, "Everything's fine!".to_string())
    };
    Ok(response)
}

pub struct StatusServer {
    address: SocketAddr,
    error_message: Mutex<Option<String>>,
}

impl Default for StatusServer {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusServer {
    pub fn new() -> Self {
        let config = environment::environment();
        let address: SocketAddr = ([127, 0, 0, 1], config.status_server.port as u16).into();
        let error_message = Mutex::new(None);

        StatusServer { address, error_message }
    }

    pub async fn start_blocking(self: Arc<Self>) {
        let listener = TcpListener::bind(self.address).await.unwrap_or_else(|_| panic!("Failed to start the StatusServer on address {}", self.address));

        loop {
            let (tcp, _) = listener.accept().await.expect("Failed to accept a TCP connection!");
            let io = TokioIo::new(tcp);

            let self_clone = Arc::clone(&self);
            tokio::task::spawn(async move {
                if let Err(error) = http1::Builder::new()
                    .timer(TokioTimer::new())
                    .serve_connection(io, service_fn(|req| status_handler(Arc::clone(&self_clone), req)))
                    .await
                {
                    let error_message = format!("Error serving status check! Details: {}", error);
                    logger().error(&error_message);
                }
            });
        }
    }

    pub fn start_non_blocking(self: &Arc<Self>) {
        let self_clone = Arc::clone(self);
        tokio::spawn(async move { self_clone.start_blocking().await });
    }

    pub fn set_error_message(&self, new_error_message: Option<String>) {
        let mut current_message = self.error_message.lock().unwrap();
        *current_message = new_error_message;
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error_message.lock().unwrap().clone()
    }
}

static STATUS_SERVER: OnceCell<Arc<StatusServer>> = OnceCell::new();

pub fn status_server() -> Arc<StatusServer> {
    STATUS_SERVER.get_or_init(|| Arc::new(StatusServer::new())).clone()
}
