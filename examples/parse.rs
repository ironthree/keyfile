use dotdesktop::KeyFile;

use std::env::args;

fn main() {
    let mut paths = args();
    paths.next();

    for path in paths {
        let contents = std::fs::read_to_string(path).unwrap();
        let parsed = KeyFile::parse(contents.as_str()).unwrap();
        println!("{:#?}", parsed);
    }
}
