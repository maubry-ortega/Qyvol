// cli/src/main.rs
// # VolleyDevByMaubry [1/∞] "Una línea de comando puede ser el filo de una revolución."
use clap::Parser;
use colored::*;
use common::Manifest;
use runtime::deploy::deploy_wasm;
use runtime::run_wasm;
use std::path::Path;
use thiserror::Error;
mod shell;
use anyhow::Result;
use runtime::cluster::manage_cluster;
use shell::start_shell;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Error al leer manifiesto: {0}")]
    ManifestError(String),
    #[error("Error al ejecutar módulo: {0}")]
    ExecutorError(String),
    #[error("Error al desplegar módulo: {0}")]
    DeployError(String),
    #[error("Error en el Shell: {0}")]
    ShellError(String),
    #[error("Error en el clúster: {0}")]
    ClusterError(String),
}

#[derive(Parser)]
#[command(name = "qyvol")]
#[command(about = "Ejecuta funciones .wasm en Qyvol Runtime", long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "text", global = true)]
    format: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Run {
        path: String,
    },
    Deploy {
        path: String,
        target: Option<String>,
    },
    Shell,
    Cluster {
        action: String,
        node: Option<String>,
    },
}

pub fn main_with_cli(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Run { path } => {
            println!("{}", "▶ Ejecutando Qyvol Runtime...".green());
            let (manifest, manifest_dir) = Manifest::from_file(Path::new(&path))
                .map_err(|e| CliError::ManifestError(e.to_string()))?;

            run_wasm(&manifest, &manifest_dir, &cli.format)
                .map_err(|e| CliError::ExecutorError(e.to_string()))?;
        }
        Commands::Deploy { path, target } => {
            println!("{}", "▶ Desplegando módulo...".green());
            let (manifest, manifest_dir) = Manifest::from_file(Path::new(&path))
                .map_err(|e| CliError::ManifestError(e.to_string()))?;
            let target = target.unwrap_or_else(|| "http://localhost:8080".to_string());

            deploy_wasm(&manifest, &manifest_dir, &target)
                .map_err(|e| CliError::DeployError(e.to_string()))?;
        }
        Commands::Shell => {
            println!("{}", "▶ Iniciando Qyvol Shell...".green());
            start_shell().map_err(|e| CliError::ShellError(e.to_string()))?;
        }
        Commands::Cluster { action, node } => {
            println!("{}", "▶ Gestionando clúster...".green());
            manage_cluster(&action, node).map_err(|e| CliError::ClusterError(e.to_string()))?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    main_with_cli(cli)?;
    Ok(())
}
