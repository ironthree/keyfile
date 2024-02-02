use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

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
    // TODO: validate input
    pub fn new(key: String, locale: Option<Locale<'static>>, value: String) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: " ".into(),
            wsr: " ".into(),
            decor: Vec::new(),
        }
    }

    // TODO: validate input
    pub fn new_borrowed<'kv: 'a>(key: &'kv str, locale: Option<Locale<'kv>>, value: &'kv str) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: " ".into(),
            wsr: " ".into(),
            decor: Vec::new(),
        }
    }

    // TODO: validate input
    pub fn new_with_decor(
        key: String,
        locale: Option<Locale<'static>>,
        value: String,
        wsl: String,
        wsr: String,
        decor: Vec<String>,
    ) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: wsl.into(),
            wsr: wsr.into(),
            decor: decor.into_iter().map(Cow::Owned).collect(),
        }
    }

    // TODO: validate input
    pub fn new_with_decor_borrowed<'kv: 'a>(
        key: &'kv str,
        locale: Option<Locale<'kv>>,
        value: &'kv str,
        wsl: &'kv str,
        wsr: &'kv str,
        decor: Vec<&'kv str>,
    ) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: wsl.into(),
            wsr: wsr.into(),
            decor: decor.iter().map(|s| Cow::Borrowed(*s)).collect(),
        }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    // TODO: validate input
    pub fn set_key(&mut self, key: String) -> Cow<str> {
        let new = Cow::Owned(key);
        std::mem::replace(&mut self.key, new)
    }

    pub fn set_key_borrowed<'k: 'a>(&mut self, key: &'k str) -> Cow<str> {
        let new = Cow::Borrowed(key);
        std::mem::replace(&mut self.key, new)
    }

    pub fn get_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn set_locale(&mut self, locale: Locale<'static>) -> Option<Locale<'a>> {
        let new = Some(locale);
        std::mem::replace(&mut self.locale, new)
    }

    pub fn set_locale_borrowed<'l: 'a>(&mut self, locale: Locale<'l>) -> Option<Locale<'a>> {
        let new = Some(locale);
        std::mem::replace(&mut self.locale, new)
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    // TODO: validate input
    pub fn set_value(&mut self, value: String) -> Cow<str> {
        let new = Cow::Owned(value);
        std::mem::replace(&mut self.value, new)
    }

    // TODO: validate input
    pub fn set_value_borrowed<'v: 'a>(&mut self, value: &'v str) -> Cow<str> {
        let new = Cow::Borrowed(value);
        std::mem::replace(&mut self.key, new)
    }

    // TODO: validate input
    pub fn set_whitespace(&mut self, wsl: String, wsr: String) -> (Cow<str>, Cow<str>) {
        let new_wsl = Cow::Owned(wsl);
        let new_wsr = Cow::Owned(wsr);
        (
            std::mem::replace(&mut self.wsl, new_wsl),
            std::mem::replace(&mut self.wsr, new_wsr),
        )
    }

    // TODO: validate input
    pub fn set_whitespace_borrowed<'w: 'a>(&mut self, wsl: &'w str, wsr: &'w str) -> (Cow<str>, Cow<str>) {
        let new_wsl = Cow::Borrowed(wsl);
        let new_wsr = Cow::Borrowed(wsr);
        (
            std::mem::replace(&mut self.wsl, new_wsl),
            std::mem::replace(&mut self.wsr, new_wsr),
        )
    }

    pub fn get_decor(&self) -> &[Cow<str>] {
        self.decor.as_slice()
    }

    pub fn set_decor(&mut self, decor: Vec<String>) -> Vec<Cow<str>> {
        let new = decor.into_iter().map(Cow::Owned).collect();
        std::mem::replace(&mut self.decor, new)
    }

    pub fn set_decor_borrowed<'d: 'a>(&mut self, decor: Vec<&'d str>) -> Vec<Cow<str>> {
        let new = decor.iter().map(|s| Cow::Borrowed(*s)).collect();
        std::mem::replace(&mut self.decor, new)
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locale<'a> {
    pub(crate) lang: Cow<'a, str>,
    pub(crate) country: Option<Cow<'a, str>>,
    // encoding: Option<&'a str>,
    pub(crate) modifier: Option<Cow<'a, str>>,
}

impl<'a> Locale<'a> {
    // TODO: validate input
    pub fn new(lang: String, country: Option<String>, modifier: Option<String>) -> Self {
        Locale {
            lang: lang.into(),
            country: country.map(Cow::Owned),
            modifier: modifier.map(Cow::Owned),
        }
    }

    // TODO: validate input
    pub fn new_borrowed<'l: 'a>(lang: &'l str, country: Option<&'l str>, modifier: Option<&'l str>) -> Self {
        Locale {
            lang: lang.into(),
            country: country.map(Cow::Borrowed),
            modifier: modifier.map(Cow::Borrowed),
        }
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

#[derive(Clone, Debug)]
pub struct Group<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) entries: IndexMap<(Cow<'a, str>, Option<Locale<'a>>), KeyValuePair<'a>>,
    pub(crate) decor: Vec<Cow<'a, str>>,
}

impl<'a> Group<'a> {
    // TODO: validate input
    pub fn new(name: String) -> Self {
        Group {
            name: name.into(),
            entries: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    // TODO: validate input
    pub fn new_borrowed<'e: 'a>(name: &'e str) -> Self {
        Group {
            name: name.into(),
            entries: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    #[allow(unused)]
    pub(crate) fn from_entries(
        name: String,
        entries: IndexMap<(Cow<'static, str>, Option<Locale<'static>>), KeyValuePair<'static>>,
        decor: Vec<String>,
    ) -> Self {
        Group {
            name: name.into(),
            entries,
            decor: decor.into_iter().map(Cow::Owned).collect(),
        }
    }

    pub(crate) fn from_entries_borrowed<'e: 'a>(
        name: &'e str,
        entries: IndexMap<(Cow<'e, str>, Option<Locale<'e>>), KeyValuePair<'e>>,
        decor: Vec<&'e str>,
    ) -> Self {
        Group {
            name: name.into(),
            entries,
            decor: decor.iter().map(|s| Cow::Borrowed(*s)).collect(),
        }
    }

    pub fn get<'k: 'a>(&self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&KeyValuePair> {
        self.entries.get(&(key.into(), locale))
    }

    pub fn get_mut<'k: 'a>(&'a mut self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&mut KeyValuePair> {
        self.entries.get_mut(&(key.into(), locale))
    }

    pub fn insert<'kv: 'a>(&mut self, kv: KeyValuePair<'kv>) -> Option<KeyValuePair> {
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
