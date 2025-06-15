# Qyvol 🚀

<div align="center">
  <img src="assets/axi.png" alt="Mascota Axi" width="150" style="margin-right: 20px;"/>
  <img src="assets/qyvol-logo.png" alt="Logo Qyvol" width="150"/>
</div>

**Qyvol** es un runtime moderno basado en WebAssembly (WASM), diseñado para ejecutar microservicios y aplicaciones portátiles, seguras y potenciadas por IA con tiempos de arranque instantáneos. Inspirado en Docker, pero optimizado para WASM, Qyvol utiliza manifiestos `.qyv` y contenedores `.qyvbin` para orquestar módulos `.wasm`, ofreciendo una alternativa ligera a los contenedores tradicionales. Con soporte para una arquitectura poliglota (Rust, Go, Julia, Kotlin, Nim), Qyvol es ideal para entornos edge, navegadores, servidores y dispositivos embebidos.

> Ejecuta módulos `.wasm` definidos por manifiestos `.qyv` con WASI Preview 2, potenciados por orquestación basada en IA, seguridad avanzada y sistemas de archivos modulares.

---

## 📦 Estructura del Proyecto

El repositorio está organizado como un **workspace de Rust** con los siguientes paquetes:

```
Qyvol/
├── cli/                 # Interfaz de línea de comandos (qyvol run, deploy, shell)
├── common/              # Estructuras compartidas (manifiestos, parser YAML, permisos)
├── runtime/             # Runtime WASM con WASI, integración de IA y redes
├── ejemplos_funcionales/ # Ejemplos de manifiestos (.qyv) y código fuente (Rust, Go)
└── contrib/             # Código fuente de ejemplo (Rust, Julia, Kotlin) para módulos .wasm
```

---

## ✨ Primer Ejemplo: Hello World

Para ejecutar un módulo "Hello World" en Rust o Go, primero instala el CLI `qyvol`:

```sh
cd cli
cargo install --path . --bin qyv
```

### Ejemplo en Rust

Ejecuta el módulo Rust desde `ejemplos_funcionales/hello`:

```sh
qyv run ejemplos_funcionales/hello/hello.qyv
```

### Ejemplo en Go

Ejecuta el módulo Go desde `ejemplos_funcionales/ejemplo`:

```sh
qyv run ejemplos_funcionales/ejemplo/ejemplo.qyv
```

### ✅ Salida Esperada (Rust)

```
▶ Iniciando Qyvol Runtime...
🔧 Cargando módulo: hello-qyvol
📂 Ruta WASM: ejemplos_funcionales/hello/hello.component.wasm
📋 Lenguaje: Rust
📋 Importaciones:
  - wasi_snapshot_preview1::fd_write
  - ...
📋 Exportaciones:
  - memory
  - _start
  - __main_void
🔍 Permisos: fs:none, net:false, exec:false
👋 ¡Hola desde Qyvol WASI!
✅ Ejecución completada
```

### 📄 Ejemplo de Manifiesto `.qyv` (Rust)

```yaml
name: hello-qyvol
entrypoint: hello.component.wasm
runtime: wasi
language: rust
permissions:
  fs: none
  net: false
  exec: false
fs:
  type: memfs
```

---

## 🖥️ Shell Interactivo

Qyvol incluye un shell interactivo para gestionar módulos y explorar el entorno. Inicia el shell con:

```sh
qyv shell
```

### Comandos Disponibles

- `help`: Muestra la lista de comandos.
- `exit`: Sale del shell.
- `ls` o `dir`: Lista los archivos en el directorio actual (compatible con Linux/macOS y Windows).
- `run <path>`: Ejecuta un módulo `.wasm` definido por un manifiesto `.qyv`.
- `deploy <path> [target]`: Despliega un módulo `.wasm` a un objetivo remoto.
- `cluster <action> [node]`: Gestiona un clúster de nodos Qyvol.
- `shell`: Inicia un nuevo shell (anidado).

Ejemplo de uso:

```
qyvol> ls
ejemplos_funcionales  contrib  cli  common  runtime
qyvol> run ejemplos_funcionales/hello/hello.qyv
▶ Iniciando Qyvol Runtime...
...
✅ Ejecución completada
qyvol> exit
```

---

## 🌟 Características Principales

- **Soporte Poliglota**: Ejecuta módulos `.wasm` compilados desde Rust, Go, Julia, Kotlin, Nim y más.
- **Integración de IA**: Ejecuta modelos de IA embebidos (ONNX, TFLite) con Julia, tract, linfa o burn para clasificación, predicción y optimización.
- **Seguridad Basada en Capacidades**: Permisos declarativos en `.qyv` garantizan ejecución con confianza cero.
- **Sistema de Archivos Modular**: Soporte para `memfs` (RAM), `diskfs` (WASI host) y `netfs` (HTTP/WebDAV/S3).
- **Redes Virtuales**: Comunicación interna vía `service://nombre/puerto` con `tokio::mpsc` y sincronización P2P usando libp2p/WebRTC.
- **Shell Predictivo**: CLI potenciada por IA (`qyvol shell`) con sugerencias contextuales.
- **Clustering Distribuido**: Orquesta instancias de Qyvol con Elixir para sincronización en edge y nube.
- **Soporte para GUI**: Ejecuta aplicaciones gráficas con Kotlin Multiplatform, Dioxus o egui, renderizadas vía WebGPU.
- **Contenedores Portátiles**: Formato `.qyvbin` empaqueta `.wasm`, manifiestos y recursos firmados para despliegue instantáneo.

---

## 📄 Formato de Manifiesto `.qyv`

Los manifiestos están escritos en YAML y definen la ejecución, permisos y entorno del módulo. Ejemplo para una aplicación gráfica:

```yaml
name: editor-texto
entrypoint: editor.wasm
runtime: wasi
language: kotlin
permissions:
  fs: read-write
  net: false
  gfx: true
type: gui
```

---

## ⚙️ Requisitos

- Rust (1.80+ recomendado)
- Binarios `.wasm` compatibles con WASI Preview 2
- `cargo install cargo-edit` (para `cargo add`)
- `wasmtime` (v25.0, integrado en el runtime)
- TinyGo (para módulos Go)
- Binaryen (para `wasm-opt`, requerido por TinyGo)
- `wasm-tools` (para adaptar módulos Go a WASI P2)
- Adaptador WASI (`wasi_snapshot_preview1.command.wasm` para Go)
- Opcional: Julia (para módulos de IA), Kotlin (para GUI), Elixir (para clustering)

---

## 🛠️ Dependencias

- [`serde`](https://crates.io/crates/serde), [`serde_yaml`](https://crates.io/crates/serde_yaml) – Parseo de YAML
- [`clap`](https://crates.io/crates/clap) – Parseo de argumentos CLI
- [`colored`](https://crates.io/crates/colored) – Salida colorida en terminal
- [`wasmtime`](https://crates.io/crates/wasmtime) – Ejecución WASM con WASI (v25.0)
- [`tokio`](https://crates.io/crates/tokio) – Runtime asíncrono para redes
- [`reqwest`](https://crates.io/crates/reqwest) – HTTP para `netfs`
- [`tract-onnx`](https://crates.io/crates/tract-onnx) – Ejecución de modelos de IA
- [`libp2p`](https://crates.io/crates/libp2p) – Redes P2P
- [`ring`](https://crates.io/crates/ring) – Firmas criptográficas para `.qyvbin`
- [`rustyline`](https://crates.io/crates/rustyline) – Shell interactivo

---

## 📌 Estado Actual

✅ **Funcionalidades Implementadas**:

- Parseo de manifiestos `.qyv`
- Ejecución de módulos `.wasm` con WASI Preview 2 (Rust y Go)
- Visualización de importaciones/exportaciones de módulos
- CLI avanzada (`qyvol run`, `qyvol shell`)
- Executor modular (`runtime/src/executor/`)
- Ejemplos funcionales en Rust y Go (`ejemplos_funcionales/`)

🧠 **Próximos Pasos**:

- CLI avanzada (`qyvol deploy`, `qyvol cluster`)
- Soporte para contenedores `.qyvbin` con firmas digitales
- Planificador basado en IA con Julia/linfa
- Sistema de archivos modular (`memfs`, `diskfs`, `netfs`)
- Redes virtuales con `tokio::mpsc` y libp2p
- Integración de GUI con Kotlin/Dioxus/egui
- Clustering distribuido con Elixir
- Dashboard web para monitoreo
- QyvolHub para módulos `.wasm` públicos

---

## 🧪 Creando Módulos para Qyvol

Qyvol utiliza el Modelo de Componentes de WebAssembly (WASI Preview 2). Los archivos `.wasm` deben ser componentes compatibles.

### Módulos desde Rust

1. Instala el target:

   ```bash
   rustup target add wasm32-wasip2
   ```

2. Compila el código:

   ```bash
   cd ejemplos_funcionales/hello
   rustc src/main.rs --target wasm32-wasip2 -o hello.component.wasm
   ```

3. Ejecuta con Qyvol:
   ```bash
   qyv run hello.qyv
   ```

### Módulos desde Go

1. Instala TinyGo, Binaryen, y `wasm-tools` (ver [Requisitos](#⚙️-requisitos)).
2. Compila el código:

   ```bash
   cd ejemplos_funcionales/ejemplo
   tinygo build -o main-go.wasm -target wasi .
   ```

3. Adapta a componente WASI P2:

   ```bash
   wasm-tools component new main-go.wasm -o ejemplo.component.wasm --adapt /ruta/a/wasi_snapshot_preview1.command.wasm
   ```

4. Ejecuta con Qyvol:
   ```bash
   qyv run ejemplo.qyv
   ```

Consulta `ejemplos_funcionales/hello/README.md` y `ejemplos_funcionales/ejemplo/README.md` para detalles completos.

---

## 🧪 Roadmap de Desarrollo

1. **Fase 1: Runtime Base (2-3 meses)**

   - Comandos CLI avanzados
   - Soporte para `.qyvbin`
   - Integración básica de IA (tract, Julia)

2. **Fase 2: Redes y Seguridad (3-4 meses)**

   - Redes virtuales (`tokio::mpsc`, libp2p)
   - Permisos basados en capacidades
   - Sistema de archivos modular

3. **Fase 3: IA y GUI (3-5 meses)**

   - Shell predictivo con Julia
   - Planificador basado en IA
   - Soporte para GUI (Kotlin, Dioxus)

4. **Fase 4: Ecosistema (3-5 meses)**

   - Repositorio público QyvolHub
   - Clustering distribuido con Elixir
   - Dashboard web y modo educativo

**Total**: 12-18 meses para un MVP completo, reducible con contribuciones open source.

---

## 🧙 Autor

**VolleyDevByMaubry**  
Creado con pasión por WebAssembly, inteligencia artificial y computación moderna.

---

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para guías sobre cómo enviar issues, pull requests o nuevos módulos `.wasm`.

---

## 📜 Licencia

Licencia MIT. Revisa [LICENSE](LICENSE) para más detalles.

---

_Potenciado por 💡 Rust, 🚀 WebAssembly, 🧠 Julia y más. Construido para el futuro de la computación._
