use async_trait::async_trait;
use fantoccini::{Client, Locator};
use reqwest::header::{HeaderValue, COOKIE};
use crate::api_wrappers::browser::{APIWrapper, BrowserAPIWrapper, implementation_utils};
use crate::environment::{environment};
use crate::{impl_browser_api_wrapper, init_new_browser_api_wrapper};
use crate::utils::constants::APIWrapperIdentificator;

init_new_browser_api_wrapper!(LetterboxdBrowserAPIWrapper);

impl APIWrapper for LetterboxdBrowserAPIWrapper {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::Letterboxd
    }
}

impl_browser_api_wrapper!(LetterboxdBrowserAPIWrapper);

impl LetterboxdBrowserAPIWrapper {
    pub async fn export_data(&self) -> Vec<u8> {
        let auth_cookie = self.client
            .get_named_cookie("letterboxd.user.CURRENT")
            .await
            .expect("Failed to get the `letterboxd.user.CURRENT` cookie! Make sure that the client is loggged in!");
        let formatted_auth_cookie = format!("letterboxd.user.CURRENT={};", auth_cookie.value());

        let http_client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&formatted_auth_cookie)
                .expect("Failed to get a valid auth cookie!")
        );

        let response = http_client
            .get("https://letterboxd.com/data/export")
            .headers(headers)
            .send()
            .await
            .expect("Failed to download letterboxd export data!");

        let bytes = response
            .bytes()
            .await
            .expect("Failed to read raw file from downloaded Letterboxd backup package!");

        bytes.to_vec()
    }

    pub async fn launch(&self) {
        self.login().await;
    }

    pub async fn close(self) {
        self.client.close().await.expect("Failed to close the browser!");
    }

    async fn login(&self) {
        self.client
            .goto("https://letterboxd.com/sign-in")
            .await
            .expect("Navigation to `letterboxd.com/sign-in` failed!");

        let config = environment();

        self.client
            .find(Locator::Id("field-username"))
            .await
            .expect("Failed to get `field-username` input!")
            .send_keys(&config.letterboxd.username)
            .await
            .expect("Failed to insert keys into `field-username` input!");
        self.client
            .find(Locator::Id("field-password"))
            .await
            .expect("Failed to get `field-password` input!")
            .send_keys(&config.letterboxd.password)
            .await
            .expect("Failed to insert keys into `field-password` input!");
        self.client
            .find(Locator::Css(".standalone-flow-button"))
            .await
            .expect("Failed to get the sign in button!")
            .click()
            .await
            .expect("Failed to click the sign in button!");

        self.client
            .wait()
            .for_element(Locator::Css(".site-logo"))
            .await
            .expect("Failed to wait for the login to finish.");
    }
}
