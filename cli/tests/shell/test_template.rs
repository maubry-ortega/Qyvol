// # VolleyDevByMaubry [35/∞] "Una plantilla es el inicio de una nueva historia."
use cli::shell::commands::template::execute_template;
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
fn test_create_rust_template() {
    let dir = test_dir("template_rust");
    let ctx = ShellContext::with_dir(&dir).unwrap();

    let result = execute_template(&["rust", "mi_proyecto"], &ctx);
    assert!(result.is_ok());

    let project = dir.join("mi_proyecto");
    assert!(project.join("Cargo.toml").exists());
    assert!(project.join("src/main.rs").exists());
    assert!(project.join("mi_proyecto.qyv").exists());
}
