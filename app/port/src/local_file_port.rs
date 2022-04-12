use mockall::automock;

#[automock]
pub trait LocalFilePort {
    fn create_file() -> anyhow::Result<()>;
}