// # VolleyDevByMaubry [13/∞] "El núcleo del ejecutor orquesta la sinfonía de módulos rebeldes."
pub mod engine;
pub mod runner;
pub mod errors;

pub use errors::ExecutorError;

use common::Manifest;
use std::path::Path;
use wasmtime::{Engine, Store};
use wasmtime::component::Linker;
use wasmtime_wasi::{add_to_linker_sync, I32Exit};

// Función síncrona normal, sin `async`.
pub fn run_wasm(manifest: &Manifest, manifest_dir: &Path, _format: &str) -> Result<(), ExecutorError> {
    // Engine por defecto, no necesita configuración `async`.
    let engine = Engine::default();

    let mut linker = Linker::<engine::MyState>::new(&engine);
    
    // Usa la versión síncrona para añadir las implementaciones de WASI.
    add_to_linker_sync(&mut linker).map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;
    
    let state = engine::build_state()?;
    let mut store = Store::new(&engine, state);

    let module_path = manifest_dir.join(&manifest.entrypoint);
    let bytes = std::fs::read(&module_path)?;

    // Llama a la función síncrona `run_component`, sin `.await`.
    let result = runner::run_component(&mut store, &linker, &bytes);

    match result {
        Ok(()) => {
            println!("\n✅ Ejecución completada.");
            Ok(())
        },
        Err(e) => {
            if let Some(exit) = e.downcast_ref::<I32Exit>() {
                if exit.0 == 0 {
                    println!("\n✅ Ejecución completada con éxito (código de salida 0).");
                    return Ok(());
                } else {
                    return Err(ExecutorError::ModuleExit(exit.0));
                }
            }
            Err(ExecutorError::ModuleLoad(e.to_string()))
        }
    }
}

impl From<std::io::Error> for ExecutorError {
    fn from(e: std::io::Error) -> Self {
        ExecutorError::ModuleLoad(e.to_string())
    }
}