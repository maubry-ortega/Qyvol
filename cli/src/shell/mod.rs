// # VolleyDevByMaubry [18/∞] "El núcleo de la shell orquesta un baile de comandos con armonía."
pub mod commands;
pub mod completion;
pub mod context;
pub mod prompt;
pub mod registry;
pub mod system;
pub mod ui;

use crate::shell::commands::CommandHandler;
use crate::shell::completion::QyvolCompleter;
use crate::shell::context::ShellContext;
use crate::shell::prompt::QyvolPrompt;
use crate::shell::ui::{print_banner, print_error};
use colored::*;
use rustyline::{
    Config, Editor, Helper, completion::Completer, highlight::Highlighter, hint::Hinter,
    history::MemHistory, validate::Validator,
};
use std::borrow::Cow;
use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command as SysCommand, Stdio};
use rlua::{Lua, Result as LuaResult, Value as LuaValue};

pub use crate::shell::commands::trait_command::Command;
#[allow(unused_imports)]
pub use crate::shell::registry::CommandRegistry;

struct QyvolHelper {
    completer: QyvolCompleter,
    prompt: QyvolPrompt,
}

impl Completer for QyvolHelper {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self, line: &str, _pos: usize, ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, _pos, ctx)
    }
}

impl Hinter for QyvolHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.completer.hint(line, pos, ctx)
    }
}

impl Highlighter for QyvolHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self, prompt: &'p str, default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(prompt)
        } else {
            Cow::Owned(self.prompt.render_clean(&ShellContext::new().unwrap()))
        }
    }
}

impl Validator for QyvolHelper {}

impl Helper for QyvolHelper {}

pub struct QyvolShell {
    editor: Editor<QyvolHelper, MemHistory>,
    context: ShellContext,
    prompt: QyvolPrompt,
    command_handler: CommandHandler,
    plugins: HashMap<String, PathBuf>,
}

impl QyvolShell {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .history_ignore_space(true)
            .max_history_size(1000)?
            .auto_add_history(true)
            .build();
        let history = MemHistory::new();
        let context = ShellContext::new()?;
        let prompt = QyvolPrompt::new(&context);
        let completer = QyvolCompleter::new();
        let helper = QyvolHelper { completer, prompt: prompt.clone() };
        let mut editor = Editor::with_history(config, history)?;
        editor.set_helper(Some(helper));
        let command_handler = CommandHandler::new();
        let plugins = Self::load_plugins();

        Ok(QyvolShell { editor, context, prompt, command_handler, plugins })
    }

    fn load_plugins() -> HashMap<String, PathBuf> {
        let mut plugins = HashMap::new();
        let plugin_dir = PathBuf::from("./plugins");
        if let Ok(entries) = fs::read_dir(&plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let is_lua = path.extension().and_then(|e| e.to_str()) == Some("lua");
                    if is_lua || is_executable(&path) {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            plugins.insert(name.to_string(), path);
                        }
                    }
                }
            }
        }
        plugins
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        print_banner();
        self.print_welcome();

        loop {
            self.context.update()?;
            let prompt_str = self.prompt.render(&self.context);

            let readline = self.editor.readline(&prompt_str);
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if let Err(e) = self.handle_command(line) {
                        print_error(&format!("Error: {e}"));
                    }
                }
                Err(rustyline::error::ReadlineError::Interrupted)
                | Err(rustyline::error::ReadlineError::Eof) => {
                    println!("{}", "¡Hasta luego! 👋".cyan());
                    break;
                }
                Err(e) => {
                    print_error(&format!("Error de entrada: {e}"));
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_command(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        // Soporte para pipelines: ls | grep foo | sort | plugin
        let pipeline: Vec<&str> = input.split('|').map(|s| s.trim()).collect();
        if pipeline.len() == 1 {
            // Comando simple
            let args: Vec<&str> = input.split_whitespace().collect();
            if args.is_empty() {
                return Ok(());
            }
            let command = args[0];
            match command {
                "exit" => {
                    println!("{}", "¡Hasta luego! 👋".cyan());
                    std::process::exit(0);
                }
                _ => {
                    if let Some(plugin_path) = self.plugins.get(command) {
                        // Ejecutar plugin externo
                        return run_plugin(plugin_path, &args[1..], None);
                    }
                    self.command_handler.execute(command, &args[1..], &mut self.context)
                }
            }
        } else {
            // Pipeline
            let mut input_data: Option<String> = None;
            for (i, cmd_str) in pipeline.iter().enumerate() {
                let args: Vec<&str> = cmd_str.split_whitespace().collect();
                if args.is_empty() { continue; }
                let command = args[0];
                if i == 0 {
                    // Primer comando: produce salida
                    if command == "ls" || command == "l" || command == "dir" {
                        use crate::shell::commands::mod_rs_ls_to_string;
                        input_data = Some(mod_rs_ls_to_string(&self.context)?);
                    } else if let Some(plugin_path) = self.plugins.get(command) {
                        input_data = Some(run_plugin_capture(plugin_path, &args[1..], None)?);
                    } else {
                        self.command_handler.execute(command, &args[1..], &mut self.context)?;
                        input_data = None;
                    }
                } else {
                    // Comandos intermedios: consumir input_data
                    if let Some(ref data) = input_data {
                        if command == "grep" && args.len() > 1 {
                            let pattern = args[1];
                            let filtered: String = data
                                .lines()
                                .filter(|line| line.contains(pattern))
                                .map(|line| format!("{}\n", line))
                                .collect();
                            input_data = Some(filtered);
                        } else if command == "cat" {
                            print!("{}", data);
                        } else if let Some(plugin_path) = self.plugins.get(command) {
                            input_data = Some(run_plugin_capture(plugin_path, &args[1..], Some(data))?);
                        } else {
                            println!("Comando '{}' no soporta pipe todavía.", command);
                        }
                    }
                }
            }
            // Al final, si hay output pendiente, mostrarlo
            if let Some(data) = input_data {
                print!("{}", data);
            }
            Ok(())
        }
    }

    fn print_welcome(&self) {
        if let Some(stats) = &self.context.project_stats {
            println!(
                "📊 {} .qyv, {} .wasm",
                stats.qyv_files.to_string().cyan(),
                stats.wasm_files.to_string().cyan()
            );
        }
        println!(
            "🚀 Usa: {} {} {} {}",
            "run".green(),
            "scan".green(),
            "info".green(),
            "help".green()
        );
    }
}

pub fn start_shell() -> Result<(), Box<dyn Error>> {
    let mut shell = QyvolShell::new()?;
    shell.run()
}

// Helpers para plugins externos
fn is_executable(path: &PathBuf) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path).map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        true // En Windows, asumimos que todo .exe/.bat es ejecutable
    }
}

fn run_plugin(path: &PathBuf, args: &[&str], input: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext == "lua" {
            return run_lua_plugin(path, args, input);
        }
    }
    let mut cmd = SysCommand::new(path);
    cmd.args(args);
    if let Some(stdin_data) = input {
        let mut child = cmd.stdin(Stdio::piped()).spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            stdin.write_all(stdin_data.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        print!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        let output = cmd.output()?;
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}

fn run_plugin_capture(path: &PathBuf, args: &[&str], input: Option<&str>) -> Result<String, Box<dyn Error>> {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext == "lua" {
            return run_lua_plugin_capture(path, args, input);
        }
    }
    let mut cmd = SysCommand::new(path);
    cmd.args(args);
    if let Some(stdin_data) = input {
        let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            stdin.write_all(stdin_data.as_bytes())?;
        }
        let output = child.wait_with_output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn run_lua_plugin(path: &PathBuf, args: &[&str], input: Option<&str>) -> Result<(), Box<dyn Error>> {
    let code = std::fs::read_to_string(path)?;
    let lua = Lua::new();
    use std::cell::RefCell;
    let output = RefCell::new(String::new());
    let globals = lua.globals();
    let args_table = lua.create_table()?;
    for (i, arg) in args.iter().enumerate() {
        args_table.set((i + 1) as i64, *arg)?;
    }
    globals.set("args", args_table)?;
    if let Some(stdin_data) = input {
        globals.set("stdin", stdin_data)?;
    } else {
        globals.set("stdin", "")?;
    }
    let result: LuaResult<LuaValue> = lua.load(&code).eval();
    match result {
        Ok(LuaValue::String(s)) => output.borrow_mut().push_str(s.to_str().unwrap_or("")),
        Ok(LuaValue::Integer(i)) => output.borrow_mut().push_str(&i.to_string()),
        Ok(LuaValue::Number(n)) => output.borrow_mut().push_str(&n.to_string()),
        Ok(LuaValue::Boolean(b)) => output.borrow_mut().push_str(&b.to_string()),
        Ok(_) => (),
        Err(e) => output.borrow_mut().push_str(&format!("[Lua error] {}", e)),
    }
    print!("{}", output.into_inner());
    Ok(())
}

fn run_lua_plugin_capture(path: &PathBuf, args: &[&str], input: Option<&str>) -> Result<String, Box<dyn Error>> {
    let code = std::fs::read_to_string(path)?;
    let lua = Lua::new();
    use std::cell::RefCell;
    let output = RefCell::new(String::new());
    let globals = lua.globals();
    let args_table = lua.create_table()?;
    for (i, arg) in args.iter().enumerate() {
        args_table.set((i + 1) as i64, *arg)?;
    }
    globals.set("args", args_table)?;
    if let Some(stdin_data) = input {
        globals.set("stdin", stdin_data)?;
    } else {
        globals.set("stdin", "")?;
    }
    let result: LuaResult<LuaValue> = lua.load(&code).eval();
    match result {
        Ok(LuaValue::String(s)) => output.borrow_mut().push_str(s.to_str().unwrap_or("")),
        Ok(LuaValue::Integer(i)) => output.borrow_mut().push_str(&i.to_string()),
        Ok(LuaValue::Number(n)) => output.borrow_mut().push_str(&n.to_string()),
        Ok(LuaValue::Boolean(b)) => output.borrow_mut().push_str(&b.to_string()),
        Ok(_) => (),
        Err(e) => output.borrow_mut().push_str(&format!("[Lua error] {}", e)),
    }
    Ok(output.into_inner())
}
