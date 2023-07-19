#![allow(unused)]
#![warn(missing_debug_implementations)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;
use self_cell::self_cell;

mod basic;
mod error;
mod parse;

pub use error::DesktopError;
use parse::ParsedFileWrapper;

#[derive(Debug)]
pub struct BasicFile<'a> {
    inner: ParsedFileWrapper<'a>,
}

impl<'a> Display for BasicFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self.inner.borrow_dependent(), f)
    }
}

impl<'a> BasicFile<'a> {
    pub fn from_contents(contents: Cow<'a, str>) -> Result<Self, DesktopError> {
        Ok(BasicFile {
            inner: ParsedFileWrapper::from_contents(contents)?,
        })
    }
}
