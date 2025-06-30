// # VolleyDevByMaubry [22/∞] "El manejador de comandos teje las acciones en un tapiz funcional."
use super::context::ShellContext;
use common::Manifest;
use runtime::run_wasm;
use std::error::Error;
use std::path::Path;

pub mod info;
pub mod nav;
pub mod scan;
pub mod template;
pub mod trait_command;

pub struct CommandHandler;

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler
    }

    pub fn execute(
        &self, command: &str, args: &[&str], context: &mut ShellContext,
    ) -> Result<(), Box<dyn Error>> {
        match command {
            "run" | "r" => execute_run(args, context),
            "info" | "i" => info::execute_info(context),
            "scan" | "s" => scan::execute_scan(context),
            "template" | "t" => template::execute_template(args, context),
            "ls" | "dir" | "l" => execute_ls(context),
            "help" | "h" => execute_help(),
            "clear" | "cls" | "c" => execute_clear(),
            "cd" | "d" => nav::execute_cd(args, context),
            "z" => nav::execute_z(args, context),
            "mkdir" | "m" => nav::execute_mkdir(args, context),
            "rm" | "x" => nav::execute_rm(args, context),
            "edit" | "e" => nav::execute_edit(args, context),
            _ => Err(format!("Comando desconocido: {command}").into()),
        }
    }
}

fn execute_run(args: &[&str], context: &ShellContext) -> Result<(), Box<dyn Error>> {
    use super::ui::print_success;
    if args.is_empty() {
        return Err("Uso: run <path-to-qyv>".into());
    }

    let input_path = args[0];
    let path = if Path::new(input_path).exists() {
        Path::new(input_path).to_path_buf()
    } else {
        let possible_paths = vec![
            context.current_dir.join(input_path),
            context.current_dir.join("examples").join(input_path),
            context.current_dir.join(input_path).with_extension("qyv"),
            context.current_dir.join("examples").join(input_path).with_extension("qyv"),
        ];

        possible_paths
            .into_iter()
            .find(|p| p.exists())
            .ok_or_else(|| format!("Archivo no encontrado: {input_path}"))?
    };

    let (manifest, manifest_dir) = Manifest::from_file(&path)?;
    run_wasm(&manifest, &manifest_dir, "text")?;
    print_success("Ejecución completada.");
    Ok(())
}

fn execute_ls(context: &ShellContext) -> Result<(), Box<dyn Error>> {
    use super::ui::{SimpleTable, print_step};
    use std::fs;
    use std::time::SystemTime;

    print_step("Listando directorio actual...");

    let mut table = SimpleTable::new(vec!["Tipo", "Nombre", "Tamaño", "Modificado", "Descripción"]);

    for entry in fs::read_dir(&context.current_dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();

        let file_type = if metadata.is_dir() {
            "📂"
        } else if name.ends_with(".qyv") {
            "🦊"
        } else if name.ends_with(".wasm") {
            "📦"
        } else if name.ends_with(".rs") {
            "🦀"
        } else if name.ends_with(".go") {
            "🐹"
        } else {
            "📄"
        };

        let size = if metadata.is_file() {
            let bytes = metadata.len();
            if bytes < 1024 {
                format!("{bytes}B")
            } else if bytes < 1024 * 1024 {
                format!("{:.1}KB", bytes as f64 / 1024.0)
            } else {
                format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
            }
        } else {
            "-".to_string()
        };

        let modified = metadata
            .modified()
            .map(|t| {
                let duration = SystemTime::now().duration_since(t).unwrap_or_default().as_secs();
                if duration < 3600 {
                    format!("{}m ago", duration / 60)
                } else if duration < 86400 {
                    format!("{}h ago", duration / 3600)
                } else {
                    format!("{}d ago", duration / 86400)
                }
            })
            .unwrap_or("?".to_string());

        let description = if metadata.is_dir() {
            "Directorio"
        } else if name.ends_with(".qyv") {
            "Manifiesto Qyvol"
        } else if name.ends_with(".wasm") {
            "Módulo WASM"
        } else if name.ends_with(".rs") {
            "Código Rust"
        } else if name.ends_with(".go") {
            "Código Go"
        } else if name == "README.md" {
            "Documentación"
        } else if name == "Cargo.toml" {
            "Config Rust"
        } else {
            "Archivo"
        };

        table.add_row(vec![file_type, &name, &size, &modified, description]);
    }

    table.print();
    Ok(())
}

fn execute_clear() -> Result<(), Box<dyn Error>> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    Ok(())
}

fn execute_help() -> Result<(), Box<dyn Error>> {
    use super::ui::{print_help_command, print_separator};

    print_separator();
    println!("📚 Comandos disponibles:");

    print_help_command("run (r)", "Ejecuta un módulo WASM", Some("run <path-to-qyv>"));
    print_help_command("info (i)", "Muestra información del entorno", Some("info"));
    print_help_command("scan (s)", "Busca módulos WASM automáticamente", Some("scan"));
    print_help_command(
        "template (t)",
        "Crea un nuevo proyecto",
        Some("template <lenguaje> <nombre>"),
    );
    print_help_command("ls (l)", "Lista archivos en el directorio actual", Some("ls"));
    print_help_command("dir", "Alias para ls", Some("dir"));
    print_help_command("help (h)", "Muestra esta ayuda", Some("help"));
    print_help_command("clear (c)", "Limpia la pantalla", Some("clear"));
    print_help_command("cls", "Alias para clear", Some("cls"));
    print_help_command("cd (d)", "Cambia el directorio actual", Some("cd <directorio>"));
    print_help_command("z", "Navega rápidamente a directorios frecuentes", Some("z <patrón>"));
    print_help_command("mkdir (m)", "Crea un directorio", Some("mkdir <nombre>"));
    print_help_command("rm (x)", "Elimina un archivo o directorio", Some("rm <nombre>"));
    print_help_command("edit (e)", "Edita un archivo", Some("edit <nombre>"));
    print_help_command("exit", "Sale del shell", Some("exit"));

    print_separator();
    Ok(())
}
