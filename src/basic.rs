use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

#[derive(Debug, PartialEq)]
pub struct KeyValuePair<'a> {
    pub(crate) key: &'a str,
    pub(crate) locale: Option<Locale<'a>>,
    pub(crate) value: &'a str,
    pub(crate) wsl: &'a str,
    pub(crate) wsr: &'a str,
    pub(crate) decor: Vec<&'a str>,
}

impl<'a> KeyValuePair<'a> {
    pub fn new(
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
pub struct Locale<'a> {
    pub(crate) lang: &'a str,
    pub(crate) country: Option<&'a str>,
    // encoding: Option<&'a str>,
    pub(crate) modifier: Option<&'a str>,
}

impl<'a> Locale<'a> {
    pub fn new(lang: &'a str, country: Option<&'a str>, modifier: Option<&'a str>) -> Self {
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
pub struct Group<'a> {
    pub(crate) name: &'a str,
    pub(crate) entries: IndexMap<(&'a str, Option<Locale<'a>>), KeyValuePair<'a>>,
    pub(crate) decor: Vec<&'a str>,
}

impl<'a> Group<'a> {
    pub fn new(
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
