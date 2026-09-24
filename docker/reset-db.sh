#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="${SCRIPT_DIR}/docker-compose.yml"

echo "Reiniciando base de datos y eliminando volúmenes existentes..."
docker compose -f "${COMPOSE_FILE}" down -v
echo "Levantando base de datos fresca y ejecutando schema_legado_simulado_nom004.sql..."
docker compose -f "${COMPOSE_FILE}" up -d
echo "✔ Base de datos reiniciada con éxito."
