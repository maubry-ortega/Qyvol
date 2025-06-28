# Manifiestos `.qyv`

Los manifiestos `.qyv` definen cómo se ejecuta un módulo WebAssembly en Qyvol. Están escritos en YAML y especifican:

- Nombre del módulo
- Entrypoint (`.wasm`)
- Runtime (ej: `wasi`)
- Lenguaje de origen
- Permisos (fs, net, exec, gfx, etc.)
- Tipo de aplicación (CLI, GUI, etc.)
- Sistema de archivos (memfs, diskfs, netfs)

## Ejemplo básico (Rust CLI)
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

## Ejemplo GUI (Kotlin)
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

## Validación y parseo
El crate `common` implementa la validación y parseo de estos manifiestos, asegurando que los permisos y rutas sean correctos antes de ejecutar el módulo.
