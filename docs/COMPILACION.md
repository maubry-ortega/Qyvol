# Compilación WASM para Qyvol

Qyvol es un **runtime de ejecución** que ejecuta módulos WebAssembly compilados con otras herramientas. Este documento explica cómo compilar código en diferentes lenguajes a módulos WASM compatibles con Qyvol.

## 🎯 Concepto Clave

**Qyvol NO compila código** - solo ejecuta módulos WASM ya compilados. Debes usar herramientas específicas para cada lenguaje:

- **Rust** → `rustc` con target `wasm32-wasip2`
- **Go** → `tinygo` + `wasm-tools`
- **C/C++** → `emscripten` o `clang`
- **Otros** → Herramientas específicas del lenguaje

---

## 🦀 Compilación desde Rust

### Requisitos

- **Rust**: Versión 1.80+ con `rustup`
- **Target WASI**: `wasm32-wasip2`

### Instalación

```bash
# Instalar target WASI P2
rustup target add wasm32-wasip2

# Verificar instalación
rustup show
```

### Compilación Básica

```bash
# Navegar al proyecto Rust
cd mi-proyecto-rust

# Compilar a componente WASI P2
rustc src/main.rs --target wasm32-wasip2 -o mi-app.component.wasm
```

### Configuración Avanzada

**Cargo.toml optimizado:**
```toml
[package]
name = "mi-app"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = "z"  # Minimizar tamaño
strip = true     # Eliminar símbolos
lto = true       # Link-time optimization

[lib]
crate-type = ["cdylib"]  # Para bibliotecas
```

**Compilación con optimizaciones:**
```bash
cargo build --target wasm32-wasip2 --release
```

### Ejemplo Completo

```rust
// src/main.rs
fn main() {
    println!("¡Hola desde Rust + WASM!");
    
    // Tu lógica aquí
    let resultado = procesar_datos();
    println!("Resultado: {}", resultado);
}

fn procesar_datos() -> i32 {
    42
}
```

**Compilar y ejecutar:**
```bash
# Compilar
rustc src/main.rs --target wasm32-wasip2 -o app.component.wasm

# Crear manifiesto
echo "name: mi-app-rust
entrypoint: app.component.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false" > app.qyv

# Ejecutar con Qyvol
qyv run app.qyv
```

---

## 🐹 Compilación desde Go

### Requisitos

- **Go**: Versión 1.17+
- **TinyGo**: Última versión estable
- **Binaryen**: Para `wasm-opt`
- **wasm-tools**: Para adaptar a componentes WASI P2
- **Adaptador WASI**: `wasi_snapshot_preview1.command.wasm`

### Instalación

```bash
# TinyGo (Linux)
wget https://github.com/tinygo-org/tinygo/releases/download/v0.34.0/tinygo_0.34.0_amd64.deb
sudo dpkg -i tinygo_0.34.0_amd64.deb

# Binaryen
wget https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz
tar -xzf binaryen-version_117-x86_64-linux.tar.gz
export PATH=$PATH:./binaryen-version_117/bin

# wasm-tools
cargo install --locked wasm-tools@1.225.0 --force

# Adaptador WASI
wget https://github.com/bytecodealliance/wasmtime/releases/download/v25.0.0/wasi_snapshot_preview1.command.wasm
```

### Compilación Básica

```bash
# Navegar al proyecto Go
cd mi-proyecto-go

# Compilar a módulo WASI P1
tinygo build -o app.wasm -target wasi .

# Adaptar a componente WASI P2
wasm-tools component new app.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm
```

### Configuración Avanzada

**go.mod básico:**
```go
module mi-app

go 1.24.1
```

**Compilación optimizada:**
```bash
# Sin información de debug
tinygo build -o app.wasm -target wasi -no-debug .

# Con optimizaciones adicionales
wasm-opt -O4 app.wasm -o app-optimized.wasm
wasm-tools component new app-optimized.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm
```

### Ejemplo Completo

```go
// main.go
package main

import "fmt"

func main() {
    fmt.Println("¡Hola desde Go + WASM!")
    
    // Tu lógica aquí
    resultado := procesarDatos()
    fmt.Printf("Resultado: %d\n", resultado)
}

func procesarDatos() int {
    return 42
}
```

**Compilar y ejecutar:**
```bash
# Compilar
tinygo build -o app.wasm -target wasi .
wasm-tools component new app.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm

# Crear manifiesto
echo "name: mi-app-go
entrypoint: app.component.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false" > app.qyv

# Ejecutar con Qyvol
qyv run app.qyv
```

---

## 🔧 Otros Lenguajes

### C/C++ con Emscripten

```bash
# Instalar Emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh

# Compilar
emcc main.c -o app.wasm -s WASI=1
wasm-tools component new app.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm
```

### AssemblyScript

```bash
# Instalar AssemblyScript
npm install -g assemblyscript

# Compilar
asc main.ts --target wasi --outFile app.wasm
wasm-tools component new app.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm
```

### Zig

```bash
# Compilar
zig build-exe main.zig -target wasm32-wasi -O ReleaseSmall
wasm-tools component new main.wasm -o app.component.wasm --adapt wasi_snapshot_preview1.command.wasm
```

---

## 📋 Checklist de Compilación

### Antes de Compilar

- [ ] Herramientas instaladas y configuradas
- [ ] Target WASI disponible
- [ ] Código compatible con WASI
- [ ] Dependencias resueltas

### Durante la Compilación

- [ ] Usar target correcto (`wasm32-wasip2` para Rust, `wasi` para Go)
- [ ] Aplicar optimizaciones de tamaño
- [ ] Verificar compatibilidad WASI
- [ ] Adaptar a componente WASI P2 si es necesario

### Después de Compilar

- [ ] Verificar que el archivo .wasm se generó
- [ ] Probar con `wasm-tools validate`
- [ ] Crear manifiesto .qyv apropiado
- [ ] Ejecutar con Qyvol para verificar

---

## 🐛 Solución de Problemas

### Errores Comunes

**Rust: "target not found"**
```bash
rustup target add wasm32-wasip2
```

**Go: "wasm-opt not found"**
```bash
export WASMOPT=/ruta/a/binaryen/bin/wasm-opt
```

**wasm-tools: "invalid component"**
```bash
# Verificar que el adaptador WASI es correcto
wasm-tools validate app.component.wasm
```

### Optimización de Tamaño

**Rust:**
```bash
# En Cargo.toml
[profile.release]
opt-level = "z"
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

**Go:**
```bash
# Compilar sin debug
tinygo build -o app.wasm -target wasi -no-debug -size=short .

# Optimizar con wasm-opt
wasm-opt -O4 -o app-optimized.wasm app.wasm
```

---

## 🔗 Recursos Adicionales

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [WASI Specification](https://wasi.dev/)
- [Rust WASI Guide](https://rustwasm.github.io/docs/wasi/)
- [TinyGo Documentation](https://tinygo.org/docs/)
- [wasm-tools Documentation](https://github.com/bytecodealliance/wasm-tools)
- [Binaryen Documentation](https://github.com/WebAssembly/binaryen)

---

## 📝 Notas Importantes

1. **Qyvol ejecuta componentes WASI P2** - asegúrate de adaptar módulos P1 si es necesario
2. **Los permisos se definen en el manifiesto** - no en el código compilado
3. **Optimiza el tamaño** - módulos más pequeños cargan más rápido
4. **Prueba siempre** - verifica que tu módulo funciona antes de desplegar
5. **Mantén compatibilidad** - usa versiones estables de las herramientas 