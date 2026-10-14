#!/usr/bin/env bash
# ==============================================================================
# MedSys-FHIR: Pipeline de Benchmarking con Telemetría docker stats y VmRSS
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
# Trazabilidad: AUD-002 / CON-004 / CVD-004 (Límites cgroups 1.0 vCPU, 256MB RAM)
# ==============================================================================
set -euo pipefail

# 1. Configuración de rutas y variables operativas
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
COMPOSE_FILE="${PROJECT_ROOT}/docker/docker-compose.yml"
CONTAINER_NAME="medsys-server"
DB_CONTAINER_NAME="medsys_fhir_postgres"
BASE_URL="${BASE_URL:-http://localhost:8080}"
CSV_OUTPUT="${SCRIPT_DIR}/benchmark_stats.csv"
K6_SCRIPT="${SCRIPT_DIR}/load_test.js"
SAMPLING_INTERVAL_SEC=5
MAX_WAIT_SEC=90
REPLICAS="${REPLICAS:-1}"

echo "================================================================================"
echo " MedSys-FHIR — Orquestación Docker y Pipeline de Benchmarking Reproducible"
echo " Límite cgroups: 1.0 vCPU | 256 MB RAM | Muestreo de Telemetría cada ${SAMPLING_INTERVAL_SEC}s"
echo " Réplicas configuradas: ${REPLICAS}"
echo "================================================================================"

# 2. Verificar dependencias obligatorias
command -v docker >/dev/null 2>&1 || { echo "[!] ERROR: 'docker' no está instalado en PATH." >&2; exit 1; }
command -v k6 >/dev/null 2>&1 || { echo "[!] ERROR: 'k6' no está instalado en PATH." >&2; exit 1; }

# 3. Levantar entorno contenerizado en segundo plano
echo "[*] Desplegando servicios contenerizados con directivas cgroups v2..."
docker compose -f "${COMPOSE_FILE}" up -d

# 4. Esperar a que el endpoint de salud responda HTTP 200
echo "[*] Esperando disponibilidad de ${BASE_URL}/health (Timeout: ${MAX_WAIT_SEC}s)..."
elapsed=0
ready=0
while [ $elapsed -lt $MAX_WAIT_SEC ]; do
    if curl -s -f -o /dev/null "${BASE_URL}/health"; then
        ready=1
        break
    fi
    sleep 2
    elapsed=$((elapsed + 2))
    echo "    ... esperando servicio (${elapsed}s transcurridos)"
done

if [ $ready -ne 1 ]; then
    echo "[!] ERROR: El middleware no respondió HTTP 200 en ${BASE_URL}/health tras ${MAX_WAIT_SEC}s." >&2
    echo "[*] Volcado de logs de medsys-server:" >&2
    docker compose -f "${COMPOSE_FILE}" logs --tail 30 medsys-server || true
    exit 1
fi

echo "[✓] Middleware medsys-server disponible y respondiendo HTTP 200."

# 5. Iniciar captura periódica de telemetría completa en segundo plano
echo "timestamp,server_cpu,server_mem,postgres_cpu,postgres_mem,vm_rss_kb,cgroup_memory_bytes" > "${CSV_OUTPUT}"
echo "[*] Iniciando muestreo de telemetría conjunta cada ${SAMPLING_INTERVAL_SEC}s -> ${CSV_OUTPUT}..."

STATS_PID=""
collect_stats() {
    while true; do
        ts=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date +"%Y-%m-%dT%H:%M:%S")
        
        # Telemetría de medsys-server
        raw_srv=$(docker stats "${CONTAINER_NAME}" --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>/dev/null || echo "0.00%,0MiB / 256MiB")
        srv_cpu=$(echo "${raw_srv}" | cut -d',' -f1)
        srv_mem=$(echo "${raw_srv}" | cut -d',' -f2)

        # Telemetría de postgres
        raw_db=$(docker stats "${DB_CONTAINER_NAME}" --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>/dev/null || echo "0.00%,0MiB / 512MiB")
        db_cpu=$(echo "${raw_db}" | cut -d',' -f1)
        db_mem=$(echo "${raw_db}" | cut -d',' -f2)

        # VmRSS real y cgroup
        vm_rss=$(docker exec "${CONTAINER_NAME}" sh -c "grep VmRSS /proc/1/status 2>/dev/null | awk '{print \$2}'" 2>/dev/null || echo "0")
        cgroup_mem=$(docker exec "${CONTAINER_NAME}" sh -c "cat /sys/fs/cgroup/memory.current 2>/dev/null || cat /sys/fs/cgroup/memory/memory.usage_in_bytes 2>/dev/null" 2>/dev/null || echo "0")

        echo "${ts},${srv_cpu},${srv_mem},${db_cpu},${db_mem},${vm_rss},${cgroup_mem}" >> "${CSV_OUTPUT}"
        sleep "${SAMPLING_INTERVAL_SEC}"
    done
}

collect_stats &
STATS_PID=$!

# Garantizar detención del proceso de muestreo en caso de aborto
cleanup() {
    if [ -n "${STATS_PID}" ] && kill -0 "${STATS_PID}" 2>/dev/null; then
        echo "[*] Deteniendo recolección de telemetría (PID: ${STATS_PID})..."
        kill "${STATS_PID}" 2>/dev/null || true
        wait "${STATS_PID}" 2>/dev/null || true
    fi
}
trap cleanup EXIT INT TERM

# 6. Ejecución de la(s) réplica(s) de k6
echo "================================================================================"
echo "[*] Ejecutando k6 con escenario de meseta sostenida a 50 VUs (load_test.js)..."
echo "================================================================================"

set +e
OVERALL_EXIT_CODE=0
for rep in $(seq 1 "${REPLICAS}"); do
    echo "[*] Iniciando réplica ${rep}/${REPLICAS}..."
    k6 run --env BASE_URL="${BASE_URL}" --out "json=${SCRIPT_DIR}/k6_replica_${rep}.json" "${K6_SCRIPT}"
    rc=$?
    if [ $rc -ne 0 ]; then
        OVERALL_EXIT_CODE=$rc
        echo "[WARN] Réplica ${rep} finalizó con código ${rc}"
    fi
done
set -e

# Detener el muestreo de recursos
cleanup
trap - EXIT INT TERM

echo ""
echo "================================================================================"
echo " Consolidación de Telemetría (docker stats) — MedSys-FHIR"
echo "================================================================================"
echo "[✓] Telemetría exportada a: ${CSV_OUTPUT}"
exit ${OVERALL_EXIT_CODE}
