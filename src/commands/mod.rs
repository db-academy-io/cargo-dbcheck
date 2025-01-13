mod course;
mod init;
mod login;
mod logout;
mod progress;

use clap::Subcommand;

use course::CoursesCommand;
use init::InitCommand;
use login::LoginCommand;
use logout::LogoutCommand;
use progress::{NextTopicCommand, PullCommand, StatusCommand, SubmitCommand, TestCommand};

use crate::{context::CommandContext, error::DbCheckError};

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

    /// Show the status of the current project
    Status(StatusCommand),

    /// Pull the current project status from db-academy.io
    Pull(PullCommand),

    /// Get course information
    Courses(CoursesCommand),

    /// Move to the next topic of the course
    #[command(name = "next")]
    NextTopic(NextTopicCommand),

    /// Submit current progress and move to the next stage of the course
    Submit(SubmitCommand),
}

pub trait CommandExecutor {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError>;
}

impl CommandExecutor for Command {
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        match self {
            Command::Login(login) => login.execute(context),
            Command::Logout(logout) => logout.execute(context),
            Command::Init(init) => init.execute(context),
            Command::Test(test) => test.execute(context),
            Command::Status(status) => status.execute(context),
            Command::Courses(courses) => courses.execute(context),
            Command::NextTopic(next_topic) => next_topic.execute(context),
            Command::Submit(submit) => submit.execute(context),
            Command::Pull(pull) => pull.execute(context),
        }
    }
}
