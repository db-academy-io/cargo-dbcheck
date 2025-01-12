use clap::Args;
use git2::Repository;
use std::path::PathBuf;

use crate::error::DbCheckError;

use super::{CommandContext, CommandExecutor};

#[derive(Debug, Args)]
pub struct InitCommand {
    /// A project id, find the list of project ids on db-academy.io
    #[arg(short, long)]
    pub project_id: String,

    /// Path to the project
    #[arg(long)]
    pub path: Option<PathBuf>,
}

impl CommandExecutor for InitCommand {
    
    fn execute(&self, _context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Initializing project repository");
        println!("Project id: {}", self.project_id);
        println!("Path: {:?}", self.path);

        let path = self.path.clone().unwrap_or_else(|| PathBuf::from("."));

        let repo = Repository::init(path).map_err(|e| DbCheckError::Git(e))?;
        println!("Repository initialized at {:?}", repo.path());

        Ok(())
    }

}
