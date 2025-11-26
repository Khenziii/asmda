use std::future::Future;
use std::pin::Pin;

pub type AsyncOutput = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type AsyncFnMut = Box<dyn FnMut() -> AsyncOutput + Send>;
pub type AsyncFn = Box<dyn Fn() -> AsyncOutput + Send>;
