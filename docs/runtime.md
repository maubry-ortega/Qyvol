# Runtime y Ejecución en Qyvol

El runtime de Qyvol ejecuta módulos WebAssembly usando WASI Preview 2, con soporte para:

- **Ejecución segura**: Permisos declarativos (fs, net, exec, gfx).
- **Sistemas de archivos**: `memfs` (RAM), `diskfs` (host), `netfs` (HTTP/S3).
- **Redes virtuales**: Comunicación interna y P2P (libp2p, tokio::mpsc).
- **Integración de IA**: Soporte para modelos ONNX, TFLite, tract, Julia, etc.
- **Clustering**: Orquestación distribuida (futuro: Elixir).

## Arquitectura
- El crate `runtime` implementa la carga, validación y ejecución de módulos `.wasm`.
- El submódulo `executor/` contiene la lógica de ejecución y manejo de estado.
- El runtime puede ser invocado desde la CLI o el shell interactivo.

## Ejemplo de ejecución
```sh
qyv run ejemplos_funcionales/hello/hello.qyv
```

## Seguridad
El runtime valida los permisos y el manifiesto antes de ejecutar cualquier módulo, garantizando aislamiento y confianza cero.
