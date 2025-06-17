// # VolleyDevByMaubry [30/∞] "Los errores son enseñanzas con forma de excepción."
use super::fixture;
use common::manifest::Manifest;

#[test]
#[should_panic]
fn parse_invalid_manifest_panics() {
    // Suponiendo que `invalid_manifest.qyv` no es YAML válido o no cumple estructura
    let path = fixture("invalid_manifest.qyv");
    let _ = Manifest::from_file(&path).unwrap();
}
