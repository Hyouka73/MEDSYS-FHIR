# Suite de Pruebas de Rendimiento y Concurrencia con k6 — MedSys-FHIR

Este directorio contiene las pruebas de carga, latencia y resiliencia implementadas para validar el desempeño del middleware de interoperabilidad **MedSys-FHIR** bajo condiciones de concurrencia clínica (Capítulo III y IV de la tesis, UNACH 2026).

---

## 🎯 Objetivos de Validación de la Hipótesis

1. **Tasa de Éxito HTTP ($\ge 99.5\%$)**: Demostrar que el middleware procesa transacciones nominales válidas con una tasa de fallo inferior a 0.5% (`http_req_failed < 0.005`).
2. **Latencia Sub-200ms ($p_{95} \le 200\text{ ms}$)**: Comprobar que bajo una meseta sostenida de **50 usuarios virtuales (VUs)** durante 5 minutos, la latencia de respuesta se mantiene estrictamente por debajo de 200 ms.
3. **Aislamiento Metodológico (AUD-003 / AUD-008 / CON-003)**: Segregar rigurosamente la prueba nominal (`load_test.js`) de la prueba de fallos (`resilience_and_errors_test.js`), evitando contaminar las métricas de latencia con códigos 404 o `OperationOutcome`.
4. **Resiliencia Operativa**: Certificar que ante inyección de fallos masivos (rutas inexistentes, IDs alfanuméricos o negativos), el **100%** de las respuestas devuelven el recurso canónico `OperationOutcome` con cabecera `Content-Type: application/fhir+json; charset=utf-8`.
5. **Conformidad Sintáctica HL7 Offline**: Validar las muestras exportadas con `org.hl7.fhir.validator-cli` forzando el modo desconectado mediante `-tx n/a`.

---

## 📁 Estructura de Scripts

| Script | Propósito | Escenario / VUs | Umbrales Clave (Thresholds) |
| :--- | :--- | :--- | :--- |
| [`smoke_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/MEDSYS-FHIR/tests/k6/smoke_test.js) | Verificación rápida de disponibilidad de todos los endpoints canónicos. | 1 VU / 1 iteración | `p(95) < 100ms`, `checks > 90%` |
| [`load_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/MEDSYS-FHIR/tests/k6/load_test.js) | Carga nominal pura sobre registros existentes (Patient: 1-1k, Encounter: 1-2.5k, Observation: 1-2.5k, Condition: 1-3k). | 50 VUs (Warm-up 1.5m, Meseta 5m, Cool-down 30s) | `p(95) <= 200ms`, `éxito >= 99.5%`, `checks >= 99.5%` |
| [`resilience_and_errors_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/MEDSYS-FHIR/tests/k6/resilience_and_errors_test.js) | Inyección continua de peticiones anómalas (IDs negativos, alfanuméricos, rutas 404, fallback universal). | 10 VUs sostenidas / 15s | `OperationOutcome rate == 100%`, `p(95) < 30ms` |
| [`run_all_benchmarks.ps1`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/MEDSYS-FHIR/tests/k6/run_all_benchmarks.ps1) | Ejecutor automatizado en PowerShell que orquesta la ejecución y reporta resultados. | Todas las suites | Reporte integral para tesis |

---

## 🚀 Instrucciones de Ejecución

### Prerrequisitos
- **k6 v2.x / v0.x** instalado (`C:\Program Files\k6\k6.exe` o en el `$PATH`).
- Servidor `medsys-server` ejecutándose localmente:
  ```powershell
  # En una terminal dedicada:
  $env:PATH = "C:\Users\Judirico\.cargo\bin;C:\msys64\mingw64\bin;" + $env:PATH
  cargo run --release -p medsys-server
  ```

### Ejecución Automatizada de Todas las Pruebas
```powershell
powershell -ExecutionPolicy Bypass -File tests/k6/run_all_benchmarks.ps1
```

### Ejecución Manual Individual
```powershell
# 1. Smoke Test
& "C:\Program Files\k6\k6.exe" run tests/k6/smoke_test.js

# 2. Load Test Nominal (Meseta sostenida de 5 minutos a 50 VUs)
& "C:\Program Files\k6\k6.exe" run --env BASE_URL=http://localhost:3000 tests/k6/load_test.js

# 3. Resilience and Errors Test
& "C:\Program Files\k6\k6.exe" run tests/k6/resilience_and_errors_test.js
```

---

## 📋 Validación Sintáctica Oficial HL7 FHIR (Modo Offline)

Para validar la conformidad de los recursos FHIR generados contra las especificaciones canónicas de HL7 FHIR R4 de manera completamente desconectada (reproducible sin acceso a `tx.fhir.org`):

```powershell
# 1. Exportar muestras JSON desde el servidor
python scripts/export_fhir_samples.py --base-url http://localhost:3000 --count 50 --output-dir output

# 2. Ejecutar validación oficial desconectada
java -jar validator_cli.jar output/*.json -version 4.0.1 -tx n/a
```
