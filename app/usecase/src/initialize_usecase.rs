use domain::errors::FileError;
use port::local_file_port::LocalFilePort;

pub fn run(local_file_port: impl LocalFilePort) -> anyhow::Result<(), FileError> {
    if let Err(e) = local_file_port.create_storage() {
        println!("Error!");
        return Err(e);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use port::local_file_port::MockLocalFilePort;

    #[test]
    fn run_test() {
        let mut local_file_port_mock = MockLocalFilePort::default();
        local_file_port_mock
            .mock_create_storage()
            .returns_with(|| Ok(()));

        let actual = run(local_file_port_mock);

        assert!(actual.is_ok());
    }
}
