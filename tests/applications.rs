use glob::glob;

use dotdesktop::BasicFile;

#[test]
fn parse_all() {
    let files = glob("/usr/share/applications/*.desktop").unwrap();

    let ignored = [
        "/usr/share/applications/firefox.desktop", // invalid locale: "ja_JP-mac"
        "/usr/share/applications/org.fedoraproject.MediaWriter.desktop", // invalid locale: "pt-BR"
    ];

    for entry in files {
        let path = entry.unwrap();
        if ignored.contains(&path.display().to_string().as_str()) {
            continue;
        }

        println!("Checking {}", path.display());

        let contents = std::fs::read_to_string(path).unwrap();
        let parsed = BasicFile::from_contents(contents.as_str().into()).unwrap();
        let written = parsed.to_string();

        assert_eq!(written, contents);
    }
}
