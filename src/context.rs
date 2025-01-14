use std::{
    fs::File,
    path::{Path, PathBuf},
};

use git2::Repository;
use keyring::Entry;
use log::{debug, info};

use crate::{course::CourseStatus, error::DbCheckError};

#[derive(Debug)]
pub struct CommandContext<'a> {
    pub _config: &'a str,
    pub secret_manager: SecretManager,
    pub path_manager: PathManager,
}

impl<'a> CommandContext<'a> {
    pub fn new(config: &'a str) -> Self {
        Self {
            _config: config,
            secret_manager: SecretManager,
            path_manager: PathManager,
        }
    }

    pub fn get_remote_server_url(&mut self) -> Result<String, DbCheckError> {
        Ok("https://db-academy.io".to_string())
    }

    pub fn get_request(&mut self, url: String) -> Result<serde_json::Value, DbCheckError> {
        debug!("Getting request for URL: {:?}", url);
        let client = reqwest::blocking::Client::new();

        let mut request_builder = client.get(url);

        if let Ok(token) = self.secret_manager.get_active_token() {
            debug!("Token exists, adding to request header as Authorization key");
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        let response = request_builder
            .send()
            .map_err(|e| DbCheckError::Network(e.to_string()))?;

        debug!("Response status: {:?}", response.status());

        response
            .json()
            .map_err(|e| DbCheckError::Network(e.to_string()))
    }

    pub fn is_repo_initialized(&mut self, path: &PathBuf) -> Result<bool, DbCheckError> {
        debug!("Checking if repo is initialized");

        let db_academy_dir = self.path_manager.get_repo_path(path)?;
        debug!("DB Academy directory: {:?}", db_academy_dir);

        let status_file = self.path_manager.get_course_status_file(path)?;
        debug!("Status file: {:?}", status_file);

        let syllabus_file = self.path_manager.get_course_syllabus_file(path)?;
        debug!("Syllabus file: {:?}", syllabus_file);

        let repo = Repository::open(path);
        debug!("Repo: {:?}", repo.is_ok());

        Ok(db_academy_dir.exists()
            && status_file.exists()
            && syllabus_file.exists()
            && repo.is_ok())
    }

    pub fn get_course_status(&self) -> Result<CourseStatus, DbCheckError> {
        debug!("Getting course status");
        let current_dir = std::env::current_dir().map_err(DbCheckError::IO)?;
        debug!("Current directory: {:?}", current_dir);
        let status_file = self.path_manager.get_course_status_file(&current_dir)?;
        debug!("Status file: {:?}", status_file);
        let json_value = self.json_from_file(&status_file)?;
        let status = CourseStatus::try_from(json_value)?;
        Ok(status)
    }

    fn json_from_file(&self, path: &PathBuf) -> Result<serde_json::Value, DbCheckError> {
        let file = File::open(path).map_err(DbCheckError::IO)?;
        let json_value =
            serde_json::from_reader(file).map_err(|e| DbCheckError::FormatError(e.to_string()))?;
        Ok(json_value)
    }
}

#[derive(Debug)]
pub struct SecretManager;

impl SecretManager {
    pub fn get_active_token(&mut self) -> Result<String, DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(DbCheckError::Keyring)?;
        let password = entry.get_password().map_err(DbCheckError::Keyring)?;
        Ok(password)
    }

    pub fn is_token_set(&mut self) -> Result<bool, DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(DbCheckError::Keyring)?;
        Ok(entry.get_password().is_ok())
    }

    pub fn save_token(&mut self, token: String) -> Result<(), DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(DbCheckError::Keyring)?;
        entry.set_password(&token).map_err(DbCheckError::Keyring)?;
        info!("The token has been saved successfully");
        Ok(())
    }

    pub fn remove_token(&mut self) -> Result<(), DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(DbCheckError::Keyring)?;
        entry.delete_credential().map_err(DbCheckError::Keyring)?;
        info!("Token has been removed.");
        Ok(())
    }
}

#[derive(Debug)]
pub struct PathManager;

impl PathManager {
    pub fn get_repo_path(&self, path: &Path) -> Result<PathBuf, DbCheckError> {
        Ok(path.join(".db-academy"))
    }

    pub fn get_course_status_file(&self, path: &Path) -> Result<PathBuf, DbCheckError> {
        Ok(self.get_repo_path(path)?.join("status.json"))
    }

    pub fn get_course_syllabus_file(&self, path: &Path) -> Result<PathBuf, DbCheckError> {
        Ok(self.get_repo_path(path)?.join("syllabus.json"))
    }
}
