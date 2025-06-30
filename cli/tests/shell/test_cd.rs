// # VolleyDevByMaubry [30/∞] "Un directorio creado es un paso más en el camino del usuario."
use cli::shell::commands::nav::execute_mkdir;
use cli::shell::context::ShellContext;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Crea un directorio temporal dentro de `tests/tmp/{name}`
fn test_dir(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("tmp").join(name);

    if root.exists() {
        let _ = fs::remove_dir_all(&root);
    }
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn test_mkdir_creates_directory() {
    let dir = test_dir("mkdir_cd");
    let nuevo_dir = dir.join("nuevo_dir");
    if nuevo_dir.exists() {
        let _ = fs::remove_dir_all(&nuevo_dir);
    }
    let mut ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_mkdir(&["nuevo_dir"], &mut ctx);
    assert!(result.is_ok());
    assert!(nuevo_dir.exists());
}
