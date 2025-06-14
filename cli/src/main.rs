// # VolleyDevByMaubry [1/∞] "Una línea de comando puede ser el filo de una revolución."
use clap::Parser;
use colored::*;
use std::path::Path;
use runtime::executor::{run_wasm, deploy_wasm};
use common::Manifest;
use thiserror::Error;
mod shell;
use self::shell::start_shell;
use runtime::cluster::manage_cluster;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Error al leer manifiesto: {0}")]
    ManifestError(String),
    #[error("Error al ejecutar módulo: {0}")]
    ExecutorError(String),
    #[error("Error al desplegar módulo: {0}")]
    DeployError(String),
}

#[derive(Parser)]
#[command(name = "qyvol")]
#[command(about = "Ejecuta funciones .wasm en Qyvol Runtime", long_about = None)]
struct Cli {
    #[arg(long, default_value = "text", global = true)]
    format: String, // Formato de salida: text, json, yaml
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Run { path: String },
    Deploy { path: String, target: Option<String> },
    Shell,
    Cluster { action: String, node: Option<String> },
}

pub fn main_with_cli(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Run { path } => {
            println!("{}", "▶ Ejecutando Qyvol Runtime...".green());
            let (manifest, manifest_dir) = Manifest::from_file(Path::new(&path))
                .map_err(|e| CliError::ManifestError(e.to_string()))?;
            run_wasm(&manifest, &manifest_dir, &cli.format)
                .map_err(|e| CliError::ExecutorError(e.to_string()))?;
            Ok(())
        }
        Commands::Deploy { path, target } => {
            println!("{}", "▶ Desplegando módulo...".green());
            let (manifest, manifest_dir) = Manifest::from_file(Path::new(&path))
                .map_err(|e| CliError::ManifestError(e.to_string()))?;
            let target = target.unwrap_or("http://localhost:8080".to_string());
            deploy_wasm(&manifest, &manifest_dir, &target)
                .map_err(|e| CliError::DeployError(e.to_string()))?;
            Ok(())
        }
        Commands::Shell => {
            println!("{}", "▶ Iniciando Qyvol Shell...".green());
            start_shell().map_err(|e| CliError::ExecutorError(e.to_string()))?;
            Ok(())
        }
        Commands::Cluster { action, node } => {
            println!("{}", "▶ Gestionando clúster...".green());
            manage_cluster(&action, node).map_err(|e| CliError::ExecutorError(e.to_string()))?;
            Ok(())
        }
    }
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    main_with_cli(cli)
}