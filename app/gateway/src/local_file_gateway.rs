use domain::errors::FileError;
use driver::local_file_driver;
use port::local_file_port::LocalFilePort;

pub struct LocalFileGateway;

impl LocalFilePort for LocalFileGateway {
    fn create_storage(&self) -> anyhow::Result<(), FileError> {
        let result = local_file_driver::create_dir();

        if let Err(..) = result {
            return Err(result.unwrap_err());
        };
        // TODO: use filename domain for create_file args.
        match local_file_driver::create_file("cmm_data".into()) {
            Ok(()) => Ok(()),
            Err(e) => {
                println!("Failed to create file: ${:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::errors::FileError;
    use driver::local_file_driver::{create_dir, create_file, mock_create_dir, mock_create_file};

    #[test]
    #[mry::lock(create_dir, create_file)]
    fn create_file_test() {
        mock_create_dir().returns_with(|| Ok(()));
        mock_create_file("cmm_data".to_string()).returns_with(|_| Ok(()));

        let actual = LocalFileGateway.create_storage();
        assert!(actual.is_ok())
    }

    #[test]
    #[mry::lock(create_dir)]
    fn create_file_failed_test() {
        mock_create_dir()
            .returns_with(|| Err(FileError::CreateFailedError("Failed to create dir.".into())));

        let actual = LocalFileGateway.create_storage();
        assert!(actual.is_err())
    }
}
