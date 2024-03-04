mod r#impl;

pub use r#impl::SystemImpl;

pub trait System {
    async fn get_boot_entries() -> Result<Vec<String>, String>;
}