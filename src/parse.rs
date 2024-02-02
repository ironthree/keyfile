use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::basic::Locale;
use crate::validate::{Country, Encoding, Language, Modifier};

static HEADER: Lazy<Regex> = Lazy::new(|| {
    // group header:
    // - opening "[",
    // - printable ASCII characters except "[" and "]",
    // - closing "]"
    Regex::new(r"^\[(?<name>[[:print:][^\[\]]]+)\]$").expect("Failed to compile hard-coded regular expression.")
});

static KEY_VALUE_PAIR: Lazy<Regex> = Lazy::new(|| {
    // key-value pair:
    // - key (only alphanumeric or "-") with optional locale specifier,
    // - optional whitespace,
    // - "=" character,
    // - optional whitespace,
    // - value (printable ASCII or UTF-8)
    //
    // locale specifier:
    // - opening "[",
    // - "<lang><_COUNTRY><.ENCODING><@MODIFIER>" (with all components except <lang> being optional),
    // - closing "]"
    Regex::new(r"^(?<key>[[:alnum:]-]+)(?:\[(?<lang>[[:alpha:]]+)(?:_(?<country>[[:alpha:]]+))?(?:\.(?<encoding>[[:alnum:]-]+))?(?:@(?<modifier>[[:alpha:]]+))?\])?(?<wsl>[[:blank:]]*)=(?<wsr>[[:blank:]]*)(?<value>.*)$")
        .expect("Failed to compile hard-coded regular expression.")
});

pub fn parse_as_header(line: &str) -> Option<&str> {
    Some(HEADER.captures(line)?.name("name")?.as_str())
}

pub fn parse_as_key_value_pair(line: &str) -> Option<(&str, Option<Locale>, &str, &str, &str)> {
    let caps = KEY_VALUE_PAIR.captures(line)?;

    // key (compound key: name, optional locale) and value
    let key = caps.name("key")?.as_str();
    let lang = caps.name("lang").map(|m| m.as_str());
    let country = caps.name("country").map(|m| m.as_str());
    let encoding = caps.name("encoding").map(|m| m.as_str());
    let modifier = caps.name("modifier").map(|m| m.as_str());
    let value = caps.name("value")?.as_str();

    // whitespace around the "="
    let wsl = caps.name("wsl")?.as_str();
    let wsr = caps.name("wsr")?.as_str();

    let locale = lang.map(|lang| {
        Locale::new_with_encoding(
            Language::new_unchecked(Cow::Borrowed(lang)),
            country.map(|c| Country::new_unchecked(Cow::Borrowed(c))),
            encoding.map(|e| Encoding::new_unchecked(Cow::Borrowed(e))),
            modifier.map(|m| Modifier::new_unchecked(Cow::Borrowed(m))),
        )
    });
    Some((key, locale, value, wsl, wsr))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

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
            (
                "Name",
                Some(Locale::new("de".try_into().unwrap(), None, None)),
                "Dateien",
                " ",
                ""
            ),
        );
        assert_eq!(
            parse_as_key_value_pair("Name[en_GB] = Files").unwrap(),
            (
                "Name",
                Some(Locale::new(
                    "en".try_into().unwrap(),
                    Some("GB".try_into().unwrap()),
                    None
                )),
                "Files",
                " ",
                " "
            )
        );
        assert_eq!(
            parse_as_key_value_pair("Name[sr@latin]= Datoteke").unwrap(),
            (
                "Name",
                Some(Locale::new(
                    "sr".try_into().unwrap(),
                    None,
                    Some("latin".try_into().unwrap())
                )),
                "Datoteke",
                "",
                " "
            )
        );
    }
}
