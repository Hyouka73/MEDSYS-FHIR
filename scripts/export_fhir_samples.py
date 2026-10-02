#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Exportador de Muestras de Recursos FHIR R4 para Validación HL7
# Consulta los endpoints canónicos de medsys-server y persiste cada recurso
# como archivo JSON individual en el directorio de salida (output/).
# Diseñado para validación por lotes con org.hl7.fhir.validator-cli.
# ==============================================================================

import argparse
import json
import os
import sys
import urllib.error
import urllib.request


def fetch_resource(url: str, output_path: str) -> bool:
    """Realiza una petición GET al endpoint FHIR y persiste el JSON formateado.
    
    Retorna True si el recurso fue recuperado y guardado exitosamente (HTTP 200),
    o False en caso de error o código no exitoso (e.g., 404).
    """
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/fhir+json",
            "User-Agent": "MedSys-FHIR-Exporter/1.0",
        },
    )

    try:
        with urllib.request.urlopen(req, timeout=10) as response:
            if response.status == 200:
                raw_bytes = response.read()
                data = json.loads(raw_bytes.decode("utf-8"))
                with open(output_path, "w", encoding="utf-8") as f:
                    json.dump(data, f, indent=2, ensure_ascii=False)
                    f.write("\n")
                return True
            else:
                print(f"[WARN] HTTP {response.status} recibido para {url}")
                return False
    except urllib.error.HTTPError as e:
        if e.code == 404:
            print(f"[WARN] 404 No encontrado: {url}")
        else:
            print(f"[WARN] HTTP {e.code} ({e.reason}) para {url}")
        return False
    except urllib.error.URLError as e:
        print(f"[ERROR] Error de conexión al consultar {url}: {e.reason}", file=sys.stderr)
        return False
    except Exception as e:
        print(f"[ERROR] Error inesperado al procesar {url}: {e}", file=sys.stderr)
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Exportador de muestras de recursos FHIR R4 para validación sintáctica oficial (HL7 validator-cli)"
    )
    parser.add_argument(
        "--base-url",
        type=str,
        default="http://localhost:3000",
        help="URL base del servidor MedSys-FHIR en ejecución (default: http://localhost:3000)",
    )
    parser.add_argument(
        "--count",
        type=int,
        default=50,
        help="Número de registros base a exportar por cada tipo de recurso (default: 50)",
    )
    parser.add_argument(
        "--output-dir",
        type=str,
        default="output",
        help="Directorio de destino para los archivos JSON exportados (default: output)",
    )

    args = parser.parse_args()
    base_url = args.base_url.rstrip("/")
    output_dir = args.output_dir
    count = args.count

    os.makedirs(output_dir, exist_ok=True)

    print("[*] Iniciando exportación de muestras FHIR R4:", flush=True)
    print(f"    - URL Base: {base_url}", flush=True)
    print(f"    - Cantidad base: {count}", flush=True)
    print(f"    - Directorio de salida: {output_dir}", flush=True)
    print("-" * 60, flush=True)

    counts = {
        "Patient": 0,
        "Encounter": 0,
        "Observation_bp": 0,
        "Observation_temp": 0,
        "Observation_hr": 0,
        "Condition": 0,
    }

    for i in range(1, count + 1):
        # 1. Patient
        url_patient = f"{base_url}/fhir/r4/Patient/{i}"
        file_patient = os.path.join(output_dir, f"Patient_{i}.json")
        if fetch_resource(url_patient, file_patient):
            counts["Patient"] += 1

        # 2. Encounter
        url_encounter = f"{base_url}/fhir/r4/Encounter/{i}"
        file_encounter = os.path.join(output_dir, f"Encounter_{i}.json")
        if fetch_resource(url_encounter, file_encounter):
            counts["Encounter"] += 1

        # 3. Observation - Panel de Presión Arterial (LOINC 85354-9)
        url_obs_bp = f"{base_url}/fhir/r4/Observation/bp-{i}"
        file_obs_bp = os.path.join(output_dir, f"Observation_bp_{i}.json")
        if fetch_resource(url_obs_bp, file_obs_bp):
            counts["Observation_bp"] += 1

        # 4. Observation - Temperatura Corporal (LOINC 8310-5)
        url_obs_temp = f"{base_url}/fhir/r4/Observation/temp-{i}"
        file_obs_temp = os.path.join(output_dir, f"Observation_temp_{i}.json")
        if fetch_resource(url_obs_temp, file_obs_temp):
            counts["Observation_temp"] += 1

        # 5. Observation - Frecuencia Cardíaca (LOINC 8867-4)
        url_obs_hr = f"{base_url}/fhir/r4/Observation/hr-{i}"
        file_obs_hr = os.path.join(output_dir, f"Observation_hr_{i}.json")
        if fetch_resource(url_obs_hr, file_obs_hr):
            counts["Observation_hr"] += 1

        # 6. Condition - Diagnóstico CIE-10
        url_condition = f"{base_url}/fhir/r4/Condition/{i}"
        file_condition = os.path.join(output_dir, f"Condition_{i}.json")
        if fetch_resource(url_condition, file_condition):
            counts["Condition"] += 1

        # Progreso periódico cada 10 registros (o al completar)
        if i % 10 == 0 or i == count:
            print(f"[*] Progreso: {i}/{count} registros procesados...", flush=True)

    total_obs = counts["Observation_bp"] + counts["Observation_temp"] + counts["Observation_hr"]
    total_all = counts["Patient"] + counts["Encounter"] + total_obs + counts["Condition"]

    print("\n" + "=" * 60, flush=True)
    print("RESUMEN DE EXPORTACIÓN FHIR R4", flush=True)
    print("=" * 60, flush=True)
    print(f"Directorio de salida: {output_dir}", flush=True)
    print(f"Total Patient exportados:          {counts['Patient']}", flush=True)
    print(f"Total Encounter exportados:        {counts['Encounter']}", flush=True)
    print(f"Total Observation (bp) exportados: {counts['Observation_bp']}", flush=True)
    print(f"Total Observation (temp) export.:  {counts['Observation_temp']}", flush=True)
    print(f"Total Observation (hr) export.:    {counts['Observation_hr']}", flush=True)
    print(f"  -> Subtotal Observation:         {total_obs}", flush=True)
    print(f"Total Condition exportados:        {counts['Condition']}", flush=True)
    print("-" * 60, flush=True)
    print(f"TOTAL RECURSOS EXPORTADOS:         {total_all}", flush=True)
    print("=" * 60, flush=True)


if __name__ == "__main__":
    main()
