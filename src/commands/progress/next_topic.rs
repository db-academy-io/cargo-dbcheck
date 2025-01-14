use clap::Args;
use log::info;

use crate::{
    commands::{CommandContext, CommandExecutor},
    error::DbCheckError,
};

#[derive(Debug, Args)]
pub struct NextTopicCommand {
    /// Mark current topic as completed
    #[arg(short, long, default_value_t = false)]
    pub mark_completed: bool,

    /// Skip completed topics
    #[arg(long, default_value_t = true)]
    pub skip_completed: bool,
}

impl CommandExecutor for NextTopicCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        info!("Next topic");
        Ok(())
    }
}
