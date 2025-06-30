// # VolleyDevByMaubry [21/∞] "La información revela el estado real del sistema y entorno."
use crate::shell::context::ShellContext;
use crate::shell::system::SystemInfo;
use crate::shell::ui::{print_info_box, print_separator};
use std::error::Error;

pub fn execute_info(context: &ShellContext) -> Result<(), Box<dyn Error>> {
    print_separator();
    let sysinfo = SystemInfo::gather();
    print_info_box(
        "Qyvol Runtime v0.1.0",
        &[
            ("Directorio", &context.current_dir.to_string_lossy()),
            ("Tipo de proyecto", context.get_project_name()),
            ("Sistema", &sysinfo.system_name),
            ("Kernel", &sysinfo.kernel_version),
            ("Versión OS", &sysinfo.os_version),
            ("Host", &sysinfo.host_name),
            ("Memoria total", &format!("{} MB", sysinfo.total_memory / 1024)),
            ("Memoria usada", &format!("{} MB", sysinfo.used_memory / 1024)),
            (
                "CPU",
                &format!(
                    "{} ({} núcleos, {:.1}% uso)",
                    sysinfo.cpu_brand, sysinfo.cpu_cores, sysinfo.cpu_usage
                ),
            ),
            ("Discos", &sysinfo.disks.join(", ").to_string()),
            ("Redes", &sysinfo.networks.join(", ").to_string()),
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
        ],
    );
    print_separator();
    Ok(())
}
