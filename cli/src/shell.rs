use rustyline::Editor;
use std::error::Error;

pub fn start_shell() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("qyvol> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line == "exit" {
                    break;
                }
                if line.is_empty() {
                    continue;
                }
                match line {
                    "help" => println!("Comandos: run, deploy, shell, cluster, exit"),
                    _ => println!("Comando no reconocido: {}", line),
                }
                // TODO: Integrar IA predictiva
            }
            Err(_) => break,
        }
    }
    Ok(())
}