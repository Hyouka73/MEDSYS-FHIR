#!/usr/bin/env bash
# ==============================================================================
# MedSys-FHIR: Pipeline de Benchmarking con Telemetría docker stats
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
# Trazabilidad: AUD-002 / CON-004 / CVD-004
# ==============================================================================
set -euo pipefail

# 1. Configuración de rutas y variables operativas
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
COMPOSE_FILE="${PROJECT_ROOT}/docker/docker-compose.yml"
CONTAINER_NAME="medsys-server"
BASE_URL="${BASE_URL:-http://localhost:8080}"
CSV_OUTPUT="${SCRIPT_DIR}/benchmark_stats.csv"
K6_SCRIPT="${SCRIPT_DIR}/load_test.js"
SAMPLING_INTERVAL_SEC=5
MAX_WAIT_SEC=90

echo "================================================================================"
echo " MedSys-FHIR — Orquestación Docker y Pipeline de Benchmarking Reproducible"
echo " Límite cgroups: 1.0 vCPU | 256 MB RAM | Muestreo docker stats cada ${SAMPLING_INTERVAL_SEC}s"
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

# 5. Iniciar captura periódica de telemetría docker stats en segundo plano
echo "timestamp,cpu_percentage,mem_usage_raw" > "${CSV_OUTPUT}"
echo "[*] Iniciando muestreo de docker stats sobre '${CONTAINER_NAME}' cada ${SAMPLING_INTERVAL_SEC}s -> ${CSV_OUTPUT}..."

STATS_PID=""
collect_stats() {
    while true; do
        ts=$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date +"%Y-%m-%dT%H:%M:%S")
        # Captura pasiva no bloqueante
        stat_line=$(docker stats "${CONTAINER_NAME}" --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>/dev/null || echo "0.00%,0MiB / 256MiB")
        if [ -n "${stat_line}" ]; then
            echo "${ts},${stat_line}" >> "${CSV_OUTPUT}"
        fi
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

# 6. Ejecución de la prueba de carga k6
echo "================================================================================"
echo "[*] Ejecutando k6 con escenario de meseta sostenida a 50 VUs (load_test.js)..."
echo "================================================================================"
set +e
k6 run --env BASE_URL="${BASE_URL}" "${K6_SCRIPT}"
K6_EXIT_CODE=$?
set -e

# Detener el muestreo de recursos
cleanup
trap - EXIT INT TERM

echo ""
echo "================================================================================"
echo " Consolidación de Telemetría de Recursos (docker stats) — MedSys-FHIR"
echo "================================================================================"

# 7. Procesar y calcular estadísticas consolidadas desde el CSV
if [ -f "${CSV_OUTPUT}" ] && [ $(wc -l < "${CSV_OUTPUT}") -gt 1 ]; then
    python3 - <<EOF || python - <<EOF || awk -F',' 'NR>1 {print \$0}' "${CSV_OUTPUT}"
import csv, re

cpu_vals = []
mem_vals_mib = []

def parse_mem(mem_str):
    m = re.match(r'([0-9.]+)\s*([A-Za-z]+)', mem_str.strip())
    if not m:
        return 0.0
    val, unit = float(m.group(1)), m.group(2).lower()
    if 'k' in unit:
        return val / 1024.0
    elif 'g' in unit:
        return val * 1024.0
    return val

with open("${CSV_OUTPUT}", mode='r') as f:
    reader = csv.reader(f)
    next(reader, None)
    for row in reader:
        if len(row) >= 3:
            try:
                cpu = float(row[1].replace('%', '').strip())
                cpu_vals.append(cpu)
            except ValueError:
                pass
            try:
                mem_part = row[2].split('/')[0]
                mem_vals_mib.append(parse_mem(mem_part))
            except Exception:
                pass

def percentile(vals, p):
    if not vals:
        return 0.0
    s = sorted(vals)
    k = (len(s) - 1) * (p / 100.0)
    f = int(k)
    c = min(f + 1, len(s) - 1)
    d = k - f
    return s[f] + (s[c] - s[f]) * d

samples = len(cpu_vals)
if samples > 0:
    cpu_avg = sum(cpu_vals) / samples
    cpu_p95 = percentile(cpu_vals, 95)
    cpu_max = max(cpu_vals)

    mem_avg = sum(mem_vals_mib) / samples
    mem_p95 = percentile(mem_vals_mib, 95)
    mem_max = max(mem_vals_mib)

    print(f"Total de Muestras Colectadas: {samples}")
    print("--------------------------------------------------------------------------------")
    print(f"Consumo de CPU:       Promedio: {cpu_avg:.2f}% | p95: {cpu_p95:.2f}% | Max: {cpu_max:.2f}% (Criterio: <= 50.0%)")
    print(f"Memoria Residente:    Promedio: {mem_avg:.2f} MiB | p95: {mem_p95:.2f} MiB | Max: {mem_max:.2f} MiB (Criterio: <= 150 MiB)")
    print("--------------------------------------------------------------------------------")
    
    cpu_ok = "APROBADO" if cpu_p95 <= 50.0 else "RECHAZADO"
    mem_ok = "APROBADO" if mem_p95 <= 150.0 else "RECHAZADO"
    print(f"Evaluación de Criterios: CPU [ {cpu_ok} ] | Memoria RSS [ {mem_ok} ]")
else:
    print("No se registraron muestras suficientes en benchmark_stats.csv")
EOF
else
    echo "[!] No se encontraron muestras en ${CSV_OUTPUT}."
fi

echo "================================================================================"
echo "[✓] Telemetría exportada a: ${CSV_OUTPUT}"
if [ ${K6_EXIT_CODE} -ne 0 ]; then
    echo "[!] ADVERTENCIA: k6 finalizó con código de retorno ${K6_EXIT_CODE}."
    exit ${K6_EXIT_CODE}
fi
echo "[✓] Benchmark finalizado exitosamente."
