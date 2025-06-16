// # VolleyDevByMaubry [16/∞] "El autocompletado ilumina el camino, guiando cada paso con precisión."
use crate::shell::context::{ShellContext, find_qyv_files};
use rustyline::completion::{Completer, Pair};
use rustyline::{Context, Result};

pub struct QyvolCompleter {
    commands: Vec<(String, String)>,
}

impl QyvolCompleter {
    pub fn new() -> Self {
        QyvolCompleter {
            commands: vec![
                ("run".to_string(), "Ejecuta un módulo WASM".to_string()),
                ("r".to_string(), "Alias para run".to_string()),
                ("deploy".to_string(), "Despliega a remoto".to_string()),
                ("cluster".to_string(), "Gestiona nodos".to_string()),
                ("info".to_string(), "Muestra información del entorno".to_string()),
                ("i".to_string(), "Alias para info".to_string()),
                ("scan".to_string(), "Busca módulos WASM".to_string()),
                ("s".to_string(), "Alias para scan".to_string()),
                ("template".to_string(), "Crea nuevo proyecto".to_string()),
                ("t".to_string(), "Alias para template".to_string()),
                ("ls".to_string(), "Lista archivos".to_string()),
                ("l".to_string(), "Alias para ls".to_string()),
                ("dir".to_string(), "Alias para ls".to_string()),
                ("help".to_string(), "Muestra ayuda".to_string()),
                ("h".to_string(), "Alias para help".to_string()),
                ("clear".to_string(), "Limpia la pantalla".to_string()),
                ("c".to_string(), "Alias para clear".to_string()),
                ("cls".to_string(), "Alias para clear".to_string()),
                ("cd".to_string(), "Cambia el directorio actual".to_string()),
                ("d".to_string(), "Alias para cd".to_string()),
                ("z".to_string(), "Navega rápidamente a directorios".to_string()),
                ("mkdir".to_string(), "Crea un directorio".to_string()),
                ("m".to_string(), "Alias para mkdir".to_string()),
                ("rm".to_string(), "Elimina un archivo o directorio".to_string()),
                ("x".to_string(), "Alias para rm".to_string()),
                ("edit".to_string(), "Edita un archivo".to_string()),
                ("e".to_string(), "Alias para edit".to_string()),
                ("exit".to_string(), "Sale del shell".to_string()),
            ],
        }
    }

    fn complete_path(&self, line: &str, context: &ShellContext) -> Vec<Pair> {
        let mut completions = Vec::new();
        if line.starts_with("run ") || line.starts_with("deploy ") || line.starts_with("r ") {
            let qyv_files = find_qyv_files(&context.current_dir).unwrap_or_default();
            for file in qyv_files {
                let relative = file.strip_prefix(&context.current_dir).unwrap_or(&file);
                let display = relative.to_string_lossy().to_string();
                let replacement = display.clone();
                completions.push(Pair { display, replacement });
            }
        }
        completions
    }
}

impl Completer for QyvolCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let context = ShellContext::new().unwrap_or_else(|_| ShellContext {
            current_dir: std::path::Path::new(".").to_path_buf(),
            project_type: crate::shell::context::ProjectType::Generic,
            project_stats: None,
            git_branch: None,
        });

        let mut completions = Vec::new();

        if line.trim().is_empty() || !line.contains(' ') {
            for (cmd, desc) in &self.commands {
                if cmd.starts_with(line) {
                    completions.push(Pair {
                        display: format!("{} - {}", cmd, desc),
                        replacement: cmd.clone(),
                    });
                }
            }
        } else {
            completions.extend(self.complete_path(line, &context));
        }

        Ok((0, completions))
    }
}

impl rustyline::hint::Hinter for QyvolCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl rustyline::highlight::Highlighter for QyvolCompleter {}

impl rustyline::validate::Validator for QyvolCompleter {}

impl rustyline::Helper for QyvolCompleter {}
