// # VolleyDevByMaubry [36/∞] "Navegar rápido es fluir entre posibilidades."
use cli::shell::commands::nav::execute_z;
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
fn test_z_matches_existing_directory() {
    let dir = test_dir("z_test");
    let target = dir.join("modulo_oculto");
    fs::create_dir_all(&target).unwrap();

    let mut ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_z(&["oculto"], &mut ctx);
    assert!(result.is_ok());
    assert_eq!(ctx.current_dir, target);
}

#[test]
fn test_z_fails_on_missing_match() {
    let dir = test_dir("z_fail");
    let mut ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_z(&["nada"], &mut ctx);
    assert!(result.is_err());
}
