use mockall::automock;

#[automock]
#[allow(clippy::module_inception)]
pub mod local_file_driver {
    use dirs::home_dir;
    use std::fs;

    pub fn create_dir() -> anyhow::Result<(), domain::errors::FileError> {
        let dir_path = home_dir().unwrap().as_path().join(".cmm");
        match fs::create_dir_all(dir_path.clone()) {
            Ok(_) => {
                println!("Create data dir {}", dir_path.to_str().unwrap());
                Ok(())
            }
            Err(e) => {
                println!("Skip to create {}", dir_path.to_str().unwrap());
                Err(domain::errors::FileError::CreateFailedError(format!(
                    "Failed to create {:?}",
                    e.kind()
                )))
            }
        }
    }

    pub fn create_file(_file_name: String) -> anyhow::Result<(), domain::errors::FileError> {
        todo!()
    }
}
