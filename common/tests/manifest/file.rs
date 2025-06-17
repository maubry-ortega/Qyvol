// # VolleyDevByMaubry [27/∞] "El archivo es la entrada; la intención, la salida."
use super::fixture;
use common::manifest::Manifest;

#[test]
fn parse_from_file() {
    let path = fixture("manifest_basic.qyv");
    let (manifest, _base) = Manifest::from_file(&path).unwrap();

    assert_eq!(manifest.name, "hello-qyvol");
    assert_eq!(manifest.entrypoint, "hello.wasm");
    assert_eq!(manifest.runtime, "wasi");
}
