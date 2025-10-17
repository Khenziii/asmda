use tokio::{task::block_in_place, runtime};

// Runs an async function in a synchronous manner by blocking the current thread.
// This should NOT be used on heavier tasks!
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    if let Ok(handle) = runtime::Handle::try_current() {
        block_in_place(|| handle.block_on(future))
    } else {
        let runtime = runtime::Runtime::new().unwrap();
        runtime.block_on(future)
    }
}
