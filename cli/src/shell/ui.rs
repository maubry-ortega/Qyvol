// # VolleyDevByMaubry [20/∞] "La interfaz pinta la experiencia, dando vida a cada interacción."
use colored::*;
use std::io::Write;

pub fn print_banner() {
    let banner = r#"
  ██████  ██    ██ ██    ██  ██████  ██      
 ██    ██  ██  ██  ██    ██ ██    ██ ██      
 ██    ██   ████   ██    ██ ██    ██ ██      
 ██ ▄▄ ██    ██     ██  ██  ██    ██ ██      
  ██████     ██      ████    ██████  ███████ 
"#;
    println!("{}", banner.bright_blue().bold());
    println!("{}", "🚀 Qyvol v0.1.0".bright_green().bold());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✅".bright_green(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌".bright_red(), message.bright_red());
}

pub fn print_step(message: &str) {
    print!("{} {}", "▶".bright_cyan(), message);
    std::io::stdout().flush().unwrap();
}

pub struct SimpleTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
}

impl SimpleTable {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
        let column_widths = headers.iter().map(|h| h.len()).collect();
        SimpleTable { headers, rows: Vec::new(), column_widths }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        let row: Vec<String> = row.iter().map(|s| s.to_string()).collect();
        for (i, cell) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(cell.chars().count());
            }
        }
        self.rows.push(row);
    }

    pub fn print(&self) {
        if self.headers.is_empty() { return; }
        let n = self.headers.len();
        // Bordes superiores
        print!("╔");
        for i in 0..n {
            print!("{}", "═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { print!("╦"); } else { println!("╗"); }
        }
        // Encabezados
        print!("║");
        for i in 0..n {
            let h = &self.headers[i];
            print!(" {:<width$} ║", h.bright_white().bold(), width = self.column_widths[i]);
        }
        println!();
        // Separador
        print!("╠");
        for i in 0..n {
            print!("{}", "═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { print!("╬"); } else { println!("╣"); }
        }
        // Filas
        for row in &self.rows {
            print!("║");
            for (i, cell) in row.iter().enumerate().take(n) {
                print!(" {:<width$} ║", cell, width = self.column_widths[i]);
            }
            println!();
        }
        // Borde inferior
        print!("╚");
        for i in 0..n {
            print!("{}", "═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { print!("╩"); } else { println!("╝"); }
        }
    }

    pub fn to_string(&self) -> String {
        if self.headers.is_empty() { return String::new(); }
        let n = self.headers.len();
        let mut out = String::new();
        // Bordes superiores
        out.push('╔');
        for i in 0..n {
            out.push_str(&"═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { out.push('╦'); } else { out.push('╗'); out.push('\n'); }
        }
        // Encabezados
        out.push('║');
        for i in 0..n {
            let h = &self.headers[i];
            out.push_str(&format!(" {:<width$} ║", h, width = self.column_widths[i]));
        }
        out.push('\n');
        // Separador
        out.push('╠');
        for i in 0..n {
            out.push_str(&"═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { out.push('╬'); } else { out.push('╣'); out.push('\n'); }
        }
        // Filas
        for row in &self.rows {
            out.push('║');
            for (i, cell) in row.iter().enumerate().take(n) {
                out.push_str(&format!(" {:<width$} ║", cell, width = self.column_widths[i]));
            }
            out.push('\n');
        }
        // Borde inferior
        out.push('╚');
        for i in 0..n {
            out.push_str(&"═".repeat(self.column_widths[i] + 2));
            if i < n - 1 { out.push('╩'); } else { out.push('╝'); out.push('\n'); }
        }
        out
    }
}

pub fn print_list_item(icon: &str, title: &str, description: Option<&str>) {
    if let Some(desc) = description {
        println!("{} {} - {}", icon, title.bright_white().bold(), desc.dimmed());
    } else {
        println!("{} {}", icon, title.bright_white().bold());
    }
}

pub fn print_info_box(title: &str, items: &[(&str, &str)]) {
    println!("{} {}", "📋".bright_blue(), title.bright_blue().bold());
    println!("{}", "─".repeat(title.len() + 3).bright_black());

    for (key, value) in items {
        println!("  {}: {}", key.bright_white(), value.cyan());
    }
}

pub struct FileTree {
    items: Vec<TreeItem>,
}

#[derive(Debug)]
struct TreeItem {
    name: String,
    is_dir: bool,
    level: usize,
    is_last: bool,
}

impl FileTree {
    pub fn new() -> Self {
        FileTree { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: &str, is_dir: bool, level: usize, is_last: bool) {
        self.items.push(TreeItem { name: name.to_string(), is_dir, level, is_last });
    }

    pub fn print(&self) {
        for item in &self.items {
            let mut prefix = String::new();
            if item.level == 0 {
                prefix.push_str(if item.is_last { "└─ " } else { "├─ " });
            } else {
                for _ in 0..item.level {
                    prefix.push_str("│  ");
                }
                prefix.push_str(if item.is_last { "└─ " } else { "├─ " });
            }

            let icon = if item.is_dir {
                "📁"
            } else if item.name.ends_with(".qyv") {
                "🦊"
            } else if item.name.ends_with(".wasm") {
                "📦"
            } else if item.name.ends_with(".rs") {
                "🦀"
            } else if item.name.ends_with(".go") {
                "🐹"
            } else {
                "📄"
            };

            let name_colored = if item.is_dir {
                item.name.bright_blue().bold()
            } else if item.name.ends_with(".qyv") {
                item.name.bright_yellow()
            } else if item.name.ends_with(".wasm") {
                item.name.bright_magenta()
            } else {
                item.name.normal()
            };

            println!("{}{} {}", prefix.bright_black(), icon, name_colored);
        }
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_help_command(command: &str, description: &str, usage: Option<&str>) {
    println!("  {} {}", command.bright_green().bold(), description);
    if let Some(usage) = usage {
        println!("    {}: {}", "Uso".dimmed(), usage.dimmed());
    }
}

pub fn print_separator() {
    println!("{}", "─".repeat(60).bright_black());
}
