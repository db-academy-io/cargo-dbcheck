mod login;
mod logout;

use keyring::Entry;

use clap::{Args, Subcommand};
use login::login;
use logout::logout;

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
}

pub trait CommandExecutor {
    fn execute(&self, context: &mut CommandContext);
}

impl CommandExecutor for Command {
    fn execute(&self, context: &mut CommandContext) {
        let _ = match self {
            Command::Login(_command) => login(context),
            Command::Logout(_command) => logout(context),
            _ => todo!(),
        };
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
    Course(CourseCommand),

    /// Move to the next topic of the course
    Next(NextCommand),

    /// Submit current progress and move to the next stage of the course
    Submit(SubmitCommand),
}

#[derive(Debug, Args)]
pub struct TestCommand {
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CourseCommand {
    /// Current course information
    #[arg(short, long, default_value = "true")]
    pub current: bool,

    /// List all courses from db-academy.io
    #[arg(short, long)]
    pub list: bool,
}

#[derive(Debug, Args)]
pub struct SubmitCommand {}

#[derive(Debug, Args)]
pub struct NextCommand {}

#[derive(Debug, Args)]
pub struct InitCommand {
    /// A project id, find the list of project ids on db-academy.io
    #[arg(short, long)]
    pub project_id: String,
}

#[derive(Debug, Args)]
pub struct LoginCommand {}

#[derive(Debug, Args)]
pub struct LogoutCommand {}
