// # VolleyDevByMaubry [12/∞] "Cargar un componente es abrir la puerta a un universo de código universal."
use crate::executor::errors::ExecutorError;
use common::Manifest;
use std::path::Path;
use wasmtime::component::Component;
use wasmtime::Engine;

pub fn load_component(
    engine: &Engine,
    manifest: &Manifest,
    base_dir: &Path,
) -> Result<Component, ExecutorError> {
    let wasm_path_str = &manifest.entrypoint;

    let path = base_dir.join(wasm_path_str);

    let component = Component::from_file(engine, &path).map_err(|e| {
        ExecutorError::ModuleLoad(format!(
            "No se pudo leer el componente desde '{}': {}",
            path.display(),
            e
        ))
    })?;

    Ok(component)
}
