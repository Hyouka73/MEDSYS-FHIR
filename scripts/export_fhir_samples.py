#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Exportador Exhaustivo de Recursos FHIR R4 para Validación HL7
# Consulta los endpoints canónicos de medsys-server y persiste cada recurso
# como archivo JSON individual en el directorio de salida (output/).
# Diseñado para validación por lotes con org.hl7.fhir.validator-cli.
#
# Rangos del universo sintético (generate_data.py, SEED=20260930):
#   Patient       IDs  1 – 1,000
#   Encounter     IDs  1 – 2,500
#   Observation   IDs  1 – 2,500  (subtipos: bp, temp, hr)
#   Condition     IDs  1 – 3,000
#
# Resolución de hallazgo AUD-007: amplía --count (default 100) y añade
# la bandera --all para cubrir el 100% del universo sintético relacional.
#
# Comando oficial de validación sintáctica offline (Capítulo III, §3.6.2):
#   java -jar validator_cli.jar output/*.json -version 4.0.1 -tx n/a
# ==============================================================================

import argparse
import glob
import json
import os
import subprocess
import sys
import time
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed

# ---------------------------------------------------------------------------
# Rangos completos del universo sintético (alineados con generate_data.py)
# ---------------------------------------------------------------------------
UNIVERSE_RANGES = {
    "Patient":          (1, 1000),   # 1,000 pacientes
    "Encounter":        (1, 2500),   # 2,500 consultas
    "Observation_bp":   (1, 2500),   # presión arterial  (LOINC 85354-9)
    "Observation_temp": (1, 2500),   # temperatura        (LOINC 8310-5)
    "Observation_hr":   (1, 2500),   # frecuencia cardíaca (LOINC 8867-4)
    "Condition":        (1, 3000),   # 3,000 diagnósticos CIE-10
}

# Hilos concurrentes por defecto (equilibrio entre velocidad y saturación)
DEFAULT_WORKERS = 10


# ---------------------------------------------------------------------------
# Construcción de URL de endpoint por tipo de recurso e ID
# ---------------------------------------------------------------------------
def _build_url(base_url: str, resource_type: str, resource_id: int) -> str:
    """Retorna la URL canónica FHIR para el recurso solicitado.
    
    No modifica el contrato RESTful de los endpoints (restricción CÓD-03).
    """
    if resource_type == "Patient":
        return f"{base_url}/fhir/r4/Patient/{resource_id}"
    if resource_type == "Encounter":
        return f"{base_url}/fhir/r4/Encounter/{resource_id}"
    if resource_type == "Observation_bp":
        return f"{base_url}/fhir/r4/Observation/bp-{resource_id}"
    if resource_type == "Observation_temp":
        return f"{base_url}/fhir/r4/Observation/temp-{resource_id}"
    if resource_type == "Observation_hr":
        return f"{base_url}/fhir/r4/Observation/hr-{resource_id}"
    if resource_type == "Condition":
        return f"{base_url}/fhir/r4/Condition/{resource_id}"
    raise ValueError(f"Tipo de recurso desconocido: {resource_type}")


def _build_filename(output_dir: str, resource_type: str, resource_id: int) -> str:
    """Retorna la ruta de archivo de destino con nomenclatura canónica limpia."""
    return os.path.join(output_dir, f"{resource_type}_{resource_id}.json")


# ---------------------------------------------------------------------------
# Descarga y persistencia atómica de un único recurso FHIR
# ---------------------------------------------------------------------------
def fetch_resource(url: str, output_path: str) -> tuple:
    """Realiza una petición GET al endpoint FHIR y persiste el JSON formateado.

    Valida:
      - Código de estado HTTP 200.
      - Cabecera Content-Type contiene 'application/fhir+json'.

    Retorna:
      (True, bytes_raw)  si el recurso fue recuperado y guardado exitosamente.
      (False, 0)         en caso de error o código no exitoso (e.g., 404).
    """
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/fhir+json",
            "User-Agent": "MedSys-FHIR-Exporter/2.0",
        },
    )

    try:
        with urllib.request.urlopen(req, timeout=15) as response:
            # Validación de código HTTP
            if response.status != 200:
                print(f"[WARN] HTTP {response.status} recibido para {url}", flush=True)
                return False, 0

            # Validación de Content-Type
            content_type = response.headers.get("Content-Type", "")
            if "application/fhir+json" not in content_type:
                print(
                    f"[WARN] Content-Type inesperado '{content_type}' para {url}",
                    flush=True,
                )
                return False, 0

            raw_bytes = response.read()
            data = json.loads(raw_bytes.decode("utf-8"))
            with open(output_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
                f.write("\n")

            return True, len(raw_bytes)

    except urllib.error.HTTPError as e:
        if e.code == 404:
            # 404 silencioso: IDs 1-2 de consultas/signos/diagnósticos pueden no existir
            pass
        else:
            print(f"[WARN] HTTP {e.code} ({e.reason}) para {url}", flush=True)
        return False, 0
    except urllib.error.URLError as e:
        print(f"[ERROR] Error de conexión al consultar {url}: {e.reason}", file=sys.stderr, flush=True)
        return False, 0
    except json.JSONDecodeError as e:
        print(f"[ERROR] JSON inválido recibido de {url}: {e}", file=sys.stderr, flush=True)
        return False, 0
    except Exception as e:
        print(f"[ERROR] Error inesperado al procesar {url}: {e}", file=sys.stderr, flush=True)
        return False, 0


# ---------------------------------------------------------------------------
# Worker concurrente: descarga un ítem de trabajo
# ---------------------------------------------------------------------------
def _worker(task: tuple) -> tuple:
    """Ejecuta fetch_resource y retorna (resource_type, success, nbytes)."""
    resource_type, url, output_path = task
    success, nbytes = fetch_resource(url, output_path)
    return resource_type, success, nbytes


# ---------------------------------------------------------------------------
# Exportación por lotes con concurrencia controlada
# ---------------------------------------------------------------------------
def export_resources(
    base_url: str,
    output_dir: str,
    ranges: dict,
    max_workers: int = DEFAULT_WORKERS,
) -> tuple:
    """Descarga todos los recursos FHIR definidos en 'ranges' con concurrencia controlada.

    Args:
        base_url:    URL base del servidor MedSys-FHIR.
        output_dir:  Directorio de destino para los archivos JSON.
        ranges:      Diccionario {resource_type: (id_start, id_end)}.
        max_workers: Hilos concurrentes máximos (evita saturar el middleware).

    Returns:
        (counts, total_bytes, elapsed_seconds)
    """
    counts = {k: 0 for k in ranges}
    total_bytes = 0
    total_tasks = sum((end - start + 1) for start, end in ranges.values())
    completed = 0
    start_time = time.monotonic()

    # Construir lista completa de tareas
    tasks = []
    for resource_type, (id_start, id_end) in ranges.items():
        for rid in range(id_start, id_end + 1):
            url = _build_url(base_url, resource_type, rid)
            path = _build_filename(output_dir, resource_type, rid)
            tasks.append((resource_type, url, path))

    print(
        f"[*] Total de solicitudes programadas: {total_tasks:,} "
        f"(hilos concurrentes: {max_workers})",
        flush=True,
    )
    print("-" * 60, flush=True)

    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(_worker, task): task for task in tasks}

        for future in as_completed(futures):
            resource_type, success, nbytes = future.result()
            completed += 1
            if success:
                counts[resource_type] += 1
                total_bytes += nbytes

            # Progreso cada 500 tareas o al finalizar
            if completed % 500 == 0 or completed == total_tasks:
                pct = completed / total_tasks * 100
                elapsed = time.monotonic() - start_time
                rate = completed / elapsed if elapsed > 0 else 0
                print(
                    f"[*] Progreso: {completed:,}/{total_tasks:,} "
                    f"({pct:.1f}%) — {rate:.1f} req/s",
                    flush=True,
                )

    elapsed = time.monotonic() - start_time
    return counts, total_bytes, elapsed


# ---------------------------------------------------------------------------
# Validador HL7 FHIR offline con parseador de severidades
# ---------------------------------------------------------------------------
def run_hl7_validator(validator_jar: str, output_dir: str) -> bool:
    """Ejecuta org.hl7.fhir.validator-cli sobre los archivos JSON exportados
    utilizando estrictamente el parámetro offline `-tx n/a`.

    Contabiliza incidencias por severidad: Information, Warning, Error, Fatal.
    """
    exact_cmd = f"java -jar {validator_jar} {output_dir}/*.json -version 4.0.1 -tx n/a"
    print(f"\n[*] Ejecutando validador oficial HL7 en modo offline (-tx n/a)...")
    print(f"    Comando referencial: {exact_cmd}\n")

    if not os.path.exists(validator_jar):
        print(f"[WARN] No se localizó el archivo '{validator_jar}'.")
        print(f"       Para ejecutar manualmente descargue validator_cli.jar y corra:")
        print(f"       {exact_cmd}")
        return False

    # En Windows, Java CLI no expande globs → pasar lista explícita de archivos
    json_files = glob.glob(os.path.join(output_dir, "*.json"))
    if not json_files:
        print(f"[WARN] No se encontraron archivos JSON en '{output_dir}' para validar.")
        return False

    print(f"[*] Archivos a validar: {len(json_files):,}", flush=True)

    cmd = ["java", "-jar", validator_jar] + sorted(json_files) + [
        "-version", "4.0.1", "-tx", "n/a"
    ]

    severity_counts = {
        "Fatal":       0,
        "Error":       0,
        "Warning":     0,
        "Information": 0,
    }

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )

        output_lines = (result.stdout + result.stderr).splitlines()
        for line in output_lines:
            print(line, flush=True)
            lower = line.lower()
            if "fatal" in lower:
                severity_counts["Fatal"] += 1
            elif "error" in lower:
                severity_counts["Error"] += 1
            elif "warning" in lower:
                severity_counts["Warning"] += 1
            elif "information" in lower:
                severity_counts["Information"] += 1

        print("\n" + "=" * 60, flush=True)
        print("RESUMEN DE INCIDENCIAS POR SEVERIDAD (HL7 validator-cli)", flush=True)
        print("=" * 60, flush=True)
        for severity, count in severity_counts.items():
            print(f"  {severity:<14}: {count:>6,}", flush=True)
        print("=" * 60, flush=True)

        has_critical = (severity_counts["Fatal"] + severity_counts["Error"]) > 0
        if has_critical:
            print(
                "[FAIL] Se detectaron incidencias de severidad Fatal o Error. "
                "La hipótesis de conformidad NO se sostiene.",
                flush=True,
            )
        else:
            print(
                "[OK] Sin incidencias Fatal ni Error. "
                "Conformidad sintáctica 100% verificada (Objetivo Específico 3).",
                flush=True,
            )

        return result.returncode == 0

    except FileNotFoundError:
        print(
            "[ERROR] Java Runtime Environment no está disponible en el PATH del sistema.",
            file=sys.stderr,
            flush=True,
        )
        return False
    except Exception as e:
        print(f"[ERROR] Error al invocar validator_cli.jar: {e}", file=sys.stderr, flush=True)
        return False


# ---------------------------------------------------------------------------
# Punto de entrada principal
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(
        description=(
            "Exportador exhaustivo de recursos FHIR R4 para validación sintáctica "
            "oficial (HL7 validator-cli). Resolución de hallazgo AUD-007."
        )
    )
    parser.add_argument(
        "--base-url",
        type=str,
        default="http://localhost:3000",
        help="URL base del servidor MedSys-FHIR en ejecución (default: http://localhost:3000)",
    )
    parser.add_argument(
        "--all",
        dest="export_all",
        action="store_true",
        help=(
            "Exportar la totalidad de los registros clínicos existentes: "
            "Patient 1-1,000 | Encounter 1-2,500 | Observation 1-2,500 (x3) | Condition 1-3,000"
        ),
    )
    parser.add_argument(
        "--count",
        type=int,
        default=100,
        help=(
            "Cantidad de muestras a exportar por recurso si no se usa --all "
            "(default: 100, elevado desde 50 por AUD-007)"
        ),
    )
    parser.add_argument(
        "--workers",
        type=int,
        default=DEFAULT_WORKERS,
        help=f"Número máximo de hilos concurrentes para la descarga (default: {DEFAULT_WORKERS})",
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="output",
        help="Directorio de destino para los archivos JSON exportados (default: output)",
    )
    parser.add_argument(
        "--validator-jar",
        type=str,
        default="validator_cli.jar",
        help="Ruta al binario ejecutable validator_cli.jar de HL7 (default: validator_cli.jar)",
    )
    parser.add_argument(
        "--validate",
        action="store_true",
        help=(
            "Ejecuta automáticamente la validación HL7 offline: "
            "java -jar validator_cli.jar output/*.json -version 4.0.1 -tx n/a"
        ),
    )

    args = parser.parse_args()
    base_url = args.base_url.rstrip("/")
    output_dir = args.output_dir
    max_workers = max(1, args.workers)

    os.makedirs(output_dir, exist_ok=True)

    # -- Determinar rangos de exportación ------------------------------------
    if args.export_all:
        ranges = UNIVERSE_RANGES.copy()
        mode_label = "COMPLETO (--all) — universo sintético total"
    else:
        n = args.count
        ranges = {
            "Patient":          (1, min(n, 1000)),
            "Encounter":        (1, min(n, 2500)),
            "Observation_bp":   (1, min(n, 2500)),
            "Observation_temp": (1, min(n, 2500)),
            "Observation_hr":   (1, min(n, 2500)),
            "Condition":        (1, min(n, 3000)),
        }
        mode_label = f"MUESTRA ({n} registros por tipo de recurso)"

    total_requests = sum((end - start + 1) for start, end in ranges.values())

    print("=" * 60, flush=True)
    print("MedSys-FHIR — EXPORTADOR DE RECURSOS FHIR R4", flush=True)
    print("=" * 60, flush=True)
    print(f"[*] Modo:               {mode_label}", flush=True)
    print(f"[*] URL Base:           {base_url}", flush=True)
    print(f"[*] Directorio salida:  {output_dir}", flush=True)
    print(f"[*] Hilos concurrentes: {max_workers}", flush=True)
    print(f"[*] Solicitudes totales:{total_requests:>8,}", flush=True)
    print("-" * 60, flush=True)
    for rtype, (rstart, rend) in ranges.items():
        n_ids = rend - rstart + 1
        print(f"    {rtype:<20} IDs {rstart:>5} – {rend:<5}  ({n_ids:,} registros)", flush=True)
    print("=" * 60, flush=True)

    # -- Exportación concurrente ---------------------------------------------
    counts, total_bytes, elapsed = export_resources(
        base_url=base_url,
        output_dir=output_dir,
        ranges=ranges,
        max_workers=max_workers,
    )

    # -- Métricas de resumen -------------------------------------------------
    total_exported = sum(counts.values())
    avg_kb = (total_bytes / total_exported / 1024) if total_exported > 0 else 0.0

    total_obs = (
        counts["Observation_bp"]
        + counts["Observation_temp"]
        + counts["Observation_hr"]
    )
    total_all_res = (
        counts["Patient"]
        + counts["Encounter"]
        + total_obs
        + counts["Condition"]
    )

    print("\n" + "=" * 60, flush=True)
    print("RESUMEN DE EXPORTACIÓN FHIR R4", flush=True)
    print("=" * 60, flush=True)
    print(f"Directorio de salida:              {output_dir}", flush=True)
    print(f"Tiempo transcurrido:               {elapsed:.2f} s", flush=True)
    print(f"Total Patient exportados:          {counts['Patient']:>6,}", flush=True)
    print(f"Total Encounter exportados:        {counts['Encounter']:>6,}", flush=True)
    print(f"Total Observation (bp) export.:    {counts['Observation_bp']:>6,}", flush=True)
    print(f"Total Observation (temp) export.:  {counts['Observation_temp']:>6,}", flush=True)
    print(f"Total Observation (hr) export.:    {counts['Observation_hr']:>6,}", flush=True)
    print(f"  -> Subtotal Observation:         {total_obs:>6,}", flush=True)
    print(f"Total Condition exportados:        {counts['Condition']:>6,}", flush=True)
    print("-" * 60, flush=True)
    print(f"TOTAL ARCHIVOS EXPORTADOS:         {total_all_res:>6,}", flush=True)
    print(f"Tamaño promedio por recurso:       {avg_kb:>8.2f} KB", flush=True)
    print("=" * 60, flush=True)

    # -- Comando canónico de validación HL7 ----------------------------------
    exact_validation_cmd = (
        f"java -jar {args.validator_jar} {output_dir}/*.json -version 4.0.1 -tx n/a"
    )
    print("\n" + "=" * 60, flush=True)
    print("COMANDO OFICIAL DE VALIDACIÓN SINTÁCTICA HL7 FHIR (OFFLINE)", flush=True)
    print("=" * 60, flush=True)
    print("Para validar los recursos exportados en modo desconectado estricto:", flush=True)
    print(f"  {exact_validation_cmd}", flush=True)
    print("=" * 60, flush=True)

    if args.validate:
        run_hl7_validator(args.validator_jar, output_dir)


if __name__ == "__main__":
    main()
