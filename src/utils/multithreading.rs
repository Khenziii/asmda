use tokio::{runtime, task};

// Runs an async function in a synchronous manner by blocking the current thread.
// This should NOT be used on heavier tasks!
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    task::block_in_place(|| {
        runtime::Handle::current().block_on(future)
    })
}
