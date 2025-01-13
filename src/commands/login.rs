use crate::error::DbCheckError;

use super::{CommandContext, CommandExecutor};

use clap::Args;
use log::info;
use std::io::{self, Write};

#[derive(Debug, Args)]
pub struct LoginCommand {}

impl CommandExecutor for LoginCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        let auth_url = context.get_remote_server_url()? + "/auth/cli";

        info!(
            "Go to the following link in your browser, and complete the sign-in prompts:\n{auth_url}\n"
        );

        print!("Once finished, enter the verification code provided in your browser: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(DbCheckError::IO)?;

        let token = buffer.trim();
        if token.is_empty() {
            return Err(DbCheckError::InternalError(
                "An empty token was provided, cancelling...".to_string(),
            ));
        }

        context.secret_manager.save_token(token.to_string())?;
        Ok(())
    }
}
