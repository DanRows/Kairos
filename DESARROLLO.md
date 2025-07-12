# 🌾 Kairos - Guía de Desarrollo v3.0

## 🚀 Scripts de Desarrollo

### 📋 Resumen de Scripts Disponibles

| Script | Propósito | Uso Recomendado |
|--------|-----------|-----------------|
| `start_local.sh` | **BIBLIA DEFINITIVA** - Arranque completo del entorno | Desarrollo diario principal |
| `quick_test.sh` | Verificación rápida de compilación | Antes de commits importantes |
| `test_frontend.sh` | Testing específico del frontend | Desarrollo de UI |
| `test_server.sh` | Testing específico del backend | Desarrollo de API |

### 🎯 start_local.sh BIBLIA EDITION v3.0

#### ✨ Nuevas Características

- **🎨 Interfaz Visual Profesional**: Banners, colores, iconos y animaciones
- **📊 Sistema de Progreso**: Barras de progreso y spinners de carga
- **🧠 Análisis Inteligente**: Detección automática de necesidad de limpieza
- **🛡️ Robustez Máxima**: Manejo de errores y cleanup automático
- **⚡ Optimización de Procesos**: Prevención de bucles de recompilación

#### 🔧 Correcciones Importantes

**Problema Resuelto**: El script entraba en bucles de recompilación
- **Antes**: `cargo watch` vigilaba todo el workspace causando recompilaciones infinitas
- **Después**: `cargo watch` solo vigila `backend/src/` específicamente
- **Beneficio**: Arranque limpio y estable sin bucles

#### 🎮 Fases de Ejecución

1. **FASE 1: CONFIGURACIÓN DEL ENTORNO**
   - Carga de variables `.env`
   - Configuración del toolchain Rust

2. **FASE 2: VERIFICACIÓN DE DEPENDENCIAS**
   - Verificación inteligente con barra de progreso
   - Instalación automática de targets WASM

3. **FASE 3: ANÁLISIS INTELIGENTE DE LIMPIEZA**
   - Detección de cambios en archivos críticos
   - Análisis de migraciones y marcadores
   - Recomendación inteligente de `cargo clean`

4. **FASE 4: GESTIÓN DE BASE DE DATOS**
   - Verificación de conectividad PostgreSQL
   - Ejecución de migraciones pendientes
   - Regeneración automática de `schema.rs`

5. **FASE 5: VERIFICACIÓN DE COMPILACIÓN**
   - Compilación inicial completa del workspace
   - Validación de integridad del código

6. **FASE 6: ARRANQUE DE SERVICIOS**
   - Backend con auto-reload (solo `backend/src/`)
   - Frontend con hot-reload de Dioxus
   - Monitoreo continuo de procesos

#### 🛑 Sistema de Parada Limpia

- **SIGTERM** primero (parada elegante)
- **SIGKILL** como respaldo (forzar si es necesario)
- Limpieza automática de procesos huérfanos
- Logs detallados de la secuencia de parada

### 💡 Flujo de Trabajo Recomendado

#### Para Desarrollo Diario:
```bash
# 1. Verificación rápida (opcional)
./quick_test.sh

# 2. Arranque completo
./start_local.sh

# 3. Desarrollo normal...
# Los cambios se recargan automáticamente

# 4. Parada limpia
# Ctrl+C cuando termines
```

#### Para Testing Específico:
```bash
# Solo frontend
./test_frontend.sh

# Solo backend
./test_server.sh
```

#### Para Depuración:
```bash
# Ver logs en tiempo real
tail -f kairos_startup.log

# Verificar procesos manualmente
ps aux | grep -E "(cargo|dx)"
```

### 🚨 Solución de Problemas

#### Si el script se queda en bucle de compilación:
```bash
# 1. Detener completamente
Ctrl+C

# 2. Limpiar procesos huérfanos
pkill -f "cargo watch"
pkill -f "dx serve"

# 3. Limpiar workspace si es necesario
cargo clean

# 4. Reiniciar
./start_local.sh
```

#### Si hay errores de PostgreSQL:
```bash
# Verificar que PostgreSQL esté corriendo
sudo service postgresql status

# Verificar variables en .env
cat .env | grep -E "(POSTGRES|DATABASE)"
```

#### Si hay errores de dependencias:
```bash
# Instalar herramientas faltantes
cargo install dioxus-cli
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-watch
```

### 📊 Monitoreo de Performance

- **Backend**: http://localhost:8080 (configurable en .env)
- **Frontend**: http://localhost:8082 (puerto fijo de dx serve)
- **Database**: Puerto configurado en .env
- **Logs**: `kairos_startup.log` (rotación automática)

### 🔄 Actualizaciones del Script

El script `start_local.sh` se auto-documenta con:
- Versión actual en el banner
- Descripción de cambios en comentarios
- Logs detallados de cada operación

Para actualizaciones futuras, modificar la versión en el header y documentar cambios.

### 🎯 Próximas Mejoras Planeadas

- [ ] Detección automática de puertos libres
- [ ] Configuración de perfiles de desarrollo (dev/staging/prod)
- [ ] Integración con Docker Compose opcional
- [ ] Backup automático de base de datos antes de migraciones
- [ ] Notificaciones de desktop cuando los servicios estén listos 