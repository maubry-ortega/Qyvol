// # VolleyDevByMaubry [6/∞] "Ejecutar WASM es encender la chispa de la computación portátil."
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use thiserror::Error;
use common::Manifest;
use crate::printer;
use reqwest::Client;
use tokio::runtime::Runtime;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Archivo WASM no encontrado: {0}")]
    WasmNotFound(String),
    #[error("Error al cargar módulo: {0}")]
    ModuleLoad(String),
    #[error("Permiso denegado: {0}")]
    PermissionDenied(String),
    #[error("Error al desplegar: {0}")]
    DeployError(String),
    #[error("Error en estilo de progreso: {0}")]
    ProgressStyleError(String),
    #[error("Ejecución interrumpida por límite de tiempo")]
    ExecutionTimeout,
}

pub fn run_wasm(manifest: &Manifest, manifest_dir: &Path, format: &str) -> Result<(), ExecutorError> {
    println!("🔧 Cargando módulo: {}", manifest.name);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .map_err(|e| ExecutorError::ProgressStyleError(e.to_string()))?
    );
    pb.set_message("Inicializando runtime...");

    let mut config = Config::new();
    config.cranelift_opt_level(OptLevel::Speed);
    config.epoch_interruption(true); // Habilitar interrupción por epoch
    let engine = Engine::new(&config).map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    if let Some(permissions) = &manifest.permissions {
        if permissions.net {
            return Err(ExecutorError::PermissionDenied("Acceso a red no permitido".to_string()));
        }
    }

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_env()
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

    // Configurar límite de ejecución con epoch
    store.set_epoch_deadline(100); // Interrumpir después de 100 epochs
    let engine_clone = engine.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(10)); // Incrementar epoch cada 10ms
            engine_clone.increment_epoch();
        }
    });

    let entrypoint_path = if Path::new(&manifest.entrypoint).is_relative() {
        manifest_dir.join(&manifest.entrypoint)
    } else {
        PathBuf::from(&manifest.entrypoint)
    };

    println!("📂 Ruta esperada de WASM: {}", entrypoint_path.display());
    if !entrypoint_path.exists() {
        return Err(ExecutorError::WasmNotFound(entrypoint_path.display().to_string()));
    }

    pb.set_message("Cargando módulo WASM...");
    let module = Module::from_file(&engine, &entrypoint_path)
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    pb.set_message("Mostrando información del módulo...");
    printer::print_module_info(&module, format)
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    pb.set_message("Instanciando módulo...");
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    let main = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(|e| ExecutorError::ModuleLoad(e.to_string()))?;

    pb.set_message("Ejecutando módulo...");
    main.call(&mut store, ()).map_err(|e| {
        if e.to_string().contains("epoch") {
            ExecutorError::ExecutionTimeout
        } else {
            ExecutorError::ModuleLoad(e.to_string())
        }
    })?;

    pb.finish_with_message("Ejecución completada");
    println!("✅ Ejecución completada");
    Ok(())
}

pub fn deploy_wasm(manifest: &Manifest, manifest_dir: &Path, target: &str) -> Result<(), ExecutorError> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .map_err(|e| ExecutorError::ProgressStyleError(e.to_string()))?
    );
    pb.set_message("Preparando despliegue...");

    let entrypoint_path = if Path::new(&manifest.entrypoint).is_relative() {
        manifest_dir.join(&manifest.entrypoint)
    } else {
        PathBuf::from(&manifest.entrypoint)
    };

    if !entrypoint_path.exists() {
        return Err(ExecutorError::WasmNotFound(entrypoint_path.display().to_string()));
    }

    pb.set_message("Enviando módulo...");
    let rt = Runtime::new().map_err(|e| ExecutorError::DeployError(e.to_string()))?;
    rt.block_on(async {
        let client = Client::new();
        let bytes = std::fs::read(&entrypoint_path)
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;
        client
            .post(format!("{}/upload", target))
            .body(bytes)
            .send()
            .await
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;
        Ok(())
    })?;

    pb.finish_with_message("Despliegue completado");
    println!("✅ Despliegue completado a {}", target);
    Ok(())
}