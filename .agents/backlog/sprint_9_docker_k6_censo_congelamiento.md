# SPRINT 9: Contenedores con Límites Estrictos, Exportación Censal (16,250 JSON), Benchmark `k6` (50 VUs) y Congelamiento `v1.2.0-eval`

## Objetivo del Sprint
Cerrar todos los hallazgos de infraestructura, exportación censal y scripts de benchmarking (`AUD-R1-B-002`, `AUD-R1-B-006`, `AUD-R1-I-001` y errores de sintaxis en scripts PowerShell reportados en `RESULTADOS_EJECUCION_R1.md`), dejando listo el pipeline experimental para el Capítulo IV y etiquetando la versión `v1.2.0-eval`.

---

## Lista de Tareas Atómicas

- [ ] **Tarea 9.1: Homologación de Límites `cgroups v2` y Usuario `medsys_ro` en `docker-compose.yml` y `docker/docker-compose.yml`**
  - **Archivos a modificar:**
    - `docker-compose.yml`
    - `docker/docker-compose.yml`
    - `crates/medsys-db/src/config.rs`
    - `crates/medsys-server/src/config.rs`
  - **Instrucciones exactas:**
    1. En ambos archivos `docker-compose.yml` (raíz y `docker/docker-compose.yml`):
       - Asegurar que el servicio `medsys-server` defina explícitamente:
         ```yaml
         deploy:
           resources:
             limits:
               cpus: '1.0'
               memory: 256M
         mem_limit: 256m
         cpus: 1.0
         ```
       - Asegurar que el servicio `postgres` (`postgres:16-alpine`) defina explícitamente los límites del Capítulo III (§3.2.4 Tabla 4):
         ```yaml
         deploy:
           resources:
             limits:
               cpus: '1.0'
               memory: 512M
         mem_limit: 512m
         cpus: 1.0
         ```
       - Configurar `DATABASE_URL` de `medsys-server` usando el rol de solo lectura `medsys_ro` (`postgres://medsys_ro:medsys_ro_pass_2026@postgres:5432/medsys_legado`).
       - Permitir seleccionar el esquema y archivo YAML mediante variables de entorno (`MAPPING_RULES_PATH: ${MAPPING_RULES_PATH:-/app/mapping_rules.yaml}`).
  - **Criterio de Aceptación:** Ambos archivos `docker-compose.yml` declaran los límites exactos de CPU y RAM para `medsys-server` (1 vCPU, 256 MB) y `postgres` (1 vCPU, 512 MB) y conectan mediante `medsys_ro`.

- [ ] **Tarea 9.2: Actualización de `scripts/export_fhir_samples.py` y `scripts/validate_fhir.ps1` para el Censo de 16,250 Recursos**
  - **Archivos a modificar:**
    - `scripts/export_fhir_samples.py`
    - `scripts/validate_fhir.ps1`
  - **Instrucciones exactas:**
    1. En `scripts/export_fhir_samples.py`:
       - Añadir `"Observation_resp": (1, 2500)` en `UNIVERSE_RANGES` y soportar `"Observation_resp"` en `_build_url` (`f"{base_url}/fhir/r4/Observation/resp-{resource_id}"`).
       - Contabilizar las respuestas HTTP 404 esperadas en `Observation_resp` (las 250 filas donde `frecuencia_respiratoria IS NULL`) como omisiones válidas (`optional_omitted_count`), verificando que con `--all` se exporten exactamente:
         - `1,000` archivos `Patient`
         - `2,500` archivos `Encounter`
         - `2,500` archivos `Observation_bp`
         - `2,500` archivos `Observation_temp`
         - `2,500` archivos `Observation_hr`
         - `2,250` archivos `Observation_resp` (y `250` omitidos por `NULL`)
         - `3,000` archivos `Condition`
         - **Total exacto: `16,250` archivos JSON FHIR R4.**
    2. En `scripts/validate_fhir.ps1` y `scripts/export_fhir_samples.py`:
       - Invocar `validator_cli.jar` en lotes para evitar exceder la longitud máxima de línea de comandos en Windows al validar los 16,250 archivos JSON (`-version 4.0.1 -tx n/a`).
  - **Criterio de Aceptación:** `python3 scripts/export_fhir_samples.py --help` funciona sin errores y documenta el universo censal de 16,250 recursos.

- [ ] **Tarea 9.3: Reparación de Scripts PowerShell/Bash de Benchmarking y Telemetría Conjunta (`k6` + `docker stats` + `VmRSS`)**
  - **Archivos a modificar:**
    - `tests/k6/load_test.js`
    - `tests/k6/smoke_test.js`
    - `tests/k6/run_benchmark_with_stats.ps1`
    - `tests/k6/run_benchmark_with_stats.sh`
    - `tests/k6/run_all_benchmarks.ps1`
  - **Instrucciones exactas:**
    1. En `tests/k6/load_test.js` y `tests/k6/smoke_test.js`:
       - Incluir el 4º subtipo de `Observation` (`resp`) en las pruebas, aceptando en `load_test.js` que para `resp-{id}` tanto HTTP 200 (cuando `frecuencia_respiratoria` está presente) como HTTP 404 con `OperationOutcome` válido (en el 10 % de filas donde `frecuencia_respiratoria IS NULL`) son respuestas nominales correctas si y solo si devuelven `Content-Type: application/fhir+json`.
    2. En `tests/k6/run_benchmark_with_stats.ps1` y `tests/k6/run_all_benchmarks.ps1`:
       - Corregir los caracteres de tubería `|` y comillas sin escapar reportados en `RESULTADOS_EJECUCION_R1.md` (sección 9, puntos 2 y 3) para que se ejecuten nativamente en PowerShell sin errores de sintaxis.
       - Agregar en el recolector de telemetría (tanto en `.ps1` como en `.sh`) la captura simultánea cada 5 segundos de:
         - `docker stats medsys-server` (CPU % y MemUsage)
         - `docker stats medsys_fhir_postgres` (CPU % y MemUsage)
         - `VmRSS` real del proceso Rust (`/proc/1/status`) y `memory.current` del cgroup v2.
       - Soportar el parámetro `-Replicas 10` (por defecto 1 corrida rápida o 10 réplicas oficiales para calcular media, desviación estándar e intervalo de confianza al 95 %).
  - **Criterio de Aceptación:** `bash -n tests/k6/run_benchmark_with_stats.sh` pasa con 0 errores de sintaxis y los scripts PowerShell tienen todas sus cadenas limpias y balanceadas.

- [ ] **Tarea 9.4: Auditoría Final de Calidad, Actualización de Versiones a `1.2.0` y Cierre del Backlog**
  - **Archivos a modificar:**
    - `Cargo.toml`, `crates/medsys-core/Cargo.toml`, `crates/medsys-db/Cargo.toml`, `crates/medsys-server/Cargo.toml` (actualizar `version = "1.2.0"`)
    - `crates/medsys-server/tests/e2e_interoperability.rs` (actualizar aserción de `server_version` a `"1.2.0"`)
    - `STATE.md`, `BACKLOG.md`, `.agents/backlog/overview.md`
  - **Instrucciones exactas:**
    1. Actualizar la versión de los 3 crates a `"1.2.0"`.
    2. Ejecutar la batería completa de cierre:
       - `cargo fmt --all -- --check`
       - `cargo clippy --workspace --all-targets -- -D warnings`
       - `cargo test --workspace`
       - `python3 scripts/generate_data.py --seed 20260930`
       - `python3 scripts/oracle_verifier.py --self-test`
       - `python3 scripts/oracle_verifier.py --run-mutations`
       - `python3 scripts/run_fault_corpus.py`
    3. Actualizar `STATE.md` y `BACKLOG.md` reflejando `36 / 36 tareas completadas (100.0%)` en versión `1.2.0-eval`.
  - **Criterio de Aceptación:** 100 % de las pruebas de Rust y Python pasan con 0 errores y 0 warnings.
