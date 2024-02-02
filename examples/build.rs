use keyfile::{Group, KeyFile, KeyValuePair};

fn main() {
    let kv1 = KeyValuePair::new_with_decor_borrowed(
        "Name".try_into().unwrap(),
        None,
        "Test".try_into().unwrap(),
        "\t".try_into().unwrap(),
        "\t".try_into().unwrap(),
        vec!["# This is a Test Name"],
    );
    let kv2 = KeyValuePair::new_borrowed("Version".try_into().unwrap(), None, "1.5".try_into().unwrap());

    let mut group = Group::new_borrowed("Desktop Entry");
    group.insert(kv1);
    group.insert(kv2);

    let mut kf = KeyFile::new();
    kf.insert_group(group);

    println!("{}", kf);
}
