// # VolleyDevByMaubry [2/∞] "El WASM no arranca. Despierta."
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use common::Manifest;
use std::path::{Path, PathBuf};

pub fn run_wasm(manifest: &Manifest, manifest_dir: &Path) {
    println!("🔧 Cargando módulo: {}", manifest.name);

    // Inicializar el motor
    let engine = Engine::default();

    // Crear el contexto WASI
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .expect("❌ Error al heredar variables de entorno")
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

    // Resolver ruta real del entrypoint
    let entrypoint_path = if Path::new(&manifest.entrypoint).is_relative() {
        manifest_dir.join(&manifest.entrypoint)
    } else {
        PathBuf::from(&manifest.entrypoint)
    };

    println!("📂 Ruta esperada de WASM: {}", entrypoint_path.display());

    if !entrypoint_path.exists() {
        println!("❌ El archivo WASM no existe: {}", entrypoint_path.display());
        return;
    }

    // Cargar el módulo
    let module = match Module::from_file(&engine, &entrypoint_path) {
        Ok(module) => module,
        Err(e) => {
            println!("❌ No se pudo cargar el .wasm: {}", e);
            return;
        }
    };

    // Mostrar importaciones y exportaciones
    println!("📋 Importaciones del módulo:");
    for import in module.imports() {
        println!("  - {}::{}", import.module(), import.name());
    }
    println!("📋 Exportaciones del módulo:");
    for export in module.exports() {
        println!("  - {}", export.name());
    }

    // Crear linker
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).expect("❌ Error al configurar linker");

    // Instanciar
    let instance = match linker.instantiate(&mut store, &module) {
        Ok(instance) => instance,
        Err(e) => {
            println!("❌ Error al instanciar: {}", e);
            return;
        }
    };

    // Ejecutar _start
    let main = match instance.get_typed_func::<(), ()>(&mut store, "_start") {
        Ok(func) => func,
        Err(e) => {
            println!("❌ No se encontró la función _start: {}", e);
            return;
        }
    };

    if let Err(e) = main.call(&mut store, ()) {
        println!("❌ Error al ejecutar: {}", e);
    } else {
        println!("✅ Ejecución completada");
    }
}

