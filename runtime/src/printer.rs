// # VolleyDevByMaubry [7/∞] "Imprimir módulos es revelar el alma oculta del código."
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize)]
pub struct ModuleInfo {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

pub fn print_module_info(module: &Module, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    let info = ModuleInfo {
        imports: module.imports().map(|i| format!("{}::{}", i.module(), i.name())).collect(),
        exports: module.exports().map(|e| e.name().to_string()).collect(),
    };

    match format.to_lowercase().as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&info)?),
        "yaml" => println!("{}", serde_yaml::to_string(&info)?),
        _ => {
            println!("📋 Importaciones del módulo:");
            for import in &info.imports {
                println!("  - {}", import);
            }
            println!("📋 Exportaciones del módulo:");
            for export in &info.exports {
                println!("  - {}", export);
            }
        }
    }
    Ok(())
}