// # VolleyDevByMaubry [9/∞] "Desplegar un módulo es lanzar una idea al cosmos digital."
use crate::executor::ExecutorError;
use common::Manifest;
use reqwest::Client;
use std::path::Path;
use tokio::runtime::Runtime;

pub fn deploy_wasm(
    manifest: &Manifest,
    manifest_dir: &Path,
    target: &str,
) -> Result<(), ExecutorError> {
    let entrypoint_path = manifest_dir.join(&manifest.entrypoint);

    if !entrypoint_path.exists() {
        return Err(ExecutorError::WasmNotFound(
            entrypoint_path.display().to_string(),
        ));
    }

    let rt = Runtime::new().map_err(|e| ExecutorError::DeployError(e.to_string()))?;
    rt.block_on(async {
        let client = Client::new();
        let bytes = std::fs::read(&entrypoint_path)
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;

        println!("🚀 Desplegando {} a {}", manifest.name, target);

        client
            .post(format!("{}/upload", target))
            .body(bytes)
            .send()
            .await
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;

        Ok::<(), ExecutorError>(())
    })?;

    println!("✅ Despliegue completado a {}", target);
    Ok(())
}
