// # VolleyDevByMaubry [24/∞] "El escaneo descubre los módulos ocultos, iluminando el horizonte."
use crate::shell::context::{ShellContext, find_qyv_files, find_wasm_files};
use crate::shell::ui::{FileTree, print_list_item, print_step};
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn execute_scan(context: &ShellContext) -> Result<(), Box<dyn Error>> {
    print_step("Escaneando módulos WASM...");

    let mut tree = FileTree::new();
    let qyv_files = find_qyv_files(&context.current_dir)?;
    let wasm_files = find_wasm_files(&context.current_dir)?;

    let wasm_paths: Vec<PathBuf> = wasm_files.into_iter().collect();

    for (i, qyv_file) in qyv_files.iter().enumerate() {
        let is_last = i == qyv_files.len() - 1;
        let parent = qyv_file.parent().unwrap_or_else(|| Path::new(""));
        let name = qyv_file.file_name().unwrap().to_string_lossy();
        let level =
            qyv_file.strip_prefix(&context.current_dir).unwrap_or(qyv_file).components().count()
                - 1;

        let has_wasm = wasm_paths.iter().any(|w| {
            w.starts_with(parent)
                && w.file_stem().unwrap().to_string_lossy() == name.trim_end_matches(".qyv")
        });
        let status = if has_wasm {
            format!("✅ {} (compilado)", context.get_project_name())
        } else {
            format!("⚠️ {} (sin WASM compilado)", context.get_project_name())
        };

        tree.add_item(&format!("{name} {status}"), false, level, is_last);
    }

    tree.print();
    print_list_item("💡", "Tip", Some("Usa 'qyv run <nombre>' para ejecutar"));
    Ok(())
}
