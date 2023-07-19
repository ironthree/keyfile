use thiserror::Error;

#[derive(Debug, Error)]
pub enum DesktopError {
    #[error("Invalid line (line {}): {}", .lineno, .line)]
    InvalidLine { line: String, lineno: usize },
    #[error("Multiple groups with the same name (line {}): {}", .lineno, .name)]
    DuplicateGroup { name: String, lineno: usize },
    #[error("Multiple key-value pairs with the same key (line {}): {}", .lineno, .key)]
    DuplicateKey { key: String, lineno: usize },
    #[error("No group with name: {}", .name)]
    MissingGroup { name: String },
}

impl DesktopError {
    pub(crate) fn invalid_line(line: String, lineno: usize) -> Self {
        DesktopError::InvalidLine { line, lineno }
    }

    pub(crate) fn duplicate_group(name: String, lineno: usize) -> Self {
        DesktopError::DuplicateGroup { name, lineno }
    }

    pub(crate) fn duplicate_key(key: String, lineno: usize) -> Self {
        DesktopError::DuplicateKey { key, lineno }
    }

    pub(crate) fn missing_group(name: String) -> Self {
        DesktopError::MissingGroup { name }
    }
}
