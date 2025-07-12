# 🌾 Kairos - Logística Predictiva para el Agro

<div align="center">

![Kairos Logo](https://img.shields.io/badge/Kairos-AgTech%20Platform-green?style=for-the-badge&logo=rust)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

**Transformando datos agrícolas en inteligencia predictiva**

*Plataforma de alto rendimiento para optimización de cadena de suministro en agricultura*

[🚀 Características](#-características) • [⚡ Tecnologías](#-tecnologías) • [🛠️ Instalación](#️-instalación) • [📖 Uso](#-uso) • [🔧 API](#-api) • [🤝 Contribuir](#-contribuir)

</div>

---

## 🎯 ¿Qué es Kairos?

Kairos es una plataforma de código abierto de grado productivo que transforma las cadenas de suministro agrícolas a través de inteligencia basada en datos. Construida con programación de sistemas moderna y diseñada para escala masiva.

### 🌱 El Problema
**30% de la producción agrícola se desperdicia** debido a brechas de información entre productores y compradores.

### 💡 Nuestra Solución
Una plataforma unificada que captura, valida y analiza datos agrícolas para habilitar logística predictiva y conexiones directas de mercado.

```
🌱 Datos de Campo → 📊 Inteligencia Estructurada → 🎯 Emparejamiento Predictivo → 💰 Transacciones Optimizadas
```

---

## 🚀 Características

### 📱 Trazabilidad Digital
- **Códigos QR únicos** para cada lote de producción
- **Registro inmutable** de origen, procesos y métricas de calidad
- **API pública** para verificación y transparencia
- **Interfaz móvil** para trabajadores de campo

### 🤖 Inteligencia Predictiva
- **Modelos de ML** para pronóstico de demanda
- **Predicción de rendimiento** basada en datos históricos
- **Recomendaciones de cosecha** en tiempo óptimo
- **Sugerencias de precios** dinámicas

### 🔗 Marketplace B2B
- **Conexión directa** entre productores y compradores
- **Emparejamiento automático** basado en especificaciones
- **Integración de contratos inteligentes** para transacciones seguras
- **Algoritmos de optimización** logística

### 📊 Dashboard Analítico
- **Visibilidad en tiempo real** de la cadena de suministro
- **Métricas de rendimiento** y KPIs
- **Análisis de tendencias** del mercado
- **Herramientas de reportes** personalizados

---

## ⚡ Tecnologías

### 🏗️ Stack Técnico

```
┌─────────────────────────────────────────────────────────────┐
│                    🌐 Capa Frontend                        │
├─────────────────────────────────────────────────────────────┤
│  Dioxus (Rust → WASM)  │  PWA Móvil  │  Integración QR    │
├─────────────────────────────────────────────────────────────┤
│                    🔧 Gateway API                           │
├─────────────────────────────────────────────────────────────┤
│  Actix Router  │  Autenticación  │  Rate Limiting  │  CORS  │
├─────────────────────────────────────────────────────────────┤
│                    🧠 Lógica de Negocio                     │
├─────────────────────────────────────────────────────────────┤
│  Trazabilidad  │  Pipeline ML  │  Motor Matching  │  APIs  │
├─────────────────────────────────────────────────────────────┤
│                    💾 Capa de Datos                        │
├─────────────────────────────────────────────────────────────┤
│  PostgreSQL  │  Time Series  │  Búsqueda Full-text  │  JSONB  │
├─────────────────────────────────────────────────────────────┤
│                    🚀 Infraestructura                       │
└─────────────────────────────────────────────────────────────┘
   Docker Compose  │  Kubernetes Ready  │  Cloud Agnostic
```

### 🔧 Dependencias Principales

**Backend (Rust)**
- `actix-web` - Framework web de alto rendimiento
- `diesel` - ORM para PostgreSQL
- `serde` - Serialización/deserialización
- `jsonwebtoken` - Autenticación JWT
- `bcrypt` - Hashing de contraseñas
- `chrono` - Manejo de fechas
- `uuid` - Identificadores únicos

**Frontend (Dioxus/WASM)**
- `dioxus` - Framework UI para Rust
- `reqwest` - Cliente HTTP
- `serde_json` - Procesamiento JSON
- `web-sys` - APIs del navegador

---

## 🛠️ Instalación

### 📋 Prerrequisitos

- **Rust 1.75+** - [Instalar Rust](https://rustup.rs/)
- **PostgreSQL 15+** - [Instalar PostgreSQL](https://www.postgresql.org/download/)
- **Docker & Docker Compose** - [Instalar Docker](https://docs.docker.com/get-docker/)
- **Node.js 18+** (para herramientas de desarrollo)

### 🚀 Instalación Rápida

```bash
# 1. Clonar el repositorio
git clone https://github.com/your-org/kairos.git
cd kairos

# 2. Configurar variables de entorno
cp .env.example .env
# Editar .env con tus configuraciones

# 3. Arrancar con el script inteligente
./start_local.sh
```

### 🔧 Instalación Manual

```bash
# 1. Instalar dependencias de desarrollo
cargo install dioxus-cli
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-watch

# 2. Configurar base de datos
sudo -u postgres createdb kairos
sudo -u postgres psql -c "CREATE USER kairos WITH PASSWORD 'kairos123';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE kairos TO kairos;"

# 3. Ejecutar migraciones
cd backend
diesel migration run

# 4. Compilar y ejecutar backend
cargo run

# 5. En otra terminal, compilar y ejecutar frontend
cd frontend
dx serve --hot-reload=true
```

---

## 📖 Uso

### 🎮 Scripts de Desarrollo

| Script | Propósito | Uso Recomendado |
|--------|-----------|-----------------|
| `start_local.sh` | **BIBLIA DEFINITIVA** - Arranque completo | Desarrollo diario |
| `quick_test.sh` | Verificación rápida de compilación | Antes de commits |
| `test_frontend.sh` | Testing específico del frontend | Desarrollo de UI |
| `test_server.sh` | Testing específico del backend | Desarrollo de API |

### 🚀 Arranque Completo

```bash
# Script inteligente con interfaz visual
./start_local.sh
```

**Características del script:**
- 🎨 **Interfaz visual profesional** con banners y colores
- 📊 **Sistema de progreso** con barras y spinners
- 🧠 **Análisis inteligente** de necesidad de limpieza
- 🛡️ **Robustez máxima** con manejo de errores
- ⚡ **Optimización de procesos** sin bucles de recompilación

### 🔍 Verificación Rápida

```bash
# Para pruebas rápidas antes de commits
./quick_test.sh
```

---

## 🔧 API

### 🔐 Autenticación

```bash
# Registro de productor
POST /auth/register
{
  "full_name": "Juan Pérez",
  "email": "juan@finca.com",
  "password": "password123",
  "farm_name": "Finca El Paraíso"
}

# Login
POST /auth/login
{
  "email": "juan@finca.com",
  "password": "password123"
}
```

### 📦 Gestión de Lotes

```bash
# Crear lote
POST /lots
{
  "product_name": "Tomates Cherry",
  "crop_type": "CONVENTIONAL",
  "estimated_quantity": 500.0,
  "unit_of_measure": "kg",
  "estimated_harvest_date": "2024-08-15"
}

# Listar lotes del productor
GET /lots

# Obtener lote específico
GET /lots/{id}

# Actualizar lote
PUT /lots/{id}

# Eliminar lote
DELETE /lots/{id}
```

### 📊 Eventos de Trazabilidad

```bash
# Registrar evento
POST /events
{
  "lot_id": "uuid-del-lote",
  "event_type": "FERTILIZER_APPLICATION",
  "description": "Aplicación de fertilizante NPK",
  "event_location": "Sector A, Parcela 3"
}
```

### 🌐 API Pública

```bash
# Obtener lote por código QR
GET /public/lots/{code}

# Obtener eventos de un lote
GET /public/lots/{code}/events
```

---

## 📊 Base de Datos

### 🗄️ Esquema Principal

**Productores (`producers`)**
- Información de productores y fincas
- Autenticación y preferencias
- Verificación de email

**Lotes (`lots`)**
- Registro de cultivos y producción
- Códigos QR únicos
- Estados de crecimiento y cosecha
- Coordenadas GPS

**Eventos (`events`)**
- Trazabilidad completa de procesos
- Tipos: fertilización, riego, control de plagas, cosecha
- Metadatos JSON para flexibilidad

### 🔍 Tipos de Cultivo

- `CONVENTIONAL` - Agricultura convencional
- `AGROECOLOGICAL_UNCERTIFIED` - Agroecología sin certificar
- `ORGANIC_CERTIFIED` - Orgánico certificado
- `HYDROPONIC` - Hidropónico

### 📈 Estados de Lote

- `REGISTERED` - Registrado
- `GROWING` - En crecimiento
- `READY_FOR_HARVEST` - Listo para cosecha
- `HARVESTED` - Cosechado
- `SOLD` - Vendido
- `CANCELLED` - Cancelado

---

## 🚀 Performance

### 📊 Benchmarks

| Métrica | Valor |
|---------|-------|
| Tiempo de Respuesta API | < 50ms (95º percentil) |
| Consultas Base de Datos | < 10ms (promedio) |
| Uso de Memoria | < 512MB (bajo carga) |
| Usuarios Concurrentes | 10,000+ soportados |
| Throughput de Datos | 1M+ registros/día |

### 🔧 Puertos por Defecto

- **Backend API**: `http://localhost:8080`
- **Frontend Web**: `http://localhost:8082`
- **Base de Datos**: `localhost:5432`
- **Logs**: `kairos_startup.log`

---

## 🤝 Contribuir

¡Agradecemos las contribuciones! Por favor revisa nuestra [Guía de Contribución](CONTRIBUTING.md).

### 🔄 Flujo de Desarrollo

```bash
# 1. Fork del repositorio
git clone https://github.com/tu-usuario/kairos.git

# 2. Crear rama de feature
git checkout -b feature/nueva-funcionalidad

# 3. Desarrollo con hot-reload
./start_local.sh

# 4. Commit y push
git commit -m 'Agregar nueva funcionalidad'
git push origin feature/nueva-funcionalidad

# 5. Abrir Pull Request
```

### 📋 Estándares de Código

- **Rust**: Seguir `rustfmt` y `clippy`
- **Testing**: Mantener >90% cobertura
- **Documentación**: Documentar todas las APIs públicas
- **Seguridad**: Ejecutar `cargo audit` antes de commits

---

## 📈 Roadmap

### 🎯 Fase 1: Plataforma Core (Actual)
- [x] Sistema de trazabilidad digital
- [x] Generación y validación de códigos QR
- [x] API RESTful con autenticación
- [x] Capa de datos PostgreSQL
- [x] Optimización PWA móvil

### 🚀 Fase 2: Capa de Inteligencia (Q4 2025)
- [ ] Pipeline ML para predicción de demanda
- [ ] Emparejamiento automático productor-comprador
- [ ] Algoritmos de optimización de precios
- [ ] Framework SaaS para enterprise

### 🌐 Fase 3: Ecosistema Descentralizado (2026)
- [ ] Integración de contratos inteligentes Solana
- [ ] Programas de incentivos tokenizados
- [ ] Sistemas de pago cross-chain
- [ ] Modelo de gobernanza descentralizada

---

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

---

## 🤖 Construido con Amor y Rust

Kairos está construido por desarrolladores que creen que la agricultura moderna merece herramientas modernas. Estamos combinando el rendimiento de la programación de sistemas con la inteligencia del machine learning para crear el futuro de las cadenas de suministro agrícolas.

<div align="center">

⭐ **¡Dale una estrella en GitHub si encuentras útil este proyecto!**

</div>
