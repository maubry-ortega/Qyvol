use common::manifest::{Manifest, Permissions};
use std::path::Path;

#[test]
fn parse_from_file() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("hello.qyv");
    let (m, _dir) = Manifest::from_file(&path).expect("read manifest file");

    assert_eq!(m.name, "hello-qyvol");
    assert_eq!(m.entrypoint, "hello.component.wasm");
    assert_eq!(m.runtime, "wasi");
    let perms = Permissions { fs: Some("none".into()), net: false, exec: false };
    assert_eq!(m.permissions, Some(perms));
}
