# Qyvol 🚀

**Qyvol** es una alternativa moderna a Docker basada en WebAssembly (WASM), diseñada para ejecutar funciones y microservicios con tiempos de arranque instantáneos, máxima portabilidad, y sin contenedores tradicionales.

> Ejecuta archivos `.wasm` definidos mediante manifiestos `.qyv`, bajo un runtime propio con soporte para WASI.

---

## 📦 Estructura del proyecto

Este repositorio está organizado como un **workspace de Rust** con los siguientes paquetes:

```
Qyvol/
├── cli/       # Interfaz de línea de comandos (qyvol run ...)
├── common/    # Estructuras compartidas, como Manifest y parser YAML
├── runtime/   # Runtime que ejecuta módulos WebAssembly usando WASI
├── examples/  # Archivos de ejemplo como hello.qyv y hello.wasm
└── hello/     # Código fuente original de ejemplo que compila a hello.wasm
```

---

## ✨ Primer ejemplo: Hello World

Ejecuta tu primer `.wasm` con un manifiesto `.qyv`:

```sh
cargo run -p cli -- run examples/hello.qyv
```

### ✅ Salida esperada

```
▶ Ejecutando Qyvol Runtime...
🔧 Cargando módulo: hello-qyvol
📂 Ruta esperada de WASM: examples/./hello.wasm
📋 Importaciones del módulo:
  - wasi_snapshot_preview1::fd_write
  - ...
📋 Exportaciones del módulo:
  - memory
  - _start
  - __main_void
👋 ¡Hola desde Qyvol WASI!
✅ Ejecución completada
```

---

## 📄 Estructura de un manifiesto `.qyv`

Los manifiestos están escritos en YAML. Ejemplo:

```yaml
name: hello-qyvol
entrypoint: hello.wasm
runtime: wasi
```

---

## ⚙️ Requisitos

* Rust (1.70+ recomendado)
* WASI-compatible WASM binary (`.wasm`)
* `cargo install cargo-edit` (para usar `cargo add`)
* [`wasmtime`](https://github.com/bytecodealliance/wasmtime) como motor de ejecución (integrado en runtime)

---

## 🛠️ Dependencias usadas

* [`serde`](https://crates.io/crates/serde) y [`serde_yaml`](https://crates.io/crates/serde_yaml) – para parseo de manifiestos
* [`clap`](https://crates.io/crates/clap) – para CLI
* [`colored`](https://crates.io/crates/colored) – para salida colorida en terminal
* [`wasmtime`](https://crates.io/crates/wasmtime) – para ejecutar `.wasm` con WASI

---

## 📌 Estado actual

✅ Ya es posible:

* Leer manifiestos `.qyv`
* Cargar un archivo `.wasm`
* Ejecutar funciones WASI
* Mostrar exportaciones/importaciones

🧠 Próximos pasos:

* Soporte para múltiples entornos (`native`, `sandbox`, `edge`)
* CLI avanzada (`qyvol build`, `qyvol deploy`)
* Dashboard web
* Formato `.qyv` compilado
* Repositorio de módulos

---

## 🧙 Autor

Proyecto iniciado por **VolleyDevByMaubry**
🧠 Construido con pasión por la tecnología, contenedores modernos y WebAssembly.

---

## 📜 Licencia

MIT License. Revisa el archivo [LICENSE](LICENSE) para más detalles.
