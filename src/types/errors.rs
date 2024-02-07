#[cfg(doc)]
use super::basic::*;

/// Error that is returned when attempting to construct a [`GroupName`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Group name: group names may only contain printable ASCII, except for the '[' and ']' characters")]
pub struct InvalidGroupName;

/// Error that is returned when attempting to construct a [`Key`] from an invalid string.
#[derive(Debug, thiserror::Error)]
pub enum InvalidKey {
    /// Error variant for non-ASCII strings.
    #[error("Invalid Key: keys must be ASCII-only")]
    NotAscii,
    /// Error variant for non-alphanumeric strings.
    #[error("Invalid Key: keys can only contain alphanumeric characters and hyphens")]
    NotAlphanumeric,
}

/// Error that is returned when attempting to construct a [`Language`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Language: language names may only contain alphabetic ASCII characters")]
pub struct InvalidLanguage;

/// Error that is returned when attempting to construct a [`Country`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphabetic ASCII characters")]
pub struct InvalidCountry;

/// Error that is returned when attempting to construct a [`Encoding`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphanmumeric ASCII characters and hyphens")]
pub struct InvalidEncoding;

/// Error that is returned when attempting to construct a [`Modifier`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphabetic ASCII characters")]
pub struct InvalidModifier;

/// Error that is returned when attempting to construct a [`Value`] from an invalid string.
#[derive(Debug, thiserror::Error)]
pub enum InvalidValue {
    /// Error variant for strings that contain control characters.
    #[error("Invalid Value: values cannot contain control characters")]
    ContainsControlCharacter,
    /// Error variant for strings that span multiple lines.
    #[error("Invalid Value: values cannot span multiple lines")]
    ContainsNewline,
}

/// Error that is returned when attempting to construct a [`Whitespace`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Whitespace: whitespace must be ' ' or '\t'")]
pub struct InvalidWhitespace;

/// Error that is returned when attempting to construct [`Decor`] from invalid strings.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Decor: decor must be either empty lines or lines that start with the '#' character")]
pub struct InvalidDecor;
