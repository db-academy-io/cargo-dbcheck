use clap::Args;
use git2::Repository;
use log::{debug, info, warn};
use std::fs::File;
use std::path::{self, Path, PathBuf};

use crate::{
    course::{Course, CourseResponseWrapper, CourseStatus, CourseStatusResponseWrapper},
    error::DbCheckError,
};

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
        info!("Initializing course project repository");
        info!("Project id: {}", self.project_id);

        let path_given = self.path.clone().unwrap_or(".".into());
        debug!("Path given: {:?}", path_given);

        let path_absolute = path::absolute(&path_given).map_err(DbCheckError::IO)?;

        debug!("Path absolute: {:?}", path_absolute);
        if context.is_repo_initialized(&path_absolute)? && !self.reinitialize {
            warn!("Repository already initialized");
            return Ok(());
        }

        if self.reinitialize {
            warn!("Reinitializing repository, all existing files will be removed...");
            if path_absolute.exists() {
                std::fs::remove_dir_all(&path_absolute).map_err(DbCheckError::IO)?;
            }
        }

        let repo = Repository::init(&path_absolute).map_err(DbCheckError::Git)?;
        info!("Repository initialized at {:?}", repo.path());

        self.create_course_files(&path_absolute, context)?;
        Ok(())
    }
}

impl InitCommand {
    fn create_course_files(
        &self,
        path: &Path,
        context: &mut CommandContext,
    ) -> Result<(), DbCheckError> {
        debug!("Creating course files");

        let dbacademydir = path.join(".db-academy");
        debug!("Creating directory: {:?}", dbacademydir);
        std::fs::create_dir_all(dbacademydir.clone()).map_err(DbCheckError::IO)?;

        let course_syllabus = self.get_course_syllabus(context)?;
        let syllabus_file =
            File::create(dbacademydir.join("syllabus.json")).map_err(DbCheckError::IO)?;
        serde_json::to_writer_pretty(syllabus_file, &course_syllabus)
            .map_err(|e| DbCheckError::IO(e.into()))?;

        let course_status = self.get_course_status(context)?;

        let status_file =
            File::create(dbacademydir.join("status.json")).map_err(DbCheckError::IO)?;
        serde_json::to_writer_pretty(status_file, &course_status)
            .map_err(|e| DbCheckError::IO(e.into()))?;
        info!("Course metadata saved");
        Ok(())
    }

    fn get_course_syllabus(&self, context: &mut CommandContext) -> Result<Course, DbCheckError> {
        debug!("Getting course syllabus");
        let url = format!("https://db-academy.io/api/course/{}", self.project_id);
        debug!("URL: {:?}", url);

        let json_value = context.get_request(url)?;
        let response_wrapper: CourseResponseWrapper = json_value.try_into()?;
        Ok(response_wrapper.body)
    }

    fn get_course_status(
        &self,
        context: &mut CommandContext,
    ) -> Result<CourseStatus, DbCheckError> {
        debug!("Getting course status");

        let url = format!("https://db-academy.io/api/course/{}", self.project_id);
        debug!("URL: {:?}", url);

        let json_value = context.get_request(url)?;
        let response_wrapper: CourseStatusResponseWrapper = json_value.try_into()?;

        let mut course_status = response_wrapper.body;
        if self.reinitialize {
            debug!("Reinitializing course status");
            course_status.current_topic = None;
        }
        Ok(course_status)
    }
}
