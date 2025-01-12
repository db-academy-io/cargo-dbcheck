use crate::error::DbCheckError;

use super::{CommandContext, CommandExecutor};
use clap::Args;

#[derive(Debug, Args)]
pub struct LogoutCommand;

impl CommandExecutor for LogoutCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Logging out...");
        if !context.secret_manager.is_token_set()? {
            println!("No token found");
            return Ok(());
        }
        context.secret_manager.remove_token()?;
        println!("Successfully logged out");
        Ok(())
    }
}
