pub mod letterboxd;

use std::future::Future;

pub trait Archiver {
    fn get_name(&self) -> &str;
}

// For platforms that support instantly exporting data.
pub trait InstantArchiver: Archiver {
    fn get_data(&self) -> impl Future<Output = Vec<u8>>;
}

// For platforms that require some time to setup the package to archive, and then for example send
// it via mail.
pub trait RequestArchiver: Archiver {
    fn request_data(&self);
}
