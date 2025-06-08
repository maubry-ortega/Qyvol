// # VolleyDevByMaubry [1/∞] "Una línea de comando puede ser el filo de una revolución."
use clap::Parser;
use colored::*;
use std::path::Path;
use runtime::executor::run_wasm;
use common::Manifest;

/// CLI de Qyvol
#[derive(Parser)]
#[command(name = "qyvol")]
#[command(about = "Ejecuta funciones .wasm en Qyvol Runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Run { path: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path } => {
            println!("{}", "▶ Ejecutando Qyvol Runtime...".green());

            match Manifest::from_file(Path::new(&path)) {
                Ok((manifest, manifest_dir)) => run_wasm(&manifest, &manifest_dir),
                Err(e) => println!("❌ Error al leer el manifiesto: {}", e),
            }
        }
    }
}


