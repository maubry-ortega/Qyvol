use common::manifest::{Manifest, Permissions};

#[test]
fn parse_basic_inline() {
    let yaml = r#"
name: hello-qyvol
entrypoint: hello.wasm
runtime: wasi
"#;
    let m: Manifest = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(m.name, "hello-qyvol");
    assert_eq!(m.entrypoint, "hello.wasm");
    assert_eq!(m.runtime, "wasi");
    assert_eq!(m.permissions, None);
}

#[test]
fn parse_with_permissions_inline() {
    let yaml = r#"
name: hello-qyvol
entrypoint: hello.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false
"#;
    let m: Manifest = serde_yaml::from_str(yaml).unwrap();
    let expected = Permissions { fs: Some("none".into()), net: false, exec: false };
    assert_eq!(m.permissions, Some(expected));
}
