use clap::Args;

#[derive(Args, Clone)]
#[command(about = "TODO")]
pub(crate) struct BatchCommand {}

impl BatchCommand {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
