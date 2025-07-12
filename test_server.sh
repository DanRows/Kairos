#!/bin/bash

echo "🧪 Test de inicio de servicios Kairos"
cd "$(dirname "$0")"

# Cargar entorno
if [ -f ".env" ]; then
    export $(grep -v '^#' .env | xargs)
    echo "✅ Variables de entorno cargadas"
else
    echo "❌ Archivo .env no encontrado"
    exit 1
fi

# Cargar Rust
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
    echo "✅ Entorno de Rust cargado"
fi

echo ""
echo "🚀 Iniciando backend..."
cargo run --package kairos-backend &
BACKEND_PID=$!
echo "Backend PID: $BACKEND_PID"

sleep 5

echo ""
echo "🌐 Iniciando frontend..."
cd frontend && dx serve --hot-reload=true --port 8081 &
FRONTEND_PID=$!
echo "Frontend PID: $FRONTEND_PID"

echo ""
echo "✅ Servicios iniciados:"
echo "   - Backend: http://localhost:8080"
echo "   - Frontend: http://localhost:8081"
echo ""
echo "Presiona Ctrl+C para detener..."

# Esperar a que los procesos terminen
wait $BACKEND_PID $FRONTEND_PID 