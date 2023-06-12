use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::path::Path;
use std::str::Lines;

use once_cell::sync::Lazy;
use regex::Regex;
use self_cell::self_cell;
//use thiserror::Error;

static HEADER: Lazy<Regex> = Lazy::new(|| {
    // group header:
    // - opening "[",
    // - printable ASCII characters except "[" and "]",
    // - closing "]"
    Regex::new(r"^\[(?<name>[[:print:][^\[\]]]+)\]$").unwrap()
});

// TODO: document that encoding modifier is not supported since only UTF-8-only files are supported
static KEY_VALUE_PAIR: Lazy<Regex> = Lazy::new(|| {
    // key-value pair:
    // - key (only alphanumeric or "-") with optional locale specifier (opening "[",
    //   "lang_COUNTRY.ENCODING@MODIFIER", closing "]"),
    // - optional whitespace,
    // - "=" character,
    // - optional whitespace,
    // - value (printable ASCII or UTF-8)
    Regex::new(r"^(?<key>[[:alnum:]-]+)(?:\[(?<lang>[[:alpha:]]+)(?:_(?<country>[[:alpha:]]+))?(?:@(?<modifier>[[:alpha:]]+))?\])?[[:blank:]]*=[[:blank:]]*(?<value>.*)$").unwrap()
});

#[derive(Debug, PartialEq)]
struct KeyValuePair<'a> {
    key: &'a str,
    locale: Option<Locale<'a>>,
    value: &'a str,
}

impl<'a> KeyValuePair<'a> {
    fn new(key: &'a str, locale: Option<Locale<'a>>, value: &'a str) -> Self {
        KeyValuePair { key, locale, value }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Locale<'a> {
    lang: &'a str,
    country: Option<&'a str>,
    // encoding: Option<&'a str>,
    modifier: Option<&'a str>,
}

impl<'a> Display for Locale<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.country, self.modifier) {
            (Some(country), Some(modifier)) => write!(f, "{}_{}@{}", self.lang, country, modifier),
            (Some(country), None) => write!(f, "{}_{}", self.lang, country),
            (None, Some(modifier)) => write!(f, "{}@{}", self.lang, modifier),
            (None, None) => write!(f, "{}", self.lang),
        }
    }
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

#[derive(Debug)]
struct Group<'a> {
    entries: BTreeMap<(&'a str, Option<Locale<'a>>), KeyValuePair<'a>>,
}

impl<'a> Group<'a> {
    fn from_entries(entries: BTreeMap<(&'a str, Option<Locale<'a>>), KeyValuePair<'a>>) -> Self {
        Group { entries }
    }
}

#[derive(Debug)]
pub struct ParsedFile<'a> {
    lines: Vec<Cow<'a, str>>,
    groups: BTreeMap<&'a str, Group<'a>>,
}

impl<'a> ParsedFile<'a> {
    fn from_contents(contents: &'a str) -> Self {
        let lines: Lines<'_> = contents.lines();
        let groups = parse_contents(lines.clone());

        ParsedFile {
            lines: lines.map(Cow::Borrowed).collect(),
            groups,
        }
    }
}

self_cell!(
    pub struct DesktopFile {
        owner: String,
        #[covariant]
        dependent: ParsedFile,
    }
);

impl Debug for DesktopFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.borrow_dependent().groups).finish()
    }
}

impl DesktopFile {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let contents = std::fs::read_to_string(path.as_ref()).unwrap();
        DesktopFile::new(contents, |s| ParsedFile::from_contents(s.as_str()))
    }
}

fn parse_contents<'a>(lines: Lines<'a>) -> BTreeMap<&'a str, Group<'a>> {
    let mut current_group: Option<(&str, BTreeMap<(&str, Option<Locale<'a>>), KeyValuePair<'a>>)> = None;

    let mut groups: BTreeMap<&str, Group> = BTreeMap::new();
    for (lineno, line) in lines.enumerate() {
        // empty lines are ignored
        if line.is_empty() {
            continue;
        // lines that begin with a "#" character are comments and are ignored
        } else if line.starts_with("#") {
            continue;
        // lines that start with "[" are either group headers or invalid
        } else if line.starts_with("[") {
            if let Some(header) = parse_as_header(line) {
                if let Some((name, entries)) = current_group.take() {
                    let existing = groups.insert(name, Group::from_entries(entries));

                    if let Some(_) = existing {
                        panic!("Duplicate group: {} (line {})", name, lineno);
                    }
                } else {
                    current_group = Some((header, BTreeMap::new()));
                }
            } else {
                panic!("Invalid line: {} (line {})", line, lineno);
            }
        // non-empty lines that start with neither "#" nor "[" are either key-value-pairs or invalid
        } else {
            if let Some(kv) = parse_as_key_value_pair(line) {
                if let Some((_name, ref mut entries)) = current_group {
                    let key_str = if let Some(ref locale) = kv.locale {
                        format!("{}[{}]", kv.key, locale)
                    } else {
                        kv.key.to_string()
                    };

                    if let Some(_previous) = entries.insert((kv.key, kv.locale), kv) {
                        panic!("Duplicate key-value pair: {} (line {})", key_str, lineno);
                    }
                }
            } else {
                panic!("Invalid line: {} (line {})", line, lineno);
            }
        }
    }

    groups
}

fn parse_as_header(line: &str) -> Option<&str> {
    Some(HEADER.captures(line)?.name("name")?.as_str())
}

fn parse_as_key_value_pair(line: &str) -> Option<KeyValuePair> {
    let caps = KEY_VALUE_PAIR.captures(line)?;

    let key = caps.name("key")?.as_str();
    let lang = caps.name("lang").map(|m| m.as_str());
    let country = caps.name("country").map(|m| m.as_str());
    let modifier = caps.name("modifier").map(|m| m.as_str());
    let value = caps.name("value")?.as_str();

    let locale = lang.map(|lang| Locale::new(lang, country, modifier));
    Some(KeyValuePair::new(key, locale, value))
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
            KeyValuePair::new("Name", None, "Files")
        );
        assert_eq!(
            parse_as_key_value_pair("Name[de]=Dateien").unwrap(),
            KeyValuePair::new("Name", Some(Locale::new("de", None, None)), "Dateien"),
        );
        assert_eq!(
            parse_as_key_value_pair("Name[en_GB]=Files").unwrap(),
            KeyValuePair::new("Name", Some(Locale::new("en", Some("GB"), None)), "Files")
        );
        assert_eq!(
            parse_as_key_value_pair("Name[sr@latin]=Datoteke").unwrap(),
            KeyValuePair::new("Name", Some(Locale::new("sr", None, Some("latin"))), "Datoteke")
        );
    }
}
