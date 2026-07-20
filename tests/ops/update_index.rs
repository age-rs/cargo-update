use cargo_update::ops::{self, Registry};
use chrono::{FixedOffset, DateTime};
use std::collections::BTreeMap;
use semver::Version as Semver;
use std::env::temp_dir;
use std::fs;

/// https://github.com/nabijaczleweli/cargo-update/pull/341
#[test]
fn truncated_not() {
    let mut reg = Registry::Sparse(Default::default());
    let mut out = vec![];

    let url = sparse_registry_from("crates.io", "cargo-update", "test-data/341/cratesio.last2");
    assert_eq!(ops::update_index(&mut reg, &url, ["cargo-update"].iter(), None, false, &Default::default(), None, &mut out),
               Ok(()));

    assert_eq!(String::from_utf8(out).unwrap(), format!("    Polling registry '{}'.\n\n", url));
    let Registry::Sparse(pkgs) = reg else { panic!() };
    assert_eq!(pkgs, expecting(true));
}

#[test]
fn truncated_workaround() {
    let mut reg = Registry::Sparse(Default::default());
    let mut out = vec![];

    let url = sparse_registry_from("kellnr", "cargo-update", "test-data/341/kellnr.last2");
    assert_eq!(ops::update_index(&mut reg, &url, ["cargo-update"].iter(), None, false, &Default::default(), None, &mut out),
               Ok(()));

    assert_eq!(String::from_utf8(out).unwrap(), format!("    Polling registry '{}'.\n\n", url));
    let Registry::Sparse(pkgs) = reg else { panic!() };
    assert_eq!(pkgs, expecting(false));
}

fn expecting(dates: bool) -> BTreeMap<String, Vec<(Semver, Option<DateTime<FixedOffset>>)>> {
    [("cargo-update".to_string(),
      vec![("21.0.2".parse().unwrap(),
            if dates {
                Some("2026-07-10T15:30:20Z".parse().unwrap())
            } else {
                None
            }),
           ("22.0.0".parse().unwrap(),
            if dates {
                Some("2026-07-12T22:56:06Z".parse().unwrap())
            } else {
                None
            })])]
        .into()
}

fn sparse_registry_from(subname: &str, package: &str, file: &str) -> String {
    let toplevel = temp_dir().join("cargo_update-test").join(format!("update_index-{}", subname));

    let mut tf = toplevel.clone();
    for next in ops::split_package_path(package) {
        let _ = fs::create_dir_all(&tf);
        tf.push(&next[..]);
    }
    fs::copy(file, &tf).unwrap();

    format!("file://{}", toplevel.canonicalize().unwrap().display()).replace(r#"\\?\"#, "").replace('\\', "/").replace(' ', "%20")
}
