# Estructura del Proyecto Qyvol

Qyvol está organizado como un workspace de Rust con los siguientes crates principales:

- **cli/**: Shell interactivo y comandos CLI para ejecutar, desplegar y gestionar módulos.
- **common/**: Tipos, utilidades y lógica compartida (manifiestos, parser YAML, permisos).
- **runtime/**: Motor de ejecución WASM, integración de IA, redes y despliegue.
- **core/**: Núcleo de la lógica de negocio y orquestación.

## Otros directorios
- **ejemplos_funcionales/** y **examples/**: Ejemplos de manifiestos `.qyv` y módulos `.wasm` en Rust, Go, etc.
- **assets/**: Imágenes y recursos gráficos.

Cada crate está desacoplado y se comunica a través de tipos y funciones definidos en `common`.

---

## Diagrama de dependencias

```
cli ─┬─> common
     ├─> runtime
     └─> core
runtime ─┬─> common
core ─┬─> common
```

- `cli` es el punto de entrada y orquesta la ejecución usando `runtime` y `core`.
- `runtime` implementa la ejecución y despliegue de módulos.
- `core` contiene la lógica de negocio principal.
- `common` centraliza los tipos y utilidades compartidas.
