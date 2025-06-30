// cli/tests/utils.rs
use std::env;
use std::fs;
use std::path::PathBuf;

/// Crea un directorio temporal dentro de `tests/tmp/{name}`
pub fn test_dir(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("tmp").join(name);

    if root.exists() {
        let _ = fs::remove_dir_all(&root);
    }
    fs::create_dir_all(&root).unwrap();
    root
}
