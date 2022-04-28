use port::local_file_port::LocalFilePort;

pub struct LocalFileGateway;

impl LocalFilePort for LocalFileGateway {
    fn create_file(&self) -> anyhow::Result<()> {
        todo!()
    }
}
