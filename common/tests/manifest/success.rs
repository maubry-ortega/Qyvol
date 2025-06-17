// # VolleyDevByMaubry [29/∞] "Cada éxito en el parseo es una validación de tu diseño."
use super::fixture;
use common::manifest::{Manifest, Permissions};

#[test]
fn parse_basic_manifest() {
    let path = fixture("manifest_basic.qyv");
    let (manifest, _) = Manifest::from_file(&path).unwrap();

    assert_eq!(manifest.name, "hello-qyvol");
    assert_eq!(manifest.entrypoint, "hello.wasm");
    assert_eq!(manifest.runtime, "wasi");
}

#[test]
fn parse_manifest_with_permissions() {
    let path = fixture("manifest_with_perms.qyv");
    let (manifest, _) = Manifest::from_file(&path).unwrap();

    let expected = Permissions { fs: Some("none".into()), net: false, exec: false };

    assert_eq!(manifest.permissions, Some(expected));
}

#[test]
fn parse_manifest_null_fs() {
    let path = fixture("manifest_null_fs.qyv");
    let (manifest, _) = Manifest::from_file(&path).unwrap();

    let expected = Permissions { fs: None, net: true, exec: false };

    assert_eq!(manifest.permissions, Some(expected));
}

#[test]
fn from_file_returns_base_dir() {
    let path = fixture("manifest_basic.qyv");
    let (_manifest, base_dir) = Manifest::from_file(&path).unwrap();

    assert_eq!(base_dir.file_name().unwrap(), "manifest");
}
