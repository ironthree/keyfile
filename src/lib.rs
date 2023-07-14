#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]

use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

static HEADER: Lazy<Regex> = Lazy::new(|| {
    // group header:
    // - opening "[",
    // - printable ASCII characters except "[" and "]",
    // - closing "]"
    Regex::new(r"^\[(?<name>[[:print:][^\[\]]]+)\]$").expect("Failed to compile hard-coded regular expression.")
});

// TODO: document that encoding modifier is not supported since only UTF-8-only files are supported
static KEY_VALUE_PAIR: Lazy<Regex> = Lazy::new(|| {
    // key-value pair:
    // - key (only alphanumeric or "-") with optional locale specifier (opening "[", "lang_COUNTRY.ENCODING@MODIFIER",
    //   closing "]"),
    // - optional whitespace,
    // - "=" character,
    // - optional whitespace,
    // - value (printable ASCII or UTF-8)
    Regex::new(r"^(?<key>[[:alnum:]-]+)(?:\[(?<lang>[[:alpha:]]+)(?:_(?<country>[[:alpha:]]+))?(?:@(?<modifier>[[:alpha:]]+))?\])?(?<wsl>[[:blank:]]*)=(?<wsr>[[:blank:]]*)(?<value>.*)$")
        .expect("Failed to compile hard-coded regular expression.")
});

#[derive(Debug, Error)]
pub enum DesktopError<'a> {
    #[error("Invalid line (line {}): {}", .lineno, .line)]
    InvalidLine { line: &'a str, lineno: usize },
    #[error("Multiple groups with the same name (line {}): {}", .lineno, .name)]
    DuplicateGroup { name: &'a str, lineno: usize },
    #[error("Multiple key-value pairs with the same key (line {}): {}", .lineno, .key)]
    DuplicateKey { key: String, lineno: usize },
}

impl<'a> DesktopError<'a> {
    fn invalid_line(line: &'a str, lineno: usize) -> Self {
        DesktopError::InvalidLine { line, lineno }
    }

    fn duplicate_group(name: &'a str, lineno: usize) -> Self {
        DesktopError::DuplicateGroup { name, lineno }
    }

    fn duplicate_key(key: String, lineno: usize) -> Self {
        DesktopError::DuplicateKey { key, lineno }
    }
}

#[derive(Debug, PartialEq)]
struct KeyValuePair<'a> {
    key: &'a str,
    locale: Option<Locale<'a>>,
    value: &'a str,
    wsl: &'a str,
    wsr: &'a str,
    decor: Vec<&'a str>,
}

impl<'a> KeyValuePair<'a> {
    fn new(
        key: &'a str,
        locale: Option<Locale<'a>>,
        value: &'a str,
        wsl: &'a str,
        wsr: &'a str,
        decor: Vec<&'a str>,
    ) -> Self {
        KeyValuePair {
            key,
            locale,
            value,
            wsl,
            wsr,
            decor,
        }
    }
}

impl<'a> Display for KeyValuePair<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.decor {
            writeln!(f, "{}", line)?;
        }

        if let Some(locale) = self.locale {
            write!(f, "{}[{}]{}={}{}", self.key, locale, self.wsl, self.wsr, self.value)?;
        } else {
            write!(f, "{}{}={}{}", self.key, self.wsl, self.wsr, self.value)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Locale<'a> {
    lang: &'a str,
    country: Option<&'a str>,
    // encoding: Option<&'a str>,
    modifier: Option<&'a str>,
}

impl<'a> Locale<'a> {
    fn new(lang: &'a str, country: Option<&'a str>, modifier: Option<&'a str>) -> Self {
        Locale {
            lang,
            country,
            modifier,
        }
    }
}

impl<'a> Display for Locale<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)?;

        if let Some(country) = self.country {
            write!(f, "_{}", country)?;
        }

        if let Some(modifier) = self.modifier {
            write!(f, "@{}", modifier)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Group<'a> {
    name: &'a str,
    entries: IndexMap<(&'a str, Option<Locale<'a>>), KeyValuePair<'a>>,
    decor: Vec<&'a str>,
}

impl<'a> Group<'a> {
    fn new(
        name: &'a str,
        entries: IndexMap<(&'a str, Option<Locale<'a>>), KeyValuePair<'a>>,
        decor: Vec<&'a str>,
    ) -> Self {
        Group { name, entries, decor }
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

#[derive(Debug)]
pub struct ParsedFile<'a> {
    groups: IndexMap<&'a str, Group<'a>>,
    decor: Vec<&'a str>,
}

impl<'a> ParsedFile<'a> {
    pub fn parse(value: &'a str) -> Result<Self, DesktopError> {
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
                    return Err(DesktopError::duplicate_group(header, lineno));
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
                        return Err(DesktopError::duplicate_key(key_str, lineno));
                    }
                }

            // line is invalid if it is neither empty, nor a comment, nor a group header, nor a key-value-pair
            } else {
                return Err(DesktopError::invalid_line(line, lineno));
            }
        }

        if let Some(collector) = current_group.take() {
            groups.insert(collector.name, collector);
            // already checked if there was a previous group with this name
        }

        Ok(ParsedFile { groups, decor })
    }
}

impl<'a> Display for ParsedFile<'a> {
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

fn parse_as_header(line: &str) -> Option<&str> {
    Some(HEADER.captures(line)?.name("name")?.as_str())
}

fn parse_as_key_value_pair(line: &str) -> Option<(&str, Option<Locale>, &str, &str, &str)> {
    let caps = KEY_VALUE_PAIR.captures(line)?;

    // key (compound key: name, optional locale) and value
    let key = caps.name("key")?.as_str();
    let lang = caps.name("lang").map(|m| m.as_str());
    let country = caps.name("country").map(|m| m.as_str());
    let modifier = caps.name("modifier").map(|m| m.as_str());
    let value = caps.name("value")?.as_str();

    // whitespace around the "="
    let wsl = caps.name("wsl")?.as_str();
    let wsr = caps.name("wsr")?.as_str();

    let locale = lang.map(|lang| Locale::new(lang, country, modifier));
    Some((key, locale, value, wsl, wsr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_as_header() {
        assert_eq!(parse_as_header("[hello world]"), Some("hello world"));
        assert_eq!(parse_as_header("[Desktop Entry]"), Some("Desktop Entry"));
        assert_eq!(
            parse_as_header("[Desktop Action new-window]"),
            Some("Desktop Action new-window")
        );
    }

    #[test]
    fn test_parse_key_value_pair() {
        assert_eq!(
            parse_as_key_value_pair("Name=Files").unwrap(),
            ("Name", None, "Files", "", "")
        );
        assert_eq!(
            parse_as_key_value_pair("Name[de] =Dateien").unwrap(),
            ("Name", Some(Locale::new("de", None, None)), "Dateien", " ", ""),
        );
        assert_eq!(
            parse_as_key_value_pair("Name[en_GB] = Files").unwrap(),
            ("Name", Some(Locale::new("en", Some("GB"), None)), "Files", " ", " ")
        );
        assert_eq!(
            parse_as_key_value_pair("Name[sr@latin]= Datoteke").unwrap(),
            (
                "Name",
                Some(Locale::new("sr", None, Some("latin"))),
                "Datoteke",
                "",
                " "
            )
        );
    }
}
