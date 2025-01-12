use clap::Args; 

use crate::commands::{CommandContext, CommandExecutor};

#[derive(Debug, Args)]
pub struct TestCommand {
    #[arg(short, long)]
    pub all: bool,
}

impl CommandExecutor for TestCommand {
    fn execute(&self, _context: &mut CommandContext) -> Result<(), anyhow::Error> {
        println!("Running tests");
        Ok(())
    }
}