use clap::Args;

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct CloseCommand {}

impl CloseCommand {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
