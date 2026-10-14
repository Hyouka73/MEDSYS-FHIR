# SPRINT 7: Banco de Pruebas Multi-Esquema (`S0`–`S3`), Reglas YAML por Arquetipo y Generador Determinista (`SEED=20260930`)

## Objetivo del Sprint
Materializar en el repositorio los **4 arquetipos relacionales (`S0`, `S1`, `S2`, `S3`)**, sus **4 archivos de configuración YAML** y el generador determinista actualizado (`scripts/generate_data.py --seed 20260930`) descritos en la Sección 3.2.2 y 3.2.3 del Capítulo III de la tesis, garantizando que los 9,000 registros clínicos produzcan exactamente **16,250 recursos FHIR R4** (`1,000 Patient + 2,500 Encounter + 9,750 Observation + 3,000 Condition`).

---

## Lista de Tareas Atómicas

- [x] **Tarea 7.1: Creación de los 4 Esquemas DDL (`S0`, `S1`, `S2`, `S3`) y Rol de Solo Lectura `medsys_ro`**
  - **Archivos a crear/modificar:**
    - `schema_legado_simulado_nom004.sql` (actualizar para incluir rol `medsys_ro` con `GRANT CONNECT`, `GRANT USAGE ON SCHEMA public`, `GRANT SELECT ON ALL TABLES IN SCHEMA public`)
    - `docker/sql/schema_s0_base.sql`
    - `docker/sql/schema_s1_flat.sql`
    - `docker/sql/schema_s2_lookup.sql`
    - `docker/sql/schema_s3_legacy.sql`
  - **Instrucciones exactas:**
    1. **`S0` (`schema_s0_base.sql` y `schema_legado_simulado_nom004.sql`):** 4 tablas normalizadas (`tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales`, `tbl_diagnosticos`) sin `DEFAULT 'CONFIRMADO'` en `tipo_diagnostico`.
    2. **`S1` (`docker/sql/schema_s1_flat.sql` — Sábana Plana Desnormalizada):** Tabla plana `tbl_sabana_consulta_externa` donde cada fila concentra los datos del paciente, de la consulta y de los signos vitales (como ocurre en exportaciones tabulares de SINBA/SIS), más tabla `tbl_sabana_diagnosticos`.
    3. **`S2` (`docker/sql/schema_s2_lookup.sql` — Esquema Normalizado con Catálogos Externos):** Tablas principales que almacenan claves enteras (`id_sexo SMALLINT`, `id_estado_consulta SMALLINT`, `id_cie10 INT`, `id_tipo_diag SMALLINT`) vinculadas a tablas de catálogo `cat_sexo_biologico`, `cat_estado_consulta`, `cat_cie10` y `cat_tipo_diagnostico`.
    4. **`S3` (`docker/sql/schema_s3_legacy.sql` — Esquema Legado con Deuda Técnica):** Columnas con nombres abreviados (`pac_curp`, `pac_nom1`, `pac_nom2`, `pac_ap_pat`, `pac_ap_mat`, `fech_nac_txt VARCHAR(20)` en formato `DD/MM/YYYY`, `presion_arterial_txt VARCHAR(15)` en formato `"120/80"`, `peso_libras NUMERIC(6,2)`, `fech_ini_txt VARCHAR(25)`, y sin restricciones `FOREIGN KEY` declaradas en el DDL).
    5. En todos los scripts DDL, crear el rol de mínimo privilegio `medsys_ro` con contraseña `'medsys_ro_pass_2026'` y permisos exclusivos `SELECT`.
  - **Criterio de Aceptación:** Los 4 archivos SQL existen en `docker/sql/` y son sintácticamente válidos en PostgreSQL 16.

- [x] **Tarea 7.2: Creación de los 4 Archivos de Reglas Declarativas YAML (`config/mapping_rules_s0.yaml` a `s3.yaml`)**
  - **Archivos a crear:**
    - `config/mapping_rules_s0.yaml` (copia canónica idéntica a `mapping_rules.yaml` v1.2.0)
    - `config/mapping_rules_s1.yaml` (usando `source_table: "tbl_sabana_consulta_externa"` y `distinct_on: "id_paciente"` para `Patient`)
    - `config/mapping_rules_s2.yaml` (usando directivas `lookup` hacia `cat_sexo_biologico`, `cat_estado_consulta`, `cat_cie10` y `cat_tipo_diagnostico`)
    - `config/mapping_rules_s3.yaml` (usando `parse_datetime: "%d/%m/%Y|%Y-%m-%d"`, `split_part` con delimitador `"/"` para separar `presion_arterial_txt` en sistólica `index: 0` y diastólica `index: 1`, `unit_conversion: "lb_to_kg"` y `parent_checks` activos)
  - **Criterio de Aceptación:** Una prueba unitaria en `medsys-core` carga los 4 archivos `config/mapping_rules_s{0..3}.yaml` con `parse_mapping_rules` y valida sus identificadores con `validate_rules_identifiers_allowlist` sin ningún error.

- [ ] **Tarea 7.3: Actualización Determinista de `scripts/generate_data.py` (`SEED=20260930` y 16,250 Recursos)**
  - **Archivos a modificar:**
    - `scripts/generate_data.py`
    - `docker/sql/02_bulk_data.sql` (regenerado determinísticamente con `SEED=20260930`)
  - **Instrucciones exactas:**
    1. Actualizar `scripts/generate_data.py` manteniendo la semilla fija `DEFAULT_SEED = 20260930` para cumplir con la distribución exacta documentada en el Capítulo III (§3.2.3):
       - **1,000 pacientes (`tbl_pacientes` IDs 1..1000):**
         - 100 % con `curp` válida de 18 caracteres conforme a la regex oficial de RENAPO.
         - Exactamente 15 % (`150` registros) con `segundo_nombre = NULL`.
         - Exactamente 10 % (`100` registros) con `telefono_contacto = NULL`.
       - **2,500 consultas (`tbl_consultas` IDs 1..2500):** vinculadas a pacientes existentes `1..1000`.
       - **2,500 signos vitales (`tbl_signos_vitales` IDs 1..2500):**
         - 100 % con `presion_sistolica`, `presion_diastolica`, `frecuencia_cardiaca` y `temperatura_celsius` presentes.
         - Exactamente **10 % (`250` registros) con `frecuencia_respiratoria = NULL`** y **90 % (`2,250` registros) con `frecuencia_respiratoria` entre 14 y 22 `/min`** (incluyendo que el registro `id_signo = 1` tenga `frecuencia_respiratoria = 18` y el registro `id_signo = 2` tenga `frecuencia_respiratoria = NULL`, para probar ambos casos desde los primeros IDs).
         - Esto garantiza matemáticamente: `2,500 (PA) + 2,500 (Temp) + 2,500 (FC) + 2,250 (FR) = 9,750` recursos `Observation`.
       - **3,000 diagnósticos (`tbl_diagnosticos` IDs 1..3000):**
         - Exactamente **10 % (`300` registros) con `tipo_diagnostico = NULL`** (para ejercitar la omisión limpia de `verificationStatus` con `optional: true`) y el 90 % restante distribuido entre `'CONFIRMADO'` y `'PRESUNTIVO'`.
    2. Añadir el argumento CLI `--schema {s0,s1,s2,s3,all}` en `scripts/generate_data.py` para que pueda generar tanto `docker/sql/02_bulk_data.sql` (`S0`) como los archivos de datos equivalentes para `S1`, `S2` y `S3` (`docker/sql/bulk_s1.sql`, `bulk_s2.sql`, `bulk_s3.sql`) a partir de exactamente las mismas entidades en memoria.
    3. Ejecutar `python3 scripts/generate_data.py --seed 20260930` para actualizar `docker/sql/02_bulk_data.sql` y verificar en `schema_legado_simulado_nom004.sql` que el signo vital 1 tenga `frecuencia_respiratoria = 18` y el signo vital 2 tenga `NULL`.
  - **Criterio de Aceptación:** Dos ejecuciones consecutivas de `python3 scripts/generate_data.py --seed 20260930` producen exactamente el mismo hash SHA-256, con 250 nulos en `frecuencia_respiratoria` y 300 nulos en `tipo_diagnostico`.

- [ ] **Tarea 7.4: Pruebas Automatizadas de Equivalencia Semántica entre Arquetipos (`S0` vs `S1` vs `S2` vs `S3`)**
  - **Archivos a modificar:**
    - `crates/medsys-core/src/engine/mod.rs`
  - **Instrucciones exactas:**
    - Agregar el test unitario `test_multi_schema_s0_s1_s2_s3_semantic_equivalence` que tome el mismo registro clínico representado en formato de fila de `S0`, `S1`, `S2` (con valores resueltos de lookup) y `S3` (con fecha `"01/01/1990"`, presión `"130/85"` y peso en libras), aplique sus respectivas reglas `config/mapping_rules_s{0..3}.yaml` y verifique que los 4 recursos JSON HL7 FHIR R4 resultantes son semánticamente idénticos.
  - **Criterio de Aceptación:** `cargo test --workspace` 100 % PASS.
