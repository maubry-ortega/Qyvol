// # VolleyDevByMaubry [34/∞] "Explorar revela los secretos ocultos en el sistema."
use cli::shell::commands::scan::execute_scan;
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
fn test_scan_detects_qyv() {
    let dir = test_dir("scan");
    let qyv_file = dir.join("mod.qyv");
    fs::write(&qyv_file, "name: mod\nentrypoint: mod.component.wasm\nruntime: wasi").unwrap();

    let ctx = ShellContext::with_dir(&dir).unwrap();
    let result = execute_scan(&ctx);
    assert!(result.is_ok());
}
