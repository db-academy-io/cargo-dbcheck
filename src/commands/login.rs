use super::{CommandContext, CommandExecutor};
use anyhow::{anyhow, Result};
use clap::Args;
use std::io::{self, Write};

#[derive(Debug, Args)]
pub struct LoginCommand {}

impl CommandExecutor for LoginCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), anyhow::Error> {
        let auth_url = context.get_remote_server_url()? + "/auth/cli";
        
        println!(
            "Go to the following link in your browser, and complete the sign-in prompts:\n{auth_url}\n"
        );
        
        print!("Once finished, enter the verification code provided in your browser: ");
        io::stdout().flush().expect("Unable to flush stdout");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let token = buffer.trim();
        if token.is_empty() {
            return Err(anyhow!("An empty token was provided, cancelling..."));
        }

        context.save_token(token.to_string())?;

        let pass = context.get_active_token()?;
        println!("Saved token is: {pass}");

        Ok(())
    }
}
