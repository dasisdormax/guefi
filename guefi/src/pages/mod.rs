pub mod boot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Boot,
    Security,
    Encryption
}