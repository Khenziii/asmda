use crate::utils::constants::ArchiverIdentificator;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

pub struct TaskConfig {
    pub run_interval_seconds: u64,
    pub callback: Callback,
    pub app_name: ArchiverIdentificator,
}

pub type Callback = Box<dyn FnMut() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
pub type ThreadCallback = Mutex<Callback>;
