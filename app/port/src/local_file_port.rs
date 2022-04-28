use domain::errors::FileError;
use mockall::automock;

#[automock]
pub trait LocalFilePort {
    fn create_file(&self) -> anyhow::Result<(), FileError>;
}
