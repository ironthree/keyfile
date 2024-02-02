use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Key<'a> {
    pub inner: Cow<'a, str>,
}

impl<'a> Key<'a> {
    #[inline(always)]
    pub(crate) fn new_unchecked<'v: 'a>(value: Cow<'v, str>) -> Self {
        Key { inner: value }
    }
}

impl<'a> From<Key<'a>> for Cow<'a, str> {
    #[inline(always)]
    fn from(value: Key<'a>) -> Self {
        value.inner
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidKey {
    #[error("Invalid Key: keys must be ASCII-only")]
    NotAscii,
    #[error("Invalid Key: keys can only contain alphanumeric characters and hyphens")]
    NotAlphanumeric,
}

impl<'a> TryFrom<Cow<'a, str>> for Key<'a> {
    type Error = InvalidKey;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        for c in value.chars() {
            if !c.is_ascii() {
                return Err(InvalidKey::NotAscii);
            }
            if !c.is_alphanumeric() && c != '-' {
                return Err(InvalidKey::NotAlphanumeric);
            }
        }

        Ok(Key { inner: value })
    }
}

impl<'a> TryFrom<&'a str> for Key<'a> {
    type Error = InvalidKey;

    #[inline(always)]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Borrowed(value))
    }
}

impl<'a> TryFrom<String> for Key<'a> {
    type Error = InvalidKey;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Key::try_from(Cow::Owned(value))
    }
}