//use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

use crate::basic::{Group, KeyValuePair, Locale};
use crate::error::KeyFileError;
use crate::parse::{parse_as_header, parse_as_key_value_pair};

#[derive(Debug)]
pub struct KeyFile<'a> {
    groups: IndexMap<&'a str, Group<'a>>,
    decor: Vec<&'a str>,
}

impl<'a> KeyFile<'a> {
    pub fn parse(value: &'a str) -> Result<Self, KeyFileError> {
        let mut current_group: Option<Group> = None;

        let mut groups: IndexMap<&str, Group> = IndexMap::new();
        let mut decor = Vec::new();

        for (lineno, line) in value.lines().enumerate() {
            // - empty lines are not meaningful
            // - lines that begin with a "#" character are comments
            if line.is_empty() || line.starts_with('#') {
                decor.push(line);

            // attempt to parse line as group header
            } else if let Some(header) = parse_as_header(line) {
                if groups.contains_key(header) {
                    return Err(KeyFileError::duplicate_group(String::from(header), lineno));
                }
                if let Some(collector) = current_group.take() {
                    groups.insert(collector.name, collector);
                    // already checked if there was a previous group with this name
                }
                current_group = Some(Group::new(header, IndexMap::new(), std::mem::take(&mut decor)));

            // attempt to parse line as key-value-pair
            } else if let Some((key, locale, value, wsl, wsr)) = parse_as_key_value_pair(line) {
                if let Some(collector) = &mut current_group {
                    let key_str = if let Some(ref locale) = locale {
                        format!("{}[{}]", key, locale)
                    } else {
                        key.to_string()
                    };

                    let kv = KeyValuePair::new(key, locale, value, wsl, wsr, std::mem::take(&mut decor));
                    if let Some(_previous) = collector.entries.insert((key, locale), kv) {
                        return Err(KeyFileError::duplicate_key(key_str, lineno));
                    }
                }

            // line is invalid if it is neither empty, nor a comment, nor a group header, nor a key-value-pair
            } else {
                return Err(KeyFileError::invalid_line(String::from(line), lineno));
            }
        }

        if let Some(collector) = current_group.take() {
            groups.insert(collector.name, collector);
            // already checked if there was a previous group with this name
        }

        Ok(KeyFile { groups, decor })
    }

    pub fn get<'k>(
        &self,
        group: &'k str,
        key: &'k str,
        locale: Option<Locale<'k>>,
    ) -> Result<Option<&'a str>, KeyFileError> {
        if let Some(group) = self.groups.get(group) {
            Ok(group.entries.get(&(key, locale)).map(|kv| kv.value))
        } else {
            Err(KeyFileError::missing_group(String::from(group)))
        }
    }

    pub fn set<'k: 'a, 'v: 'a>(
        &mut self,
        group: &'k str,
        key: &'k str,
        locale: Option<Locale<'k>>,
        value: &'v str,
        decor: Vec<&'v str>,
    ) -> Result<(), KeyFileError> {
        if let Some(group) = self.groups.get_mut(group) {
            group
                .entries
                .entry((key, locale))
                .and_modify(|v| v.value = value)
                .or_insert(KeyValuePair::new(key, locale, value, "", "", decor));
            Ok(())
        } else {
            Err(KeyFileError::missing_group(String::from(group)))
        }
    }

    pub fn remove<'k>(
        &mut self,
        group: &'k str,
        key: &'k str,
        locale: Option<Locale<'k>>,
    ) -> Result<Option<&'a str>, KeyFileError> {
        if let Some(group) = self.groups.get_mut(group) {
            if !group.entries.contains_key(&(key, locale)) {
                return Ok(None);
            }

            let mut old_value = None;
            let mut new_entries = IndexMap::new();

            for ((ekey, elocale), value) in group.entries.drain(..) {
                if ekey == key && elocale == locale {
                    old_value = Some(value.value);
                } else {
                    new_entries.insert((ekey, elocale), value);
                }
            }

            group.entries = new_entries;
            Ok(old_value)
        } else {
            Err(KeyFileError::missing_group(String::from(group)))
        }
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
