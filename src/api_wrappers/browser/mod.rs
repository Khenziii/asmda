use async_trait::async_trait;
use fantoccini::Client;
use crate::api_wrappers::APIWrapper;

pub mod letterboxd;

#[async_trait]
pub(super) trait BrowserAPIWrapper: APIWrapper {
    fn from_client(client: Client) -> Self;
    async fn new() -> Self;
}

mod implementation_utils {
    use fantoccini::{ClientBuilder, Client};
    use crate::api_wrappers::browser::BrowserAPIWrapper;

    pub async fn default_constructor<T>(from_client: fn(Client) -> T) -> T
    where
        T: BrowserAPIWrapper,
    {
        let client = ClientBuilder::native()
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
