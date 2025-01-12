use clap::Args; 

use crate::{commands::{CommandContext, CommandExecutor}, error::DbCheckError};

#[derive(Debug, Args)]
pub struct TestCommand {
    #[arg(short, long)]
    pub all: bool,
}

impl CommandExecutor for TestCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Running tests");
        Ok(())
    }
}