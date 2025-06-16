// # VolleyDevByMaubry [23/∞] "La navegación fluye libre, saltando entre mundos con agilidad."
use crate::shell::context::ShellContext;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

pub fn execute_cd(args: &[&str], context: &mut ShellContext) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("Uso: cd <directorio>".into());
    }
    let path = Path::new(args[0]);
    let new_dir =
        if path.is_absolute() { path.to_path_buf() } else { context.current_dir.join(path) };
    if new_dir.is_dir() {
        std::env::set_current_dir(&new_dir)?;
        context.update()?;
        Ok(())
    } else {
        Err(format!("Directorio no encontrado: {}", args[0]).into())
    }
}

pub fn execute_z(args: &[&str], context: &mut ShellContext) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("Uso: z <patrón>".into());
    }
    let pattern = args[0].to_lowercase();
    let home = std::env::var("HOME").unwrap_or_default();
    let search_dirs = vec![
        context.current_dir.clone(),
        Path::new(&home).to_path_buf(),
        context.current_dir.join("examples"),
        context.current_dir.join("ejemplos_funcionales"),
    ];

    for dir in search_dirs {
        if !dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(&pattern) && entry.path().is_dir() {
                std::env::set_current_dir(entry.path())?;
                context.update()?;
                return Ok(());
            }
        }
    }
    Err(format!("No se encontró un directorio que coincida con: {}", pattern).into())
}

pub fn execute_mkdir(args: &[&str], context: &mut ShellContext) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("Uso: mkdir <nombre>".into());
    }
    let path = context.current_dir.join(args[0]);
    fs::create_dir(&path)?;
    context.update()?;
    println!("✅ Directorio '{}' creado.", args[0]);
    Ok(())
}

pub fn execute_rm(args: &[&str], context: &mut ShellContext) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("Uso: rm <nombre>".into());
    }
    let path = context.current_dir.join(args[0]);
    if path.is_dir() {
        fs::remove_dir_all(&path)?;
    } else if path.is_file() {
        fs::remove_file(&path)?;
    } else {
        return Err(format!("No existe: {}", args[0]).into());
    }
    context.update()?;
    println!("✅ '{}' eliminado.", args[0]);
    Ok(())
}

pub fn execute_edit(args: &[&str], context: &mut ShellContext) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("Uso: edit <nombre>".into());
    }
    let path = context.current_dir.join(args[0]);
    if !path.exists() {
        File::create(&path)?;
    }
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "notepad".to_string());
    Command::new(editor).arg(&path).spawn()?.wait()?;
    context.update()?;
    Ok(())
}
