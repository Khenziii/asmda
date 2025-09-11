use fantoccini::{ClientBuilder};
use crate::api_wrappers::{APIWrapper};

pub struct BrowserAPIWrapper;

impl APIWrapper for BrowserAPIWrapper {
    fn get_name(&self) -> &str {
        "browser"
    }
}

impl BrowserAPIWrapper {
    pub async fn launch(&self) {
        // TODO: check if a WebDriver compatible process is running on port 4444.
        // If not this API should be unreachable.

        let client = ClientBuilder::native().connect("http://localhost:4444").await.expect("Failed to connect to WebDriver!");
        client.goto("https://khenzii.dev").await.expect("Navigation to `khenzii.dev` failed!");

        let content = client.source().await.expect("Failed to get page's HTML content!");
        println!("{}", content);
    }
}
