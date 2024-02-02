use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyFileError {
    #[error("Invalid line (line {}): {}", .lineno, .line)]
    InvalidLine { line: String, lineno: usize },
    #[error("Multiple groups with the same name (line {}): {}", .lineno, .name)]
    DuplicateGroup { name: String, lineno: usize },
    #[error("Multiple key-value pairs with the same key (line {}): {}", .lineno, .key)]
    DuplicateKey { key: String, lineno: usize },
}

impl KeyFileError {
    pub(crate) fn invalid_line(line: String, lineno: usize) -> Self {
        KeyFileError::InvalidLine { line, lineno }
    }

    pub(crate) fn duplicate_group(name: String, lineno: usize) -> Self {
        KeyFileError::DuplicateGroup { name, lineno }
    }

    pub(crate) fn duplicate_key(key: String, lineno: usize) -> Self {
        KeyFileError::DuplicateKey { key, lineno }
    }
}
