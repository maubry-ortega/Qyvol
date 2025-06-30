![CI Status](https://github.com/maubry-ortega/Qyvol/actions/workflows/rust-ci.yml/badge.svg)

# Qyvol 🚀

<div align="center">
  <img src="assets/axi.png" alt="Mascota Axi" width="150" style="margin-right: 20px;"/>
  <img src="assets/qyvol-logo.png" alt="Logo Qyvol" width="150"/>
</div>

**Qyvol** es un runtime moderno basado en WebAssembly (WASM) para ejecutar microservicios y aplicaciones portátiles de forma segura. Diseñado con una arquitectura modular y un shell interactivo avanzado.

---

## ✨ ¿Qué hace Qyvol?

- **🚀 Ejecuta módulos WASM** compilados con otras herramientas (Rust, Go, etc.)
- **🛡️ Sistema de permisos** granular para control de acceso
- **🖥️ Shell interactivo** con autocompletado inteligente
- **📦 Despliegue simple** de aplicaciones WASM
- **🔧 Soporte multi-lenguaje** (Rust, Go, y más)

---

## 🚀 Instalación Rápida

```bash
# Instalar Qyvol CLI
cd cli
cargo install --path . --bin qyv

# Ejecutar tu primer módulo WASM
qyv run examples/hello/hello.qyv

# Iniciar shell interactivo
qyv shell
```

---

## 📋 Comandos Principales

| Comando | Descripción |
|---------|-------------|
| `qyv run <archivo.qyv>` | Ejecuta un módulo WASM |
| `qyv shell` | Inicia el shell interactivo |
| `qyv deploy <archivo.qyv> <servidor>` | Despliega un módulo |
| `qyv cluster add-node <nodo>` | Gestiona clústeres |

---

## 🔧 Flujo de Trabajo

### 1. Compilar Código a WASM
Usa herramientas específicas para compilar tu código a módulos WASM:

**Rust:**
```bash
cd ejemplos_funcionales/hello
rustc src/main.rs --target wasm32-wasip2 -o hello.component.wasm
```

**Go:**
```bash
cd ejemplos_funcionales/ejemplo
tinygo build -o main-go.wasm -target wasi .
wasm-tools component new main-go.wasm -o ejemplo.component.wasm --adapt wasi-adapter.wasm
```

### 2. Crear Manifiesto .qyv
Define la configuración de ejecución:

```yaml
name: mi-aplicacion
entrypoint: mi-app.component.wasm
runtime: wasi
permissions:
  fs: none
  net: false
  exec: false
```

### 3. Ejecutar con Qyvol
```bash
qyv run mi-aplicacion.qyv
```

---

## 📖 Documentación

- **[Estructura del Proyecto](docs/ESTRUCTURA.md)** - Arquitectura y organización
- **[CLI y Shell](docs/CLI.md)** - Guía completa de comandos
- **[Runtime WASM](docs/RUNTIME.md)** - Motor de ejecución
- **[Manifiestos .qyv](docs/MANIFESTOS.md)** - Configuración de módulos
- **[Compilación WASM](docs/COMPILACION.md)** - Guía de compilación
- **[Despliegue y Clústeres](docs/DESPLIEGUE.md)** - Despliegue distribuido
- **[Recomendaciones](docs/RECOMENDACIONES.md)** - Mejoras y roadmap

---

## 🎯 Casos de Uso

### Desarrollo Local
```bash
# Compilar código a WASM
cd ejemplos_funcionales/hello
rustc src/main.rs --target wasm32-wasip2 -o hello.component.wasm

# Ejecutar con Qyvol
qyv run examples/hello/hello.qyv
```

### Despliegue en Producción
```bash
# Desplegar módulo compilado
qyv deploy examples/hello/hello.qyv https://mi-servidor.com
```

### Gestión de Clústeres
```bash
# Añadir nodos al clúster
qyv cluster add-node servidor1.example.com
```

---

## 🧪 Ejemplos Incluidos

El proyecto incluye ejemplos listos para usar:

- **Rust**: `examples/hello/` - Módulo básico en Rust
- **Go**: `examples/main/` - Módulo básico en Go
- **Código fuente**: `ejemplos_funcionales/` - Para aprender a compilar

---

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! 

1. Fork el repositorio
2. Crea una rama para tu feature
3. Commit tus cambios
4. Abre un Pull Request

Consulta [RECOMENDACIONES.md](docs/RECOMENDACIONES.md) para áreas de mejora.

---

## 🧙 Autor

**VolleyDevByMaubry**  
Creado con pasión por WebAssembly, inteligencia artificial y computación moderna.

---

## 📜 Licencia

Licencia MIT. Revisa [LICENSE](LICENSE).

_Potenciado por 💡 Rust, 🚀 WebAssembly, 🧠 Julia y más._
