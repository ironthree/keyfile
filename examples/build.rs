#![allow(missing_docs)]

use keyfile::{Group, KeyFile, KeyValuePair, types::Key, types::Value};

fn main() -> anyhow::Result<()> {
    let kv1 = KeyValuePair::from_fields(
        "Name".try_into()?,
        None,
        Value::try_from("Test")?,
        "\t".try_into()?,
        "\t".try_into()?,
        vec!["# This is a Test Name"].try_into()?,
    );
    let kv2 = KeyValuePair::new(Key::try_from("Version")?, Value::from(1.5));

    let mut group = Group::new("Desktop Entry".try_into()?);
    group.insert(kv1);
    group.insert(kv2);

    let mut kf = KeyFile::new();
    kf.insert_group(group);

    println!("{}", kf);

    Ok(())
}
