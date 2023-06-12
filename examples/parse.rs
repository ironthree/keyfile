use dotdesktop::DesktopFile;

use std::env::args;

fn main() {
    let mut paths = args();
    paths.next();

    for path in paths {
        println!("{:#?}", DesktopFile::from_path(path));
    }
}
