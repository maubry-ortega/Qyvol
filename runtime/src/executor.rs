use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use std::path::{Path, PathBuf};
use common::Manifest;
use crate::printer;

pub fn run_wasm(manifest: &Manifest, manifest_dir: &Path) {
    println!("🔧 Cargando módulo: {}", manifest.name);

    let engine = Engine::default();
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .expect("❌ Error al heredar variables de entorno")
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

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

    let module = match Module::from_file(&engine, &entrypoint_path) {
        Ok(m) => m,
        Err(e) => {
            println!("❌ No se pudo cargar el .wasm: {}", e);
            return;
        }
    };

    printer::print_module_info(&module);

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).expect("❌ Error al configurar linker");

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(e) => {
            println!("❌ Error al instanciar: {}", e);
            return;
        }
    };

    let main = match instance.get_typed_func::<(), ()>(&mut store, "_start") {
        Ok(f) => f,
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

