use clap::Args;

use crate::{commands::CommandExecutor, context::CommandContext, error::DbCheckError};

#[derive(Debug, Args)]
pub struct UpdateCommand {}

impl CommandExecutor for UpdateCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Updating");
        Ok(())
    }
}