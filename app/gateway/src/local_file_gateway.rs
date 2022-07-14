use domain::errors::FileError;
#[mockall_double::double]
use driver::local_file_driver::local_file_driver;
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
    use mockall::predicate::*;

    #[test]
    fn create_file_test() {
        let create_dir_ctx = local_file_driver::create_dir_context();
        create_dir_ctx
            .expect()
            .with()
            .times(1)
            .return_once(|| Ok(()));
        let create_file_ctx = local_file_driver::create_file_context();
        create_file_ctx
            .expect()
            .with(eq("cmm_data".to_string()))
            .times(1)
            .return_once(|_| Ok(()));

        let actual = LocalFileGateway.create_file();
        assert!(actual.is_ok())
    }

    #[test]
    fn create_file_failed_test() {
        let create_dir_ctx = local_file_driver::create_dir_context();
        create_dir_ctx
            .expect()
            .with()
            .times(1)
            .return_once(|| Err(FileError::CreateFailedError("Failed to create dir.".into())));

        let actual = LocalFileGateway.create_file();
        assert!(actual.is_err())
    }
}
