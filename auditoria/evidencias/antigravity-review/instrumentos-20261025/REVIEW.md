# Auditoría Técnica de Instrumentos Experimentales (AUD-INSTRUMENTOS-20261025)

**Rol:** `reviewer-auditor`  
**Alcance:** Instrumentos de recolección, oráculo y agregación experimental (`scripts/experimental_evidence.py`, `scripts/oracle_verifier.py`, `scripts/run_all_cap4_experiments.py`, `scripts/test_experimental_evidence.py`, `tools/experimental/evidence_runners.py`, `tools/experimental/aggregate_ct.py`).  
**Estado:** Inspección de código concluida en modo estricto de solo lectura.

---

### Evaluación de Puntos Críticos de Rigor Metodológico

1. **Exportación conserva cuerpo original:**  
   *Verificado.* En [`scripts/experimental_evidence.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L81-L89), `fetch()` almacena la carga útil binaria y sus metadatos (`body_bytes`, `body_sha256`, y `raw_body_base64` ante fallas de codificación UTF-8). En las líneas 227–230, `export()` escribe directamente a disco los bytes reales recibidos del transporte HTTP (`write_bytes()`), evitando la re-serialización JSON que provocaría redondeo de flotantes o supresión de claves duplicadas.

2. **Cero discrepancias requiere censo real:**  
   *Verificado.* En [`scripts/experimental_evidence.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L34-L38) y [`scripts/experimental_evidence.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L300-L304), la comparación contra el modelo oráculo exige `full_census(model)` (1,000 Patient, 2,500 Encounter, 9,750 Observation, 3,000 Condition). Cualquier censo menor con 0 discrepancias produce estado `INCOMPLETE`, nunca `PASS`. Adicionalmente, el consolidador [`scripts/run_all_cap4_experiments.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/run_all_cap4_experiments.py#L40-L42) rechaza con `ValueError` cualquier reporte `source_vs_rust` que no contenga exactamente 16,250 recursos esperados y observados sin discrepancias.

3. **Mutaciones no son corrección del middleware:**  
   *Verificado.* En [`scripts/experimental_evidence.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L313-L316), el reporte se tipifica formalmente como `oracle_sensitivity_only` con la advertencia explícita: *"Mutated independent expectations test oracle sensitivity, not middleware correctness"*. En [`scripts/oracle_verifier.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/oracle_verifier.py#L904) se imprime la advertencia de autoprueba Python y se bloquearon los antiguos sondeos acotados a 100 recursos. En [`scripts/run_all_cap4_experiments.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/run_all_cap4_experiments.py#L17), el criterio CA-2 exige *ambos* reportes por separado (`source_vs_rust` y `oracle_sensitivity_only`).

4. **Corpus conserva errores de inyección y restauración:**  
   *Verificado.* En [`scripts/experimental_evidence.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L383-L386), las excepciones PostgreSQL (`psycopg.Error`) registran la inyección como `status: 'BLOCKED'` y no como rechazo HTTP 422 del servidor. En las líneas 391–400, la restauración atómica en bloque `finally` valida el borrado mediante censo cero y compara controles HTTP nominales antes y después; cualquier fallo en la restauración marca `status: 'FAIL'`, activa la bandera `control_or_restoration_failure` y detiene la prueba arrojando `ValueError` inmediato para evitar la contaminación del laboratorio.

5. **Reportes HL7 se asocian por recurso:**  
   *Verificado.* En [`tools/experimental/evidence_runners.py`](file:///D:/Proyectos%20X/MedSys-FHIR/tools/experimental/evidence_runners.py#L125-L140), cada `OperationOutcome` emitido por el validador oficial es correlacionado individualmente mediante la extensión estándar `http://hl7.org/fhir/StructureDefinition/operationoutcome-file`. Si un outcome no se encuentra en el lote esperado, se duplica o falta, el proceso marca `infra = True` y el estado final se bloquea en `BLOCKED`.

6. **CT no decide viabilidad:**  
   *Verificado.* En [`tools/experimental/aggregate_ct.py`](file:///D:/Proyectos%20X/MedSys-FHIR/tools/experimental/aggregate_ct.py#L154-L159), la salida produce `kind: 'k6_resource_measurement'`. En [`scripts/run_all_cap4_experiments.py`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/run_all_cap4_experiments.py#L101-L105), el criterio CT se marca como `characterization_is_binding: False` y `thesis_verdict: 'Requires original manuscript decision rule; no inferred viability verdict'`.

---

### Hallazgos de Código Demostrables

No se detectaron aprobaciones falsas ni pérdida de evidencia en el flujo principal de pruebas. Se identifican **2 hallazgos acotados** en los instrumentos:

1. **Desacoplamiento del código de salida de fallas en `adapt()`**  
   - **Archivo y Líneas:** [`scripts/experimental_evidence.py:469-476`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L469-L476)  
   - **Mecanismo:** Cuando `adapt()` se ejecuta con `--lab-id` para los esquemas `s0` y `s3`, se invoca `faults()` y se almacena `record['faults_exit_code']`. Sin embargo, la determinación del estado del ciclo en la línea 476 únicamente computa:  
     `status='PASS' if not (exported or compared or mutated) else 'FAIL'`  
     Si `faults()` concluyera con código de salida 1 (fallo o restauración anómala), el estado registrado en `schema-runs.json` y propagado a `ca5-contract.json` seguiría marcándose como `PASS` si exportación, comparación y mutaciones tuvieron éxito. Si bien la auditoría CA-3 formal se valida de forma independiente a través de `fault_report.json` en el consolidador, existe una inconsistencia interna dentro del registro de `adapt`.

2. **Mapeo rígido de columnas primarias en S3 ante eventual extensión del corpus**  
   - **Archivo y Líneas:** [`scripts/experimental_evidence.py:349-351`](file:///D:/Proyectos%20X/MedSys-FHIR/scripts/experimental_evidence.py#L349-L351)  
   - **Mecanismo:** La resolución de llave primaria para borrado e inyección define:  
     `pk = {'Patient': 'id_paciente', 'Encounter': 'id_consulta', 'Observation': 'sig_id' if args.schema == 's3' else 'id_signo', 'Condition': 'id_diagnostico'}[case['resource_type']]`  
     En el esquema real de S3 (`tbl_pacientes_s3`, `tbl_consultas_s3`, `tbl_diagnosticos_s3`), las columnas reales son `pac_id`, `con_id` y `diag_id`. En el corpus actual de 100 casos no se manifiesta el fallo porque los únicos 3 casos asignados a S3 (T2-16, T2-17, T2-18) pertenecen exclusivamente a `Observation` (`tbl_signos_s3`), donde `sig_id` es correcto; no obstante, una extensión de casos de falla para S3 sobre los otros tres recursos provocaría un error SQL por columna inexistente.

