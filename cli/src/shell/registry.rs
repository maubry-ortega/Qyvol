// # VolleyDevByMaubry [38/∞] "El registro de comandos es el corazón extensible de la shell."
use crate::shell::commands::trait_command::Command;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry { commands: HashMap::new() }
    }

    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|b| b.as_ref())
    }

    pub fn execute(
        &self, name: &str, args: &[&str], context: &mut crate::shell::context::ShellContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(cmd) = self.get(name) {
            cmd.execute(args, context)
        } else {
            Err(format!("Comando desconocido: {name}").into())
        }
    }
}
