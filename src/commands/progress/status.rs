use clap::Args;

use crate::{commands::{CommandContext, CommandExecutor}, error::DbCheckError};

#[derive(Debug, Args)]
pub struct StatusCommand {}

impl CommandExecutor for StatusCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Status");
        Ok(())
    }
}