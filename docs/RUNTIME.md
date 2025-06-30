# Runtime WASM de Qyvol

El runtime de Qyvol es el motor de ejecución que permite ejecutar módulos WebAssembly de manera segura y eficiente. Está construido sobre `wasmtime` y proporciona integración con WASI para acceso al sistema.

## 🏗️ Arquitectura

### Componentes Principales

```
┌─────────────────────────────────────────────────────────────┐
│                        Runtime                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Engine    │  │   Linker    │  │    Store    │        │
│  │             │  │             │  │             │        │
│  │ • WASM      │  │ • WASI      │  │ • State     │        │
│  │   Engine    │  │   Bindings  │  │ • Context   │        │
│  │ • Component │  │ • Host      │  │ • Resources │        │
│  │   Model     │  │   Functions │  │             │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Loader    │  │   Runner    │  │   Deploy    │        │
│  │             │  │             │  │             │        │
│  │ • File      │  │ • Component │  │ • HTTP      │        │
│  │   Loading   │  │   Execution │  │   Upload    │        │
│  │ • Validation│  │ • Error     │  │ • Cluster   │        │
│  │ • Parsing   │  │   Handling  │  │   Mgmt      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### Flujo de Ejecución

1. **Parsing del Manifiesto**: Se lee y valida el archivo `.qyv`
2. **Carga del Componente**: Se carga el archivo WASM desde disco
3. **Inicialización del Engine**: Se crea el motor WASM con configuración
4. **Configuración WASI**: Se establece el contexto WASI con permisos
5. **Ejecución**: Se ejecuta el componente WASM
6. **Limpieza**: Se liberan recursos y se manejan errores

## 🔧 Motor de Ejecución

### Engine

El `Engine` es el núcleo del runtime que proporciona:

- **Compilación JIT**: Compila WASM a código nativo
- **Optimizaciones**: Aplica optimizaciones de rendimiento
- **Gestión de memoria**: Maneja la memoria del módulo WASM
- **Soporte de componentes**: Compatible con WASM Component Model

```rust
let engine = Engine::default();
```

### Linker

El `Linker` conecta el módulo WASM con el entorno host:

- **Bindings WASI**: Proporciona acceso al sistema operativo
- **Host functions**: Funciones personalizadas del runtime
- **Import/Export**: Gestión de imports y exports del módulo

```rust
let mut linker = Linker::<MyState>::new(&engine);
wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
```

### Store

El `Store` mantiene el estado de ejecución:

- **Contexto WASI**: Información del entorno de ejecución
- **Tabla de recursos**: Gestión de handles y recursos
- **Estado del módulo**: Variables y memoria del módulo

```rust
let state = engine::build_state()?;
let mut store = Store::new(&engine, state);
```

## 🛡️ Sistema de Seguridad

### Sandboxing

Cada módulo WASM se ejecuta en un sandbox aislado:

- **Memoria aislada**: Cada módulo tiene su propia memoria
- **Recursos limitados**: Acceso controlado a recursos del sistema
- **Permisos granulares**: Control fino sobre capacidades

### Permisos

Los permisos se definen en el manifiesto `.qyv`:

```yaml
permissions:
  fs: none        # Acceso al sistema de archivos
  net: false      # Acceso a red
  exec: false     # Ejecución de comandos externos
```

### Validación

El runtime valida:

- **Integridad del módulo**: Verifica que el WASM sea válido
- **Permisos**: Comprueba que los permisos sean coherentes
- **Recursos**: Valida acceso a recursos del sistema

## 📦 Gestión de Componentes

### Carga de Componentes

```rust
pub fn load_component(
    engine: &Engine,
    manifest: &Manifest,
    base_dir: &Path,
) -> Result<Component, ExecutorError> {
    let wasm_path_str = &manifest.entrypoint;
    let path = base_dir.join(wasm_path_str);
    
    let component = Component::from_file(engine, &path)
        .map_err(|e| ExecutorError::ModuleLoad(format!(
            "No se pudo leer el componente desde '{}': {}",
            path.display(),
            e
        )))?;
    
    Ok(component)
}
```

### Ejecución de Componentes

```rust
pub fn run_component(
    store: &mut Store<MyState>,
    linker: &Linker<MyState>,
    component_bytes: &[u8],
) -> Result<()> {
    let component = Component::new(store.engine(), component_bytes)?;
    let command = Command::instantiate(&mut *store, &component, linker)?;
    let result = command.wasi_cli_run().call_run(store)?;
    
    if let Err(()) = result {
        anyhow::bail!("El componente terminó con un código de salida de error.");
    }
    
    Ok(())
}
```

## 🌐 Integración WASI

### Contexto WASI

El runtime proporciona un contexto WASI personalizado:

```rust
pub struct MyState {
    pub wasi_ctx: WasiCtx,
    pub table: ResourceTable,
}

impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
```

### Configuración WASI

```rust
pub fn build_state() -> Result<MyState> {
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build();
    
    Ok(MyState {
        wasi_ctx,
        table: ResourceTable::new()
    })
}
```

## 🚀 Sistema de Despliegue

### Despliegue HTTP

El runtime incluye capacidades de despliegue remoto:

```rust
pub fn deploy_wasm(
    manifest: &Manifest,
    manifest_dir: &Path,
    target: &str,
) -> Result<(), ExecutorError> {
    let entrypoint_path = manifest_dir.join(&manifest.entrypoint);
    
    if !entrypoint_path.exists() {
        return Err(ExecutorError::WasmNotFound(
            entrypoint_path.display().to_string()
        ));
    }
    
    let rt = Runtime::new()
        .map_err(|e| ExecutorError::DeployError(e.to_string()))?;
    
    rt.block_on(async {
        let client = Client::new();
        let bytes = std::fs::read(&entrypoint_path)
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;
        
        println!("🚀 Desplegando {} a {}", manifest.name, target);
        
        client
            .post(format!("{target}/upload"))
            .body(bytes)
            .send()
            .await
            .map_err(|e| ExecutorError::DeployError(e.to_string()))?;
        
        Ok::<(), ExecutorError>(())
    })?;
    
    println!("✅ Despliegue completado a {target}");
    Ok(())
}
```

### Gestión de Clúster

Soporte básico para gestión de clústeres distribuidos:

```rust
pub fn manage_cluster(action: &str, node: Option<String>) -> Result<(), ClusterError> {
    match action {
        "add-node" => {
            let node = node.ok_or(ClusterError::NodeRequired)?;
            println!("Añadiendo nodo: {node}");
            Ok(())
        }
        _ => Err(ClusterError::InvalidAction(action.to_string())),
    }
}
```

## 🔍 Información de Módulos

### Análisis de Componentes

El runtime puede analizar información de componentes WASM:

```rust
pub fn print_component_info(
    component: &Component,
    fmt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nℹ️  Información del Componente:");
    
    let exports: Vec<String> = component
        .component_type()
        .exports(component.engine())
        .map(|(name, _)| name.to_string())
        .collect();
    
    let info = serde_json::json!({ "exports": exports });
    
    match fmt.to_lowercase().as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&info)?),
        "yaml" => println!("{}", serde_yaml::to_string(&info)?),
        _ => {
            println!("📋 Exportaciones de alto nivel del componente:");
            if exports.is_empty() {
                println!("  - (Ninguna)");
            } else {
                for e in &exports {
                    println!("  - {e}");
                }
            }
        }
    }
    Ok(())
}
```

## 🐛 Manejo de Errores

### Tipos de Error

```rust
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
```

### Recuperación de Errores

El runtime implementa estrategias de recuperación:

- **Reintentos automáticos**: Para errores transitorios
- **Fallbacks**: Módulos de respaldo
- **Logging detallado**: Para debugging
- **Graceful degradation**: Degradación elegante

## 📊 Métricas y Monitoreo

### Métricas Disponibles

- **Tiempo de carga**: Tiempo para cargar un módulo WASM
- **Tiempo de ejecución**: Duración de la ejecución
- **Uso de memoria**: Consumo de memoria del módulo
- **Errores**: Tasa de errores y tipos
- **Throughput**: Módulos ejecutados por segundo

### Instrumentación

```rust
// Ejemplo de métricas (futuro)
pub struct RuntimeMetrics {
    pub load_time: Duration,
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub error_count: u64,
}
```

## 🔮 Próximas Funcionalidades

### En Desarrollo

- [ ] **Sistema de permisos granular**: Control fino sobre recursos
- [ ] **Hot reloading**: Recarga en caliente de módulos
- [ ] **Profiling**: Análisis de rendimiento detallado
- [ ] **Debugging**: Herramientas de debugging WASM
- [ ] **Optimizaciones**: Mejoras de rendimiento

### Planificadas

- [ ] **Sistema de archivos virtual**: memfs, netfs, diskfs
- [ ] **Red virtual**: Comunicación entre módulos
- [ ] **Integración con IA**: Modelos ONNX y Julia
- [ ] **Clústeres distribuidos**: Orquestación avanzada
- [ ] **GUI nativa**: Interfaz gráfica con egui/Dioxus

## 🧪 Testing

### Tests Unitarios

```bash
# Ejecutar tests del runtime
cargo test -p runtime

# Tests específicos del executor
cargo test -p runtime executor

# Tests de integración
cargo test --test integration
```

### Fixtures de Testing

Los tests utilizan módulos WASM de ejemplo para validar:

- Carga correcta de componentes
- Ejecución exitosa
- Manejo de errores
- Validación de permisos
- Funcionalidad de despliegue

## 📚 Referencias

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [WASI Specification](https://wasi.dev/)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [WASM Component Model](https://github.com/WebAssembly/component-model) 