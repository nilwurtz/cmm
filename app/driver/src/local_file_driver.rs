use mockall::automock;

#[automock]
#[allow(clippy::module_inception)]
pub mod local_file_driver {
    pub fn create_dir() -> anyhow::Result<(), domain::errors::FileError> {
        todo!()
    }
    pub fn create_file(_file_name: String) -> anyhow::Result<(), domain::errors::FileError> {
        todo!()
    }
}
