use clap::Args;

use crate::commands::{CommandContext, CommandExecutor};

#[derive(Debug, Args)]
pub struct NextTopicCommand {}

impl CommandExecutor for NextTopicCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), anyhow::Error> {
        println!("Next topic");
        Ok(())
    }
}