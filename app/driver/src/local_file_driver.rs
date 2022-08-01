use dirs::home_dir;
use std::fs::{self, OpenOptions};

#[mry::mry]
pub fn create_dir() -> anyhow::Result<(), domain::errors::FileError> {
    let dir_path = home_dir().unwrap().as_path().join(".cmm");
    if let Err(..) = fs::read_dir(dir_path.clone()) {
        match fs::create_dir_all(dir_path.clone()) {
            Ok(_) => {
                tracing::info!("Create data dir {}", dir_path.to_str().unwrap());
                Ok(())
            }
            Err(e) => Err(domain::errors::FileError::CreateFailedError(format!(
                "Failed to create {:?}",
                e.kind()
            ))),
        }
    } else {
        tracing::debug!("Skip to create {}", dir_path.to_str().unwrap());
        Ok(())
    }
}

#[mry::mry]
pub fn create_file(file_name: String) -> anyhow::Result<(), domain::errors::FileError> {
    let file_path = home_dir().unwrap().as_path().join(".cmm").join(file_name);
    if let Err(e) = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path.clone())
    {
        tracing::error!("Failed to create file: {}", file_path.to_str().unwrap());
        return Err(domain::errors::FileError::CreateFailedError(format!(
            "Failed to create {:?}",
            e.kind()
        )));
    }
    Ok(())
}
