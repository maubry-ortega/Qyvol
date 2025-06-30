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

        Ok(QyvolShell { editor, context, prompt, command_handler })
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
            _ => self.command_handler.execute(command, &args[1..], &mut self.context),
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
