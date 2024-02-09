//! ## Implementations of the data structures for KeyFiles
//!
//! This module contains the definitions of [`KeyFile`], [`KeyValuePair`], [`Group`], and [`Locale`], built on top of
//! the fundamental data types from the [`types`] module.
//!
//! - [`KeyFile`]: a collection of groups that mapping group names to groups of key-value pairs ("entries")
//! - [`KeyValuePair`]: a single key-value pair ("entry")
//! - [`Group`]: a group with a name ("header") and a map of key-value pairs ("entries")
//!
//! The definitions of [`KeyFile`], [`KeyValuePair`] and [`Group`] all include format-preserving representations of
//! whitespace and comments and / or empty lines. Parsing a file, making no modifications, and converting it back into a
//! string should yield a string that is an exact match for the original.

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use thiserror::Error;

use crate::parse::{parse_as_header, parse_as_key_value_pair};
use crate::types::*;

#[cfg(doc)]
use crate::types;

/// Error that is returned when attempting to parse an invalid KeyFile
#[derive(Debug, Error)]
pub enum KeyFileError {
    /// Error variant for syntax errors.
    #[error("Invalid line (line {}): {}", .lineno, .line)]
    #[allow(missing_docs)]
    InvalidLine { line: String, lineno: usize },
    /// Error variant for multiple groups with the same name.
    #[error("Multiple groups with the same name (line {}): {}", .lineno, .name)]
    #[allow(missing_docs)]
    DuplicateGroup { name: String, lineno: usize },
    /// Error variant for multiple keys in the same group with the same name.
    #[error("Multiple key-value pairs with the same key (line {}): {}", .lineno, .key)]
    #[allow(missing_docs)]
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

/// Struct representing a parsed or manually constructed KeyFile
#[derive(Clone, Debug, Default)]
pub struct KeyFile<'a> {
    pub(crate) groups: IndexMap<Cow<'a, str>, Group<'a>>,
    pub(crate) decor: Vec<Cow<'a, str>>,
}

impl<'a> KeyFile<'a> {
    /// Method for creating a new and empty [`KeyFile`].
    pub fn new() -> Self {
        KeyFile {
            groups: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    /// Method for parsing a string into a [`KeyFile`].
    pub fn parse(value: &'a str) -> Result<Self, KeyFileError> {
        let mut current_group: Option<Group> = None;

        let mut groups: IndexMap<Cow<str>, Group> = IndexMap::new();
        let mut decor = Vec::new();

        for (lineno, line) in value.lines().enumerate() {
            // - empty lines are not meaningful
            // - lines that begin with a "#" character are comments
            if line.is_empty() || line.starts_with('#') {
                decor.push(Cow::Borrowed(line));

            // attempt to parse line as group header
            } else if let Some(header) = parse_as_header(line) {
                if groups.contains_key(header) {
                    return Err(KeyFileError::duplicate_group(String::from(header), lineno));
                }
                if let Some(collector) = current_group.take() {
                    // this clone is cheap since collector.name is always a Cow::Borrowed
                    groups.insert(collector.name.clone(), collector);
                    // already checked if there was a previous group with this name
                }
                current_group = Some(Group::from_entries(
                    GroupName::new_unchecked(header.into()),
                    IndexMap::new(),
                    Decor::new_unchecked(std::mem::take(&mut decor)),
                ));

            // attempt to parse line as key-value-pair
            } else if let Some((key, locale, value, wsl, wsr)) = parse_as_key_value_pair(line) {
                if let Some(collector) = &mut current_group {
                    let key_str = if let Some(ref locale) = &locale {
                        format!("{}[{}]", key, locale)
                    } else {
                        key.to_string()
                    };

                    let kv = KeyValuePair::new_with_decor(
                        Key::new_unchecked(key.into()),
                        // this clone is cheap since locale contains only Cow::Borrowed
                        locale.clone(),
                        Value::new_unchecked(value.into()),
                        Whitespace::new_unchecked(wsl.into()),
                        Whitespace::new_unchecked(wsr.into()),
                        Decor::new_unchecked(std::mem::take(&mut decor)),
                    );
                    if let Some(_previous) = collector.entries.insert((key.into(), locale), kv) {
                        return Err(KeyFileError::duplicate_key(key_str, lineno));
                    }
                }

            // line is invalid if it is neither empty, nor a comment, nor a group header, nor a key-value-pair
            } else {
                return Err(KeyFileError::invalid_line(String::from(line), lineno));
            }
        }

        if let Some(collector) = current_group.take() {
            // this clone is cheap since collector.name is always a Cow::Borrowed
            groups.insert(collector.name.clone(), collector);
            // already checked if there was a previous group with this name
        }

        Ok(KeyFile { groups, decor })
    }

    /// Method for converting a [`KeyFile`] that possibly references borrowed data into
    /// a [`KeyFile`] with a `'static` lifetime.
    pub fn into_owned(self) -> KeyFile<'static> {
        let mut owned = KeyFile::new();

        for (_group_name, group) in self.groups {
            owned.insert_group(group.into_owned());
        }

        for line in self.decor {
            owned.decor.push(Cow::Owned(line.into_owned()));
        }

        owned
    }

    /// Method for getting a reference to the [`Group`] with the given name.
    ///
    /// If there is no group with the given name, then [`None`] is returned.
    pub fn get_group(&self, name: &str) -> Option<&Group> {
        self.groups.get(name)
    }

    /// Method for getting a mutable reference to the [`Group`] with the given name.
    ///
    /// If there is no group with the given name, then [`None`] is returned.
    pub fn get_group_mut(&'a mut self, name: &str) -> Option<&mut Group> {
        self.groups.get_mut(name)
    }

    /// Method for inserting a new [`Group`] into the [`KeyFile`].
    ///
    /// The group will be appended as the last group in the [`KeyFile`].
    ///
    /// Inserting a group with the same name as an already existing group will
    /// replace the existing group. In this case, the replaced group is returned.
    pub fn insert_group<'g: 'a>(&mut self, group: Group<'g>) -> Option<Group> {
        // This clone is cheap only if the group.name is a Cow::Borrowed(&str).
        // If group.name is a Cow::Owned(String), the String needs to be copied.
        self.groups.insert(group.name.clone(), group)
    }

    /// Method for removing a [`Group`] with the given name.
    ///
    /// If there is no group with the given name, then [`None`] is returned.
    ///
    /// This operation preserves the order of remaining groups.
    pub fn remove_group(&mut self, name: &str) -> Option<Group> {
        self.groups.shift_remove(name)
    }
}

impl<'a> Display for KeyFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_name, group) in &self.groups {
            write!(f, "{}", group)?;
        }

        for line in &self.decor {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

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
