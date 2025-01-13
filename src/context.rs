use std::path::PathBuf;

use git2::Repository;
use keyring::Entry;
use log::info;

use crate::error::DbCheckError;

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
        let client = reqwest::blocking::Client::new();

        let mut request_builder = client.get(url);

        if let Ok(token) = self.secret_manager.get_active_token() {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        let response = request_builder
            .send()
            .map_err(|e| DbCheckError::Network(e.to_string()))?;
        Ok(response
            .json()
            .map_err(|e| DbCheckError::Network(e.to_string()))?)
    }

    pub fn is_repo_initialized(&mut self, path: &PathBuf) -> Result<bool, DbCheckError> {
        let db_academy_dir = self.path_manager.get_repo_path(path)?;
        let status_file = self.path_manager.get_course_status_file(path)?;
        let syllabus_file = self.path_manager.get_course_syllabus_file(path)?;
        let repo = Repository::open(path);
        Ok(db_academy_dir.exists()
            && status_file.exists()
            && syllabus_file.exists()
            && repo.is_ok())
    }
}

#[derive(Debug)]
pub struct SecretManager;

impl SecretManager {
    #[allow(dead_code)]
    pub fn get_active_token(&mut self) -> Result<String, DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(|e| DbCheckError::Keyring(e))?;
        let password = entry.get_password().map_err(|e| DbCheckError::Keyring(e))?;
        Ok(password)
    }

    pub fn is_token_set(&mut self) -> Result<bool, DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(|e| DbCheckError::Keyring(e))?;
        Ok(entry.get_password().is_ok())
    }

    pub fn save_token(&mut self, token: String) -> Result<(), DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(|e| DbCheckError::Keyring(e))?;
        entry
            .set_password(&token)
            .map_err(|e| DbCheckError::Keyring(e))?;
        info!("The token has been saved successfully");
        Ok(())
    }

    pub fn remove_token(&mut self) -> Result<(), DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(|e| DbCheckError::Keyring(e))?;
        entry
            .delete_credential()
            .map_err(|e| DbCheckError::Keyring(e))?;
        info!("Token has been removed.");
        Ok(())
    }
}

#[derive(Debug)]
pub struct PathManager;

impl PathManager {
    pub fn get_repo_path(&self, path: &PathBuf) -> Result<PathBuf, DbCheckError> {
        Ok(path.join(".db-academy"))
    }

    pub fn get_course_status_file(&self, path: &PathBuf) -> Result<PathBuf, DbCheckError> {
        Ok(self.get_repo_path(path)?.join("status.json"))
    }

    pub fn get_course_syllabus_file(&self, path: &PathBuf) -> Result<PathBuf, DbCheckError> {
        Ok(self.get_repo_path(path)?.join("syllabus.json"))
    }
}
