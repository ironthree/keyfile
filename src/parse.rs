use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;
use self_cell::self_cell;

use crate::basic::{Group, KeyValuePair, Locale};
use crate::DesktopError;

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

#[derive(Debug)]
pub(crate) struct ParsedFile<'a> {
    groups: IndexMap<&'a str, Group<'a>>,
    decor: Vec<&'a str>,
}

impl<'a> ParsedFile<'a> {
    fn parse(value: &'a str) -> Result<Self, DesktopError> {
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
                    return Err(DesktopError::duplicate_group(String::from(header), lineno));
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
                return Err(DesktopError::invalid_line(String::from(line), lineno));
            }
        }

        if let Some(collector) = current_group.take() {
            groups.insert(collector.name, collector);
            // already checked if there was a previous group with this name
        }

        Ok(ParsedFile { groups, decor })
    }

    fn get(&self, group: &'a str, key: &'a str, locale: Option<Locale<'a>>) -> Result<Option<&'a str>, DesktopError> {
        if let Some(group) = self.groups.get(group) {
            Ok(group.entries.get(&(key, locale)).map(|kv| kv.value))
        } else {
            Err(DesktopError::missing_group(String::from(group)))
        }
    }

    fn set(
        &mut self,
        group: &'a str,
        key: &'a str,
        locale: Option<Locale<'a>>,
        value: &'a str,
        decor: Vec<&'a str>,
    ) -> Result<(), DesktopError> {
        if let Some(group) = self.groups.get_mut(group) {
            group
                .entries
                .entry((key, locale))
                .and_modify(|v| v.value = value)
                .or_insert(KeyValuePair::new(key, locale, value, "", "", decor));
            Ok(())
        } else {
            Err(DesktopError::missing_group(String::from(group)))
        }
    }

    fn remove(
        &mut self,
        group: &'a str,
        key: &'a str,
        locale: Option<Locale<'a>>,
    ) -> Result<Option<&'a str>, DesktopError> {
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
            Err(DesktopError::missing_group(String::from(group)))
        }
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

self_cell! {
    pub(crate) struct ParsedFileWrapper<'a> {
        owner: Cow<'a, str>,
        #[covariant]
        dependent: ParsedFile,
    }
}

impl<'a> Debug for ParsedFileWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParsedFileWrapper")
            .field("owner", self.borrow_owner())
            .field("dependent", self.borrow_dependent())
            .finish()
    }
}

impl<'a> ParsedFileWrapper<'a> {
    pub(crate) fn from_contents(contents: Cow<'a, str>) -> Result<Self, DesktopError> {
        ParsedFileWrapper::try_new(contents, |owner| ParsedFile::parse(owner))
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
