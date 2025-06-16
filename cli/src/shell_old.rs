// # VolleyDevByMaubry [4/∞] "El shell interactivo corta el caos con comandos precisos."
use rustyline::Editor;
use std::error::Error;
use std::process::Command;
use crate::Cli;
use clap::Parser; 

pub fn start_shell() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new()?;
    println!("Qyvol Shell - Usa 'help' para comandos, 'exit' para salir");

    loop {
        let readline = rl.readline("qyvol> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Split input into command and arguments
                let args: Vec<&str> = line.split_whitespace().collect();
                let command = args[0];

                match command {
                    "exit" => break,
                    "help" => {
                        println!("Comandos disponibles:");
                        println!("  help        - Muestra esta ayuda");
                        println!("  exit        - Sale del shell");
                        println!("  ls/dir      - Lista archivos en el directorio actual");
                        println!("  run <path>  - Ejecuta un módulo .wasm");
                        println!("  deploy <path> [target] - Despliega un módulo .wasm");
                        println!("  cluster <action> [node] - Gestiona el clúster");
                        println!("  shell       - Inicia un nuevo shell (anidado)");
                    }
                    "ls" | "dir" => {
                        // Delegate to system shell
                        let output = if cfg!(target_os = "windows") {
                            Command::new("cmd").args(&["/C", "dir"]).output()
                        } else {
                            Command::new("ls").output()
                        };
                        match output {
                            Ok(output) => {
                                if output.status.success() {
                                    println!("{}", String::from_utf8_lossy(&output.stdout));
                                } else {
                                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                                }
                            }
                            Err(e) => eprintln!("Error ejecutando comando: {}", e),
                        }
                    }
                    _ => {
                        // Try to parse as a CLI command
                        let cli_args = std::iter::once("qyvol").chain(args.iter().copied()).collect::<Vec<&str>>();
                        match Cli::try_parse_from(cli_args) {
                            Ok(cli) => {
                                // Execute the parsed command
                                if let Err(e) = crate::main_with_cli(cli) {
                                    eprintln!("Error: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Comando no reconocido: {}. Usa 'help' para ver comandos.", line);
                                eprintln!("Detalles: {}", e);
                            }
                        }
                    }
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}