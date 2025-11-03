use asmda::api_wrappers::APIWrapper;
use asmda::api_wrappers::browser::{BrowserAPIWrapper, implementation_utils};
use asmda::utils::constants::APIWrapperIdentificator;
use asmda::{impl_browser_api_wrapper, init_new_browser_api_wrapper};
use async_trait::async_trait;
use fantoccini::Client;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

static MULTITHREADED_BUFFER: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(String::from(""))));

init_new_browser_api_wrapper!(TestBrowserAPIWrapper);

impl APIWrapper for TestBrowserAPIWrapper {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::Tests
    }
}

impl_browser_api_wrapper!(TestBrowserAPIWrapper);

impl TestBrowserAPIWrapper {
    pub async fn visit_website(&self, value_to_return: String) -> String {
        self.client.goto("https://khenzii.dev").await.unwrap();
        value_to_return
    }
}

#[cfg(test)]
mod tests {
    mod browser {
        use super::super::*;
        use asmda::logger::logger;
        use asmda::utils::startup::install_crypto_ring_default_provider;
        use std::time::Duration;
        use tokio::time::sleep;

        // Makes sure that the connections can run either in parallel or in a queue.
        #[tokio::test]
        async fn webdriver_connections_queue() {
            install_crypto_ring_default_provider();

            // We connect the first client to WebDriver and wait for a while.
            let first_webdriver_connection_handle = tokio::spawn(async move {
                logger().debug("first webdriver connected!");

                let first_test_browser = TestBrowserAPIWrapper::new().await;
                let returned_value = first_test_browser
                    .visit_website(String::from("stale connected!"))
                    .await;
                let mut lock = MULTITHREADED_BUFFER.lock().unwrap();
                *lock = returned_value;
            });
            sleep(Duration::from_secs(5)).await;

            // Now this connection could fail, as one client is already connected.
            let _second_webdriver_connection_handle = tokio::spawn(async move {
                logger().debug("second webdriver connected!");

                let second_test_browser = TestBrowserAPIWrapper::new().await;
                let returned_value = second_test_browser
                    .visit_website(String::from("Connected!"))
                    .await;
                let mut lock = MULTITHREADED_BUFFER.lock().unwrap();
                *lock = returned_value;
            });
            sleep(Duration::from_secs(5)).await;

            // We kill the first connection.
            first_webdriver_connection_handle.abort();
            logger().debug("first webdriver disconnected!");

            // And now, after a while the second client should be able to connect and (if it didn't
            // do it before) extract our wanted data.
            sleep(Duration::from_secs(10)).await;
            let value = MULTITHREADED_BUFFER.lock().unwrap().clone();
            assert_eq!(value, String::from("Connected!"));
        }
    }
}
