use clap::Args;
use std::path::PathBuf;
use git2::Repository;

use super::CommandContext;

#[derive(Debug, Args)]
pub struct InitCommand {
    /// A project id, find the list of project ids on db-academy.io
    #[arg(short, long)]
    pub project_id: String,

    /// Path to the project
    #[arg(long)]
    pub path: Option<PathBuf>,
}

pub fn init(
    _context: &mut CommandContext,
    command: &InitCommand,
) -> Result<(), anyhow::Error> {
    println!("Initializing project repository");
    println!("Project id: {}", command.project_id);
    println!("Path: {:?}", command.path);

    let path = command.path.clone().unwrap_or_else(|| PathBuf::from("."));

    let repo = Repository::init(path)?;
    println!("Repository initialized at {:?}", repo.path());

    Ok(())
}
