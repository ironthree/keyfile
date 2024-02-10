//! ## Implementations of the fundamental data types of KeyFiles
//!
//! This module defines wrapper types for strings that can only hold valid values by construction:
//!
//! - [`GroupName`]: printable ASCII characters except `[` and `]`
//! - [`Key`]: alphanumeric ASCII characters and the `-` character
//! - [`Language`]: alphabetic ASCII characters
//! - [`Country`]: alphabetic ASCII characters
//! - [`Encoding`]: alphanumeric ASCII characters and the `-` character
//! - [`Modifier`]: alphabetic ASCII characters
//! - [`Value`]: no control characters (including `\n` and `\r`)
//! - [`Whitespace`]: space characters (` `) and / or TAB characters (`\t`)
//! - [`Decor`]: list of strings that are either empty or start with the `#` character
//!
//!
//! Additionally, this module contains the definition of [`Locale`], which is a composite of [`Language`], [`Country`]
//! (optional), [`Encoding`] (optional), and [`Modifier`] (optional).
//!
//! These implementations should match the basic file format as described in the [Desktop Entry Specification].
//!
//! [Desktop Entry Specification]: https://specifications.freedesktop.org/desktop-entry-spec/latest/

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) const REGEX_ERROR: &str = "Failed to compile hard-coded regular expression.";

pub(crate) const GROUPNAME_REGEX: &str = r"[[:print:]&&[^\[\]]]+";
pub(crate) const KEY_REGEX: &str = r"[[:alnum:]-]+";
pub(crate) const LANGUAGE_REGEX: &str = r"[[:alpha:]]+";
pub(crate) const COUNTRY_REGEX: &str = r"[[:alpha:]]+";
pub(crate) const ENCODING_REGEX: &str = r"[[:alnum:]-]+";
pub(crate) const MODIFIER_REGEX: &str = r"[[:alpha:]]+";
pub(crate) const VALUE_REGEX: &str = r"[^[:cntrl:]]*";
pub(crate) const WHITESPACE_REGEX: &str = r"[[:blank:]]*";

static GROUPNAME: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{GROUPNAME_REGEX}$")).expect(REGEX_ERROR));
static KEY: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{KEY_REGEX}$")).expect(REGEX_ERROR));
static LANGUAGE: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{LANGUAGE_REGEX}$")).expect(REGEX_ERROR));
static COUNTRY: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{COUNTRY_REGEX}$")).expect(REGEX_ERROR));
static ENCODING: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{ENCODING_REGEX}$")).expect(REGEX_ERROR));
static MODIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{MODIFIER_REGEX}$")).expect(REGEX_ERROR));
static VALUE: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{VALUE_REGEX}$")).expect(REGEX_ERROR));
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"^{WHITESPACE_REGEX}$")).expect(REGEX_ERROR));
static LOCALE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(r"(?<lang>{LANGUAGE_REGEX})(?:_(?<country>{COUNTRY_REGEX}))?(?:\.(?<encoding>{ENCODING_REGEX}))?(?:@(?<modifier>{MODIFIER_REGEX}))?")).expect(REGEX_ERROR)
});

/// ## Error that is returned when attempting to initialize a type with an invalid input for that type
///
/// The newtype wrappers in this module ensure that only valid strings can be used for manually building keyfiles.
/// Attempting to construct a type from a string that is invalid for that specific type will yield one of the variants
/// of this error.
#[derive(Debug, thiserror::Error)]
pub enum InvalidString {
    /// An invalid string was passed to [`GroupName::try_from`].
    #[error("Invalid group name: may only contain printable ASCII, except for the '[' and ']' characters")]
    GroupName,
    /// An invalid string was passed to [`Key::try_from`].
    #[error("Invalid key name: may only contain alphanumeric ASCII characters and the '-' character")]
    Key,
    /// An invalid string was passed to [`Language::try_from`].
    #[error("Invalid lanugage: may only contain alphabetic ASCII characters")]
    Language,
    /// An invalid string was passed to [`Country::try_from`].
    #[error("Invalid country: may only contaun alphabetic ASCII characters")]
    Country,
    /// An invalid string was passed to [`Encoding::try_from`].
    #[error("Invalid encoding: may only contain alphanumeric ASCII characters and the '-' character")]
    Encoding,
    /// An invalid string was passed to [`Modifier::try_from`].
    #[error("Invalid modifier: may only contain alphabetic ASCII characters")]
    Modifier,
    /// An invalid string was passed to [`Value::try_from`].
    #[error("Invalid value: may not contain control characters")]
    Value,
    /// An invalid string was passed to [`Whitespace::try_from`].
    #[error("Invalid whitespace: may only contain space (' ') or tab ('\t')")]
    Whitespace,
    /// An invalid list of strings was passed to [`Decor::try_from`].
    #[error("Invalid decor: may only contain empty strings or strings that start with the '#' character")]
    Decor,
    /// An invalid string was passed to [`Locale::try_from`].
    #[error("Invalid locale: unrecognized format")]
    Locale,
}

/// ## Newtype struct wrapping strings that are valid group names
///
/// New instances of `GroupName` can only be created from strings that are valid group names:
///
/// ```
/// use keyfile::types::GroupName;
///
/// let group = GroupName::try_from("hello").unwrap();
/// let error = GroupName::try_from("[[[[[").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::GroupName;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = GroupName::try_from("hello").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct GroupName<'a> {
    inner: Cow<'a, str>,
}

impl<'a> GroupName<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'n: 'a>(value: Cow<'n, str>) -> Self {
        GroupName { inner: value }
    }
}

impl<'a> From<GroupName<'a>> for Cow<'a, str> {
    fn from(value: GroupName<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for GroupName<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !GROUPNAME.is_match(&value) {
            return Err(InvalidString::GroupName);
        }

        Ok(GroupName { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for GroupName<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        GroupName::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for GroupName<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        GroupName::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid keys
///
/// New instances of `Key` can only be created from strings that are valid key names:
///
/// ```
/// use keyfile::types::Key;
///
/// let key = Key::try_from("hello").unwrap();
/// let error = Key::try_from("//!!/").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Key;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Key::try_from("hello").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Key<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Key<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'v, str>) -> Self {
        Key { inner: value }
    }
}

impl<'a> From<Key<'a>> for Cow<'a, str> {
    #[inline(always)]
    fn from(value: Key<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Key<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !KEY.is_match(&value) {
            return Err(InvalidString::Key);
        }

        Ok(Key { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Key<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Key<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid language identifiers
///
/// New instances of `Language` can only be created from strings that are valid POSIX locale language identifiers:
///
/// ```
/// use keyfile::types::Language;
///
/// let lang = Language::try_from("de").unwrap();
/// let error = Language::try_from("42").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Language;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Language::try_from("de").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Language<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Language<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'v, str>) -> Self {
        Language { inner: value }
    }
}

impl<'a> From<Language<'a>> for Cow<'a, str> {
    fn from(value: Language<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Language<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !LANGUAGE.is_match(&value) {
            return Err(InvalidString::Language);
        }

        Ok(Language { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Language<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Language::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Language<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Language::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid country identifiers
///
/// New instances of `Country` can only be created from strings that are valid POSIX locale country / territory
/// identifiers:
///
/// ```
/// use keyfile::types::Country;
///
/// let country = Country::try_from("DE").unwrap();
/// let error = Country::try_from("!!").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Country;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Country::try_from("EN").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Country<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Country<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'v, str>) -> Self {
        Country { inner: value }
    }
}

impl<'a> From<Country<'a>> for Cow<'a, str> {
    fn from(value: Country<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Country<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !COUNTRY.is_match(&value) {
            return Err(InvalidString::Country);
        }

        Ok(Country { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Country<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Country::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Country<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Country::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid encoding identifiers
///
/// New instances of `Encoding` can only be created from strings that are valid POSIX locale encoding identifiers:
///
/// ```
/// use keyfile::types::Encoding;
///
/// let encoding = Encoding::try_from("utf8").unwrap();
/// let error = Encoding::try_from("morse!").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Encoding;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Encoding::try_from("iso88591").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Encoding<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Encoding<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'a, str>) -> Self {
        Encoding { inner: value }
    }
}

impl<'a> From<Encoding<'a>> for Cow<'a, str> {
    fn from(value: Encoding<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Encoding<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !ENCODING.is_match(&value) {
            return Err(InvalidString::Encoding);
        }

        Ok(Encoding { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Encoding<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Encoding::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Encoding<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Encoding::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid locale modifiers
///
/// New instances of `Encoding` can only be created from strings that are valid POSIX locale modifiers:
///
/// ```
/// use keyfile::types::Modifier;
///
/// let modifier = Modifier::try_from("latin").unwrap();
/// let error = Modifier::try_from("!foo!").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Modifier;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Modifier::try_from("cyrillic").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Modifier<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Modifier<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'a, str>) -> Self {
        Modifier { inner: value }
    }
}

impl<'a> From<Modifier<'a>> for Cow<'a, str> {
    fn from(value: Modifier<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Modifier<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !MODIFIER.is_match(&value) {
            return Err(InvalidString::Modifier);
        }

        Ok(Modifier { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Modifier<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Modifier::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Modifier<'a> {
    type Error = InvalidString;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Modifier::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid values
///
/// New instances of `Value` can only be created from strings that are valid value strings:
///
/// ```
/// use keyfile::types::Value;
///
/// let value = Value::try_from("WORLD").unwrap();
/// let error = Value::try_from("new\nline").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Value;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Value::try_from("WORLD").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Value<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Value<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'a, str>) -> Self {
        Value { inner: value }
    }
}

impl<'a> From<Value<'a>> for Cow<'a, str> {
    #[inline(always)]
    fn from(value: Value<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Value<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !VALUE.is_match(&value) {
            return Err(InvalidString::Value);
        }

        Ok(Value { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Value<'a> {
    type Error = InvalidString;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Value::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Value<'a> {
    type Error = InvalidString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Value::try_from(Cow::Owned(value))
    }
}

impl From<bool> for Value<'static> {
    fn from(value: bool) -> Self {
        match value {
            true => Value::new_unchecked(Cow::Borrowed("true")),
            false => Value::new_unchecked(Cow::Borrowed("false")),
        }
    }
}

macro_rules! impl_from_for_value {
    ($t:ty) => {
        impl From<$t> for Value<'static> {
            fn from(value: $t) -> Self {
                Value::new_unchecked(Cow::Owned(value.to_string()))
            }
        }
    }
}

impl_from_for_value!(i8);
impl_from_for_value!(i16);
impl_from_for_value!(i32);
impl_from_for_value!(i64);

impl_from_for_value!(u8);
impl_from_for_value!(u16);
impl_from_for_value!(u32);
impl_from_for_value!(u64);

impl_from_for_value!(f32);
impl_from_for_value!(f64);

/// ## Newtype struct wrapping strings that are valid whitespace
///
/// New instances of `Whitespace` can only be created from strings that are valid whitespace
/// (i.e. spaces / tabs surrounding the `=` character in a key-value pair):
///
/// ```
/// use keyfile::types::Whitespace;
///
/// let whitespace = Whitespace::try_from(" ").unwrap();
/// let error = Whitespace::try_from("HELLO").unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`String`], [`&str`], and [`Cow<str>`]. All values are stored as
/// [`Cow<str>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Whitespace;
/// use std::borrow::Cow;
///
/// let inner: Cow<str> = Whitespace::try_from("\t").unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Whitespace<'a> {
    inner: Cow<'a, str>,
}

impl<'a> Whitespace<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'v, str>) -> Self {
        Whitespace { inner: value }
    }
}

impl<'a> From<Whitespace<'a>> for Cow<'a, str> {
    #[inline(always)]
    fn from(value: Whitespace<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Whitespace<'a> {
    type Error = InvalidString;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !WHITESPACE.is_match(&value) {
            return Err(InvalidString::Whitespace);
        }

        Ok(Whitespace { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Whitespace<'a> {
    type Error = InvalidString;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Whitespace::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Whitespace<'a> {
    type Error = InvalidString;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Whitespace::try_from(Cow::Owned(value))
    }
}

/// ## Newtype struct wrapping strings that are valid comments and / or empty lines
///
/// New instances of `Decor` can only be created from strings that are valid comment lines
/// (a `#` character followed by an arbitrary string of UTF-8) or empty lines:
///
/// ```
/// use keyfile::types::Decor;
///
/// let decor = Decor::try_from(vec!["# This is a comment"]).unwrap();
/// let error = Decor::try_from(vec!["This=is not a comment"]).unwrap_err();
/// ```
///
/// The [`TryFrom`] trait is implemented for [`Vec<String>`], [`Vec<&str>`], and [`Vec<Cow<str>>`]. All values are
/// stored as [`Vec<Cow<str>>`] values internally.
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::Decor;
/// use std::borrow::Cow;
///
/// let inner: Vec<Cow<str>> = Decor::try_from(vec![""]).unwrap().into();
/// ```
#[derive(Clone, Debug)]
pub struct Decor<'a> {
    inner: Vec<Cow<'a, str>>,
}

impl<'a> Decor<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Vec<Cow<'a, str>>) -> Self {
        Decor { inner: value }
    }
}

impl<'a> From<Decor<'a>> for Vec<Cow<'a, str>> {
    #[inline(always)]
    fn from(value: Decor<'a>) -> Self {
        value.inner
    }
}

impl<'a> TryFrom<Vec<Cow<'a, str>>> for Decor<'a> {
    type Error = InvalidString;

    fn try_from(value: Vec<Cow<'a, str>>) -> Result<Self, Self::Error> {
        for line in &value {
            if !line.is_empty() && !line.starts_with('#') {
                return Err(InvalidString::Decor);
            }
        }

        Ok(Decor { inner: value })
    }
}

impl<'a> TryFrom<Vec<&'a str>> for Decor<'a> {
    type Error = InvalidString;

    fn try_from(value: Vec<&'a str>) -> Result<Self, Self::Error> {
        Decor::try_from(value.into_iter().map(Cow::Borrowed).collect::<Vec<_>>())
    }
}

impl<'a> TryFrom<Vec<String>> for Decor<'a> {
    type Error = InvalidString;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Decor::try_from(value.into_iter().map(Cow::Owned).collect::<Vec<_>>())
    }
}

/// ## Locale identifier (language, country / territory, encoding, and modifier)
///
/// This struct represents a locale identifier as used on UNIX / POSIX systems.
///
/// This type contains a non-optional [`Language`], and optional [`Country`], [`Encoding`], and [`Modifier`], which
/// are all stored as [`Cow<str>`] internally to avoid copying strings unless necessary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locale<'a> {
    pub(crate) lang: Cow<'a, str>,
    pub(crate) country: Option<Cow<'a, str>>,
    pub(crate) encoding: Option<Cow<'a, str>>,
    pub(crate) modifier: Option<Cow<'a, str>>,
}

impl<'a> TryFrom<&'a str> for Locale<'a> {
    type Error = InvalidString;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let Some(caps) = LOCALE.captures(value) else {
            return Err(InvalidString::Locale);
        };

        let Some(lang) = caps.name("lang").map(|m| m.as_str()) else {
            return Err(InvalidString::Locale);
        };

        let country = caps.name("country").map(|m| m.as_str());
        let encoding = caps.name("encoding").map(|m| m.as_str());
        let modifier = caps.name("modifier").map(|m| m.as_str());

        if encoding.is_some() {
            // This is an error: Constructing an encoding modifier is not supported since only UTF-8 encoded strings
            // can be set as values, so no valid value could be set for a KeyValuePair with this Locale set.
            return Err(InvalidString::Encoding);
        }

        Ok(Locale::new_with_encoding(
            Language::new_unchecked(Cow::Borrowed(lang)),
            country.map(|c| Country::new_unchecked(Cow::Borrowed(c))),
            encoding.map(|e| Encoding::new_unchecked(Cow::Borrowed(e))),
            modifier.map(|m| Modifier::new_unchecked(Cow::Borrowed(m))),
        ))
    }
}

impl<'a> Locale<'a> {
    /// Method for creating a new [`Locale`]
    pub fn new<'l: 'a>(lang: Language<'l>, country: Option<Country<'l>>, modifier: Option<Modifier<'l>>) -> Self {
        Locale {
            lang: lang.into(),
            country: country.map(Into::into),
            encoding: None,
            modifier: modifier.map(Into::into),
        }
    }

    pub(crate) fn new_with_encoding<'l: 'a>(
        lang: Language<'l>,
        country: Option<Country<'l>>,
        encoding: Option<Encoding<'l>>,
        modifier: Option<Modifier<'l>>,
    ) -> Self {
        Locale {
            lang: lang.into(),
            country: country.map(Into::into),
            encoding: encoding.map(Into::into),
            modifier: modifier.map(Into::into),
        }
    }

    /// ### Method for converting a `Locale<'a>` into a `Locale<'static>`
    ///
    /// This is a "deep copy" which converts any [`Cow::Borrowed`] into [`Cow::Owned`] by copying the underlying string
    /// into a new "owned" value.
    pub fn into_owned(self) -> Locale<'static> {
        Locale {
            lang: Cow::Owned(self.lang.into_owned()),
            country: self.country.map(|c| Cow::Owned(c.into_owned())),
            encoding: self.encoding.map(|c| Cow::Owned(c.into_owned())),
            modifier: self.modifier.map(|c| Cow::Owned(c.into_owned())),
        }
    }

    /// Method for getting the language identifier
    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    /// ### Method for setting the language identifier
    ///
    /// The replaced string is returned.
    pub fn set_lang<'l: 'a>(&mut self, lang: Language<'l>) -> Cow<str> {
        std::mem::replace(&mut self.lang, lang.into())
    }

    /// Method for getting the country / territory identifier
    pub fn get_country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    /// ### Method for getting the country / territory identifier
    ///
    /// If this method replaces an existing identifier, it is returned.
    pub fn set_country<'c: 'a>(&mut self, country: Option<Country<'c>>) -> Option<Cow<str>> {
        std::mem::replace(&mut self.country, country.map(Into::into))
    }

    /// Method for getting the encoding identifier
    pub fn get_encoding(&self) -> Option<&str> {
        self.encoding.as_deref()
    }

    /// Method for getting the locale modifier
    pub fn get_modifier(&self) -> Option<&str> {
        self.modifier.as_deref()
    }

    /// ### Method for setting the locale modifier
    ///
    /// If this method replaces an existing modifier, it is returned.
    pub fn set_modifier<'m: 'a>(&mut self, modifier: Option<Modifier<'m>>) -> Option<Cow<str>> {
        std::mem::replace(&mut self.modifier, modifier.map(Into::into))
    }
}

impl<'a> Display for Locale<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)?;

        if let Some(country) = &self.country {
            write!(f, "_{}", country)?;
        }

        if let Some(modifier) = &self.modifier {
            write!(f, "@{}", modifier)?;
        }

        Ok(())
    }
}
