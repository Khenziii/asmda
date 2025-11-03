use tokio::runtime;

// Runs an async function in a synchronous manner by blocking the current thread.
// This should NOT be used on heavier tasks!
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    if let Ok(handle) = runtime::Handle::try_current() {
        std::thread::spawn(move || handle.block_on(future))
            .join()
            .expect("Thread has panicked!")
    } else {
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }
}
