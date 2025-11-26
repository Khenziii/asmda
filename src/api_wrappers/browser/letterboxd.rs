use crate::api_wrappers::browser::{APIWrapper, BrowserAPIWrapper, implementation_utils};
use crate::environment::environment;
use crate::utils::constants::APIWrapperIdentificator;
use crate::{impl_browser_api_wrapper, init_new_browser_api_wrapper};
use anyhow::{Context, Result};
use async_trait::async_trait;
use fantoccini::{Client, Locator};
use reqwest::header::{COOKIE, HeaderValue};

init_new_browser_api_wrapper!(LetterboxdBrowserAPIWrapper);

impl APIWrapper for LetterboxdBrowserAPIWrapper {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::Letterboxd
    }
}

impl_browser_api_wrapper!(LetterboxdBrowserAPIWrapper);

impl LetterboxdBrowserAPIWrapper {
    pub async fn export_data(&self) -> Result<Vec<u8>> {
        let auth_cookie = self.client
            .get_named_cookie("letterboxd.user.CURRENT")
            .await
            .context("Failed to get the `letterboxd.user.CURRENT` cookie! Make sure that the client is loggged in!")?;
        let formatted_auth_cookie = format!("letterboxd.user.CURRENT={};", auth_cookie.value());

        let http_client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&formatted_auth_cookie)
                .context("Failed to get a valid auth cookie!")?,
        );

        let response = http_client
            .get("https://letterboxd.com/data/export")
            .headers(headers)
            .send()
            .await
            .context("Failed to download letterboxd export data!")?;

        let bytes = response
            .bytes()
            .await
            .context("Failed to read raw file from downloaded Letterboxd backup package!")?;

        Ok(bytes.to_vec())
    }

    pub async fn launch(&self) -> Result<()> {
        self.login().await?;
        Ok(())
    }

    pub async fn close(self) {
        self.client
            .close()
            .await
            .expect("Failed to close the browser!");
    }

    async fn login(&self) -> Result<()> {
        self.client
            .goto("https://letterboxd.com/sign-in")
            .await
            .context("Navigation to `letterboxd.com/sign-in` failed!")?;

        let config = environment();

        self.client
            .find(Locator::Id("field-username"))
            .await
            .context("Failed to get `field-username` input!")?
            .send_keys(&config.letterboxd.username)
            .await
            .context("Failed to insert keys into `field-username` input!")?;
        self.client
            .find(Locator::Id("field-password"))
            .await
            .context("Failed to get `field-password` input!")?
            .send_keys(&config.letterboxd.password)
            .await
            .context("Failed to insert keys into `field-password` input!")?;
        self.client
            .find(Locator::Css(".standalone-flow-button"))
            .await
            .context("Failed to get the sign in button!")?
            .click()
            .await
            .context("Failed to click the sign in button!")?;

        self.client
            .wait()
            .for_element(Locator::Css(".site-logo"))
            .await
            .context("Failed to wait for the login to finish.")?;

        Ok(())
    }
}
