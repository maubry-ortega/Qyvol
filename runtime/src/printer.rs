use wasmtime::Module;

pub fn print_module_info(module: &Module) {
    println!("📋 Importaciones del módulo:");
    for import in module.imports() {
        println!("  - {}::{}", import.module(), import.name());
    }

    println!("📋 Exportaciones del módulo:");
    for export in module.exports() {
        println!("  - {}", export.name());
    }
}

