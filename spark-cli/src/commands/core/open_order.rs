use clap::Args;

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct OpenCommand {}

impl OpenCommand {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
