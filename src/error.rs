use std::fmt::Display;

pub enum DbCheckError {
    /// An error related to IO operations
    IO(std::io::Error),

    /// An error related to git operations
    Git(git2::Error),

    /// An error related to keyring operations
    Keyring(keyring::Error),

    /// An internal DBCheck error
    InternalError(String),
}

impl Display for DbCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbCheckError::IO(e) => write!(f, "IO error: {}", e),
            DbCheckError::Keyring(e) => write!(f, "Keyring error: {}", e),
            DbCheckError::InternalError(e) => write!(f, "Internal error: {}", e),
            DbCheckError::Git(e) => write!(f, "Git error: {}", e),
        }
    }
}
