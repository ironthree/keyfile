use dotdesktop::BasicFile;

use std::env::args;

fn main() {
    let mut paths = args();
    paths.next();

    for path in paths {
        let contents = std::fs::read_to_string(path).unwrap();
        let parsed = BasicFile::from_contents(contents.as_str().into()).unwrap();
        println!("{:#?}", parsed);
    }
}
