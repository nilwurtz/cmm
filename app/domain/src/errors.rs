#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum FileError {
    #[error("Cmm Error: {0}")]
    CreateFailedError(String),
}