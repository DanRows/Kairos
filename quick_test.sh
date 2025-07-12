#!/bin/bash

# ===============================
# Kairos - Script de Prueba Rápida
# ===============================
# Prueba rápida de compilación y funcionalidad básica
# ===============================

# Colores
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${BLUE}${BOLD}🧪 KAIROS - PRUEBA RÁPIDA${NC}"
echo -e "${BLUE}==============================${NC}"

# Verificar entorno
if [ -f ".env" ]; then
    echo -e "${GREEN}✅ Archivo .env encontrado${NC}"
    export $(grep -v '^#' .env | xargs)
else
    echo -e "${RED}❌ Archivo .env no encontrado${NC}"
    exit 1
fi

# Verificar Rust
if command -v cargo > /dev/null; then
    echo -e "${GREEN}✅ Cargo disponible${NC}"
else
    echo -e "${RED}❌ Cargo no encontrado${NC}"
    exit 1
fi

# Compilar backend
echo -e "\n${YELLOW}📦 Compilando backend...${NC}"
if cargo check -p kairos-backend; then
    echo -e "${GREEN}✅ Backend compila correctamente${NC}"
else
    echo -e "${RED}❌ Backend falló al compilar${NC}"
    exit 1
fi

# Compilar frontend
echo -e "\n${YELLOW}🌐 Compilando frontend...${NC}"
if cargo check -p kairos-frontend; then
    echo -e "${GREEN}✅ Frontend compila correctamente${NC}"
else
    echo -e "${RED}❌ Frontend falló al compilar${NC}"
    exit 1
fi

# Verificar workspace completo
echo -e "\n${YELLOW}🔧 Verificando workspace completo...${NC}"
if cargo check --workspace; then
    echo -e "${GREEN}✅ Workspace completo compila sin errores${NC}"
else
    echo -e "${RED}❌ Workspace tiene errores de compilación${NC}"
    exit 1
fi

echo -e "\n${GREEN}${BOLD}🎉 ¡TODAS LAS PRUEBAS PASARON!${NC}"
echo -e "${GREEN}El proyecto está listo para usar start_local.sh${NC}" 