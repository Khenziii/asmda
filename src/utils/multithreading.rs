use tokio::runtime;

// Runs an async function in a synchronous manner by blocking the current thread.
// This should NOT be used on heavier tasks!
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let runtime = runtime::Handle::current();
    runtime.block_on(future)
}
