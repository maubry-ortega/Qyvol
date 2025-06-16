// # VolleyDevByMaubry [19/∞] "El prompt susurra el estado del mundo, claro y sin sombras."
use crate::shell::context::{ProjectType, ShellContext};
use colored::*;
use std::path::Path;
use strip_ansi_escapes::strip;

#[derive(Clone)]
pub struct QyvolPrompt {
    show_git: bool,
    show_context: bool,
    show_stats: bool,
}

impl QyvolPrompt {
    pub fn new(_context: &ShellContext) -> Self {
        QyvolPrompt {
            show_git: true,
            show_context: true,
            show_stats: true,
        }
    }

    pub fn render(&self, context: &ShellContext) -> String {
        let mut parts = Vec::new();

        parts.push(context.get_project_icon().to_string());
        parts.push("qyvol".bright_blue().bold().to_string());

        let path = self.format_path(&context.current_dir);
        parts.push(path.bright_cyan().to_string());

        if self.show_context && context.project_type != ProjectType::Generic {
            parts.push(
                format!("[{}]", context.get_project_name())
                    .bright_yellow()
                    .to_string(),
            );
        }

        if self.show_stats {
            if let Some(stats) = &context.project_stats {
                if stats.qyv_files > 0 || stats.wasm_files > 0 {
                    parts.push(self.format_stats(stats));
                }
            }
        }

        if self.show_git {
            if let Some(branch) = &context.git_branch {
                parts.push(format!("({})", branch).bright_magenta().to_string());
            }
        }

        let prompt = format!("{} ❯ ", parts.join(" "));
        prompt
    }

    pub fn render_clean(&self, context: &ShellContext) -> String {
        let rendered = self.render(context);
        let raw_bytes = rendered.as_bytes();
        let bytes = strip(raw_bytes);

        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }

    fn format_path(&self, path: &Path) -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        let path_str = path.to_string_lossy();

        if !home.is_empty() && path_str.starts_with(&home) {
            path_str.replacen(&home, "~", 1)
        } else {
            let components: Vec<_> = path.components().collect();
            if components.len() > 3 {
                format!(
                    ".../{}/{}",
                    components[components.len() - 2]
                        .as_os_str()
                        .to_string_lossy(),
                    components[components.len() - 1]
                        .as_os_str()
                        .to_string_lossy()
                )
            } else {
                path_str.to_string()
            }
        }
    }

    fn format_stats(&self, stats: &crate::shell::context::ProjectStats) -> String {
        let mut parts = Vec::new();
        if stats.qyv_files > 0 {
            parts.push(format!("{}qyv", stats.qyv_files));
        }
        if stats.wasm_files > 0 {
            parts.push(format!("{}wasm", stats.wasm_files));
        }
        format!("({})", parts.join(",")).bright_black().to_string()
    }
}
