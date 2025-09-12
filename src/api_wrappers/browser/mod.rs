use crate::api_wrappers::APIWrapper;

pub mod letterboxd;

pub trait BrowserAPIWrapper: APIWrapper {
    async fn init(&self) {
        // TODO: check if a WebDriver compatible process is running on port 4444.
        // If not this API should be unreachable.
    }
}
