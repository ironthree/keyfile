#![allow(missing_docs)]

use glob::glob;

use keyfile::KeyFile;

#[test]
fn parse_all() -> anyhow::Result<()> {
    let files = glob("/usr/share/applications/*.desktop")?;

    let ignored = [
        "/usr/share/applications/org.fedoraproject.MediaWriter.desktop", // invalid locale: "pt-BR"
        "/usr/share/applications/org.mozilla.firefox.desktop", // invalid locale: "ja_JP-mac"
        "/usr/share/applications/gnome-wifi-panel.desktop", // invalid control character in Keywords[el]: "\t"
    ];

    for entry in files {
        let path = entry?;
        if ignored.contains(&path.display().to_string().as_str()) {
            continue;
        }

        println!("Checking {}", path.display());

        let contents = std::fs::read_to_string(path)?;
        let parsed = KeyFile::parse(contents.as_str())?;
        let written = parsed.to_string();

        assert_eq!(written, contents);
    }

    Ok(())
}
