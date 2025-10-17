use crate::api_wrappers::APIWrapper;
use crate::logger::logger;
use async_trait::async_trait;
use fantoccini::Client;
use fantoccini::ClientBuilder;
use std::time::Duration;
use tokio::time::sleep;

pub mod letterboxd;

#[async_trait]
pub trait BrowserAPIWrapper: APIWrapper {
    fn from_client(client: Client) -> Self;
    async fn new() -> Self;
}

async fn get_client() -> Client {
    loop {
        let client = ClientBuilder::rustls()
            .expect("Failed to use rustls to build a browser client!")
            .connect("http://localhost:4444")
            .await;

        match client {
            Ok(c) => return c,
            Err(_) => {
                logger().error(
                    "Failed to establish a connection to WebDriver (:4444)! Make sure that it's running. If it already is, this error has probably appeared because another client is already connected.",
                );
                logger().error("Retrying in 5 seconds...");

                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

pub mod implementation_utils {
    use super::get_client;
    use crate::api_wrappers::browser::BrowserAPIWrapper;
    use fantoccini::Client;

    pub async fn default_constructor<T>(from_client: fn(Client) -> T) -> T
    where
        T: BrowserAPIWrapper,
    {
        let client = get_client().await;
        from_client(client)
    }
}

#[macro_export]
macro_rules! init_new_browser_api_wrapper {
    ($name:ident) => {
        pub struct $name {
            client: Client,
        }
    };
}

// A helpful macro adding some common boilerplate.
#[macro_export]
macro_rules! impl_browser_api_wrapper {
    ($type:ty) => {
        #[async_trait]
        impl BrowserAPIWrapper for $type {
            fn from_client(client: Client) -> Self {
                Self { client }
            }

            async fn new() -> Self {
                implementation_utils::default_constructor(Self::from_client).await
            }
        }

        impl $type {
            pub async fn new() -> Self {
                <$type as BrowserAPIWrapper>::new().await
            }
        }
    };
}
