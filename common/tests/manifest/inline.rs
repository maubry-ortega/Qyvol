// # VolleyDevByMaubry [28/∞] "Incrustar manifiestos es como grabar runas en piedra digital."
use super::fixture;
use common::manifest::{Manifest, Permissions};

#[test]
fn parse_basic_inline() {
    let content = std::fs::read_to_string(fixture("manifest_basic.qyv")).unwrap();
    let manifest: Manifest = serde_yaml::from_str(&content).unwrap();

    assert_eq!(manifest.name, "hello-qyvol");
    assert_eq!(manifest.entrypoint, "hello.wasm");
    assert_eq!(manifest.runtime, "wasi");
}

#[test]
fn parse_with_permissions_inline() {
    let content = std::fs::read_to_string(fixture("manifest_with_perms.qyv")).unwrap();
    let manifest: Manifest = serde_yaml::from_str(&content).unwrap();

    let expected = Permissions { fs: Some("none".into()), net: false, exec: false };

    assert_eq!(manifest.permissions, Some(expected));
}
