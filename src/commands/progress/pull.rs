use clap::Args;
use log::info;

use crate::{commands::CommandExecutor, context::CommandContext, error::DbCheckError};

#[derive(Debug, Args)]
pub struct PullCommand {}

impl CommandExecutor for PullCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        info!("Updating");
        Ok(())
    }
}
