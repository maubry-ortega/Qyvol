// # VolleyDevByMaubry [7/∞] "Imprimir módulos es revelar el alma oculta del código."
use serde::{Deserialize, Serialize};
use serde_yaml;
use wasmtime::component::Component;
use wasmtime::Module;

#[derive(Serialize, Deserialize)]
pub struct ModuleInfo {
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

pub fn print_module_info(module: &Module, fmt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let info = ModuleInfo {
        imports: module
            .imports()
            .map(|i| format!("{}::{}", i.module(), i.name()))
            .collect(),
        exports: module.exports().map(|e| e.name().to_string()).collect(),
    };

    match fmt.to_lowercase().as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&info)?),
        "yaml" => println!("{}", serde_yaml::to_string(&info)?),
        _ => {
            println!("📋 Importaciones del módulo:");
            for i in &info.imports {
                println!("  - {}", i);
            }
            println!("📋 Exportaciones del módulo:");
            for e in &info.exports {
                println!("  - {}", e);
            }
        }
    }
    Ok(())
}

pub fn print_component_info(
    component: &Component,
    fmt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nℹ️  Información del Componente:");

    let exports: Vec<String> = component
        .component_type()
        .exports(component.engine())
        .map(|(name, _)| name.to_string())
        .collect();

    let info = serde_json::json!({ "exports": exports });

    match fmt.to_lowercase().as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&info)?),
        "yaml" => println!("{}", serde_yaml::to_string(&info)?),
        _ => {
            println!("📋 Exportaciones de alto nivel del componente:");
            if exports.is_empty() {
                println!("  - (Ninguna)");
            } else {
                for e in &exports {
                    println!("  - {}", e);
                }
            }
        }
    }
    Ok(())
}
