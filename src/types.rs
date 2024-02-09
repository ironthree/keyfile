//! ## Implementations of the building blocks of KeyFiles
//!
//! This module contains the definitions of [`KeyValuePair`], [`Group`], and [`Locale`], built on top of the fundamental
//! data types from the [`basic`] module.
//!
//! - [`KeyValuePair`]: key-value pair ("entry")
//! - [`Group`]: group name ("header") and collection of key-value pairs ("entries")
//! - [`Locale`]: key modifier in key-value pairs for translated values
//!
//! The definitions of both [`KeyValuePair`] and [`Group`] include format-preserving representations of whitespace and
//! preceding comments and / or empty lines.

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

pub mod basic;
pub mod errors;

use basic::*;

/// ## Key-value pair and its associated data
///
/// This includes format-preserving representations of preceding comment lines, empty lines,
/// and whitespace around the `=` separator character.
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
    /// Method to construct a new plain key-value pair
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

    /// Method to construct a new key-value pair with explicitly set whitespace and decor
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

    /// Method for converting a [`KeyValuePair`] that possibly references borrowed data into
    /// a [`KeyValuePair`] with a `'static` lifetime
    pub fn into_owned(self) -> KeyValuePair<'static> {
        let owned_decor = self.decor.into_iter().map(|c| Cow::Owned(c.into_owned())).collect();

        KeyValuePair {
            key: Cow::Owned(self.key.into_owned()),
            locale: self.locale.map(Locale::into_owned),
            value: Cow::Owned(self.value.into_owned()),
            wsl: Cow::Owned(self.wsl.into_owned()),
            wsr: Cow::Owned(self.wsr.into_owned()),
            decor: owned_decor,
        }
    }

    /// Method for getting the key string
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Method for setting the key string
    ///
    /// The replaced key string is returned.
    pub fn set_key<'k: 'a>(&mut self, key: Key<'k>) -> Cow<str> {
        std::mem::replace(&mut self.key, key.into())
    }

    /// Method for getting the optional locale string
    pub fn get_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    /// Method for setting the optional locale string
    ///
    /// If this method replaces an existing locale string, it is returned.
    pub fn set_locale<'l: 'a>(&mut self, locale: Locale<'l>) -> Option<Locale<'a>> {
        std::mem::replace(&mut self.locale, Some(locale))
    }

    /// Method for getting the value string
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Method for setting the value string
    ///
    /// The replaced value string is returned.
    pub fn set_value<'v: 'a>(&mut self, value: Value<'v>) -> Cow<str> {
        std::mem::replace(&mut self.value, value.into())
    }

    /// Method for getting the whitespace surrounding the `=` separator
    pub fn get_whitespace(&self) -> (&str, &str) {
        (&self.wsl, &self.wsr)
    }

    /// Method for setting the whitespace surrounding the `=` separator
    ///
    /// The replaced strings are returned.
    pub fn set_whitespace<'w: 'a>(&mut self, wsl: Whitespace<'w>, wsr: Whitespace<'w>) -> (Cow<str>, Cow<str>) {
        (
            std::mem::replace(&mut self.wsl, wsl.into()),
            std::mem::replace(&mut self.wsr, wsr.into()),
        )
    }

    /// Method for getting the comment lines / empty lines preceding the [`KeyValuePair`]
    pub fn get_decor(&self) -> &[Cow<str>] {
        self.decor.as_slice()
    }

    /// Method for setting the comment lines / empty lines preceding the [`KeyValuePair`]
    ///
    /// The replaced strings are returned.
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

/// ## Locale identifier
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locale<'a> {
    pub(crate) lang: Cow<'a, str>,
    pub(crate) country: Option<Cow<'a, str>>,
    pub(crate) encoding: Option<Cow<'a, str>>,
    pub(crate) modifier: Option<Cow<'a, str>>,
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

    /// Method for converting a [`Locale`] that possibly references borrowed data into
    /// a [`Locale`] with a `'static` lifetime
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

    /// Method for setting the language identifier
    ///
    /// The replaced string is returned.
    pub fn set_lang<'l: 'a>(&mut self, lang: Language<'l>) -> Cow<str> {
        std::mem::replace(&mut self.lang, lang.into())
    }

    /// Method for getting the country / territory identifier
    pub fn get_country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    /// Method for getting the country / territory identifier
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

    /// Method for setting the locale modifier
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

/// ## Named group of key-value pairs and its associated data
///
/// This includes a format-preserving representation of preceding comment lines and empty lines.
#[derive(Clone, Debug)]
pub struct Group<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) entries: IndexMap<(Cow<'a, str>, Option<Locale<'a>>), KeyValuePair<'a>>,
    pub(crate) decor: Vec<Cow<'a, str>>,
}

impl<'a> Group<'a> {
    /// Method for creating a new and empty [`Group`]
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

    /// Method for converting a [`Group`] that possibly references borrowed data into
    /// a [`Group`] with a `'static` lifetime
    pub fn into_owned(self) -> Group<'static> {
        let owned_name: Cow<'static, str> = Cow::Owned(self.name.into_owned());

        let mut owned = Group::new(GroupName::new_unchecked(owned_name.clone()));

        for (_key, kv) in self.entries {
            owned.insert(kv.into_owned());
        }

        for line in self.decor {
            owned.decor.push(Cow::Owned(line.into_owned()));
        }

        owned
    }

    /// Method for getting a reference to the [`KeyValuePair`] associated with the given key
    ///
    /// If there is no key-value pair associated with the given key, then [`None`] is returned.
    pub fn get<'k: 'a>(&self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&KeyValuePair> {
        self.entries.get(&(key.into(), locale))
    }

    /// Method for getting a mutable reference to the [`KeyValuePair`] associated with the given key
    ///
    /// If there is no key-value pair associated with the given key, then [`None`] is returned.
    pub fn get_mut<'k: 'a>(&'a mut self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&mut KeyValuePair> {
        self.entries.get_mut(&(key.into(), locale))
    }

    /// Method for inserting a new [`KeyValuePair`] into the [`Group`]
    ///
    /// The key-value pair will be appended as the last entry in the [`Group`].
    ///
    /// Inserting a key-value pair with the same key as an already existing key-value pair will
    /// replace the existing key-value pair. In this case, the replaced value is returned.
    pub fn insert<'kv: 'a>(&mut self, kv: KeyValuePair<'kv>) -> Option<KeyValuePair> {
        // This clone is cheap only if the kv.key is a Cow::Borrowed(&str).
        // If kv.key is a Cow::Owned(String), the String needs to be copied.
        self.entries.insert((kv.key.clone(), kv.locale.clone()), kv)
    }

    /// Method for removing a [`KeyValuePair`] associated with the given key
    ///
    /// If there is no key-value pair associated with the given key, then [`None`] is returned.
    ///
    /// This operation preserves the order of remaining key-value pairs.
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
