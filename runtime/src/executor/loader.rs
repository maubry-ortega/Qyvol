// # VolleyDevByMaubry [12/∞] "Cargar un componente es abrir la puerta a un universo de código universal."
use wasmtime::component::Component;
use wasmtime::Engine;
use common::Manifest;
use crate::executor::errors::ExecutorError;
use std::path::Path;

/// Carga un archivo Wasm como un Componente de Wasmtime.
pub fn load_component(
    engine: &Engine,
    manifest: &Manifest,
    base_dir: &Path,
) -> Result<Component, ExecutorError> {
    // Corregido: El campo que usas en tu main.rs es `entrypoint`.
    // `manifest.entrypoint` es un `String`, no un `Option<String>`, así que `as_ref` no es necesario.
    let wasm_path_str = &manifest.entrypoint;

    let path = base_dir.join(wasm_path_str);
    
    let component = Component::from_file(engine, &path).map_err(|e| {
        ExecutorError::ModuleLoad(format!("No se pudo leer el componente desde '{}': {}", path.display(), e))
    })?;

    Ok(component)
}