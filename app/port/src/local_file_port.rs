use domain::errors::FileError;

#[mry::mry]
pub trait LocalFilePort {
    fn create_storage(&self) -> anyhow::Result<(), FileError>;
}
