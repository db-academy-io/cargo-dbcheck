use clap::Args;
use git2::Repository;
use std::fs::File;
use std::path::{PathBuf, self};

use crate::{course::{Course, CourseResponseWrapper, CourseStatus, CourseStatusResponseWrapper}, error::DbCheckError};

use super::{CommandContext, CommandExecutor};

#[derive(Debug, Args)]
pub struct InitCommand {
    /// A project id, find the list of project ids on db-academy.io
    #[arg(short, long)]
    pub project_id: String,

    /// Path to the project
    #[arg(long)]
    pub path: Option<PathBuf>,

    /// Reinitialize the repository, remove all existing files and start course from scratch
    #[arg(long, default_value_t = false)]
    pub reinitialize: bool,
}

impl CommandExecutor for InitCommand {
    
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Initializing course project repository");
        println!("Project id: {}", self.project_id);
        
        let path_given = self.path.clone().unwrap_or(".".into());
        
        let path_absolute = path::absolute(&path_given)
            .map_err(|e| DbCheckError::IO(e))?;
        
        if context.is_repo_initialized(&path_absolute)? && !self.reinitialize {
            println!("Repository already initialized");
            return Ok(());
        }

        if self.reinitialize {
            println!("[WARNING] Reinitializing repository, all existing files will be removed...");
            std::fs::remove_dir_all(&path_absolute).map_err(|e| DbCheckError::IO(e))?;
        }

        let repo = Repository::init(&path_absolute).map_err(|e| DbCheckError::Git(e))?;
        println!("Repository initialized at {:?}", repo.path());

        self.create_course_files(&path_absolute, context)?;
        Ok(())
    }
}

impl InitCommand {
    fn create_course_files(&self, path: &PathBuf, context: &mut CommandContext) -> Result<(), DbCheckError> {
        let dir = path.join(".db-academy");
        std::fs::create_dir_all(dir.clone()).map_err(|e| DbCheckError::IO(e))?;
        let course_syllabus = self.get_course_syllabus(context)?;
        let syllabus_file = File::create(dir.join("syllabus.json")).map_err(|e| DbCheckError::IO(e))?;
        serde_json::to_writer_pretty(syllabus_file, &course_syllabus).map_err(|e| DbCheckError::IO(e.into()))?;
        println!("Course syllabus saved");

        let course_status = self.get_course_status(context)?;
        let status_file = File::create(dir.join("status.json")).map_err(|e| DbCheckError::IO(e))?;
        serde_json::to_writer_pretty(status_file, &course_status).map_err(|e| DbCheckError::IO(e.into()))?;
        println!("Course status saved");
        Ok(())
    }

    fn get_course_syllabus(&self, context: &mut CommandContext) -> Result<Course, DbCheckError> {
        let url = format!("https://db-academy.io/api/course/{}", self.project_id);

        let json_value = context.get_request(url)?;
        let response_wrapper: CourseResponseWrapper = json_value.try_into()?;
        Ok(response_wrapper.body)
    }

    fn get_course_status(&self, context: &mut CommandContext) -> Result<CourseStatus, DbCheckError> {
        let url = format!("https://db-academy.io/api/course/{}", self.project_id);
        let json_value = context.get_request(url)?;
        let response_wrapper: CourseStatusResponseWrapper = json_value.try_into()?;

        let mut course_status = response_wrapper.body;
        if self.reinitialize {
            course_status.current_topic = None;
        }
        Ok(course_status)
    }
}
