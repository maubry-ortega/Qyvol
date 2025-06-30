# Despliegue y Clústeres en Qyvol

Qyvol proporciona capacidades de despliegue y gestión de clústeres para ejecutar módulos WebAssembly compilados en entornos distribuidos. Este documento explica cómo desplegar y gestionar aplicaciones WASM con Qyvol.

## 🎯 Concepto Clave

**Qyvol ejecuta módulos WASM ya compilados** - no compila código. El flujo de trabajo es:

1. **Compilar código** con herramientas externas (Rust, Go, etc.)
2. **Crear manifiesto** `.qyv` con configuración
3. **Desplegar** el módulo WASM con Qyvol
4. **Gestionar** en clústeres distribuidos

---

## 🚀 Despliegue Individual

### Preparación del Módulo

Antes de desplegar, asegúrate de tener:

1. **Módulo WASM compilado** (ej: `app.component.wasm`)
2. **Manifiesto .qyv** con configuración
3. **Servidor objetivo** configurado

### Comando de Despliegue

```bash
qyv deploy <archivo.qyv> <servidor>
```

**Ejemplo:**
```bash
# Desplegar aplicación Rust
qyv deploy examples/hello/hello.qyv https://mi-servidor.com

# Desplegar aplicación Go
qyv deploy examples/main/main.qyv https://api.example.com
```

### Configuración de Despliegue

**Manifiesto para despliegue:**
```yaml
name: mi-aplicacion
entrypoint: app.component.wasm
runtime: wasi
language: rust
permissions:
  fs: read
  net: true
  exec: false
deploy:
  replicas: 3
  resources:
    memory: "128MB"
    cpu: "0.5"
  health_check:
    path: "/health"
    interval: "30s"
```

### Opciones de Despliegue

| Opción | Descripción | Ejemplo |
|--------|-------------|---------|
| `--replicas` | Número de instancias | `--replicas 5` |
| `--memory` | Límite de memoria | `--memory 256MB` |
| `--cpu` | Límite de CPU | `--cpu 1.0` |
| `--env` | Variables de entorno | `--env NODE_ENV=production` |
| `--port` | Puerto de exposición | `--port 8080` |

---

## 🏗️ Gestión de Clústeres

### Arquitectura de Clúster

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Nodo Master   │    │   Nodo Worker   │    │   Nodo Worker   │
│                 │    │                 │    │                 │
│ • Orquestación  │◄──►│ • Ejecución     │◄──►│ • Ejecución     │
│ • Balanceo      │    │ • Monitoreo     │    │ • Monitoreo     │
│ • Gestión       │    │ • Reportes      │    │ • Reportes      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Configuración del Clúster

#### 1. Inicializar Clúster

```bash
# Crear clúster local
qyv cluster init

# Configurar nodo master
qyv cluster set-master localhost:8080
```

#### 2. Añadir Nodos

```bash
# Añadir nodo worker
qyv cluster add-node servidor1.example.com:8080

# Añadir múltiples nodos
qyv cluster add-node servidor2.example.com:8080
qyv cluster add-node servidor3.example.com:8080
```

#### 3. Verificar Estado

```bash
# Listar nodos
qyv cluster list-nodes

# Verificar salud del clúster
qyv cluster health

# Ver métricas
qyv cluster metrics
```

### Despliegue en Clúster

```bash
# Desplegar aplicación en todo el clúster
qyv cluster deploy app.qyv

# Desplegar con configuración específica
qyv cluster deploy app.qyv --replicas 10 --memory 512MB

# Desplegar en nodos específicos
qyv cluster deploy app.qyv --nodes servidor1,servidor2
```

---

## 🔧 Configuración Avanzada

### Balanceo de Carga

Qyvol implementa balanceo de carga automático:

- **Round Robin**: Distribución equitativa
- **Least Connections**: Nodo con menos conexiones
- **Weighted**: Basado en capacidad del nodo

**Configuración:**
```yaml
cluster:
  load_balancer:
    algorithm: "least_connections"
    health_check:
      interval: "10s"
      timeout: "5s"
      retries: 3
```

### Escalado Automático

```bash
# Configurar auto-scaling
qyv cluster autoscale app.qyv --min 2 --max 10 --cpu-threshold 80

# Escalar manualmente
qyv cluster scale app.qyv --replicas 5
```

### Monitoreo y Logs

```bash
# Ver logs de aplicación
qyv cluster logs app.qyv

# Ver logs de nodo específico
qyv cluster logs app.qyv --node servidor1

# Monitorear métricas en tiempo real
qyv cluster monitor app.qyv
```

---

## 🔐 Seguridad y Permisos

### Permisos de Red

```yaml
permissions:
  net:
    allowed_hosts: ["api.example.com", "db.example.com"]
    allowed_ports: [80, 443, 5432]
    outbound_only: true
```

### Permisos de Archivos

```yaml
permissions:
  fs:
    read: ["/app/data", "/app/config"]
    write: ["/app/logs", "/app/temp"]
    exclude: ["/app/secrets"]
```

### Autenticación

```bash
# Configurar autenticación
qyv cluster auth --token "mi-token-secreto"

# Usar certificados TLS
qyv cluster auth --cert cert.pem --key key.pem
```

---

## 📊 Monitoreo y Métricas

### Métricas Disponibles

- **Rendimiento**: CPU, memoria, latencia
- **Red**: Ancho de banda, conexiones
- **Aplicación**: Requests/segundo, errores
- **Clúster**: Estado de nodos, balanceo

### Dashboard de Monitoreo

```bash
# Iniciar dashboard web
qyv cluster dashboard

# Acceder a métricas específicas
qyv cluster metrics --app app.qyv --node servidor1
```

### Alertas

```bash
# Configurar alertas
qyv cluster alerts --cpu-threshold 90 --memory-threshold 85

# Ver alertas activas
qyv cluster alerts --list
```

---

## 🔄 CI/CD Integration

### Pipeline de Despliegue

```yaml
# .github/workflows/deploy.yml
name: Deploy to Qyvol Cluster

on:
  push:
    branches: [main]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Compile Rust to WASM
        run: |
          rustup target add wasm32-wasip2
          rustc src/main.rs --target wasm32-wasip2 -o app.component.wasm
      
      - name: Deploy to Qyvol
        run: |
          qyv cluster deploy app.qyv --server ${{ secrets.QYVOL_SERVER }}
```

### Despliegue Automático

```bash
# Configurar webhook para auto-deploy
qyv cluster webhook --url https://api.github.com/webhooks

# Despliegue con rollback automático
qyv cluster deploy app.qyv --rollback-on-failure
```

---

## 🐛 Solución de Problemas

### Problemas Comunes

**Error: "Module not found"**
```bash
# Verificar que el módulo WASM existe
ls -la app.component.wasm

# Verificar permisos de archivo
chmod 644 app.component.wasm
```

**Error: "Connection refused"**
```bash
# Verificar que el servidor está ejecutándose
qyv cluster health

# Verificar conectividad de red
ping servidor.example.com
```

**Error: "Permission denied"**
```bash
# Verificar configuración de permisos en manifiesto
cat app.qyv

# Ajustar permisos si es necesario
```

### Logs de Debug

```bash
# Habilitar logs detallados
qyv cluster deploy app.qyv --debug

# Ver logs del runtime
qyv cluster logs app.qyv --level debug
```

---

## 📋 Checklist de Despliegue

### Antes del Despliegue

- [ ] Módulo WASM compilado y probado
- [ ] Manifiesto .qyv configurado correctamente
- [ ] Permisos definidos apropiadamente
- [ ] Servidor objetivo accesible
- [ ] Recursos suficientes disponibles

### Durante el Despliegue

- [ ] Verificar conectividad de red
- [ ] Monitorear logs de despliegue
- [ ] Verificar salud de la aplicación
- [ ] Probar funcionalidad básica

### Después del Despliegue

- [ ] Verificar métricas de rendimiento
- [ ] Configurar monitoreo y alertas
- [ ] Documentar configuración
- [ ] Planificar mantenimiento

---

## 🔗 Recursos Adicionales

- [Documentación de Compilación](COMPILACION.md)
- [Especificación de Manifiestos](MANIFESTOS.md)
- [Guía del Runtime](RUNTIME.md)
- [Comandos CLI](CLI.md)

---

## 📝 Notas Importantes

1. **Qyvol ejecuta módulos WASM** - asegúrate de compilar tu código primero
2. **Los permisos se aplican en tiempo de ejecución** - configura apropiadamente
3. **El clúster requiere configuración de red** - verifica conectividad
4. **Monitorea siempre** - configura alertas y métricas
5. **Prueba en staging** - valida antes de desplegar en producción 