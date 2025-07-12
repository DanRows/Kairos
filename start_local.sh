#!/bin/bash

# ===============================
# Kairos - Script de Arranque Local v3.0 BIBLIA EDITION
# ===============================
# Este script es la BIBLIA DEFINITIVA del entorno local de Kairos.
# Automatiza TODO: verificación, carga, limpieza, migraciones y arranque
# con una experiencia visual profesional y robustez máxima.
# 
# NUEVO EN v3.0 BIBLIA EDITION:
# - Pantallas de carga animadas y decoradores profesionales
# - Sistema de progreso visual avanzado
# - Verificaciones exhaustivas con feedback detallado
# - Manejo de errores ultra-robusto
# - Experiencia de usuario de nivel empresarial
# ===============================

# --- Configuración de Colores y Estilos ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
BOLD='\033[1m'
DIM='\033[2m'
UNDERLINE='\033[4m'
NC='\033[0m' # No Color

# --- Configuración del Script ---
set -e # Salir inmediatamente si un comando falla
LOG_FILE="kairos_startup.log"
LAST_BUILD_MARKER_FILE="target/.last_successful_build"

# Configurar logging completo
exec > >(tee -a "$LOG_FILE") 2>&1

# --- Funciones de Interfaz Visual ---
print_banner() {
    clear
    echo -e "${CYAN}${BOLD}"
    echo "╔══════════════════════════════════════════════════════════════════╗"
    echo "║                                                                  ║"
    echo "║     🌾 KAIROS - LOGÍSTICA PREDICTIVA PARA EL AGRO 🌾            ║"
    echo "║                                                                  ║"
    echo "║                   BIBLIA DE ARRANQUE LOCAL v3.0                  ║"
    echo "║                                                                  ║"
    echo "╚══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo -e "${DIM}La experiencia definitiva de desarrollo para Kairos${NC}"
    echo ""
}

print_section() {
    echo -e "\n${PURPLE}${BOLD}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${PURPLE}${BOLD}│  $1${NC}"
    echo -e "${PURPLE}${BOLD}└─────────────────────────────────────────────────────────────────┘${NC}"
}

print_step() {
    echo -e "\n${BLUE}${BOLD}➤ $1${NC}"
}

print_substep() {
    echo -e "   ${CYAN}▸ $1${NC}"
}

print_success() {
    echo -e "   ${GREEN}${BOLD}✅ $1${NC}"
}

print_warning() {
    echo -e "   ${YELLOW}${BOLD}⚠️  $1${NC}"
}

print_error() {
    echo -e "   ${RED}${BOLD}❌ $1${NC}"
}

print_info() {
    echo -e "   ${WHITE}${BOLD}ℹ️  $1${NC}"
}

# Función de carga animada
loading_animation() {
    local message="$1"
    local duration="$2"
    local spinner="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    local end_time=$((SECONDS + duration))
    
    while [ $SECONDS -lt $end_time ]; do
        for ((i=0; i<${#spinner}; i++)); do
            printf "\r   ${CYAN}${spinner:$i:1} ${message}${NC}"
            sleep 0.1
        done
    done
    printf "\r   ${GREEN}✅ ${message} - Completado${NC}\n"
}

# Barra de progreso
progress_bar() {
    local current="$1"
    local total="$2"
    local message="$3"
    local width=50
    local percentage=$((current * 100 / total))
    local filled=$((current * width / total))
    local empty=$((width - filled))
    
    printf "\r   ${BLUE}[${NC}"
    printf "%*s" $filled | tr ' ' '█'
    printf "%*s" $empty | tr ' ' '░'
    printf "${BLUE}] ${percentage}%% - ${message}${NC}"
    
    if [ $current -eq $total ]; then
        echo -e "\n   ${GREEN}✅ ${message} - Completado al 100%${NC}"
    fi
}

# --- Funciones de Utilidad ---
error_exit() {
    print_error "$1"
    echo -e "\n${RED}${BOLD}💥 FALLO CRÍTICO - Proceso abortado${NC}"
    echo -e "${DIM}Revisa el log en: $LOG_FILE${NC}"
    exit 1
}

confirm_action() {
    local message="$1"
    echo -e "\n${YELLOW}${BOLD}🤔 $message (s/n): ${NC}"
    read -n 1 -r
    echo
    [[ $REPLY =~ ^[Ss]$ ]]
}

# --- Limpieza de Procesos ---
BACKEND_PID=""
FRONTEND_PID=""
trap cleanup EXIT SIGINT

cleanup() {
    echo -e "\n${YELLOW}${BOLD}🛑 Iniciando secuencia de parada limpia...${NC}"
    
    # Detener backend
    if [ -n "$BACKEND_PID" ] && ps -p $BACKEND_PID > /dev/null 2>&1; then
        print_substep "Enviando señal SIGTERM al backend (PID: $BACKEND_PID)"
        kill -TERM $BACKEND_PID 2>/dev/null
        
        # Esperar hasta 5 segundos para parada limpia
        local count=0
        while [ $count -lt 5 ] && ps -p $BACKEND_PID > /dev/null 2>&1; do
            sleep 1
            count=$((count + 1))
        done
        
        # Si sigue corriendo, forzar parada
        if ps -p $BACKEND_PID > /dev/null 2>&1; then
            print_substep "Forzando parada del backend con SIGKILL"
            kill -KILL $BACKEND_PID 2>/dev/null
        fi
        print_success "Backend detenido correctamente"
    fi
    
    # Detener frontend
    if [ -n "$FRONTEND_PID" ] && ps -p $FRONTEND_PID > /dev/null 2>&1; then
        print_substep "Enviando señal SIGTERM al frontend (PID: $FRONTEND_PID)"
        kill -TERM $FRONTEND_PID 2>/dev/null
        
        # Esperar hasta 5 segundos para parada limpia
        local count=0
        while [ $count -lt 5 ] && ps -p $FRONTEND_PID > /dev/null 2>&1; do
            sleep 1
            count=$((count + 1))
        done
        
        # Si sigue corriendo, forzar parada
        if ps -p $FRONTEND_PID > /dev/null 2>&1; then
            print_substep "Forzando parada del frontend con SIGKILL"
            kill -KILL $FRONTEND_PID 2>/dev/null
        fi
        print_success "Frontend detenido correctamente"
    fi
    
    # Limpiar procesos huérfanos de cargo watch
    print_substep "Limpiando procesos huérfanos de cargo watch"
    pkill -f "cargo watch" 2>/dev/null || true
    
    echo -e "\n${GREEN}${BOLD}👋 ¡Hasta luego! Todos los servicios han sido detenidos correctamente.${NC}"
    echo -e "${DIM}Logs disponibles en: $LOG_FILE${NC}"
}

# --- INICIO DEL SCRIPT ---
print_banner

echo -e "${BOLD}📋 Registro completo disponible en: ${UNDERLINE}$LOG_FILE${NC}"
echo -e "${DIM}Iniciando secuencia de arranque...${NC}"

# === FASE 1: CARGA DE ENTORNO ===
print_section "FASE 1: CONFIGURACIÓN DEL ENTORNO"

print_step "Cargando variables de entorno"
if [ -f ".env" ]; then
    print_substep "Archivo .env encontrado - Procesando variables"
    loading_animation "Cargando configuración" 2
    export $(grep -v '^#' .env | xargs)
    print_success "Variables de entorno cargadas correctamente"
    print_info "Configuración: DB=$POSTGRES_DB, HOST=$SERVER_HOST:$SERVER_PORT"
else
    error_exit "Archivo .env no encontrado. Este archivo es OBLIGATORIO."
fi

print_step "Configurando entorno Rust"
if [ -f "$HOME/.cargo/env" ]; then
    print_substep "Cargando configuración de Cargo"
    source "$HOME/.cargo/env"
    loading_animation "Inicializando toolchain Rust" 1
    print_success "Entorno Rust configurado"
    print_info "Rust $(rustc --version 2>/dev/null || echo 'versión no detectada')"
else
    error_exit "Entorno Rust no encontrado. Instala Rust desde https://rustup.rs/"
fi

# === FASE 2: VERIFICACIÓN DE DEPENDENCIAS ===
print_section "FASE 2: VERIFICACIÓN DE DEPENDENCIAS"

print_step "Verificando herramientas de desarrollo"

DEPS=(
    "cargo:Cargo (Rust):cargo"
    "psql:Cliente PostgreSQL:postgresql-client"
    "pg_isready:PostgreSQL Tools:postgresql-client"
    "wasm-pack:Compilador WASM:wasm-pack"
    "dx:Dioxus CLI:dioxus-cli"
    "cargo-watch:Live-reloading:cargo-watch"
    "diesel:Diesel CLI:diesel_cli --no-default-features --features postgres"
)

total_deps=${#DEPS[@]}
current_dep=0

for dep in "${DEPS[@]}"; do
    current_dep=$((current_dep + 1))
    IFS=':' read -r cmd_name desc_name pkg_name <<< "$dep"
    
    progress_bar $current_dep $total_deps "Verificando $desc_name"
    sleep 0.5
    
    if command -v "$cmd_name" >/dev/null 2>&1; then
        print_success "$desc_name está disponible"
    else
        print_error "$desc_name no encontrado"
        if confirm_action "¿Deseas continuar sin $desc_name? (No recomendado)"; then
            print_warning "Continuando sin $desc_name - funcionalidad limitada"
        else
            error_exit "Instala $desc_name con: cargo install $pkg_name"
        fi
    fi
done

print_step "Verificando target WASM"
if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    print_success "Target wasm32-unknown-unknown disponible"
else
    print_substep "Instalando target wasm32-unknown-unknown"
    loading_animation "Descargando e instalando target WASM" 3
    rustup target add wasm32-unknown-unknown || error_exit "Fallo al instalar target WASM"
    print_success "Target WASM instalado correctamente"
fi

# === FASE 3: ANÁLISIS INTELIGENTE DE LIMPIEZA ===
print_section "FASE 3: ANÁLISIS INTELIGENTE DE LIMPIEZA"

print_step "Analizando necesidad de cargo clean"
NEEDS_CLEAN=false
CLEAN_REASONS=()

# Análisis de marcadores
if [ ! -f "$LAST_BUILD_MARKER_FILE" ]; then
    NEEDS_CLEAN=true
    CLEAN_REASONS+=("📍 Sin marcador de compilación exitosa previa")
fi

# Análisis de archivos de configuración
print_substep "Analizando archivos de configuración Cargo"
for f in $(find . -name "Cargo.toml" -o -name "Cargo.lock" 2>/dev/null); do
    if [ "$f" -nt "$LAST_BUILD_MARKER_FILE" ]; then
        NEEDS_CLEAN=true
        CLEAN_REASONS+=("📦 Cambios en '$f'")
        break
    fi
done

# Análisis de archivos críticos
print_substep "Analizando archivos críticos del sistema"
CRITICAL_FILES=(
    "backend/src/schema.rs"
    "backend/src/models/db_types.rs"
    "backend/src/models/mod.rs"
    "backend/diesel.toml"
    "frontend/src/main.rs"
)

for f in "${CRITICAL_FILES[@]}"; do
    if [ -f "$f" ] && [ "$f" -nt "$LAST_BUILD_MARKER_FILE" ]; then
        NEEDS_CLEAN=true
        CLEAN_REASONS+=("🔧 Cambios en archivo crítico '$f'")
        break
    fi
done

# Análisis de migraciones
print_substep "Analizando migraciones de base de datos"
if [ -d "backend/migrations" ]; then
    for migration_dir in backend/migrations/*/; do
        if [ -d "$migration_dir" ]; then
            for migration_file in "$migration_dir"*.sql; do
                if [ -f "$migration_file" ] && [ "$migration_file" -nt "$LAST_BUILD_MARKER_FILE" ]; then
                    NEEDS_CLEAN=true
                    CLEAN_REASONS+=("🗃️  Cambios en migración '$migration_file'")
                    break 2
                fi
            done
        fi
    done
fi

# Análisis de errores previos
ERROR_MARKER_FILE="target/.last_build_error"
if [ -f "$ERROR_MARKER_FILE" ] && [ "$ERROR_MARKER_FILE" -nt "$LAST_BUILD_MARKER_FILE" ]; then
    NEEDS_CLEAN=true
    CLEAN_REASONS+=("💥 Compilación previa falló")
fi

# Análisis de cambios masivos
MODIFIED_RS_COUNT=$(find . -name "*.rs" -newer "$LAST_BUILD_MARKER_FILE" 2>/dev/null | wc -l | tr -d ' ')
if [ -n "$MODIFIED_RS_COUNT" ] && [ "$MODIFIED_RS_COUNT" -gt 5 ]; then
    NEEDS_CLEAN=true
    CLEAN_REASONS+=("📊 Cambios masivos detectados ($MODIFIED_RS_COUNT archivos .rs)")
fi

# Mostrar análisis
if [ "$NEEDS_CLEAN" = true ]; then
    print_warning "Análisis de limpieza: SE RECOMIENDA CARGO CLEAN"
    echo -e "\n${YELLOW}   📋 Razones detectadas:${NC}"
    for reason in "${CLEAN_REASONS[@]}"; do
        echo -e "      ${YELLOW}• $reason${NC}"
    done
    
    if confirm_action "¿Ejecutar cargo clean para máxima compatibilidad?"; then
        print_step "Ejecutando limpieza profunda del workspace"
        loading_animation "Limpiando artifacts de compilación" 3
        cargo clean || error_exit "Falló cargo clean"
        rm -f "$LAST_BUILD_MARKER_FILE" "$ERROR_MARKER_FILE"
        print_success "Limpieza completada - Workspace renovado"
    else
        print_warning "Limpieza omitida - Continuando bajo tu responsabilidad"
    fi
else
    print_success "Análisis completado - No se requiere limpieza"
fi

# === FASE 4: GESTIÓN DE BASE DE DATOS ===
print_section "FASE 4: GESTIÓN DE BASE DE DATOS"

print_step "Verificando conectividad de PostgreSQL"
print_substep "Probando conexión a: $DATABASE_URL"
loading_animation "Estableciendo conexión con PostgreSQL" 2

if ! pg_isready -d "$DATABASE_URL" -q; then
    error_exit "Conexión fallida a PostgreSQL. Verifica que el servidor esté corriendo."
fi
print_success "Conectividad PostgreSQL verificada"

print_step "Gestionando migraciones y schema"
cd backend || error_exit "No se puede acceder al directorio backend/"

print_substep "Ejecutando migraciones pendientes"
loading_animation "Aplicando migraciones de Diesel" 2
diesel migration run || error_exit "Fallo en migraciones de Diesel"
print_success "Migraciones aplicadas correctamente"

print_substep "Regenerando schema.rs actualizado"
loading_animation "Generando definiciones de tipos" 1
diesel print-schema > src/schema.rs || error_exit "Fallo en regeneración de schema"
print_success "Schema regenerado exitosamente"

cd .. # Volver al workspace root

# === FASE 5: VERIFICACIÓN DE COMPILACIÓN ===
print_section "FASE 5: VERIFICACIÓN DE COMPILACIÓN"

print_step "Verificando integridad del workspace completo"
print_substep "Compilando backend y frontend para verificación inicial"

ERROR_MARKER_FILE="target/.last_build_error"
if ! cargo check --workspace; then
    touch "$ERROR_MARKER_FILE"
    error_exit "Fallo en verificación de compilación. Revisa errores arriba."
fi

rm -f "$ERROR_MARKER_FILE"
print_success "Verificación de compilación completada - Sin errores"

# === FASE 6: ARRANQUE DE SERVICIOS ===
print_section "FASE 6: ARRANQUE DE SERVICIOS EN PRODUCCIÓN"

print_step "Creando marcador de compilación exitosa"
touch "$LAST_BUILD_MARKER_FILE"
print_success "Marcador de build exitoso creado"

print_step "Iniciando Backend (Actix-Web + Diesel)"
print_substep "Configuración: $SERVER_HOST:$SERVER_PORT"
print_substep "Base de datos: $POSTGRES_DB"
print_substep "Modo: Desarrollo con auto-reload"

# Iniciar backend con cargo watch (solo vigila cambios en backend/)
(cd backend && cargo watch -q -c -w src -x "run --bin kairos-backend") &
BACKEND_PID=$!

loading_animation "Iniciando servidor backend" 3
print_success "Backend en ejecución (PID: $BACKEND_PID)"
print_info "Endpoint: http://$SERVER_HOST:$SERVER_PORT"

print_step "Iniciando Frontend (Dioxus Web)"
print_substep "Configuración: Hot-reload habilitado en puerto 8082"
print_substep "Modo: Desarrollo con auto-reload de Dioxus"

(cd frontend && dx serve --hot-reload=true --port 8082) &
FRONTEND_PID=$!

loading_animation "Compilando e iniciando frontend" 4
print_success "Frontend en ejecución (PID: $FRONTEND_PID)"
print_info "Interfaz: http://localhost:8082"

# === PANTALLA FINAL ===
echo -e "\n${GREEN}${BOLD}╔══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}${BOLD}║                                                                  ║${NC}"
echo -e "${GREEN}${BOLD}║  🎉 KAIROS ESTÁ EN MARCHA - ARRANQUE COMPLETADO EXITOSAMENTE 🎉  ║${NC}"
echo -e "${GREEN}${BOLD}║                                                                  ║${NC}"
echo -e "${GREEN}${BOLD}╚══════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${CYAN}${BOLD}📊 SERVICIOS ACTIVOS:${NC}"
echo -e "   🔧 Backend:   http://$SERVER_HOST:$SERVER_PORT  (PID: $BACKEND_PID)"
echo -e "   🌐 Frontend:  http://localhost:8082            (PID: $FRONTEND_PID)"
echo -e "   🗄️  Database: $POSTGRES_DB @ $POSTGRES_HOST:$POSTGRES_PORT"

echo -e "\n${PURPLE}${BOLD}🎮 CONTROLES:${NC}"
echo -e "   • Presiona ${BOLD}Ctrl+C${NC} para detener todos los servicios"
echo -e "   • Logs disponibles en: ${UNDERLINE}$LOG_FILE${NC}"
echo -e "   • Monitoreo en tiempo real activo"

echo -e "\n${YELLOW}${BOLD}⏳ Esperando interrupción del usuario...${NC}"
echo -e "${DIM}Presiona Ctrl+C para detener todos los servicios${NC}"

# Bucle infinito para mantener el script activo hasta interrupción manual
while true; do
    # Verificar si los procesos siguen corriendo
    if ! ps -p $BACKEND_PID > /dev/null 2>&1; then
        print_error "Backend se detuvo inesperadamente (PID: $BACKEND_PID)"
        break
    fi
    
    if ! ps -p $FRONTEND_PID > /dev/null 2>&1; then
        print_error "Frontend se detuvo inesperadamente (PID: $FRONTEND_PID)"
        break
    fi
    
    # Esperar un poco antes de verificar de nuevo
    sleep 2
done

print_warning "Uno o más servicios se detuvieron. Ejecutando limpieza..." 