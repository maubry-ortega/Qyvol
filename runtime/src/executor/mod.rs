// # VolleyDevByMaubry [13/∞] "El núcleo del ejecutor orquesta la sinfonía de módulos rebeldes."
pub mod engine;
pub mod errors;
pub mod runner;

pub use errors::ExecutorError;

use common::Manifest;
use std::path::Path;
use wasmtime::component::Linker;
use wasmtime::{Engine, Store};

pub fn run_wasm(
    manifest: &Manifest, manifest_dir: &Path, _format: &str,
) -> Result<(), ExecutorError> {
    let engine = Engine::default();

    let mut linker = Linker::<engine::MyState>::new(&engine);

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    let state = engine::build_state()?;
    let mut store = Store::new(&engine, state);

    let module_path = manifest_dir.join(&manifest.entrypoint);
    let bytes =
        std::fs::read(&module_path).map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    let result = runner::run_component(&mut store, &linker, &bytes);

    match result {
        Ok(()) => {
            println!("\n✅ Ejecución completada.");
            Ok(())
        }
        Err(e) => Err(ExecutorError::ModuleLoad(e.to_string())),
    }
}
