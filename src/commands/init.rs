use clap::Args;
use git2::Repository;
use std::{fs::File, path::PathBuf};

use crate::{course::{Course, ResponseWrapper}, error::DbCheckError};

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

        let repo = Repository::init(path.clone()).map_err(|e| DbCheckError::Git(e))?;
        println!("Repository initialized at {:?}", repo.path());

        self.create_course_file(&path)?;
        Ok(())
    }
}

impl InitCommand {
    fn create_course_file(&self, path: &PathBuf) -> Result<(), DbCheckError> {
        let dir = path.join(".db-academy");
        std::fs::create_dir_all(dir.clone()).map_err(|e| DbCheckError::IO(e))?;
        let course = self.get_course_syllabus()?;
        let course_file = File::create(dir.join("syllabus.json")).map_err(|e| DbCheckError::IO(e))?;
        serde_json::to_writer_pretty(course_file, &course).map_err(|e| DbCheckError::IO(e.into()))?;
        Ok(())
    }

    fn get_course_syllabus(&self) -> Result<Course, DbCheckError> {
        let url = format!("https://db-academy.io/api/course/{}", self.project_id);
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send().map_err(|e| DbCheckError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DbCheckError::Network(format!("Failed to get course syllabus: {}", response.status())));
        }
        let response_wrapper: ResponseWrapper = response.json().map_err(|e| DbCheckError::Network(e.to_string()))?;
        Ok(response_wrapper.body)
    }
}
