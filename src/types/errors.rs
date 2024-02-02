#[derive(Debug, thiserror::Error)]
#[error("Invalid Group name: group names may only contain printable ASCII, except for the '[' and ']' characters")]
pub struct InvalidGroupName;

#[derive(Debug, thiserror::Error)]
pub enum InvalidKey {
    #[error("Invalid Key: keys must be ASCII-only")]
    NotAscii,
    #[error("Invalid Key: keys can only contain alphanumeric characters and hyphens")]
    NotAlphanumeric,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid Language: language names may only contain alphabetic ASCII characters")]
pub struct InvalidLanguage;

#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphabetic ASCII characters")]
pub struct InvalidCountry;

#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphanmumeric ASCII characters and hyphens")]
pub struct InvalidEncoding;

#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain alphabetic ASCII characters")]
pub struct InvalidModifier;

#[derive(Debug, thiserror::Error)]
pub enum InvalidValue {
    #[error("Invalid Value: values cannot contain control characters")]
    ContainsControlCharacter,
    #[error("Invalid Value: values cannot span multiple lines")]
    ContainsNewline,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid Whitespace: whitespace must be ' ' or '\t'")]
pub struct InvalidWhitespace;

#[derive(Debug, thiserror::Error)]
#[error("Invalid Decor: decor must be either empty lines or lines that start with the '#' character")]
pub struct InvalidDecor;
