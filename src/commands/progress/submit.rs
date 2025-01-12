use clap::Args;

use crate::{commands::{CommandContext, CommandExecutor}, error::DbCheckError};


#[derive(Debug, Args)]
pub struct SubmitCommand {}

impl CommandExecutor for SubmitCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Submitting");
        Ok(())
    }
}