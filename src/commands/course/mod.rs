use clap::Args;

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
    fn execute(&self, context: &mut CommandContext) -> Result<(), anyhow::Error> {
        Ok(())
    }
}