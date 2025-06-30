# Manifiestos `.qyv`

Los manifiestos `.qyv` son archivos de configuración en formato YAML que definen cómo se ejecuta un módulo WebAssembly en Qyvol. Proporcionan una forma declarativa de especificar la configuración de ejecución, permisos y recursos.

## 📋 Estructura Básica

### Formato General

```yaml
name: nombre-del-modulo
entrypoint: ruta/al/archivo.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false
```

### Campos Requeridos

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| `name` | `string` | Nombre identificativo del módulo | `"hello-qyvol"` |
| `entrypoint` | `string` | Ruta al archivo WASM | `"hello.component.wasm"` |
| `runtime` | `string` | Runtime de ejecución | `"wasi"` |

### Campos Opcionales

| Campo | Tipo | Descripción | Por Defecto |
|-------|------|-------------|-------------|
| `permissions` | `object` | Configuración de permisos | Ver sección de permisos |
| `language` | `string` | Lenguaje de origen | `"rust"` |
| `type` | `string` | Tipo de aplicación | `"cli"` |
| `fs` | `object` | Configuración del sistema de archivos | `"memfs"` |

## 🛡️ Sistema de Permisos

### Estructura de Permisos

```yaml
permissions:
  fs: none        # Acceso al sistema de archivos
  net: false      # Acceso a red
  exec: false     # Ejecución de comandos externos
```

### Tipos de Permisos

#### Permisos de Sistema de Archivos (`fs`)

| Valor | Descripción |
|-------|-------------|
| `none` | Sin acceso al sistema de archivos |
| `read` | Solo lectura de archivos |
| `write` | Lectura y escritura de archivos |
| `read-write` | Acceso completo (alias de `write`) |

#### Permisos de Red (`net`)

| Valor | Descripción |
|-------|-------------|
| `false` | Sin acceso a red |
| `true` | Acceso completo a red |

#### Permisos de Ejecución (`exec`)

| Valor | Descripción |
|-------|-------------|
| `false` | No puede ejecutar comandos externos |
| `true` | Puede ejecutar comandos externos |

## 📁 Configuración del Sistema de Archivos

### Tipos de Sistema de Archivos

```yaml
fs:
  type: memfs      # Tipo de sistema de archivos
  source: null     # Fuente de datos (opcional)
  cache: false     # Habilitar caché (opcional)
```

### Tipos Disponibles

| Tipo | Descripción | Uso |
|------|-------------|-----|
| `memfs` | Sistema de archivos en memoria | Aplicaciones temporales |
| `diskfs` | Sistema de archivos del host | Acceso a archivos locales |
| `netfs` | Sistema de archivos de red | Datos remotos |

### Ejemplos de Configuración

#### Sistema de Archivos en Memoria

```yaml
name: app-temp
entrypoint: temp.component.wasm
runtime: wasi
permissions:
  fs: read-write
  net: false
  exec: false
fs:
  type: memfs
  cache: true
```

#### Sistema de Archivos de Red

```yaml
name: app-remote
entrypoint: remote.component.wasm
runtime: wasi
permissions:
  fs: read
  net: true
  exec: false
fs:
  type: netfs
  source: https://api.example.com/data/
  cache: true
```

## 🏷️ Metadatos del Módulo

### Información del Lenguaje

```yaml
name: rust-app
entrypoint: app.component.wasm
runtime: wasi
language: rust
permissions:
  fs: none
  net: false
  exec: false
```

### Tipos de Aplicación

```yaml
name: gui-app
entrypoint: gui.component.wasm
runtime: wasi
type: gui
permissions:
  fs: read
  net: false
  exec: false
  gfx: true
```

## 📝 Ejemplos Completos

### Aplicación CLI Básica (Rust)

```yaml
name: hello-qyvol
entrypoint: hello.component.wasm
runtime: wasi
language: rust
permissions:
  fs: none
  net: false
  exec: false
```

### Aplicación con Acceso a Archivos

```yaml
name: file-processor
entrypoint: processor.component.wasm
runtime: wasi
language: rust
permissions:
  fs: read-write
  net: false
  exec: false
fs:
  type: diskfs
  cache: true
```

### Aplicación de Red

```yaml
name: web-client
entrypoint: client.component.wasm
runtime: wasi
language: go
permissions:
  fs: read
  net: true
  exec: false
fs:
  type: memfs
```

### Aplicación GUI

```yaml
name: text-editor
entrypoint: editor.component.wasm
runtime: wasi
language: kotlin
type: gui
permissions:
  fs: read-write
  net: false
  exec: false
  gfx: true
fs:
  type: diskfs
```

## 🔍 Validación de Manifiestos

### Validaciones Implementadas

El crate `common` implementa validaciones para:

- **Formato YAML**: Sintaxis correcta
- **Campos requeridos**: Presencia de campos obligatorios
- **Tipos de datos**: Tipos correctos para cada campo
- **Rutas de archivos**: Existencia del archivo WASM
- **Permisos coherentes**: Consistencia en la configuración

### Código de Validación

```rust
#[derive(Debug, Deserialize, PartialEq)]
pub struct Permissions {
    pub fs: Option<String>,
    pub net: bool,
    pub exec: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Manifest {
    pub name: String,
    pub entrypoint: String,
    pub runtime: String,
    pub permissions: Option<Permissions>,
}

impl Manifest {
    pub fn from_file(path: &Path) -> Result<(Manifest, PathBuf), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let manifest: Manifest = serde_yaml::from_str(&content)?;
        let base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Ok((manifest, base_dir))
    }
}
```

## 🧪 Testing de Manifiestos

### Fixtures de Testing

El proyecto incluye fixtures para testing en `common/tests/fixtures/manifest/`:

- `hello.qyv` - Manifiesto básico válido
- `manifest_basic.qyv` - Configuración básica
- `manifest_with_perms.qyv` - Con permisos específicos
- `manifest_null_fs.qyv` - Sin sistema de archivos
- `invalid_manifest.qyv` - Manifiesto inválido para testing

### Ejecutar Tests

```bash
# Tests específicos de manifiestos
cargo test -p common manifest

# Tests de parsing
cargo test -p common parsing

# Tests de validación
cargo test -p common validation
```

## 🔧 Herramientas de Desarrollo

### Generación de Manifiestos

El shell de Qyvol incluye comandos para generar manifiestos:

```bash
# Crear manifiesto para proyecto Rust
template rust my-app

# Crear manifiesto para proyecto Go
template go my-app

# Crear manifiesto personalizado
template custom my-app
```

### Validación en Tiempo Real

```bash
# Validar manifiesto
qyv validate manifest.qyv

# Validar con formato específico
qyv validate manifest.qyv --format json
```

## 🔮 Extensiones Futuras

### Permisos Granulares

```yaml
permissions:
  fs:
    read: ["/data", "/config"]
    write: ["/temp", "/logs"]
    exec: false
  net:
    http: true
    tcp: false
    udp: false
  system:
    env: ["PATH", "HOME"]
    args: true
    stdio: true
```

### Configuración de Recursos

```yaml
resources:
  memory:
    initial: "64MB"
    maximum: "256MB"
  cpu:
    cores: 2
    priority: "normal"
  network:
    bandwidth: "10MB/s"
    latency: "100ms"
```

### Variables de Entorno

```yaml
env:
  DEBUG: "true"
  LOG_LEVEL: "info"
  API_KEY: "${API_KEY}"
  DATABASE_URL: "postgresql://localhost/db"
```

### Dependencias

```yaml
dependencies:
  - name: "database"
    version: "1.0.0"
    source: "qyvolhub://db/postgres"
  - name: "cache"
    version: "2.1.0"
    source: "qyvolhub://cache/redis"
```

## 📚 Referencias

- [YAML Specification](https://yaml.org/spec/)
- [Serde YAML](https://docs.rs/serde_yaml/)
- [WASI Permissions](https://wasi.dev/)
- [WebAssembly Component Model](https://github.com/WebAssembly/component-model) 