# INFORME CONSOLIDADO DE EJECUCIÓN — FASE 0 (RAMA `fase0-sync`)
**Proyecto MedSys-FHIR — Mediación Relacional a HL7 FHIR R4**  
**Fecha de corte:** 2026-10-02  
**Rama:** `fase0-sync`  
**Estado:** COMPLETA / SIN MERGE  

---

## 1. Resumen Ejecutivo Consolidado

La **Fase 0 de Sincronización (`fase0-sync`)** tuvo como objetivo armonizar los artefactos normativos, esquemas relacionales DDL y especificaciones de mapeo declarativo previo al inicio de las pruebas de carga y validación censal, garantizando la observancia estricta del principio rector de fidelidad clínica:

> *"El middleware transforma; no corrige ni completa. Nunca sustituye un dato ausente por un valor inventado."* (Decisiones D-006 y D-029).

En esta fase se ejecutaron dos intervenciones críticas:
1. **Tarea 1 — Eliminación del DEFAULT 'CONFIRMADO' en `tbl_diagnosticos.tipo_diagnostico`:** Corrección a nivel DDL para permitir la nulidad natural de la certeza diagnóstica y habilitar la ruta de degradación por omisión en el recurso FHIR `Condition` (omisión del atributo `verificationStatus`).
2. **Tarea 2 — Inclusión de la Regla de Mapeo de Frecuencia Respiratoria en YAML:** Incorporación formal del mapeo declarativo para Frecuencia Respiratoria (`Observation` categoría `vital-signs`, código LOINC `9279-1`, unidad UCUM `/min`) conforme a la decisión D-016, evitando la pérdida de información clínica almacenada en `tbl_signos_vitales.frecuencia_respiratoria`.

---

## PARTE I: ELIMINACIÓN DE DEFAULT 'CONFIRMADO' EN `tipo_diagnostico`

### 1.1 Contexto y Justificación Técnica
Para la generación del recurso HL7 FHIR R4 `Condition`, la ausencia de certeza diagnóstica (`NULL` en `tbl_diagnosticos.tipo_diagnostico`) debe resolverse mediante degradación por omisión (omitiendo el elemento `verificationStatus` del payload JSON resultante), mientras que un valor desconocido o fuera de catálogo debe ser rechazado bajo una política estricta de fallo seguro (Fail-Closed, emitiendo un `OperationOutcome` con código HTTP 422).

La presencia de la cláusula `DEFAULT 'CONFIRMADO'` en la definición DDL de la columna violaba este principio al forzar que la propia base de datos afirmara certeza médica cuando el dato no había sido capturado por el profesional de la salud, impidiendo ejercitar la ruta de omisión ante nulos.

### 1.2 Acciones Realizadas en Git
- **Rama:** Creación y activación de `fase0-sync` (`git checkout -b fase0-sync`).
- **Modificación Aplicada:** Se localizó la definición de la columna en `schema_legado_simulado_nom004.sql`:
  - **Antes:**
    ```sql
    tipo_diagnostico VARCHAR(20) DEFAULT 'CONFIRMADO' CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
    ```
  - **Después:**
    ```sql
    tipo_diagnostico VARCHAR(20) CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
    ```
- **Commit Registrado:**
  ```bash
  git commit -m "fase0: eliminar DEFAULT 'CONFIRMADO' de tipo_diagnostico (principio de no alteración de datos)"
  # Hash: 24d99eb
  ```

### 1.3 Revisión de Archivos e Inserciones (`INSERT`)
1. **`schema_legado_simulado_nom004.sql`:**
   - Sentencias de inserción de prueba:
     ```sql
     INSERT INTO tbl_diagnosticos (id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico)
     VALUES 
     (1, 1, 'I10', 'Hipertensión esencial (primaria)', 'CONFIRMADO', '2026-09-18'),
     (2, 2, 'G43.9', 'Migraña, no especificada', 'CONFIRMADO', '2026-09-18');
     ```
   - **Hallazgo:** Todas las filas especifican explícitamente la columna `tipo_diagnostico` con el valor literal `'CONFIRMADO'`.
   - **Acción:** No se requirió modificar ningún `INSERT`, pues ninguno dependía del valor por defecto. La integridad y el contenido del dataset de prueba permanecen 100% idénticos.
2. **`scripts/generate_data.py`:**
   - **Estado:** EXISTE en el repositorio (`scripts/generate_data.py`).
   - **Hallazgo:** El generador sintético puebla los 3,000 registros de diagnósticos asignando explícitamente todas las columnas en la sentencia (`id_consulta`, `id_paciente`, `codigo_cie10`, `descripcion_diagnostico`, `tipo_diagnostico`, `fecha_diagnostico`), alternando de manera determinista los valores `'CONFIRMADO'` y `'PRESUNTIVO'`.
   - **Acción:** Ninguna inserción en el script omitía la columna, por lo que no se modificó `scripts/generate_data.py`.

### 1.4 `git diff` del Cambio DDL
```diff
diff --git a/schema_legado_simulado_nom004.sql b/schema_legado_simulado_nom004.sql
index 32fff05..ca6a01f 100644
--- a/schema_legado_simulado_nom004.sql
+++ b/schema_legado_simulado_nom004.sql
@@ -63,7 +63,7 @@ CREATE TABLE IF NOT EXISTS tbl_diagnosticos (
     id_paciente INT REFERENCES tbl_pacientes(id_paciente) ON DELETE RESTRICT,
     codigo_cie10 VARCHAR(10) NOT NULL,
     descripcion_diagnostico VARCHAR(255) NOT NULL,
-    tipo_diagnostico VARCHAR(20) DEFAULT 'CONFIRMADO' CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
+    tipo_diagnostico VARCHAR(20) CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
     fecha_diagnostico DATE NOT NULL
 );
```

### 1.5 Validación y Análisis Semántico SQL (PostgreSQL 16)
- **Modo de Validación:** Análisis estático y formal bajo la semántica relacional del estándar SQL / PostgreSQL 16.
- **Carga de DDL sin errores:** La cláusula `tipo_diagnostico VARCHAR(20) CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO'))` es sintácticamente válida en PostgreSQL 16.
- **Comportamiento ante omisión (admisión de NULL):** La columna carece de restricción `NOT NULL` y de cláusula `DEFAULT`. Una inserción que omite la columna asigna `NULL`. En el estándar SQL y en PostgreSQL, la evaluación de una restricción `CHECK` sobre un valor `NULL` (`NULL IN ('PRESUNTIVO', 'CONFIRMADO')`) resulta en lógica trivaluada `UNKNOWN` (`NULL`). Según la documentación oficial de PostgreSQL:
  > *"A check constraint is satisfied if the check expression evaluates to true or the null value. Since most expressions evaluate to the null value if any operand is null, they will not prevent null values in the constrained columns."*
  Por tanto, el registro se inserta exitosamente con `NULL`, permitiendo ejercitar la lógica de omisión de `verificationStatus` en el middleware.
- **Rechazo ante valores no permitidos (`CHECK constraint`):** Una inserción con un valor fuera del dominio evalúa a `FALSE`, provocando la interrupción transaccional inmediata con error de violación de `CHECK constraint`.
- **Validación con Pruebas Automatizadas:**
  - `cargo test -p medsys-db`: **22/22 pruebas PASADAS**, incluyendo `test_schema_sql_contract_integrity` (que compila e inspecciona `schema_legado_simulado_nom004.sql`) y las pruebas de entidad para `LegacyDiagnostico` / `DiagnosticoEntity`.
  - `cargo test -p medsys-server`: **13/13 pruebas PASADAS** (8 e2e de interoperabilidad y 5 de integración HTTP/Axum).

---

## PARTE II: MAPEO DE FRECUENCIA RESPIRATORIA EN `mapping_rules.yaml`

### 2.1 Contexto y Decisión D-016
La tabla `tbl_signos_vitales` contiene la columna `frecuencia_respiratoria INT NULL`, pero la especificación declarativa `mapping_rules.yaml` (v1.1.0) únicamente contemplaba mapeos para Presión Arterial (`blood_pressure_panel`) y Temperatura Corporal (`body_temperature`).

Conforme a la decisión de auditoría **D-016**:
- Omitir este dato constituía una pérdida deliberada de información clínica persistida.
- Se acordó modelarla como una `Observation` independiente (categoría `vital-signs`, código LOINC `9279-1`, display `"Respiratory rate"`, valor `valueQuantity` en UCUM `"/min"`).
- Se exigió la inclusión de un comentario de verificación: `# VERIFICAR código en loinc.org`.
- Mantener la versión declarada `1.1.0` en el encabezado sin alterar (reportada como pendiente).

### 2.2 Modificación Aplicada en `mapping_rules.yaml`
Se insertó la regla escalar para Frecuencia Respiratoria inmediatamente después del recurso 3B (Temperatura Corporal) y antes del recurso 4 (`Condition`):

```yaml
  # ----------------------------------------------------------------------------
  # RECURSO 3C: Observation - Frecuencia Respiratoria (LOINC 9279-1)
  # ----------------------------------------------------------------------------
  - resource_type: "Observation"
    profile: "http://hl7.org/fhir/StructureDefinition/resprate"
    source_table: "tbl_signos_vitales"
    primary_key: "id_signo"
    observation_type: "respiratory_rate"
    mappings:
      - constant_value: "final"
        target_path: "status"
      - constant_value: "vital-signs"
        target_path: "category[0].coding[0].code"
        system: "http://terminology.hl7.org/CodeSystem/observation-category"
      - constant_value: "9279-1"
        target_path: "code.coding[0].code"
        display: "Respiratory rate"
        system: "http://loinc.org"
        # VERIFICAR código en loinc.org
      - source_column: "id_paciente"
        target_path: "subject.reference"
        transform: "reference:Patient/{value}"
      - source_column: "id_consulta"
        target_path: "encounter.reference"
        transform: "reference:Encounter/{value}"
      - source_column: "fecha_registro"
        target_path: "effectiveDateTime"
        transform: "datetime_iso8601"
      - source_column: "frecuencia_respiratoria"
        target_path: "valueQuantity.value"
        unit: "/min"
        code: "/min"
        system: "http://unitsofmeasure.org"
```

- **Commit Registrado:**
  ```bash
  git commit -m "fase0: agregar mapeo de frecuencia respiratoria (Observation vital-signs, LOINC 9279-1)"
  # Hash: 22582bc
  ```

### 2.3 `git diff` de la Modificación
```diff
diff --git a/mapping_rules.yaml b/mapping_rules.yaml
index 12b11be..53e49a3 100644
--- a/mapping_rules.yaml
+++ b/mapping_rules.yaml
@@ -159,6 +159,40 @@ resources:
         code: "Cel"
         system: "http://unitsofmeasure.org"
 
+  # ----------------------------------------------------------------------------
+  # RECURSO 3C: Observation - Frecuencia Respiratoria (LOINC 9279-1)
+  # ----------------------------------------------------------------------------
+  - resource_type: "Observation"
+    profile: "http://hl7.org/fhir/StructureDefinition/resprate"
+    source_table: "tbl_signos_vitales"
+    primary_key: "id_signo"
+    observation_type: "respiratory_rate"
+    mappings:
+      - constant_value: "final"
+        target_path: "status"
+      - constant_value: "vital-signs"
+        target_path: "category[0].coding[0].code"
+        system: "http://terminology.hl7.org/CodeSystem/observation-category"
+      - constant_value: "9279-1"
+        target_path: "code.coding[0].code"
+        display: "Respiratory rate"
+        system: "http://loinc.org"
+        # VERIFICAR código en loinc.org
+      - source_column: "id_paciente"
+        target_path: "subject.reference"
+        transform: "reference:Patient/{value}"
+      - source_column: "id_consulta"
+        target_path: "encounter.reference"
+        transform: "reference:Encounter/{value}"
+      - source_column: "fecha_registro"
+        target_path: "effectiveDateTime"
+        transform: "datetime_iso8601"
+      - source_column: "frecuencia_respiratoria"
+        target_path: "valueQuantity.value"
+        unit: "/min"
+        code: "/min"
+        system: "http://unitsofmeasure.org"
+
   # ----------------------------------------------------------------------------
   # RECURSO 4: Condition (Diagnósticos clínicos codificados bajo CIE-10)
   # ----------------------------------------------------------------------------
```

### 2.4 Validación de Pruebas y Diagnóstico de Compatibilidad en Rust
- **Línea Base Previa (`main`):** 59 pruebas pasadas (24 en `medsys-core`, 19 en `medsys-db`, 3 en `persistence_integration`, 8 en `e2e_interoperability`, 5 en `server_integration`).
- **Post-Modificación (`fase0-sync`):**
  - Deserialización y validación estructural del YAML: **100% OK** (sin errores de sintaxis).
  - Suites `medsys-db`, `persistence_integration`, `e2e_interoperability` y `server_integration`: **35/35 pruebas PASADAS (100% OK)**.
  - Suite `medsys-core`: **22 pruebas pasadas**, **2 pruebas fallidas** exclusivamente por aserciones estáticas de conteo previo:
    1. `test_parse_embedded_specification_yaml`: Falló en `assert_eq!(rules.resources.len(), 5)` al recibir `6` (ahora hay 6 recursos en el YAML).
    2. `test_observation_resource_mappings`: Falló en `assert_eq!(observations.len(), 2)` al recibir `3` (ahora hay 3 reglas de `Observation`).
- **Capacidad de Interpretación del Código Rust Actual (Solo Lectura):**
  - **Motor declarativo genérico (`evaluator.rs`):** **SÍ la interpreta.** La función `evaluate_resource_mapping` interpreta dinámicamente todos los atributos (`source_column`, `constant_value`, `target_path`, `system`, `display`, `unit`, `code`).
  - **Transformadores estáticos tipados (`transform.rs`):** **NO la interpretan.** No existe la función `transform_observation_respiratory_rate`.
  - **Manejadores HTTP Axum (`observation.rs` y `legacy.rs`):** **NO la interpretan.** El endpoint `GET /fhir/r4/Observation/{id}` no posee prefijo para frecuencia respiratoria (`resp-` o `rr-`), y `GET /fhir/r4/Observation` (`list_observations`) solo empaqueta `bp`, `temp` y `hr`.
  - *En apego a las restricciones, no se modificó ningún archivo de código Rust, reportándose los cambios necesarios como pendientes.*

---

## PARTE III: ESTADO INTEGRAL CONSOLIDADO DE LA RAMA `fase0-sync`

### 3.1 Historial de Commits en `fase0-sync`
```text
* 24d99eb (HEAD -> fase0-sync) fase0: eliminar DEFAULT 'CONFIRMADO' de tipo_diagnostico (principio de no alteración de datos)
* e507782 docs(auditoria): consolidar reporte de ejecucion de fase0-sync
* 22582bc fase0: agregar mapeo de frecuencia respiratoria (Observation vital-signs, LOINC 9279-1)
* 31ade2f docs: create audit documentation files for decisions, sources, and registry
* 38efb31 fix(AUD-007): ampliar export_fhir_samples.py con --all y concurrencia controlada
* 18bbca1 feat(docker,k6): orquestación docker y pipeline de benchmarking reproducible (PROMPT-COD-02)
```

### 3.2 Matriz de Modificaciones de Archivos
| Archivo | Tipo de Cambio | Líneas Afectadas | Propósito Técnico |
| :--- | :---: | :---: | :--- |
| `schema_legado_simulado_nom004.sql` | DDL / SQL | -1 / +1 | Eliminación de `DEFAULT 'CONFIRMADO'` en `tbl_diagnosticos.tipo_diagnostico`. |
| `mapping_rules.yaml` | YAML Declarativo | +34 | Adición de regla Observation 3C (`respiratory_rate`, LOINC `9279-1`, UCUM `/min`). |
| `auditoria/REPORTE_EJECUCION_FASE0.md` | Documentación / Auditoría | Nuevo | Consolidación del informe técnico integral de la Fase 0. |

### 3.3 Inventario de Reglas y Recursos Clínicos
| Recurso / Regla FHIR R4 | Tabla Origen | Subtipo / Identificador | Código Terminológico | Unidad UCUM |
| :--- | :--- | :--- | :--- | :---: |
| **Patient** | `tbl_pacientes` | — | CURP (`2.16.840.1.113883.4.629`) | — |
| **Encounter** | `tbl_consultas` | — | `AMB` (`v3-ActCode`) | — |
| **Observation (3A)** | `tbl_signos_vitales` | `blood_pressure_panel` | LOINC `85354-9` (comp: `8480-6`, `8462-4`) | `mmHg` |
| **Observation (3B)** | `tbl_signos_vitales` | `body_temperature` | LOINC `8310-5` | `Cel` |
| **Observation (3C)** *(NUEVA)* | `tbl_signos_vitales` | `respiratory_rate` | LOINC `9279-1` *(VERIFICAR)* | `/min` |
| **Condition** | `tbl_diagnosticos` | — | CIE-10 (`sid/icd-10`) | — |

- **Total de recursos en YAML:** Incrementado de 5 a 6.
- **Total de reglas de `Observation` ($N$):** Incrementado de $N=2$ a $N+1=3$.

### 3.4 Matriz de Verificación de Criterios de Aceptación
| Criterio de Aceptación | Tarea Asociada | Estado | Evidencia / Observaciones |
| :--- | :---: | :---: | :--- |
| Ausencia de `DEFAULT 'CONFIRMADO'` | Tarea 1 (DDL) | **CUMPLIDO** | `grep` confirmó 0 ocurrencias en esquemas DDL, código y scripts (`schema_legado_simulado_nom004.sql` actualizado en commit `24d99eb`). Únicas menciones corresponden a citas históricas en documentación de auditoría (`DECISIONES.md` y este informe). |
| Admisión de `NULL` con `CHECK` intacto | Tarea 1 (DDL) | **CUMPLIDO** | `tipo_diagnostico VARCHAR(20) CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO'))`. |
| Integridad de sentencias `INSERT` | Tarea 1 (DDL) | **CUMPLIDO** | Todos los INSERTs en DDL y scripts especifican explícitamente sus valores. |
| YAML parseable sin errores | Tarea 2 (YAML) | **CUMPLIDO** | `serde_yaml` y `parse_mapping_rules` validan la sintaxis y estructura sin fallos. |
| Replicación exacta del patrón escalar | Tarea 2 (YAML) | **CUMPLIDO** | Estructura clonada de Temperatura (3B) con campos LOINC `9279-1` y UCUM `/min`. |
| Conteo de Observation $N \to N+1$ | Tarea 2 (YAML) | **CUMPLIDO** | De 2 a 3 reglas Observation; total de recursos pasa de 5 a 6. |
| Restricción de no modificar código Rust | Tarea 2 (YAML) | **CUMPLIDO** | Ningún archivo `.rs` fue alterado; cambios requeridos fueron diagnosticados y documentados. |
| Unicidad y limpieza del diff en YAML | Tarea 2 (YAML) | **CUMPLIDO** | El diff de `mapping_rules.yaml` contiene estrictamente las 34 líneas de la nueva regla. |

---

## 4. Hoja de Ruta Técnica para Siguientes Fases (Pendientes Reportados)

Para la siguiente iteración de desarrollo (Fase 1 / Sprints de Middleware):
1. **Actualización de Pruebas Unitarias en Rust (`crates/medsys-core/src/engine/mod.rs`):**
   - Ajustar aserción línea 97: `assert_eq!(rules.resources.len(), 6);`
   - Ajustar aserción línea 171: `assert_eq!(observations.len(), 3);`
   - Incorporar pruebas unitarias dedicadas a la deserialización y evaluación del subtipo `respiratory_rate`.
2. **Implementación de Transformador Tipado (`crates/medsys-core/src/engine/transform.rs`):**
   - Crear función canónica `transform_observation_respiratory_rate`.
   - Implementar degradación por omisión ante `None` en `signo.frecuencia_respiratoria`.
3. **Ampliación de Endpoints HTTP Axum (`crates/medsys-server/src/handlers/observation.rs`):**
   - Incorporar prefijo `resp-{id}` / `rr-{id}` en `get_observation`.
   - Incorporar emisión de `obs_rr` en el bundle de `list_observations`.
4. **Armonización de Documentación de Tesis:**
   - Actualizar conteos de recursos en Cap. III (de 14,000 a 16,500 instancias estimadas con 4 observaciones por consulta).
   - Eliminar justificación previa de "redundancia técnica sobre un tercer escalar" en Cap. II y III conforme a D-016.
