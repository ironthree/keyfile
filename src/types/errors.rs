#[cfg(doc)]
use super::basic::*;

/// Error that is returned when attempting to construct a [`GroupName`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Group name: group names may only contain printable ASCII, except for the '[' and ']' characters")]
pub struct InvalidGroupName;

/// Error that is returned when attempting to construct a [`Key`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Key: keys can only contain alphanumeric ASCII characters and hyphens")]
pub struct InvalidKey;

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
#[error("Invalid Value: values cannot contain control characters")]
pub struct InvalidValue;

/// Error that is returned when attempting to construct a [`Whitespace`] from an invalid string.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Whitespace: whitespace must be ' ' or '\t'")]
pub struct InvalidWhitespace;

/// Error that is returned when attempting to construct [`Decor`] from invalid strings.
#[derive(Debug, thiserror::Error)]
#[error("Invalid Decor: decor must be either empty lines or lines that start with the '#' character")]
pub struct InvalidDecor;
