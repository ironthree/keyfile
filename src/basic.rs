use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;

use crate::validate::{Decor, GroupName, Key, Value, Whitespace};

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
    pub fn new(key: Key<'static>, locale: Option<Locale<'static>>, value: Value<'static>) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: " ".into(),
            wsr: " ".into(),
            decor: Vec::new(),
        }
    }

    pub fn new_borrowed<'kv: 'a>(key: Key<'kv>, locale: Option<Locale<'kv>>, value: Value<'kv>) -> Self {
        KeyValuePair {
            key: key.into(),
            locale,
            value: value.into(),
            wsl: " ".into(),
            wsr: " ".into(),
            decor: Vec::new(),
        }
    }

    pub fn new_with_decor(
        key: Key<'static>,
        locale: Option<Locale<'static>>,
        value: Value<'static>,
        wsl: Whitespace<'static>,
        wsr: Whitespace<'static>,
        decor: Decor<'static>,
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

    pub fn new_with_decor_borrowed<'kv: 'a>(
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

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn set_key(&mut self, key: Key<'static>) -> Cow<str> {
        std::mem::replace(&mut self.key, key.into())
    }

    pub fn set_key_borrowed<'k: 'a>(&mut self, key: Key<'k>) -> Cow<str> {
        std::mem::replace(&mut self.key, key.into())
    }

    pub fn get_locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn set_locale(&mut self, locale: Locale<'static>) -> Option<Locale<'a>> {
        std::mem::replace(&mut self.locale, Some(locale))
    }

    pub fn set_locale_borrowed<'l: 'a>(&mut self, locale: Locale<'l>) -> Option<Locale<'a>> {
        std::mem::replace(&mut self.locale, Some(locale))
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: Value<'static>) -> Cow<str> {
        std::mem::replace(&mut self.value, value.into())
    }

    pub fn set_value_borrowed<'v: 'a>(&mut self, value: Value<'v>) -> Cow<str> {
        std::mem::replace(&mut self.key, value.into())
    }

    pub fn set_whitespace(&mut self, wsl: Whitespace<'static>, wsr: Whitespace<'static>) -> (Cow<str>, Cow<str>) {
        (
            std::mem::replace(&mut self.wsl, wsl.into()),
            std::mem::replace(&mut self.wsr, wsr.into()),
        )
    }

    pub fn set_whitespace_borrowed<'w: 'a>(
        &mut self,
        wsl: Whitespace<'w>,
        wsr: Whitespace<'w>,
    ) -> (Cow<str>, Cow<str>) {
        (
            std::mem::replace(&mut self.wsl, wsl.into()),
            std::mem::replace(&mut self.wsr, wsr.into()),
        )
    }

    pub fn get_decor(&self) -> &[Cow<str>] {
        self.decor.as_slice()
    }

    pub fn set_decor(&mut self, decor: Decor<'static>) -> Vec<Cow<str>> {
        std::mem::replace(&mut self.decor, decor.into())
    }

    pub fn set_decor_borrowed<'d: 'a>(&mut self, decor: Decor<'d>) -> Vec<Cow<str>> {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locale<'a> {
    pub(crate) lang: Cow<'a, str>,
    pub(crate) country: Option<Cow<'a, str>>,
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

    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    // TODO: validate input
    pub fn set_lang(&mut self, lang: String) -> Cow<str> {
        let new = Cow::Owned(lang);
        std::mem::replace(&mut self.lang, new)
    }

    // TODO: validate input
    pub fn set_lang_borrowed<'l: 'a>(&mut self, lang: &'l str) -> Cow<str> {
        let new = Cow::Borrowed(lang);
        std::mem::replace(&mut self.lang, new)
    }

    pub fn get_country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    // TODO: validate input
    pub fn set_country(&mut self, country: Option<String>) -> Option<Cow<str>> {
        let new = country.map(Cow::Owned);
        std::mem::replace(&mut self.country, new)
    }

    // TODO: validate input
    pub fn set_country_borrowed<'c: 'a>(&mut self, country: Option<&'c str>) -> Option<Cow<str>> {
        let new = country.map(Cow::Borrowed);
        std::mem::replace(&mut self.country, new)
    }

    pub fn get_modifier(&self) -> Option<&str> {
        self.modifier.as_deref()
    }

    // TODO: validate input
    pub fn set_modifier(&mut self, modifier: Option<String>) -> Option<Cow<str>> {
        let new = modifier.map(Cow::Owned);
        std::mem::replace(&mut self.modifier, new)
    }

    // TODO: validate input
    pub fn set_modifier_borrowed<'m: 'a>(&mut self, modifier: Option<&'m str>) -> Option<Cow<str>> {
        let new = modifier.map(Cow::Borrowed);
        std::mem::replace(&mut self.modifier, new)
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
    pub fn new(name: GroupName<'static>) -> Self {
        Group {
            name: name.into(),
            entries: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    pub fn new_borrowed<'e: 'a>(name: GroupName<'e>) -> Self {
        Group {
            name: name.into(),
            entries: IndexMap::new(),
            decor: Vec::new(),
        }
    }

    #[allow(unused)]
    pub(crate) fn from_entries(
        name: GroupName<'static>,
        entries: IndexMap<(Cow<'static, str>, Option<Locale<'static>>), KeyValuePair<'static>>,
        decor: Decor<'static>,
    ) -> Self {
        Group {
            name: name.into(),
            entries,
            decor: decor.into(),
        }
    }

    pub(crate) fn from_entries_borrowed<'e: 'a>(
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

    pub fn get<'k: 'a>(&self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&KeyValuePair> {
        self.entries.get(&(key.into(), locale))
    }

    pub fn get_mut<'k: 'a>(&'a mut self, key: &'k str, locale: Option<Locale<'k>>) -> Option<&mut KeyValuePair> {
        self.entries.get_mut(&(key.into(), locale))
    }

    pub fn insert<'kv: 'a>(&mut self, kv: KeyValuePair<'kv>) -> Option<KeyValuePair> {
        // This clone is cheap only if the kv.key is a Cow::Borrowed(&str).
        // If kv.key is a Cow::Owned(String), the String needs to be copied.
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
