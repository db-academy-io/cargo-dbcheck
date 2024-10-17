use std::str;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
    /// A user token to interact with db-academy.io
    #[arg(short, long)]
    pub user_token: String,
}
