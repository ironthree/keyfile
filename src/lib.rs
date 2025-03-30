//! # KeyFile file format implementation
//!
//! The KeyFile format is part of the XDG / FreeDesktop [Desktop Entry Specification] and is also implemented in Glib as
//! [Glib.KeyFile]. It is the file format that is used for application menu entries (`.desktop` files), icon theme
//! metadata, and various other kinds of configuration files (mostly on Linux).
//!
//! This crate attempts to provide a Rust implementation of this file format that supports both zero-copy parsing and
//! format-preserving representation (i.e. parsing file contents into a [`KeyFile`] and writing it back to a string
//! should result in the original string, including whitespace and comments):
//!
//! ```
//! use keyfile::KeyFile;
//!
//! let original = "[Hello World]\none=one\ntwo = two\n";
//!
//! let keyfile = KeyFile::parse(original).unwrap();
//! let string = keyfile.to_string();
//!
//! assert_eq!(original, &string);
//! ```
//!
//! Parsing a string into a [`KeyFile`] with [`KeyFile::parse`] does not cause any string copies from the input
//! string - this results in a [`KeyFile`] whose lifetime is tied to the lifetime of the input string. This makes it
//! possible to avoid unnecessary allocations when modifying the underlying data. For example, overriding a value or
//! removing a key-value pair with a new value just causes the references to the input string to be dropped.
//!
//! However, this means [`KeyFile`] values with a limited lifetime like this cannot be returned from the scope that
//! limits the lifetime of the input string. It is possible to transform the [`KeyFile`] into one with a `'static`
//! lifetime by calling [`KeyFile::into_owned`] (which causes all strings that were borrowed from the input to be
//! copied, while owned values are *not* copied) - the new value can then be moved independently of the input string:
//!
//! ```
//! use keyfile::KeyFile;
//!
//! let original = String::from("[Hello World]\none=one\ntwo = two\n");
//!
//! let borrowed = KeyFile::parse(&original).unwrap();
//! let owned: KeyFile<'static> = borrowed.into_owned();
//! ```
//!
//! ## Limitations
//!
//! This crate only supports parsing and writing UTF-8 encoded KeyFiles. The spec technically supports key-value pairs
//! where values use encodings other than UTF-8 (by using the "encoding" modifier in the locale), but KeyFiles
//! themselves are supposed to be UTF-8 encoded. Hence, all values are parsed as UTF-8, even if the "encoding" specifier
//! is set to a different encoding, and it is not possible to programmatically create new key-value pairs that set the
//! "encoding" modifier.
//!
//! [Desktop Entry Specification]: https://specifications.freedesktop.org/desktop-entry-spec/latest/
//! [Glib.KeyFile]: https://docs.gtk.org/glib/struct.KeyFile.html

mod keyfile;
mod parse;
pub mod types;

pub use crate::keyfile::*;
