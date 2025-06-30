// # VolleyDevByMaubry [17/∞] "El contexto guarda el pulso del sistema, uniendo pasado y presente."
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ShellContext {
    pub current_dir: PathBuf,
    pub project_type: ProjectType,
    pub project_stats: Option<ProjectStats>,
    pub git_branch: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Rust,
    Go,
    Qyvol,
    Mixed,
    Generic,
}

#[derive(Debug, Clone)]
pub struct ProjectStats {
    pub qyv_files: usize,
    pub wasm_files: usize,
    pub rust_projects: usize,
    pub go_projects: usize,
    pub total_size: u64,
}

impl ShellContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let current_dir = std::env::current_dir()?;
        let project_type = detect_project_type(&current_dir)?;
        let project_stats = calculate_project_stats(&current_dir)?;
        let git_branch = detect_git_branch(&current_dir);

        Ok(ShellContext {
            current_dir,
            project_type,
            project_stats: Some(project_stats),
            git_branch,
        })
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.current_dir = std::env::current_dir()?;
        self.project_type = detect_project_type(&self.current_dir)?;
        self.project_stats = Some(calculate_project_stats(&self.current_dir)?);
        self.git_branch = detect_git_branch(&self.current_dir);
        Ok(())
    }

    pub fn get_project_icon(&self) -> &'static str {
        match self.project_type {
            ProjectType::Rust => "🦀",
            ProjectType::Go => "🐹",
            ProjectType::Qyvol => "🚀",
            ProjectType::Mixed => "⚡",
            ProjectType::Generic => "📁",
        }
    }

    pub fn get_project_name(&self) -> &'static str {
        match self.project_type {
            ProjectType::Rust => "rust",
            ProjectType::Go => "go",
            ProjectType::Qyvol => "qyvol",
            ProjectType::Mixed => "mixed",
            ProjectType::Generic => "generic",
        }
    }

    pub fn with_dir<P: AsRef<Path>>(dir: P) -> Result<Self, Box<dyn Error>> {
        let current_dir = dir.as_ref().to_path_buf();
        let project_type = detect_project_type(&current_dir)?;
        let project_stats = calculate_project_stats(&current_dir)?;
        let git_branch = detect_git_branch(&current_dir);

        Ok(ShellContext {
            current_dir,
            project_type,
            project_stats: Some(project_stats),
            git_branch,
        })
    }
}

fn detect_project_type(dir: &Path) -> Result<ProjectType, Box<dyn Error>> {
    let has_cargo = dir.join("Cargo.toml").exists();
    let has_go_mod = dir.join("go.mod").exists();
    let qyv_files = find_qyv_files(dir)?.len();

    if qyv_files > 0 {
        if has_cargo && has_go_mod {
            Ok(ProjectType::Mixed)
        } else if has_cargo {
            Ok(ProjectType::Rust)
        } else if has_go_mod {
            Ok(ProjectType::Go)
        } else {
            Ok(ProjectType::Qyvol)
        }
    } else if has_cargo {
        Ok(ProjectType::Rust)
    } else if has_go_mod {
        Ok(ProjectType::Go)
    } else {
        Ok(ProjectType::Generic)
    }
}

fn calculate_project_stats(dir: &Path) -> Result<ProjectStats, Box<dyn Error>> {
    let qyv_files = find_qyv_files(dir)?;
    let wasm_files = find_wasm_files(dir)?;
    let rust_projects = count_rust_projects(dir)?;
    let go_projects = count_go_projects(dir)?;
    let total_size = calculate_total_size(&wasm_files)?;

    Ok(ProjectStats {
        qyv_files: qyv_files.len(),
        wasm_files: wasm_files.len(),
        rust_projects,
        go_projects,
        total_size,
    })
}

pub fn find_qyv_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut qyv_files = Vec::new();
    find_files_recursive(dir, &mut qyv_files, |path| {
        path.extension().and_then(|s| s.to_str()) == Some("qyv")
    })?;
    Ok(qyv_files)
}

pub fn find_wasm_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut wasm_files = Vec::new();
    find_files_recursive(dir, &mut wasm_files, |path| {
        path.extension().and_then(|s| s.to_str()) == Some("wasm")
    })?;
    Ok(wasm_files)
}

fn find_files_recursive<F>(
    dir: &Path, files: &mut Vec<PathBuf>, predicate: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(&Path) -> bool + Copy,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if !["target", "node_modules", ".git", "dist", "build"].contains(&dir_name) {
                    find_files_recursive(&path, files, predicate)?;
                }
            } else if predicate(&path) {
                files.push(path);
            }
        }
    }
    Ok(())
}

fn count_rust_projects(dir: &Path) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    count_projects_recursive(dir, &mut count, |path| path.join("Cargo.toml").exists())?;
    Ok(count)
}

fn count_go_projects(dir: &Path) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    count_projects_recursive(dir, &mut count, |path| path.join("go.mod").exists())?;
    Ok(count)
}

fn count_projects_recursive<F>(
    dir: &Path, count: &mut usize, predicate: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(&Path) -> bool + Copy,
{
    if dir.is_dir() {
        if predicate(dir) {
            *count += 1;
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if !["target", "node_modules", ".git"].contains(&dir_name) {
                    count_projects_recursive(&path, count, predicate)?;
                }
            }
        }
    }
    Ok(())
}

fn calculate_total_size(files: &[PathBuf]) -> Result<u64, Box<dyn Error>> {
    let mut total = 0;
    for file in files {
        if let Ok(metadata) = fs::metadata(file) {
            total += metadata.len();
        }
    }
    Ok(total)
}

fn detect_git_branch(dir: &Path) -> Option<String> {
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        return None;
    }

    let head_file = git_dir.join("HEAD");
    if let Ok(content) = fs::read_to_string(head_file) {
        if content.starts_with("ref: refs/heads/") {
            return Some(content.trim().replace("ref: refs/heads/", ""));
        }
    }

    None
}

impl ProjectStats {
    pub fn format_size(&self) -> String {
        if self.total_size < 1024 {
            format!("{}B", self.total_size)
        } else if self.total_size < 1024 * 1024 {
            format!("{:.1}KB", self.total_size as f64 / 1024.0)
        } else {
            format!("{:.1}MB", self.total_size as f64 / (1024.0 * 1024.0))
        }
    }
}
