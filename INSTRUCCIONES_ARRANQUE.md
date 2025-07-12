# Kairos - Instrucciones de Arranque

## Forma Correcta de Ejecutar Kairos

### Opcion 1: Usar el Launcher (Recomendado)
```bash
# Desde el directorio raiz del proyecto (/mnt/d/Kairos)
./start_kairos.sh
```

### Opcion 2: Ejecutar Directamente
```bash
# Cambiar al directorio del proyecto
cd kairos

# Ejecutar el script principal
./start_local.sh
```

## Problema Comun

**ERROR**: `could not find Cargo.toml in /mnt/d/Kairos or any parent directory`

**CAUSA**: El script se esta ejecutando desde el directorio incorrecto.

**SOLUCION**: El workspace de Rust esta en `kairos/`, no en la raiz del proyecto.

## Estructura del Proyecto

```
/mnt/d/Kairos/                    <- Directorio raiz
├── start_kairos.sh               <- Script launcher (USAR ESTE)
├── INSTRUCCIONES_ARRANQUE.md     <- Este archivo
└── kairos/                       <- Directorio del proyecto Rust
    ├── Cargo.toml                <- Workspace principal
    ├── start_local.sh            <- Script principal
    ├── backend/
    │   └── Cargo.toml
    ├── frontend/
    │   └── Cargo.toml
    └── shared/
        └── Cargo.toml
```

## Proceso de Arranque

1. **Verificacion**: El script valida la estructura del proyecto
2. **Dependencias**: Verifica que todas las herramientas esten instaladas
3. **Base de Datos**: Ejecuta migraciones si es necesario
4. **Compilacion**: cargo watch y dx serve manejan la compilacion automatica
5. **Servicios**: Inicia backend (puerto 8080) y frontend (puerto 8082)

## Solucion de Problemas

### Si el script falla:
1. Verifica que estes en el directorio correcto: `/mnt/d/Kairos`
2. Usa el launcher: `./start_kairos.sh`
3. Revisa el log: `kairos/kairos_startup.log`

### Si hay problemas de permisos:
```bash
chmod +x start_kairos.sh
chmod +x kairos/start_local.sh
```

### Si PostgreSQL no esta corriendo:
```bash
sudo service postgresql start
```

## Endpoints Disponibles

- **Backend API**: http://localhost:8080
- **Frontend Web**: http://localhost:8082
- **Documentacion**: Los endpoints se documentan automaticamente

## Controles

- **Ctrl+C**: Detiene todos los servicios limpiamente
- **Logs**: `tail -f kairos/kairos_startup.log`
- **Procesos**: `ps aux | grep kairos` 