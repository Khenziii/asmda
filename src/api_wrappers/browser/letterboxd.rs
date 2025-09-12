use tokio::time::{sleep, Duration};
use fantoccini::{Client, ClientBuilder, Locator};
use crate::api_wrappers::browser::{APIWrapper, BrowserAPIWrapper};
use crate::environment::{environment};

pub struct LetterboxdBrowserAPIWrapper {
    client: Client,
}

impl APIWrapper for LetterboxdBrowserAPIWrapper {
    fn get_name(&self) -> &str {
        "letterboxd_browser"
    }
}

impl BrowserAPIWrapper for LetterboxdBrowserAPIWrapper {}

impl LetterboxdBrowserAPIWrapper {
    pub async fn new() -> Self {
        Self {
            client: ClientBuilder::native()
                .connect("http://localhost:4444")
                .await
                .expect("Failed to connect to WebDriver!"),
        }
    }

    async fn login(&self) {
        self.client
            .goto("https://letterboxd.com/sign-in")
            .await
            .expect("Navigation to `letterboxd.com/sign-in` failed!");

        let environment = environment();

        self.client
            .find(Locator::Id("field-username"))
            .await
            .expect("Failed to get `field-username` input!")
            .send_keys(&environment.letterboxd.username)
            .await
            .expect("Failed to insert keys into `field-username` input!");
        self.client
            .find(Locator::Id("field-password"))
            .await
            .expect("Failed to get `field-password` input!")
            .send_keys(&environment.letterboxd.password)
            .await
            .expect("Failed to insert keys into `field-password` input!");
        self.client
            .find(Locator::Css(".standalone-flow-button"))
            .await
            .expect("Failed to get the sign in button!")
            .click()
            .await
            .expect("Failed to click the sign in button!");

        sleep(Duration::from_secs(15)).await;
    }

    pub async fn export_data(&self) {
        self.client
            .goto("https://letterboxd.com/data/export")
            .await
            .expect("Navigation to letterboxd.com/data/export failed!");

        sleep(Duration::from_secs(10)).await;
    }

    pub async fn launch(&self) {
        self.init().await;
        self.login().await;
    }

    // TODO: something's wrong here...
    pub async fn close(self) {
        self.client.close().await.expect("Failed to close the browser!");
    }
}
