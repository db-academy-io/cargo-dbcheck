use clap::Args;
use log::info;

use crate::{
    commands::{CommandContext, CommandExecutor},
    error::DbCheckError,
};

#[derive(Debug, Args)]
pub struct SubmitCommand {}

impl CommandExecutor for SubmitCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        info!("Submitting");
        Ok(())
    }
}
