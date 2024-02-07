use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

use super::errors::*;

static GROUPNAME: Lazy<Regex> = Lazy::new(|| {
    // keep in sync with src/parse.rs:HEADER
    Regex::new(r"^[[:print:]&&[^\[\]]]+$").expect("Failed to compile hard-coded regular expression.")
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

/// Newtype struct wrapping strings that are valid group names.
///
/// New instances of `GroupName` can only be created from strings that are valid group names:
///
/// ```
/// use keyfile::types::basic::GroupName;
///
/// let group = GroupName::try_from("hello").unwrap();
/// let error = GroupName::try_from("[[[[[").unwrap_err();
/// ```
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::GroupName;
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

/// Newtype struct wrapping strings that are valid keys.
///
/// New instances of `Key` can only be created from strings that are valid key names:
///
/// ```
/// use keyfile::types::basic::Key;
///
/// let key = Key::try_from("hello").unwrap();
/// let error = Key::try_from("//!!/").unwrap_err();
/// ```
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Key;
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

/// Newtype struct wrapping strings that are valid language identifiers.
///
/// New instances of `Language` can only be created from strings that are valid POSIX locale language identifiers:
///
/// ```
/// use keyfile::types::basic::Language;
///
/// let lang = Language::try_from("de").unwrap();
/// let error = Language::try_from("42").unwrap_err();
/// ```
///
/// The inner string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Language;
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

/// Newtype struct wrapping strings that are valid country identifiers.
///
/// New instances of `Country` can only be created from strings that are valid POSIX locale country / terretory
/// identifiers:
///
/// ```
/// use keyfile::types::basic::Country;
///
/// let country = Country::try_from("DE").unwrap();
/// let error = Country::try_from("!!").unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Country;
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

/// Newtype struct wrapping strings that are valid encoding identifiers.
///
/// New instances of `Encoding` can only be created from strings that are valid POSIX locale encoding identifiers:
///
/// ```
/// use keyfile::types::basic::Encoding;
///
/// let encoding = Encoding::try_from("utf8").unwrap();
/// let error = Encoding::try_from("morse!").unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Encoding;
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
    type Error = InvalidEncoding;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !ENCODING.is_match(&value) {
            return Err(InvalidEncoding);
        }

        Ok(Encoding { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Encoding<'a> {
    type Error = InvalidEncoding;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Encoding::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Encoding<'a> {
    type Error = InvalidEncoding;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Encoding::try_from(Cow::Owned(value))
    }
}

/// Newtype struct wrapping strings that are valid locale modifiers.
///
/// New instances of `Encoding` can only be created from strings that are valid POSIX locale modifiers:
///
/// ```
/// use keyfile::types::basic::Modifier;
///
/// let modifier = Modifier::try_from("latin").unwrap();
/// let error = Modifier::try_from("!foo!").unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Modifier;
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
    type Error = InvalidModifier;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        if !MODIFIER.is_match(&value) {
            return Err(InvalidModifier);
        }

        Ok(Modifier { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Modifier<'a> {
    type Error = InvalidModifier;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Modifier::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Modifier<'a> {
    type Error = InvalidModifier;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Modifier::try_from(Cow::Owned(value))
    }
}

/// Newtype struct wrapping strings that are valid values.
///
/// New instances of `Value` can only be created from strings that are valid value strings:
///
/// ```
/// use keyfile::types::basic::Value;
///
/// let value = Value::try_from("WORLD").unwrap();
/// let error = Value::try_from("new\nline").unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Value;
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

/// Newtype struct wrapping strings that are valid whitespace.
///
/// New instances of `Whitespace` can only be created from strings that are valid whitespace
/// (i.e. spaces / tabs surrounding the `=` character in a key-value pair):
///
/// ```
/// use keyfile::types::basic::Whitespace;
///
/// let whitespace = Whitespace::try_from(" ").unwrap();
/// let error = Whitespace::try_from("HELLO").unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Whitespace;
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

/// Newtype struct wrapping strings that are valid comments and / or empty lines.
///
/// New instances of `Decor` can only be created from strings that are valid comment lines
/// (a `#` character followed by an arbitrary string of UTF-8) or empty lines:
///
/// ```
/// use keyfile::types::basic::Decor;
///
/// let decor = Decor::try_from(vec!["# This is a comment"]).unwrap();
/// let error = Decor::try_from(vec!["This=is not a comment"]).unwrap_err();
/// ```
///
/// The contained string can always be obtained by using the [`From::from`] method:
///
/// ```
/// use keyfile::types::basic::Decor;
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
