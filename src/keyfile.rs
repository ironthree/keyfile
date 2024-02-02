use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

use crate::basic::{Group, KeyValuePair};
use crate::error::KeyFileError;
use crate::parse::{parse_as_header, parse_as_key_value_pair};
use crate::validate::{Key, Value, Whitespace};

#[derive(Clone, Debug, Default)]
pub struct KeyFile<'a> {
    pub(crate) groups: IndexMap<Cow<'a, str>, Group<'a>>,
    pub(crate) decor: Vec<&'a str>,
}

impl<'a> KeyFile<'a> {
    pub fn new() -> Self {
        KeyFile {
            groups: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    pub fn parse(value: &'a str) -> Result<Self, KeyFileError> {
        let mut current_group: Option<Group> = None;

        let mut groups: IndexMap<Cow<str>, Group> = IndexMap::new();
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
                    // this clone is cheap since collector.name is always a Cow::Borrowed
                    groups.insert(collector.name.clone(), collector);
                    // already checked if there was a previous group with this name
                }
                current_group = Some(Group::from_entries_borrowed(
                    header,
                    IndexMap::new(),
                    std::mem::take(&mut decor),
                ));

            // attempt to parse line as key-value-pair
            } else if let Some((key, locale, value, wsl, wsr)) = parse_as_key_value_pair(line) {
                if let Some(collector) = &mut current_group {
                    let key_str = if let Some(ref locale) = &locale {
                        format!("{}[{}]", key, locale)
                    } else {
                        key.to_string()
                    };

                    let kv = KeyValuePair::new_with_decor_borrowed(
                        Key::new_unchecked(key.into()),
                        // this clone is cheap since locale contains only Cow::Borrowed
                        locale.clone(),
                        Value::new_unchecked(value.into()),
                        Whitespace::new_unchecked(wsl.into()),
                        Whitespace::new_unchecked(wsr.into()),
                        std::mem::take(&mut decor),
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

    pub fn get_group(&self, name: &str) -> Option<&Group> {
        self.groups.get(name)
    }

    pub fn get_group_mut(&'a mut self, name: &str) -> Option<&mut Group> {
        self.groups.get_mut(name)
    }

    pub fn insert_group<'g: 'a>(&mut self, group: Group<'g>) -> Option<Group> {
        // This clone is cheap only if the group.name is a Cow::Borrowed(&str).
        // If group.name is a Cow::Owned(String), the String needs to be copied.
        self.groups.insert(group.name.clone(), group)
    }

    // This method preserves order by calling the order-preserving IndexMap::shift_remove method.
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
