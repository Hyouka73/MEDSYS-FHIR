# Suite de Pruebas de Rendimiento y Concurrencia con k6 — MedSys-FHIR

Este directorio contiene las pruebas de carga, latencia y resiliencia implementadas para validar el desempeño del middleware de interoperabilidad **MedSys-FHIR** bajo condiciones de concurrencia clínica (Sección 4 de la tesis, UNACH 2026).

---

## 🎯 Objetivos de Validación de la Tesis

1. **Latencia Sub-50ms**: Demostrar que la traducción semántica desacoplada (relacional legado NOM-004 ➔ FHIR R4) responde con una latencia de percentil 95 ($p_{95}$) inferior a **50 ms** en lecturas individuales.
2. **Capacidad de Concurrencia**: Validar que el servidor basado en Axum y Tokio procesa solicitudes concurrentes sin degradación de memoria, pérdidas de conexión ni saturación del pool asíncrono.
3. **Resiliencia Operativa**: Certificar que bajo inyección de fallos masivos (rutas inexistentes, IDs alfanuméricos inválidos), el **100%** de las respuestas devuelven el recurso canónico `OperationOutcome` con cabecera `Content-Type: application/fhir+json; charset=utf-8` en tiempos sub-milisegundo.

---

## 📁 Estructura de Scripts

| Script | Propósito | Escenario / VUs | Umbrales Clave (Thresholds) |
| :--- | :--- | :--- | :--- |
| [`smoke_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/tests/k6/smoke_test.js) | Verificación rápida de disponibilidad de todos los endpoints canónicos. | 1 VU / 1 iteración | `p(95) < 100ms`, `checks > 90%` |
| [`load_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/tests/k6/load_test.js) | Simulación de carga clínica concurrente (Patient, Encounter, Observation BP/Temp, Bundles). | Rampa escalonada hasta 20 VUs | `p(95) < 50ms`, `p(99) < 100ms`, `éxito > 95%` |
| [`resilience_and_errors_test.js`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/tests/k6/resilience_and_errors_test.js) | Inyección continua de peticiones anómalas (400, 404, fallback universal). | 10 VUs sostenidas / 15s | `OperationOutcome rate == 100%`, `p(95) < 30ms` |
| [`run_all_benchmarks.ps1`](file:///c:/Users/Judirico/Documents/MedSys-FHIR/tests/k6/run_all_benchmarks.ps1) | Ejecutor automatizado en PowerShell que orquesta la ejecución y reporta resultados. | Todas las suites | Reporte integral para tesis |

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
# Smoke Test
& "C:\Program Files\k6\k6.exe" run tests/k6/smoke_test.js

# Load Test con URL personalizada
& "C:\Program Files\k6\k6.exe" run --env BASE_URL=http://localhost:3000 tests/k6/load_test.js

# Resilience Test
& "C:\Program Files\k6\k6.exe" run tests/k6/resilience_and_errors_test.js
```
