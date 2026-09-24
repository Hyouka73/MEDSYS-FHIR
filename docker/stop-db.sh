#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="${SCRIPT_DIR}/docker-compose.yml"

echo "Deteniendo contenedor PostgreSQL 16..."
docker compose -f "${COMPOSE_FILE}" down
echo "✔ Contenedor detenido correctamente."
