# Instrucciones para Compilar Módulos WebAssembly para Qyvol

Este README proporciona los pasos para compilar módulos WebAssembly en Rust y Go, dirigidos a la arquitectura `wasm32-wasip2` para su uso con el runtime Qyvol en la carpeta `ejemplos_funcionales`. Los ejemplos incluyen `hello` (Rust) y `ejemplo` (Go), diseñados para ejecutarse como componentes WASI Preview 2 (P2).

## Requisitos Previos

- **Rust**: Instala [Rust](https://www.rust-lang.org/tools/install) usando `rustup` (versión 1.80+ recomendada).
- **TinyGo**: Para el ejemplo en Go, instala [TinyGo](https://tinygo.org/getting-started/install/).
- **Binaryen**: Instala [Binaryen](https://github.com/WebAssembly/binaryen/releases) para `wasm-opt`, requerido por TinyGo.
- **wasm-tools**: Instala con `cargo install wasm-tools` para adaptar módulos Go a componentes WASI P2.
- **Adaptador WASI**: Descarga `wasi_snapshot_preview1.command.wasm` desde las [releases de Wasmtime](https://github.com/bytecodealliance/wasmtime/releases).
- **Wasmtime**: Usa `wasmtime` y `wasmtime-wasi` versión `25.0` (configurado en `runtime/Cargo.toml`).
- Asegúrate de que `cargo` y `rustc` estén disponibles en tu entorno.

## Pasos para la Compilación

### Módulo Rust (`hello`)

1. **Agregar el Target de WebAssembly**
   Instala el target `wasm32-wasip2` para Rust:

   ```bash
   rustup target add wasm32-wasip2
   ```

2. **Verificar la Instalación de Rust**
   Revisa las toolchains y targets instalados:

   ```bash
   rustup show
   ```

3. **Compilar el Proyecto**
   Navega al directorio `ejemplos_funcionales/hello` y compila el código Rust a un componente WASI P2:

   ```bash
   cd ejemplos_funcionales/hello
   rustc src/main.rs --target wasm32-wasip2 -o hello.component.wasm
   ```

4. **Verificar el Archivo Generado**
   El archivo `hello.component.wasm` estará en `ejemplos_funcionales/hello`. Ejecútalo con Qyvol:

   ```bash
   qyv run hello.qyv
   ```

   Ejemplo de `hello.qyv`:

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
