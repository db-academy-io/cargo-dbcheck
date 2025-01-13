use clap::Args;
use log::info;

use crate::error::DbCheckError;

use super::{CommandContext, CommandExecutor};

#[derive(Debug, Args)]
pub struct CoursesCommand {
    /// Current course information
    #[arg(short, long, default_value = "true")]
    pub current: bool,

    /// List all courses from db-academy.io
    #[arg(short, long)]
    pub list: bool,
}

impl CommandExecutor for CoursesCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        info!("Listing courses");
        Ok(())
    }
}
