mod init;
mod login;
mod logout;
mod course;
mod progress;

use keyring::Entry;

use clap::Subcommand;

use init::InitCommand;
use login::LoginCommand;
use logout::LogoutCommand;
use course::CoursesCommand;
use progress::{SubmitCommand, NextTopicCommand, TestCommand};


#[derive(Debug)]
pub struct CommandContext<'a> {
    pub _config: &'a str,
}

impl<'a> CommandContext<'a> {
    pub fn save_token(&mut self, token: String) -> Result<(), anyhow::Error> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username)?;
        entry.set_password(&token)?;
        println!("Token stored securely.");
        Ok(())
    }

    pub fn get_active_token(&mut self) -> Result<String, anyhow::Error> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username)?;
        let password = entry.get_password()?;
        Ok(password)
    }

    pub fn remove_token(&mut self) -> Result<(), anyhow::Error> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username)?;
        entry.delete_credential()?;
        Ok(())
    }

    pub fn get_remote_server_url(&mut self) -> Result<String, anyhow::Error> {
        Ok("https://db-academy.io".to_string())
    }
}

pub trait CommandExecutor {
    fn execute(&self, context: &mut CommandContext) -> Result<(), anyhow::Error>;
}

impl CommandExecutor for Command {
    fn execute(&self, context: &mut CommandContext) -> Result<(), anyhow::Error> {
        let _ = match self {
            Command::Login(command) => command.execute(context),
            Command::Logout(command) => command.execute(context),
            Command::Init(command) => command.execute(context),
            Command::Courses(command) => command.execute(context),
            _ => todo!(),
        };
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Authenticate on db-academy.io
    Login(LoginCommand),

    /// Logout from db-academy.io
    Logout(LogoutCommand),

    /// Init a project repo
    Init(InitCommand),

    /// Run tests for the current stage
    Test(TestCommand),

    /// Get course information
    Courses(CoursesCommand),

    /// Move to the next topic of the course
    #[command(name = "next")]
    NextTopic(NextTopicCommand),

    /// Submit current progress and move to the next stage of the course
    Submit(SubmitCommand),
}
