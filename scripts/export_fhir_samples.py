#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Exportador Exhaustivo de Recursos FHIR R4 para Validación HL7
# Consulta los endpoints canónicos de medsys-server y persiste cada recurso
# como archivo JSON individual en el directorio de salida (output/).
# Diseñado para validación por lotes con org.hl7.fhir.validator-cli.
#
# Censo completo del universo sintético relacional (generate_data.py, SEED=20260930):
#   - 1,000 Patient
#   - 2,500 Encounter
#   - 9,750 Observation:
#       * 2,500 Observation_bp   (LOINC 85354-9)
#       * 2,500 Observation_temp (LOINC 8310-5)
#       * 2,500 Observation_hr   (LOINC 8867-4)
#       * 2,250 Observation_resp (LOINC 9279-1, y 250 omitidas limpiamente por NULL)
#   - 3,000 Condition (CIE-10)
#   Total exacto: 16,250 archivos JSON FHIR R4
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
# Rangos censales del universo sintético (alineados con generate_data.py)
# ---------------------------------------------------------------------------
UNIVERSE_RANGES = {
    "Patient":          (1, 1000),   # 1,000 pacientes
    "Encounter":        (1, 2500),   # 2,500 consultas
    "Observation_bp":   (1, 2500),   # 2,500 presión arterial  (LOINC 85354-9)
    "Observation_temp": (1, 2500),   # 2,500 temperatura corporal (LOINC 8310-5)
    "Observation_hr":   (1, 2500),   # 2,500 frecuencia cardíaca (LOINC 8867-4)
    "Observation_resp": (1, 2500),   # 2,250 frecuencia respiratoria (LOINC 9279-1, 250 omitidas por NULL)
    "Condition":        (1, 3000),   # 3,000 diagnósticos CIE-10
}

# Hilos concurrentes por defecto
DEFAULT_WORKERS = 10

# Tamaño de lote para validación HL7 en Windows
VALIDATOR_BATCH_SIZE = 200


# ---------------------------------------------------------------------------
# Construcción de URL de endpoint por tipo de recurso e ID
# ---------------------------------------------------------------------------
def _build_url(base_url: str, resource_type: str, resource_id: int) -> str:
    """Retorna la URL canónica FHIR para el recurso solicitado."""
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
    if resource_type == "Observation_resp":
        return f"{base_url}/fhir/r4/Observation/resp-{resource_id}"
    if resource_type == "Condition":
        return f"{base_url}/fhir/r4/Condition/{resource_id}"
    raise ValueError(f"Tipo de recurso desconocido: {resource_type}")


def _build_filename(output_dir: str, resource_type: str, resource_id: int) -> str:
    """Retorna la ruta de archivo de destino con nomenclatura canónica limpia."""
    return os.path.join(output_dir, f"{resource_type}_{resource_id}.json")


# ---------------------------------------------------------------------------
# Descarga y persistencia atómica de un único recurso FHIR
# ---------------------------------------------------------------------------
def fetch_resource(url: str, output_path: str, is_optional: bool = False) -> tuple:
    """Realiza una petición GET al endpoint FHIR y persiste el JSON formateado.

    Retorna:
      (True, bytes_raw, False)  si el recurso fue guardado.
      (False, 0, True)          si fue una omisión controlada (HTTP 404 en recurso opcional).
      (False, 0, False)         en caso de error real.
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
            if response.status != 200:
                print(f"[WARN] HTTP {response.status} recibido para {url}", flush=True)
                return False, 0, False

            content_type = response.headers.get("Content-Type", "")
            if "application/fhir+json" not in content_type:
                print(
                    f"[WARN] Content-Type inesperado '{content_type}' para {url}",
                    flush=True,
                )
                return False, 0, False

            raw_bytes = response.read()
            data = json.loads(raw_bytes.decode("utf-8"))
            with open(output_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
                f.write("\n")

            return True, len(raw_bytes), False

    except urllib.error.HTTPError as e:
        if e.code == 404:
            if is_optional:
                # 404 esperado en Observation_resp cuando frecuencia_respiratoria IS NULL
                return False, 0, True
            pass
        else:
            print(f"[WARN] HTTP {e.code} ({e.reason}) para {url}", flush=True)
        return False, 0, False
    except urllib.error.URLError as e:
        print(f"[ERROR] Error de conexión al consultar {url}: {e.reason}", file=sys.stderr, flush=True)
        return False, 0, False
    except json.JSONDecodeError as e:
        print(f"[ERROR] JSON inválido recibido de {url}: {e}", file=sys.stderr, flush=True)
        return False, 0, False
    except Exception as e:
        print(f"[ERROR] Error inesperado al procesar {url}: {e}", file=sys.stderr, flush=True)
        return False, 0, False


# ---------------------------------------------------------------------------
# Worker concurrente
# ---------------------------------------------------------------------------
def _worker(task: tuple) -> tuple:
    """Ejecuta fetch_resource y retorna (resource_type, success, nbytes, omitted)."""
    resource_type, url, output_path, is_optional = task
    success, nbytes, omitted = fetch_resource(url, output_path, is_optional)
    return resource_type, success, nbytes, omitted


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

    Returns:
        (counts, omitted_counts, total_bytes, elapsed_seconds)
    """
    counts = {k: 0 for k in ranges}
    omitted_counts = {k: 0 for k in ranges}
    total_bytes = 0
    total_tasks = sum((end - start + 1) for start, end in ranges.values())
    completed = 0
    start_time = time.monotonic()

    tasks = []
    for resource_type, (id_start, id_end) in ranges.items():
        is_opt = (resource_type == "Observation_resp")
        for rid in range(id_start, id_end + 1):
            url = _build_url(base_url, resource_type, rid)
            path = _build_filename(output_dir, resource_type, rid)
            tasks.append((resource_type, url, path, is_opt))

    print(
        f"[*] Total de solicitudes programadas: {total_tasks:,} "
        f"(hilos concurrentes: {max_workers})",
        flush=True,
    )
    print("-" * 60, flush=True)

    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(_worker, task): task for task in tasks}

        for future in as_completed(futures):
            resource_type, success, nbytes, omitted = future.result()
            completed += 1
            if success:
                counts[resource_type] += 1
                total_bytes += nbytes
            elif omitted:
                omitted_counts[resource_type] += 1

            if completed % 1000 == 0 or completed == total_tasks:
                pct = completed / total_tasks * 100
                elapsed = time.monotonic() - start_time
                rate = completed / elapsed if elapsed > 0 else 0
                print(
                    f"[*] Progreso: {completed:,}/{total_tasks:,} "
                    f"({pct:.1f}%) — {rate:.1f} req/s",
                    flush=True,
                )

    elapsed = time.monotonic() - start_time
    return counts, omitted_counts, total_bytes, elapsed


# ---------------------------------------------------------------------------
# Validador HL7 FHIR offline con parseador de severidades y ejecución por lotes
# ---------------------------------------------------------------------------
def run_hl7_validator(validator_jar: str, output_dir: str, batch_size: int = VALIDATOR_BATCH_SIZE) -> bool:
    """Ejecuta org.hl7.fhir.validator-cli sobre los archivos JSON exportados
    utilizando el parámetro offline `-tx n/a`.

    Ejecuta en lotes de tamaño batch_size para no exceder la longitud máxima
    de línea de comandos en Windows (32,767 caracteres).
    """
    exact_cmd = f"java -jar {validator_jar} {output_dir}/*.json -version 4.0.1 -tx n/a"
    print(f"\n[*] Ejecutando validador oficial HL7 en modo offline (-tx n/a)...")
    print(f"    Comando referencial: {exact_cmd}\n")

    if not os.path.exists(validator_jar):
        print(f"[WARN] No se localizó el archivo '{validator_jar}'.")
        print(f"       Para ejecutar manualmente descargue validator_cli.jar y corra:")
        print(f"       {exact_cmd}")
        return False

    json_files = sorted(glob.glob(os.path.join(output_dir, "*.json")))
    if not json_files:
        print(f"[WARN] No se encontraron archivos JSON en '{output_dir}' para validar.")
        return False

    total_files = len(json_files)
    print(f"[*] Total de archivos JSON a validar: {total_files:,} (en lotes de {batch_size})", flush=True)

    severity_counts = {
        "Fatal":       0,
        "Error":       0,
        "Warning":     0,
        "Information": 0,
    }

    all_ok = True
    num_batches = (total_files + batch_size - 1) // batch_size

    for b_idx in range(num_batches):
        batch = json_files[b_idx * batch_size : (b_idx + 1) * batch_size]
        print(f"[*] Validando lote {b_idx + 1}/{num_batches} ({len(batch)} archivos)...", flush=True)

        cmd = ["java", "-jar", validator_jar] + batch + [
            "-version", "4.0.1", "-tx", "n/a"
        ]

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
                lower = line.lower()
                if "fatal" in lower:
                    severity_counts["Fatal"] += 1
                elif "error" in lower:
                    severity_counts["Error"] += 1
                elif "warning" in lower:
                    severity_counts["Warning"] += 1
                elif "information" in lower:
                    severity_counts["Information"] += 1

            if result.returncode != 0:
                all_ok = False

        except FileNotFoundError:
            print(
                "[ERROR] Java Runtime Environment no está disponible en el PATH del sistema.",
                file=sys.stderr,
                flush=True,
            )
            return False
        except Exception as e:
            print(f"[ERROR] Error al invocar validator_cli.jar en lote {b_idx + 1}: {e}", file=sys.stderr, flush=True)
            return False

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
        print('[INFO] El parser histórico de consola no encontró incidencias críticas; esto no acredita conformidad exhaustiva. Conservar informes estructurados por recurso con el runner de evidencia primaria.', flush=True)

    return all_ok and not has_critical


# ---------------------------------------------------------------------------
# Punto de entrada principal
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(
        description=(
            "Exportador exhaustivo del censo completo de 16,250 recursos FHIR R4 para validación "
            "sintáctica oficial (HL7 validator-cli). Tesis UNACH 2026."
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
            "Exportar el censo completo de 16,250 recursos clínicos FHIR R4: "
            "Patient 1-1,000 | Encounter 1-2,500 | Observation (bp, temp, hr, resp) 1-2,500 "
            "[9,750 emitidos + 250 omitidos] | Condition 1-3,000"
        ),
    )
    parser.add_argument(
        "--count",
        type=int,
        default=100,
        help=(
            "Cantidad de muestras a exportar por recurso si no se usa --all (default: 100)"
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
            "Ejecuta automáticamente la validación HL7 offline por lotes: "
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
        mode_label = "CENSO COMPLETO (--all) — 16,250 recursos FHIR R4 censales"
    else:
        n = args.count
        ranges = {
            "Patient":          (1, min(n, 1000)),
            "Encounter":        (1, min(n, 2500)),
            "Observation_bp":   (1, min(n, 2500)),
            "Observation_temp": (1, min(n, 2500)),
            "Observation_hr":   (1, min(n, 2500)),
            "Observation_resp": (1, min(n, 2500)),
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
        print(f"    {rtype:<20} IDs {rstart:>5} – {rend:<5}  ({n_ids:,} solicitudes)", flush=True)
    print("=" * 60, flush=True)

    # -- Exportación concurrente ---------------------------------------------
    counts, omitted_counts, total_bytes, elapsed = export_resources(
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
        + counts["Observation_resp"]
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
    print(f"Total Observation (resp) export.:  {counts['Observation_resp']:>6,}", flush=True)
    if omitted_counts.get("Observation_resp", 0) > 0:
        print(f"  (Resp omitidas por NULL en BD):  {omitted_counts['Observation_resp']:>6,}", flush=True)
    print(f"  -> Subtotal Observation:         {total_obs:>6,}", flush=True)
    print(f"Total Condition exportados:        {counts['Condition']:>6,}", flush=True)
    print("-" * 60, flush=True)
    print(f"TOTAL ARCHIVOS JSON EXPORTADOS:    {total_all_res:>6,}", flush=True)
    print(f"Tamaño promedio por recurso:       {avg_kb:>8.2f} KB", flush=True)
    print("=" * 60, flush=True)

    # -- Comando canónico de validación HL7 ----------------------------------
    exact_validation_cmd = (
        f"java -jar {args.validator_jar} {output_dir}/*.json -version 4.0.1 -tx n/a"
    )
    print("\n" + "=" * 60, flush=True)
    print("COMANDO OFICIAL DE VALIDACIÓN SINTÁCTICA HL7 FHIR (OFFLINE)", flush=True)
    print("=" * 60, flush=True)
    print("Para validar los recursos exportados en modo desconectado estricto por lotes:", flush=True)
    print(f"  {exact_validation_cmd}", flush=True)
    print("=" * 60, flush=True)

    if args.validate:
        validator_ok = run_hl7_validator(args.validator_jar, output_dir)
    else:
        validator_ok = True
    missing = total_requests - total_exported - sum(omitted_counts.values())
    if missing:
        print(f'[FAIL] {missing} solicitudes sin recurso ni omisión registrada.', file=sys.stderr)
    if args.export_all and total_exported != 16250:
        print('[FAIL] Censo completo distinto de 16,250; no es una exportación completa.', file=sys.stderr)
        return 1
    return 0 if not missing and validator_ok else 1


if __name__ == "__main__":
    sys.exit(main())
