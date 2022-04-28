use port::local_file_port::LocalFilePort;

pub fn create_storage(local_file_port: impl LocalFilePort) -> anyhow::Result<()> {
    local_file_port.create_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use port::local_file_port::MockLocalFilePort;

    #[test]
    fn create_storage_test() {
        let mut local_file_port_mock = MockLocalFilePort::new();
        local_file_port_mock
            .expect_create_file()
            .times(1)
            .returning(|| Ok(()));

        let actual = create_storage(local_file_port_mock);

        assert!(actual.is_ok());
    }
}