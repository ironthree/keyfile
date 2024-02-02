use std::borrow::Cow;
use std::fmt::Debug;

use once_cell::sync::Lazy;
use regex::Regex;

static GROUPNAME: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:HEADER
    Regex::new(r"^[[:print:][^\[\]]]+$").expect("Failed to compile hard-coded regular expression.")
});

static LANGUAGE: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:KEY_VALUE_PAIR
    Regex::new(r"^[[:alpha:]]+$").expect("Failed to compile hard-coded regular expression.")
});

static COUNTRY: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:KEY_VALUE_PAIR
    Regex::new(r"^[[:alpha:]]+$").expect("Failed to compile hard-coded regular expression.")
});

static ENCODING: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:KEY_VALUE_PAIR
    Regex::new(r"^[[:alnum:]-]+$").expect("Failed to compile hard-coded regular expression.")
});

static MODIFIER: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:KEY_VALUE_PAIR
    Regex::new(r"^[[:alpha:]]+$").expect("Failed to compile hard-coded regular expression.")
});

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
#[error("Invalid Group name: group names may only contain printable ASCII, except for the '[' and ']' characters")]
pub struct InvalidGroupName;

impl<'a> TryFrom<Cow<'a, str>> for GroupName<'a> {
    type Error = InvalidGroupName;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !GROUPNAME.is_match(&value) {
            return Err(InvalidGroupName);
        }

        Ok(GroupName { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for GroupName<'a> {
    type Error = InvalidGroupName;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        GroupName::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for GroupName<'a> {
    type Error = InvalidGroupName;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        GroupName::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
pub enum InvalidKey {
    #[error("Invalid Key: keys must be ASCII-only")]
    NotAscii,
    #[error("Invalid Key: keys can only contain alphanumeric characters and hyphens")]
    NotAlphanumeric,
}

impl<'a> TryFrom<Cow<'a, str>> for Key<'a> {
    type Error = InvalidKey;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        for c in value.chars() {
            if !c.is_ascii() {
                return Err(InvalidKey::NotAscii);
            }
            if !c.is_alphanumeric() && c != '-' {
                return Err(InvalidKey::NotAlphanumeric);
            }
        }

        Ok(Key { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Key<'a> {
    type Error = InvalidKey;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Key<'a> {
    type Error = InvalidKey;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
#[error("Invalid Language: language names may only contain printable ASCII")]
pub struct InvalidLanguage;

impl<'a> TryFrom<Cow<'a, str>> for Language<'a> {
    type Error = InvalidLanguage;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !LANGUAGE.is_match(&value) {
            return Err(InvalidLanguage);
        }

        Ok(Language { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Language<'a> {
    type Error = InvalidLanguage;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Language::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Language<'a> {
    type Error = InvalidLanguage;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Language::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
#[error("Invalid Country: country names may only contain printable ASCII")]
pub struct InvalidCountry;

impl<'a> TryFrom<Cow<'a, str>> for Country<'a> {
    type Error = InvalidCountry;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !COUNTRY.is_match(&value) {
            return Err(InvalidCountry);
        }

        Ok(Country { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Country<'a> {
    type Error = InvalidCountry;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Country::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Country<'a> {
    type Error = InvalidCountry;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Country::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
pub enum InvalidValue {
    #[error("Invalid Value: values cannot contain control characters")]
    ContainsControlCharacter,
    #[error("Invalid Value: values cannot span multiple lines")]
    ContainsNewline,
}

impl<'a> TryFrom<Cow<'a, str>> for Value<'a> {
    type Error = InvalidValue;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        for c in value.chars() {
            if c.is_control() {
                return Err(InvalidValue::ContainsControlCharacter);
            }
            if c == '\n' || c == '\r' {
                return Err(InvalidValue::ContainsNewline);
            }
        }

        Ok(Value { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Value<'a> {
    type Error = InvalidValue;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Value::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Value<'a> {
    type Error = InvalidValue;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Value::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
#[error("Invalid Whitespace: whitespace must be ' ' or '\t'")]
pub struct InvalidWhitespace;

impl<'a> TryFrom<Cow<'a, str>> for Whitespace<'a> {
    type Error = InvalidWhitespace;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        for c in value.chars() {
            if !c.is_ascii_whitespace() {
                return Err(InvalidWhitespace);
            }
        }

        Ok(Whitespace { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Whitespace<'a> {
    type Error = InvalidWhitespace;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Whitespace::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Whitespace<'a> {
    type Error = InvalidWhitespace;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Whitespace::try_from(Cow::Owned(value))
    }
}

// =============================================================================

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

#[derive(Debug, thiserror::Error)]
#[error("Invalid Decor: decor must be either empty lines or lines that start with the '#' character")]
pub struct InvalidDecor;

impl<'a> TryFrom<Vec<Cow<'a, str>>> for Decor<'a> {
    type Error = InvalidDecor;

    fn try_from(value: Vec<Cow<'a, str>>) -> Result<Self, Self::Error> {
        for line in &value {
            if !line.is_empty() && !line.starts_with('#') {
                return Err(InvalidDecor);
            }
        }

        Ok(Decor { inner: value })
    }
}

impl<'a> TryFrom<Vec<&'a str>> for Decor<'a> {
    type Error = InvalidDecor;

    fn try_from(value: Vec<&'a str>) -> Result<Self, Self::Error> {
        Decor::try_from(value.into_iter().map(Cow::Borrowed).collect::<Vec<_>>())
    }
}

impl<'a> TryFrom<Vec<String>> for Decor<'a> {
    type Error = InvalidDecor;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Decor::try_from(value.into_iter().map(Cow::Owned).collect::<Vec<_>>())
    }
}
