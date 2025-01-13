use clap::Args;
use git2::Repository;
use std::fs::{File};
use std::path::{self, PathBuf};

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

    /// Start course from the first topic
    #[arg(long, default_value_t = false)]
    pub no_progress: bool,
}

impl CommandExecutor for InitCommand {
    
    fn execute(&self, context: &mut CommandContext) -> Result<(), DbCheckError> {
        println!("Initializing course project repository");
        println!("Project id: {}", self.project_id);
        let path = self.path.clone().unwrap_or(".".into());
        println!("Path: {}", path.to_string_lossy());

        let repo = Repository::init(&path).map_err(|e| DbCheckError::Git(e))?;
        println!("Repository initialized at {:?}", repo.path());

        self.create_course_files(&path, context)?;
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
        if self.no_progress {
            course_status.current_topic = None;
        }
        Ok(course_status)
    }
}
