# Script de PowerShell para iniciar Kairos en WSL

# Mensaje de inicio
Write-Host "Iniciando Kairos..." -ForegroundColor Green

# Verificar si WSL está instalado
if (!(Get-Command wsl -ErrorAction SilentlyContinue)) {
    Write-Host "WSL no está instalado. Instalando..." -ForegroundColor Yellow
    wsl --install
    Write-Host "Por favor, reinicie su computadora y ejecute este script nuevamente." -ForegroundColor Yellow
    exit 1
}

# Verificar si Docker Desktop está instalado
if (!(Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "Docker Desktop no está instalado. Descargando e instalando..." -ForegroundColor Yellow
    $dockerUrl = "https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe"
    $installerPath = "$env:TEMP\DockerDesktopInstaller.exe"
    
    Invoke-WebRequest -Uri $dockerUrl -OutFile $installerPath
    Start-Process -FilePath $installerPath -Wait
    
    Write-Host "Por favor, reinicie su computadora y ejecute este script nuevamente." -ForegroundColor Yellow
    exit 1
}

# Verificar si Docker Desktop está corriendo
$dockerRunning = $false
try {
    $null = docker info
    $dockerRunning = $true
} catch {
    Write-Host "Docker Desktop no está corriendo. Iniciando..." -ForegroundColor Yellow
    Start-Process "C:\Program Files\Docker\Docker\Docker Desktop.exe"
    
    # Esperar a que Docker esté listo
    $attempts = 0
    $maxAttempts = 30
    while ($attempts -lt $maxAttempts) {
        try {
            $null = docker info
            $dockerRunning = $true
            break
        } catch {
            Start-Sleep -Seconds 2
            $attempts++
        }
    }
}

if (!$dockerRunning) {
    Write-Host "No se pudo iniciar Docker Desktop después de varios intentos." -ForegroundColor Red
    exit 1
}

# Obtener la ruta del script actual
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path

# Ejecutar el script bash en WSL
Write-Host "Ejecutando script de inicio en WSL..." -ForegroundColor Yellow
wsl bash "$scriptPath/start.sh"

# Verificar el resultado
if ($LASTEXITCODE -eq 0) {
    Write-Host "Kairos iniciado correctamente!" -ForegroundColor Green
    Write-Host "Frontend: http://localhost:8081" -ForegroundColor Green
    Write-Host "Backend: http://localhost:8080" -ForegroundColor Green
} else {
    Write-Host "Error al iniciar Kairos." -ForegroundColor Red
    exit 1
} 