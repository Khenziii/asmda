use crate::utils::constants::ArchiverIdentificator;
use crate::utils::types::AsyncFnMut;
use std::sync::Mutex;

pub struct TaskConfig {
    pub run_interval_seconds: u64,
    pub callback: AsyncFnMut,
    pub app_name: ArchiverIdentificator,
    pub is_enabled: bool,
}

pub type ThreadCallback = Mutex<AsyncFnMut>;
