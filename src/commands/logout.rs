use crate::error::DbCheckError;

use super::{CommandContext, CommandExecutor};
use clap::Args;
use log::{info, warn};

#[derive(Debug, Args)]
pub struct LogoutCommand;

impl CommandExecutor for LogoutCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        info!("Logging out...");
        if !context.secret_manager.is_token_set()? {
            warn!("No token found");
            return Ok(());
        }
        context.secret_manager.remove_token()?;
        info!("Successfully logged out");
        Ok(())
    }
}
