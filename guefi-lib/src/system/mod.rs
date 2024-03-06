mod local;

use std::future::Future;

pub use local::LocalSystem;

pub trait System {
    fn get_boot_entries() -> impl Future<Output = Result<Vec<String>, String>> + 'static + Send;
}