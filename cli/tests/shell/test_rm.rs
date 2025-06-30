// # VolleyDevByMaubry [33/∞] "Eliminar es dejar espacio para lo nuevo."
use cli::shell::commands::nav::execute_rm;
use cli::shell::context::ShellContext;
use std::env;
use std::fs;
use std::fs::File;
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
fn test_rm_file() {
    let dir = test_dir("rm_file");
    let file = dir.join("temp.txt");
    File::create(&file).unwrap();

    let mut ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_rm(&["temp.txt"], &mut ctx);
    assert!(result.is_ok());
    assert!(!file.exists());
}

#[test]
fn test_rm_directory() {
    let dir = test_dir("rm_dir");
    let subdir = dir.join("to_delete");
    fs::create_dir_all(&subdir).unwrap();

    let mut ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_rm(&["to_delete"], &mut ctx);
    assert!(result.is_ok());
    assert!(!subdir.exists());
}
