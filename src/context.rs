use keyring::Entry;

use crate::error::DbCheckError;

#[derive(Debug)]
pub struct CommandContext<'a> {
    pub _config: &'a str,
    pub secret_manager: SecretManager,
}

impl<'a> CommandContext<'a> {
    pub fn new(config: &'a str) -> Self {
        Self {
            _config: config,
            secret_manager: SecretManager,
        }
    }

    pub fn get_remote_server_url(&mut self) -> Result<String, DbCheckError> {
        Ok("https://db-academy.io".to_string())
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
        println!("The token has been saved successfully");
        Ok(())
    }

    pub fn remove_token(&mut self) -> Result<(), DbCheckError> {
        let service = "db-academy-io";
        let username = "db-academy-io-secret-token";
        let entry = Entry::new(service, username).map_err(|e| DbCheckError::Keyring(e))?;
        entry
            .delete_credential()
            .map_err(|e| DbCheckError::Keyring(e))?;
        println!("Token has been removed.");
        Ok(())
    }
}
