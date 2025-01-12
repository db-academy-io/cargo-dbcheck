use clap::Args;

use crate::{commands::{CommandContext, CommandExecutor}, error::DbCheckError};

#[derive(Debug, Args)]
pub struct NextTopicCommand {}

impl CommandExecutor for NextTopicCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Next topic");
        Ok(())
    }
}