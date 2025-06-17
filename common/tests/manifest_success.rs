use common::manifest::{Manifest, Permissions};
use std::path::Path;

/// Helper para resolver el path de un fixture
fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name)
}

#[test]
fn parse_basic_manifest() {
    let txt = std::fs::read_to_string(fixture("manifest_basic.qyv")).unwrap();
    let m: Manifest = serde_yaml::from_str(&txt).unwrap();

    assert_eq!(m.name, "hello-qyvol");
    assert_eq!(m.entrypoint, "hello.wasm");
    assert_eq!(m.runtime, "wasi");
    assert_eq!(m.permissions, None);
}

#[test]
fn parse_manifest_with_permissions() {
    let txt = std::fs::read_to_string(fixture("manifest_with_perms.qyv")).unwrap();
    let m: Manifest = serde_yaml::from_str(&txt).unwrap();
    let expected = Permissions { fs: Some("none".into()), net: false, exec: false };

    assert_eq!(m.permissions, Some(expected));
}

#[test]
fn parse_manifest_null_fs() {
    let txt = std::fs::read_to_string(fixture("manifest_null_fs.qyv")).unwrap();
    let m: Manifest = serde_yaml::from_str(&txt).unwrap();
    let expected = Permissions { fs: None, net: true, exec: false };

    assert_eq!(m.permissions, Some(expected));
}

#[test]
fn from_file_returns_base_dir() {
    let path = fixture("manifest_basic.qyv");
    let (m, dir) = Manifest::from_file(&path).unwrap();
    assert_eq!(m.name, "hello-qyvol");
    assert_eq!(dir, path.parent().unwrap());
}
