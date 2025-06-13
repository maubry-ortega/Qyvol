# Instrucciones para Compilar WebAssembly

Este README proporciona los pasos para compilar un proyecto en Rust dirigido a la arquitectura `wasm32-wasip1`.

## Requisitos Previos

- Instala [Rust](https://www.rust-lang.org/tools/install) usando `rustup`.
- Asegúrate de que `cargo` esté disponible en tu entorno.

## Pasos para la Compilación

1. **Agregar el Target de WebAssembly**
   Instala el target `wasm32-wasip1` para Rust:

   ```bash
   rustup target add wasm32-wasip1
   ```

2. **Verificar la Instalación de Rust**
   Revisa las toolchains y targets instalados de Rust:

   ```bash
   rustup show
   ```

3. **Compilar el Proyecto**
   Compila el proyecto en modo release para el target `wasm32-wasip1`:

   ```bash
   cargo build --target wasm32-wasip1 --release
   ```

4. **Copiar el Archivo Generado**
   Copia el binario WebAssembly generado al directorio de ejemplos:
   ```bash
   cp target/wasm32-wasip1/release/hello.wasm ../examples/
   ```

## Notas

- Asegúrate de estar en el directorio del proyecto al ejecutar estos comandos.
- El archivo `hello.wasm` estará ubicado en el directorio `../examples/` después de completar los pasos.
