use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;

pub mod errors;
use errors::*;

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

// =============================================================================

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

// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct KeyValuePair<'a> {
    pub(crate) key: Cow<'a, str>,
    pub(crate) locale: Option<Locale<'a>>,
    pub(crate) value: Cow<'a, str>,
    pub(crate) wsl: Cow<'a, str>,
    pub(crate) wsr: Cow<'a, str>,
    pub(crate) decor: Vec<Cow<'a, str>>,
}

impl<'a> KeyValuePair<'a> {
    pub fn new<'kv: 'a>(key: Key<'kv>, locale: Option<Locale<'kv>>, value: Value<'kv>) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: " ".into(),
            wsr: " ".into(),
            decor: Vec::new(),
        }
    }

    pub fn new_with_decor<'kv: 'a>(
        key: Key<'kv>,
        locale: Option<Locale<'kv>>,
        value: Value<'kv>,
        wsl: Whitespace<'kv>,
        wsr: Whitespace<'kv>,
        decor: Decor<'kv>,
    ) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: wsl.into(),
            wsr: wsr.into(),
            decor: decor.into(),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn set_key<'k: 'a>(&mut self, key: Key<'k>) -> Cow<str> {
        std::mem::replace(&mut self.key, key.into())
    }

    pub fn get_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn set_locale<'l: 'a>(&mut self, locale: Locale<'l>) -> Option<Locale<'a>> {
        std::mem::replace(&mut self.locale, Some(locale))
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value<'v: 'a>(&mut self, value: Value<'v>) -> Cow<str> {
        std::mem::replace(&mut self.value, value.into())
    }

    pub fn set_whitespace<'w: 'a>(&mut self, wsl: Whitespace<'w>, wsr: Whitespace<'w>) -> (Cow<str>, Cow<str>) {
        (
            std::mem::replace(&mut self.wsl, wsl.into()),
            std::mem::replace(&mut self.wsr, wsr.into()),
        )
    }

    pub fn get_decor(&self) -> &[Cow<str>] {
        self.decor.as_slice()
    }

    pub fn set_decor<'d: 'a>(&mut self, decor: Decor<'d>) -> Vec<Cow<str>> {
        std::mem::replace(&mut self.decor, decor.into())
    }
}

impl<'a> Display for KeyValuePair<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.decor {
            writeln!(f, "{}", line)?;
        }

        if let Some(locale) = &self.locale {
            write!(f, "{}[{}]{}={}{}", self.key, locale, self.wsl, self.wsr, self.value)?;
        } else {
            write!(f, "{}{}={}{}", self.key, self.wsl, self.wsr, self.value)?;
        }

        Ok(())
    }
}

// =============================================================================

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locale<'a> {
    pub(crate) lang: Cow<'a, str>,
    pub(crate) country: Option<Cow<'a, str>>,
    pub(crate) encoding: Option<Cow<'a, str>>,
    pub(crate) modifier: Option<Cow<'a, str>>,
}

impl<'a> Locale<'a> {
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

    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    pub fn set_lang<'l: 'a>(&mut self, lang: Language<'l>) -> Cow<str> {
        std::mem::replace(&mut self.lang, lang.into())
    }

    pub fn get_country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    pub fn set_country<'c: 'a>(&mut self, country: Option<Country<'c>>) -> Option<Cow<str>> {
        std::mem::replace(&mut self.country, country.map(Into::into))
    }

    pub fn get_encoding(&self) -> Option<&str> {
        self.encoding.as_deref()
    }

    pub fn get_modifier(&self) -> Option<&str> {
        self.modifier.as_deref()
    }

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

// =============================================================================

#[derive(Clone, Debug)]
pub struct Group<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) entries: IndexMap<(Cow<'a, str>, Option<Locale<'a>>), KeyValuePair<'a>>,
    pub(crate) decor: Vec<Cow<'a, str>>,
}

impl<'a> Group<'a> {
    pub fn new<'e: 'a>(name: GroupName<'e>) -> Self {
        Group {
            name: name.into(),
            entries: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    pub(crate) fn from_entries<'e: 'a>(
        name: GroupName<'e>,
        entries: IndexMap<(Cow<'e, str>, Option<Locale<'e>>), KeyValuePair<'e>>,
        decor: Decor<'e>,
    ) -> Self {
        Group {
            name: name.into(),
            entries,
            decor: decor.into(),
        }
    }

    pub fn get<'k: 'a>(&self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&KeyValuePair> {
        self.entries.get(&(key.into(), locale))
    }

    pub fn get_mut<'k: 'a>(&'a mut self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&mut KeyValuePair> {
        self.entries.get_mut(&(key.into(), locale))
    }

    pub fn insert<'kv: 'a>(&mut self, kv: KeyValuePair<'kv>) -> Option<KeyValuePair> {
        // This clone is cheap only if the kv.key is a Cow::Borrowed(&str).
        // If kv.key is a Cow::Owned(String), the String needs to be copied.
        self.entries.insert((kv.key.clone(), kv.locale.clone()), kv)
    }

    // This method preserves order by calling the order-preserving IndexMap::shift_remove method.
    pub fn remove<'k: 'a>(&mut self, key: &'k str, locale: Option<Locale<'k>>) -> Option<KeyValuePair> {
        self.entries.shift_remove(&(key.into(), locale))
    }
}

impl<'a> Display for Group<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.decor {
            writeln!(f, "{}", line)?;
        }
        writeln!(f, "[{}]", self.name)?;

        for kv in self.entries.values() {
            writeln!(f, "{}", kv)?;
        }

        Ok(())
    }
}
