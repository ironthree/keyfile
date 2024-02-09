#![allow(missing_docs)]

use keyfile::KeyFile;

use std::env::args;

fn main() -> anyhow::Result<()> {
    let mut paths = args();
    paths.next();

    for path in paths {
        let contents = std::fs::read_to_string(path)?;
        let parsed = KeyFile::parse(contents.as_str())?;
        println!("{:#?}", parsed);
    }

    Ok(())
}
