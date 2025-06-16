// # VolleyDevByMaubry [21/∞] "La información revela los secretos del entorno con un solo vistazo."
use crate::shell::context::ShellContext;
use crate::shell::ui::{print_info_box, print_separator};
use std::error::Error;

pub fn execute_info(context: &ShellContext) -> Result<(), Box<dyn Error>> {
    print_separator();
    print_info_box(
        "Qyvol Runtime v0.1.0",
        &[
            ("Directorio", &context.current_dir.to_string_lossy()),
            ("Tipo de proyecto", context.get_project_name()),
            (
                "Módulos .qyv",
                &context
                    .project_stats
                    .as_ref()
                    .map_or("0".to_string(), |s| s.qyv_files.to_string()),
            ),
            (
                "Módulos .wasm",
                &context
                    .project_stats
                    .as_ref()
                    .map_or("0".to_string(), |s| s.wasm_files.to_string()),
            ),
            (
                "Proyectos Rust",
                &context
                    .project_stats
                    .as_ref()
                    .map_or("0".to_string(), |s| s.rust_projects.to_string()),
            ),
            (
                "Proyectos Go",
                &context
                    .project_stats
                    .as_ref()
                    .map_or("0".to_string(), |s| s.go_projects.to_string()),
            ),
            (
                "Tamaño total",
                &context.project_stats.as_ref().map_or("0B".to_string(), |s| s.format_size()),
            ),
            ("Memoria WASM", "12.4MB disponible"),
            ("Red", "Habilitada"),
            ("FS Backend", "diskfs"),
        ],
    );
    print_separator();
    Ok(())
}
