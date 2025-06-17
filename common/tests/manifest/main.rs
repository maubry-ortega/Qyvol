// # VolleyDevByMaubry [26/∞] "El archivo es la entrada; la intención, la salida."
pub mod failure;
pub mod file;
pub mod inline;
pub mod success;

/// Helper global para acceder a fixtures YAML
pub fn fixture(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("manifest")
        .join(name)
}
