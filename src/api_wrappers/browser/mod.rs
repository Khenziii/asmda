use crate::api_wrappers::APIWrapper;
use async_trait::async_trait;
use fantoccini::Client;

pub mod letterboxd;

#[async_trait]
pub trait BrowserAPIWrapper: APIWrapper {
    fn from_client(client: Client) -> Self;
    async fn new() -> Self;
}

pub mod implementation_utils {
    use crate::api_wrappers::browser::BrowserAPIWrapper;
    use fantoccini::{Client, ClientBuilder};

    pub async fn default_constructor<T>(from_client: fn(Client) -> T) -> T
    where
        T: BrowserAPIWrapper,
    {
        let client = ClientBuilder::rustls()
            .expect("Failed to use rustls to build a browser client!")
            .connect("http://localhost:4444")
            .await
            .expect("Failed to connect to WebDriver on port 4444! Is it surely running?");

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
