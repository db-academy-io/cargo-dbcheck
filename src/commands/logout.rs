use super::{CommandContext, CommandExecutor};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct LogoutCommand;

impl CommandExecutor for LogoutCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), anyhow::Error> {
        context.remove_token()?;
        println!("Successfully logged out");
        Ok(())
    }
}
