// # VolleyDevByMaubry [11/∞] "Los errores son las cicatrices de una ejecución que desafía lo imposible."
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Archivo WASM no encontrado: {0}")]
    WasmNotFound(String),
    #[error("Error al cargar o ejecutar el módulo: {0}")]
    ModuleLoad(String),
    #[error("Permiso denegado: {0}")]
    PermissionDenied(String),
    #[error("Error de despliegue: {0}")]
    DeployError(String),
    #[error("El módulo finalizó con código de salida no exitoso: {0}")]
    ModuleExit(i32),
}

impl From<wasmtime::Error> for ExecutorError {
    fn from(err: wasmtime::Error) -> Self {
        ExecutorError::ModuleLoad(err.to_string())
    }
}