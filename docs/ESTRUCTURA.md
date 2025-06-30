# Estructura del Proyecto Qyvol

## 🏗️ Arquitectura General

Qyvol es un **runtime de ejecución** que ejecuta módulos WebAssembly compilados con herramientas externas. La arquitectura se divide en tres componentes principales:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI & Shell   │    │   Runtime WASM  │    │   Common Types  │
│                 │    │                 │    │                 │
│ • Comandos      │◄──►│ • Ejecutor      │◄──►│ • Manifiestos   │
│ • Shell         │    │ • Estado        │    │ • Utilidades    │
│ • Autocompletado│    │ • Permisos      │    │ • Validación    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Módulos WASM  │
                    │   (Externos)    │
                    │                 │
                    │ • Rust (.wasm)  │
                    │ • Go (.wasm)    │
                    │ • C/C++ (.wasm) │
                    └─────────────────┘
```

---

## 📁 Estructura de Directorios

```
Qyvol/
├── 📦 cli/                    # Interfaz de línea de comandos
│   ├── src/
│   │   ├── main.rs           # Punto de entrada CLI
│   │   └── shell/            # Shell interactivo
│   │       ├── commands/     # Comandos del shell
│   │       ├── completion.rs # Autocompletado
│   │       ├── context.rs    # Contexto de ejecución
│   │       ├── prompt.rs     # Prompt personalizado
│   │       └── ui.rs         # Interfaz de usuario
│   └── Cargo.toml
│
├── 🚀 runtime/               # Motor de ejecución WASM
│   ├── src/
│   │   ├── lib.rs           # API pública del runtime
│   │   ├── executor/        # Sistema de ejecución
│   │   │   ├── engine.rs    # Motor WASM (Wasmtime)
│   │   │   ├── runner.rs    # Ejecutor de módulos
│   │   │   ├── state.rs     # Gestión de estado
│   │   │   └── loader.rs    # Cargador de módulos
│   │   ├── deploy.rs        # Sistema de despliegue
│   │   ├── cluster.rs       # Gestión de clústeres
│   │   └── printer.rs       # Formateo de salida
│   └── Cargo.toml
│
├── 🔧 common/               # Tipos y utilidades compartidas
│   ├── src/
│   │   ├── lib.rs          # API pública
│   │   └── manifest.rs     # Definición de manifiestos
│   ├── tests/              # Tests de integración
│   └── Cargo.toml
│
├── 📚 docs/                 # Documentación
│   ├── estructura.md       # Este archivo
│   ├── CLI.md             # Guía de comandos
│   ├── RUNTIME.md         # Documentación del runtime
│   ├── MANIFESTOS.md      # Especificación de manifiestos
│   ├── COMPILACION.md     # Guía de compilación WASM
│   └── RECOMENDACIONES.md # Mejoras y roadmap
│
├── 🧪 examples/            # Ejemplos de manifiestos .qyv
│   ├── hello/             # Ejemplo básico Rust
│   └── main/              # Ejemplo básico Go
│
├── 🔨 ejemplos_funcionales/ # Código fuente para compilar
│   ├── hello/             # Proyecto Rust
│   │   ├── src/main.rs    # Código fuente Rust
│   │   ├── Cargo.toml     # Configuración Rust
│   │   └── README.md      # Instrucciones de compilación
│   └── ejemplo/           # Proyecto Go
│       ├── main.go        # Código fuente Go
│       ├── go.mod         # Configuración Go
│       └── README.md      # Instrucciones de compilación
│
├── 🎨 assets/              # Recursos gráficos
│   ├── qyvol-logo.png     # Logo principal
│   ├── axi.png           # Mascota Axi
│   └── axis.png          # Variante de mascota
│
├── 📋 Cargo.toml          # Configuración del workspace
├── 📄 README.md           # Documentación principal
├── 📜 LICENSE             # Licencia MIT
└── 🔧 rustfmt.toml        # Configuración de formato
```

---

## 🔄 Flujo de Trabajo

### 1. Desarrollo de Código
Los desarrolladores escriben código en sus lenguajes preferidos:

```rust
// ejemplos_funcionales/hello/src/main.rs
fn main() {
    println!("¡Hola desde Rust!");
}
```

```go
// ejemplos_funcionales/ejemplo/main.go
package main

import "fmt"

func main() {
    fmt.Println("¡Hola desde Go!")
}
```

### 2. Compilación a WASM
Se usan herramientas específicas para compilar a módulos WASM:

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

### 3. Creación de Manifiesto
Se define la configuración de ejecución en un archivo `.qyv`:

```yaml
# examples/hello/hello.qyv
name: hello-qyvol
entrypoint: hello.component.wasm
runtime: wasi
language: rust
permissions:
  fs: none
  net: false
  exec: false
```

### 4. Ejecución con Qyvol
El runtime Qyvol ejecuta el módulo WASM:

```bash
qyv run examples/hello/hello.qyv
```

---

## 🧩 Componentes Principales

### CLI (`cli/`)
**Propósito**: Interfaz de usuario para interactuar con Qyvol

**Características**:
- Comandos de línea de comandos (`run`, `deploy`, `cluster`)
- Shell interactivo con autocompletado
- Gestión de contexto de ejecución
- Interfaz de usuario amigable

**Flujo**:
```
Usuario → CLI → Runtime → Módulo WASM
```

### Runtime (`runtime/`)
**Propósito**: Motor de ejecución de módulos WASM

**Componentes**:
- **Executor**: Ejecuta módulos WASM usando Wasmtime
- **State**: Gestiona el estado de ejecución
- **Loader**: Carga y valida módulos WASM
- **Deploy**: Sistema de despliegue remoto
- **Cluster**: Gestión de clústeres distribuidos

**Flujo de Ejecución**:
```
1. Cargar manifiesto .qyv
2. Validar permisos
3. Cargar módulo WASM
4. Ejecutar con Wasmtime
5. Gestionar entrada/salida
6. Limpiar recursos
```

### Common (`common/`)
**Propósito**: Tipos y utilidades compartidas

**Funciones**:
- Definición de estructuras de manifiestos
- Validación de configuración
- Utilidades de serialización
- Tests de integración

---

## 🔐 Sistema de Permisos

Qyvol implementa un sistema de permisos granular:

```yaml
permissions:
  fs: none | read | write | full    # Acceso al sistema de archivos
  net: false | true                 # Acceso a red
  exec: false | true                # Ejecución de comandos
```

**Niveles de Seguridad**:
- **Sandbox**: Sin acceso a recursos externos
- **Restringido**: Acceso limitado y controlado
- **Completo**: Acceso total (solo para desarrollo)

---

## 🚀 Despliegue y Clústeres

### Despliegue Individual
```bash
qyv deploy app.qyv https://servidor.com
```

### Gestión de Clústeres
```bash
qyv cluster add-node servidor1.example.com
qyv cluster deploy app.qyv
```

**Arquitectura de Clúster**:
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Nodo Master   │    │   Nodo Worker   │    │   Nodo Worker   │
│                 │    │                 │    │                 │
│ • Orquestación  │◄──►│ • Ejecución     │◄──►│ • Ejecución     │
│ • Balanceo      │    │ • Monitoreo     │    │ • Monitoreo     │
│ • Gestión       │    │ • Reportes      │    │ • Reportes      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

---

## 🔧 Herramientas de Desarrollo

### Compilación
- **Rust**: `rustc` con target `wasm32-wasip2`
- **Go**: `tinygo` + `wasm-tools`
- **C/C++**: `emscripten` o `clang`
- **Otros**: Herramientas específicas del lenguaje

### Validación
- `wasm-tools validate` - Validar módulos WASM
- `wasm-opt` - Optimizar tamaño y rendimiento
- Tests de integración en `common/tests/`

### Debugging
- Logs detallados del runtime
- Información de estado de ejecución
- Herramientas de profiling WASM

---

## 📊 Métricas y Monitoreo

### Métricas del Runtime
- Tiempo de carga de módulos
- Uso de memoria
- Tiempo de ejecución
- Errores y excepciones

### Monitoreo de Clúster
- Estado de nodos
- Distribución de carga
- Latencia de red
- Disponibilidad de servicios

---

## 🔄 Ciclo de Desarrollo

```
1. Escribir código (Rust, Go, etc.)
   ↓
2. Compilar a WASM (herramientas externas)
   ↓
3. Crear manifiesto .qyv
   ↓
4. Probar localmente (qyv run)
   ↓
5. Desplegar (qyv deploy)
   ↓
6. Monitorear y escalar
```

---

## 🎯 Casos de Uso

### Microservicios
- Aplicaciones modulares y escalables
- Despliegue independiente
- Gestión de dependencias simplificada

### Edge Computing
- Ejecución cercana al usuario
- Baja latencia
- Uso eficiente de recursos

### Serverless
- Ejecución bajo demanda
- Escalado automático
- Pago por uso

### IoT y Embebidos
- Código portable
- Bajo consumo de recursos
- Seguridad mejorada

---

## 🔮 Roadmap

### Corto Plazo
- [ ] Mejoras en el shell interactivo
- [ ] Optimizaciones de rendimiento
- [ ] Documentación expandida

### Mediano Plazo
- [ ] Soporte para más lenguajes
- [ ] Herramientas de debugging avanzadas
- [ ] Integración con orquestadores

### Largo Plazo
- [ ] Ecosistema de plugins
- [ ] Herramientas de desarrollo visuales
- [ ] Integración con cloud providers
