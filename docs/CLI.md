# CLI de Qyvol

El CLI de Qyvol es la interfaz principal para interactuar con el runtime WASM. Proporciona comandos para ejecutar, desplegar y gestionar módulos WASM de manera eficiente.

## 🚀 Instalación

```bash
cd cli
cargo install --path . --bin qyv
```

## 📋 Comandos Principales

### `qyv run <path>`

Ejecuta un módulo WASM especificado por su manifiesto `.qyv`.

```bash
# Ejecutar un módulo específico
qyv run examples/hello/hello.qyv

# Ejecutar con formato de salida específico
qyv run examples/hello/hello.qyv --format json
```

**Parámetros:**
- `path`: Ruta al archivo manifiesto `.qyv`
- `--format`: Formato de salida (text, json, yaml) - por defecto: "text"

### `qyv shell`

Inicia el shell interactivo de Qyvol con autocompletado y navegación avanzada.

```bash
qyv shell
```

**Características del Shell:**
- Autocompletado inteligente
- Historial de comandos
- Detección automática del tipo de proyecto
- Estadísticas en tiempo real
- Prompt contextual personalizado

### `qyv deploy <path> [target]`

Despliega un módulo WASM a un servidor remoto.

```bash
# Desplegar a servidor local
qyv deploy examples/hello/hello.qyv http://localhost:8080

# Desplegar a servidor remoto
qyv deploy examples/hello/hello.qyv https://api.qyvol.io
```

**Parámetros:**
- `path`: Ruta al archivo manifiesto `.qyv`
- `target`: URL del servidor destino (opcional, por defecto: http://localhost:8080)

### `qyv cluster <action> [node]`

Gestiona nodos del clúster distribuido.

```bash
# Añadir un nodo al clúster
qyv cluster add-node node1.example.com

# Listar nodos (futuro)
qyv cluster list-nodes
```

**Acciones disponibles:**
- `add-node`: Añade un nuevo nodo al clúster

## 🖥️ Shell Interactivo

El shell interactivo proporciona una experiencia de desarrollo rica con múltiples comandos integrados.

### Comandos del Shell

| Comando | Alias | Descripción | Uso |
|---------|-------|-------------|-----|
| `run <path>` | `r` | Ejecuta un módulo WASM | `run examples/hello/hello.qyv` |
| `info` | `i` | Muestra información del entorno | `info` |
| `scan` | `s` | Busca módulos WASM automáticamente | `scan` |
| `template <lang> <name>` | `t` | Crea un nuevo proyecto | `template rust my-app` |
| `ls` | `l`, `dir` | Lista archivos en el directorio actual | `ls` |
| `cd <dir>` | `d` | Cambia el directorio actual | `cd examples` |
| `mkdir <name>` | `m` | Crea un directorio | `mkdir my-project` |
| `rm <name>` | `x` | Elimina un archivo o directorio | `rm old-file.txt` |
| `edit <name>` | `e` | Edita un archivo | `edit manifest.qyv` |
| `help` | `h` | Muestra ayuda | `help` |
| `clear` | `c`, `cls` | Limpia la pantalla | `clear` |
| `exit` | - | Sale del shell | `exit` |

### Características Avanzadas

#### Autocompletado Inteligente

El shell incluye autocompletado contextual que sugiere:
- Comandos disponibles
- Archivos `.qyv` en el directorio actual
- Rutas de navegación
- Opciones de comandos

#### Detección de Proyecto

El shell detecta automáticamente el tipo de proyecto:
- 🦀 **Rust**: Detecta `Cargo.toml`
- 🐹 **Go**: Detecta `go.mod`
- 🚀 **Qyvol**: Detecta archivos `.qyv`
- ⚡ **Mixed**: Múltiples tipos de proyecto
- 📁 **Generic**: Proyecto genérico

#### Estadísticas del Proyecto

Muestra estadísticas en tiempo real:
- Número de archivos `.qyv`
- Número de archivos `.wasm`
- Proyectos Rust y Go detectados
- Tamaño total de módulos WASM

#### Navegación Rápida

Comando `z` para navegación rápida a directorios frecuentes:
```bash
z examples    # Navega a directorio examples
z hello       # Navega a directorio que contenga "hello"
```

## 🔧 Configuración

### Variables de Entorno

| Variable | Descripción | Por Defecto |
|----------|-------------|-------------|
| `QYVOL_HOME` | Directorio de configuración | `~/.qyvol` |
| `QYVOL_LOG_LEVEL` | Nivel de logging | `info` |
| `QYVOL_DEFAULT_TARGET` | Servidor de despliegue por defecto | `http://localhost:8080` |

### Archivo de Configuración

El CLI busca configuración en `~/.qyvol/config.toml`:

```toml
[cli]
default_format = "text"
history_size = 1000
auto_complete = true

[deploy]
default_target = "http://localhost:8080"
timeout = 30

[shell]
prompt_style = "detailed"
show_stats = true
```

## 🐛 Manejo de Errores

### Códigos de Error

| Código | Descripción |
|--------|-------------|
| `1` | Error de argumentos inválidos |
| `2` | Error al leer manifiesto |
| `3` | Error de ejecución WASM |
| `4` | Error de despliegue |
| `5` | Error del shell |
| `6` | Error del clúster |

### Logging

El CLI incluye logging detallado que se puede configurar:

```bash
# Habilitar logging detallado
QYVOL_LOG_LEVEL=debug qyv run examples/hello/hello.qyv

# Logging a archivo
qyv run examples/hello/hello.qyv 2> qyvol.log
```

## 📝 Ejemplos de Uso

### Flujo de Desarrollo Típico

```bash
# 1. Iniciar shell
qyv shell

# 2. Explorar proyecto
ls
info

# 3. Crear nuevo proyecto
template rust my-wasm-app
cd my-wasm-app

# 4. Compilar a WASM
cargo build --target wasm32-wasi --release

# 5. Ejecutar con Qyvol
run my-wasm-app.qyv

# 6. Desplegar
deploy my-wasm-app.qyv https://my-server.com
```

### Automatización

```bash
# Script para ejecutar múltiples módulos
#!/bin/bash
for qyv_file in examples/*.qyv; do
    echo "Ejecutando $qyv_file..."
    qyv run "$qyv_file"
done
```

## 🔮 Próximas Funcionalidades

- [ ] Soporte para plugins del CLI
- [ ] Integración con IDEs
- [ ] Comandos de debugging WASM
- [ ] Sistema de templates avanzado
- [ ] Integración con CI/CD
- [ ] Comandos de profiling y performance 