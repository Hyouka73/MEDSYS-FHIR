#!/usr/bin/env bash
# ==============================================================================
# Script de inicio PostgreSQL 16 para MedSys-FHIR (Linux / macOS / WSL)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="${SCRIPT_DIR}/docker-compose.yml"

echo "=========================================================="
echo "  MedSys-FHIR — Inicialización de PostgreSQL 16 (NOM-004)"
echo "=========================================================="

if ! command -v docker &> /dev/null; then
    echo "[ERROR] El comando 'docker' no fue encontrado."
    exit 1
fi

echo "[1/3] Verificando motor Docker..."
if ! docker info &> /dev/null; then
    echo "[ERROR] Docker no está ejecutándose. Inicia Docker y reintenta."
    exit 1
fi
echo "  ✔ Docker en ejecución."

echo "[2/3] Levantando contenedor con Docker Compose..."
docker compose -f "${COMPOSE_FILE}" up -d

echo "[3/3] Esperando verificación de salud..."
CONTAINER="medsys_fhir_postgres"
RETRIES=15
COUNT=0
HEALTHY=false

while [ $COUNT -lt $RETRIES ]; do
    sleep 2
    STATUS=$(docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$CONTAINER" 2>/dev/null || echo "starting")
    if [ "$STATUS" = "healthy" ]; then
        HEALTHY=true
        break
    fi
    COUNT=$((COUNT + 1))
    echo "  ... verificando ($COUNT/$RETRIES) - Estado: $STATUS"
done

echo ""
if [ "$HEALTHY" = true ]; then
    echo "=========================================================="
    echo "  ✔ Base de datos PostgreSQL 16 lista y saludable!"
    echo "=========================================================="
else
    echo "  ⚠ Contenedor en ejecución, pero aún inicializándose."
fi

echo ""
echo "Configuración de Acceso:"
echo "  DATABASE_URL: postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy"
echo ""
