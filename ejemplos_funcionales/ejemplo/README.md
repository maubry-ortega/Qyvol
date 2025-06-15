# Instrucciones para Compilar el Módulo Go para Qyvol

Este README proporciona los pasos para compilar el módulo Go en el directorio `ejemplos_funcionales/ejemplo` a un componente WebAssembly (WASM) compatible con WASI Preview 2 (P2) para el runtime Qyvol.

## Requisitos Previos

Para compilar el módulo Go, instala las siguientes herramientas:

- **Go**: Instala [Go](https://golang.org/doc/install) (versión 1.17+ recomendada) para soportar TinyGo.
- **TinyGo**: Descarga la última versión desde [TinyGo Releases](https://github.com/tinygo-org/tinygo/releases). Por ejemplo, para Linux (amd64):

  ```bash
  wget https://github.com/tinygo-org/tinygo/releases/download/v0.34.0/tinygo_0.34.0_amd64.deb
  sudo dpkg -i tinygo_0.34.0_amd64.deb
  ```

  Verifica la instalación:

  ```bash
  tinygo version
  ```

  _Nota_: Usa la versión adecuada para tu plataforma (e.g., `.tar.gz` para macOS, `.zip` para Windows).

- **Binaryen**: Descarga `wasm-opt` desde [Binaryen Releases](https://github.com/WebAssembly/binaryen/releases) (recomendado: v117 o superior). Por ejemplo, para Linux:

  ```bash
  wget https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz
  tar -xzf binaryen-version_117-x86_64-linux.tar.gz
  ```

  Configura el `PATH`:

  ```bash
  export PATH=$PATH:/ruta/a/binaryen-version_117/bin
  ```

  Para Windows, descomprime el `.zip` y agrega la carpeta `bin` a `%PATH%`:

  ```powershell
  $env:PATH += ";C:\ruta\a\binaryen-version_117\bin"
  ```

  Verifica:

  ```bash
  wasm-opt --version
  ```

- **wasm-tools**: Instala desde [wasm-tools](https://github.com/bytecodealliance/wasm-tools) (versión 1.225.0 recomendada para compatibilidad):

  ```bash
  cargo install --locked wasm-tools@1.225.0 --force
  ```

  Verifica:

  ```bash
  wasm-tools -V
  ```

- **Adaptador WASI**: Descarga `wasi_snapshot_preview1.command.wasm` para Wasmtime v25.0 desde [Wasmtime Releases](https://github.com/bytecodealliance/wasmtime/releases/tag/v25.0.0). Por ejemplo:

  ```bash
  wget https://github.com/bytecodealliance/wasmtime/releases/download/v25.0.0/wasi_snapshot_preview1.command.wasm
  ```

  Guárdalo en una ubicación accesible (e.g., `/ruta/a/wasi-adapter/wasi_snapshot_preview1.command.wasm`).

- **Qyvol CLI**: Asegúrate de tener el CLI `qyv` instalado:
  ```bash
  cd cli
  cargo install --path . --bin qyv
  ```

## Pasos para la Compilación

1. **Navegar al Directorio del Ejemplo**
   Cambia al directorio `ejemplos_funcionales/ejemplo`:

   ```bash
   cd ejemplos_funcionales/ejemplo
   ```

2. **Compilar el Código Go**
   Compila `main.go` a un módulo WASI Preview 1 (P1) con TinyGo:

   ```bash
   tinygo build -o main-go.wasm -target wasi .
   ```

   _Nota_: Si encuentras errores con `wasm-opt`, configura la variable de entorno:

   ```bash
   export WASMOPT=/ruta/a/binaryen-version_117/bin/wasm-opt
   ```

   O en Windows (PowerShell):

   ```powershell
   $env:WASMOPT="C:\ruta\a\binaryen-version_117\bin\wasm-opt.exe"
   ```

3. **Adaptar a Componente WASI P2**
   Convierte el módulo P1 a un componente WASI P2 usando `wasm-tools`:

   ```bash
   wasm-tools component new main-go.wasm -o ejemplo.component.wasm --adapt /ruta/a/wasi_snapshot_preview1.command.wasm
   ```

4. **Verificar y Ejecutar**
   El archivo `ejemplo.component.wasm` estará en `ejemplos_funcionales/ejemplo`. Ejecútalo con Qyvol:
   ```bash
   qyv run ejemplo.qyv
   ```
   Ejemplo de `ejemplo.qyv`:
   ```yaml
   name: ejemplo-go
   entrypoint: ejemplo.component.wasm
   runtime: wasi
   language: go
   permissions:
     fs: none
     net: false
     exec: false
   ```

## Notas

- Asegúrate de estar en `ejemplos_funcionales/ejemplo` al compilar.
- El archivo `ejemplo.component.wasm` es un componente WASI P2, compatible con Qyvol (usando `wasmtime` v25.0).
- Verifica que `runtime/Cargo.toml` incluya:
  ```toml
  wasmtime = "25.0"
  wasmtime-wasi = "25.0"
  ```
- Usa `qyvol shell` para ejecutar el módulo interactivamente:
  ```bash
  qyv shell
  qyvol> run ejemplos_funcionales/ejemplo/ejemplo.qyv
  qyvol> ls
  ```
- Para optimizar el tamaño del módulo, usa el flag `-no-debug` con TinyGo:
  ```bash
  tinygo build -o main-go.wasm -target wasi -no-debug .
  ```
- Si usas Windows, ajusta las rutas (e.g., `C:\ruta\a\wasm-opt.exe`) y usa `\` en lugar de `/`.
