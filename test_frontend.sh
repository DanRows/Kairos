#!/bin/bash
echo "🧪 Probando frontend Kairos..."
cd frontend
echo "📁 Directorio actual: $(pwd)"
echo "📦 Compilando frontend..."
cargo check
echo "🚀 Intentando iniciar servidor de desarrollo..."
dx serve --port 8082 