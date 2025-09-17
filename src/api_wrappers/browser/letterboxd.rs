use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::json;
use reqwest::header::{HeaderValue, COOKIE};
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
    // TODO: move this to a generic BrowserAPIWrapper. This will stay the same for every Browser
    // wrapper.
    pub async fn new() -> Self {
        let capabilities =
            json!({
                "moz:firefoxOptions": {
                    "prefs": {
                        "browser.download.dir": "~/.cache/asmda",
                        "browser.helperApps.alwaysAsk.force": false,
                    }
                }
            })
            .as_object()
            .unwrap()
            .clone();

        Self {
            client: ClientBuilder::native()
                .capabilities(capabilities)
                .connect("http://localhost:4444")
                .await
                .expect("Failed to connect to FireFox WebDriver on port 4444! Is it surely running?"),
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

        self.client
            .wait()
            .for_element(Locator::Css(".site-logo"))
            .await
            .expect("Failed to wait for the login to finish.");
    }

    pub async fn export_data(&self) -> Vec<u8> {
        println!("exporting letterboxd data!");

        let auth_cookie = self.client
            .get_named_cookie("letterboxd.user.CURRENT")
            .await
            .expect("Failed to get the `letterboxd.user.CURRENT` cookie! Make sure that the client is loggged in!");
        let formatted_auth_cookie = format!("letterboxd.user.CURRENT={};", auth_cookie.value());

        println!("got the auth cookie!");

        let http_client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&formatted_auth_cookie)
                .expect("Failed to get a valid auth cookie!")
        );

        println!("headers defined!");

        let response = http_client
            .get("https://letterboxd.com/data/export")
            .headers(headers)
            .send()
            .await
            .expect("Failed to download letterboxd export data!");

        print!("send the request!");

        let bytes = response
            .bytes()
            .await
            .expect("Failed to read file raw from downloaded Letterboxd backup package!");

        println!("converted to bytes!");

        bytes.to_vec()
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
