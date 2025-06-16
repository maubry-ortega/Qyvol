// # VolleyDevByMaubry [25/∞] "Las plantillas siembran nuevos proyectos, listos para crecer."
use crate::shell::context::ShellContext;
use crate::shell::ui::{print_step, print_success};
use std::error::Error;
use std::fs::{create_dir_all, write};

pub fn execute_template(args: &[&str], context: &ShellContext) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Uso: template <lenguaje> <nombre-proyecto>".into());
    }

    let language = args[0];
    let project_name = args[1];
    let project_path = context.current_dir.join(project_name);

    print_step(&format!("Creando proyecto {} en {}...", language, project_name));

    create_dir_all(&project_path)?;

    match language.to_lowercase().as_str() {
        "rust" => {
            // Crear estructura para Rust
            let src_path = project_path.join("src");
            create_dir_all(&src_path)?;

            let main_rs = r#"
fn main() {
    println!("¡Hola desde Qyvol WASI!");
}
"#;
            write(src_path.join("main.rs"), main_rs)?;

            let cargo_toml = format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
                project_name
            );
            write(project_path.join("Cargo.toml"), cargo_toml)?;

            let qyv = format!(
                r#"name: {}
entrypoint: {}.component.wasm
runtime: wasi
language: rust
permissions:
  fs: none
  net: false
  exec: false
fs:
  type: memfs
"#,
                project_name, project_name
            );
            write(project_path.join(format!("{}.qyv", project_name)), qyv)?;

            let readme = format!("# {}\n\nProyecto Qyvol generado en Rust.", project_name);
            write(project_path.join("README.md"), readme)?;
        }
        "go" => {
            // Crear estructura para Go
            let src_path = project_path.join("main.go");

            let main_go = r#"
package main

import "fmt"

func main() {
    fmt.Println("¡Hola desde Qyvol WASI!")
}
"#;
            write(&src_path, main_go)?;

            let go_mod = format!(
                r#"module {}

go 1.21
"#,
                project_name
            );
            write(project_path.join("go.mod"), go_mod)?;

            let qyv = format!(
                r#"name: {}
entrypoint: {}.component.wasm
runtime: wasi
language: go
permissions:
  fs: none
  net: false
  exec: false
fs:
  type: memfs
"#,
                project_name, project_name
            );
            write(project_path.join(format!("{}.qyv", project_name)), qyv)?;

            let readme = format!("# {}\n\nProyecto Qyvol generado en Go.", project_name);
            write(project_path.join("README.md"), readme)?;
        }
        _ => return Err(format!("Lenguaje no soportado: {}", language).into()),
    }

    print_success(&format!(
        "Proyecto creado! Usa 'cd {} && qyv run {}.qyv'",
        project_name, project_name
    ));
    Ok(())
}
