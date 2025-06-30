// # VolleyDevByMaubry [32/∞] "Editar es reescribir el destino de los datos."
use cli::shell::commands::nav::execute_edit;
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
fn test_edit_creates_file() {
    let dir = test_dir("edit");
    unsafe {
        env::set_var("EDITOR", "true");
    } // "true" hace nada pero retorna 0
    let mut ctx = ShellContext::with_dir(&dir).unwrap();

    let result = execute_edit(&["archivo.txt"], &mut ctx);
    assert!(result.is_ok());
    assert!(dir.join("archivo.txt").exists());
}
