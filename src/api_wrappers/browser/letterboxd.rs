use fantoccini::{ClientBuilder, Locator};
use crate::api_wrappers::browser::{APIWrapper, BrowserAPIWrapper};
use crate::environment::{environment};

pub struct LetterboxdBrowserAPIWrapper;

impl APIWrapper for LetterboxdBrowserAPIWrapper {
    fn get_name(&self) -> &str {
        "letterboxd_browser"
    }
}

impl BrowserAPIWrapper for LetterboxdBrowserAPIWrapper {}

impl LetterboxdBrowserAPIWrapper {
    async fn login(&self) {
        let client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .expect("Failed to connect to WebDriver!");
        client
            .goto("https://letterboxd.com/sign-in")
            .await
            .expect("Navigation to `letterboxd.com/sign-in` failed!");

        // TODO: check if we've been redirected to `letterboxd.com` and abort if so (this means
        // we're logged in).

        let environment = environment();

        client
            .find(Locator::Id("field-username"))
            .await
            .expect("Failed to get `field-username` input!")
            .send_keys(&environment.letterboxd.username)
            .await
            .expect("Failed to insert keys into `field-username` input!");
        client
            .find(Locator::Id("field-password"))
            .await
            .expect("Failed to get `field-password` input!")
            .send_keys(&environment.letterboxd.password)
            .await
            .expect("Failed to insert keys into `field-password` input!");
        client
            .find(Locator::Css(".standalone-flow-button"))
            .await
            .expect("Failed to get the sign in button!")
            .click()
            .await
            .expect("Failed to click the sign in button!");
    }

    pub async fn launch(&self) {
        self.init().await;
        self.login().await;
    }
}
