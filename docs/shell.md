# Shell Interactivo de Qyvol

El shell (`qyvol shell`) permite gestionar y explorar módulos de forma interactiva.

## Comandos principales
- `help`: Lista de comandos disponibles.
- `exit`: Salir del shell.
- `ls`/`dir`: Listar archivos del directorio actual.
- `run <path>`: Ejecutar un módulo `.wasm` definido por un manifiesto `.qyv`.
- `deploy <path> [target]`: Desplegar un módulo a un objetivo remoto.
- `cluster <action> [node]`: Gestionar clústeres de nodos Qyvol.
- `shell`: Iniciar un nuevo shell anidado.

## Ejemplo de sesión
```
qyvol> ls
ejemplos_funcionales  cli  common  runtime
qyvol> run ejemplos_funcionales/hello/hello.qyv
▶ Iniciando Qyvol Runtime...
✅ Ejecución completada
qyvol> exit
```

## Implementación
El shell está implementado en el crate `cli`, usando `rustyline` para la entrada interactiva y módulos propios para autocompletado, contexto y UI.
