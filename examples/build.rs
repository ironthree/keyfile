use keyfile::{Group, KeyFile, KeyValuePair};

fn main() {
    let kv1 = KeyValuePair::new_with_decor_borrowed("Name", None, "Test", "\t", "\t", vec!["# This is a Test Name"]);
    let kv2 = KeyValuePair::new_borrowed("Version", None, "1.5");

    let mut group = Group::new_borrowed("Desktop Entry");
    group.insert(kv1);
    group.insert(kv2);

    let mut kf = KeyFile::new();
    kf.insert_group(group);

    println!("{}", kf);
}
