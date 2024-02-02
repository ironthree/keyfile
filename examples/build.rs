use keyfile::{Group, KeyFile, KeyValuePair};

fn main() -> anyhow::Result<()> {
    let kv1 = KeyValuePair::new_with_decor_borrowed(
        "Name".try_into()?,
        None,
        "Test".try_into()?,
        "\t".try_into()?,
        "\t".try_into()?,
        vec!["# This is a Test Name"].try_into()?,
    );
    let kv2 = KeyValuePair::new_borrowed("Version".try_into()?, None, "1.5".try_into()?);

    let mut group = Group::new_borrowed("Desktop Entry".try_into()?);
    group.insert(kv1);
    group.insert(kv2);

    let mut kf = KeyFile::new();
    kf.insert_group(group);

    println!("{}", kf);

    Ok(())
}
