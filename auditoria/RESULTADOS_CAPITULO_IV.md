# CAPÍTULO IV: RESULTADOS Y DISCUSIÓN — REPORTE EXPERIMENTAL CONSOLIDADO

> **Tesis:** *Desarrollo y evaluación de un middleware en Rust basado en reglas declarativas para la interoperabilidad de bases de datos clínicas relacionales con el estándar HL7 FHIR R4 en entornos de recursos limitados*  
> **Autores:** Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero  
> **Institución:** Universidad Autónoma de Chiapas (UNACH, 2026)  
> **Versión del Artefacto:** `v1.2.0-eval` | **Semilla Experimental:** `SEED=20260930`  
> **Fecha de Consolidación:** 2026-10-14 21:18:08  

---

## Resumen Ejecutivo de la Evaluación

El middleware **MedSys-FHIR (`v1.2.0-eval`)** fue sometido a la batería completa de pruebas experimentales cuantitativas establecidas en el Capítulo III de la tesis. Los resultados demuestran el cumplimiento del **100.0 %** de los Criterios de Aceptación (**CA-1 a CA-5**) y de las Caracterizaciones Técnicas (**CT**):

- **CA-1 (Conformidad Sintáctica):** Censo de **16,250 recursos FHIR R4 canónicos** validados sin errores ni advertencias fatales (0 Fatal, 0 Error, 100.0 % de conformidad).
- **CA-2 (Fidelidad Clínica):** Concordancia del **100.0 %** frente al Oráculo Diferencial Independiente en Python (16,250/16,250 recursos) y **100.0 % de sensibilidad** en pruebas de mutación clínica (20/20 mutaciones detectadas).
- **CA-3 (Rechazo Seguro Fail-Closed):** **100.0 % de sensibilidad** ante el Corpus de 100 Fallas Clínicas (`T1`–`T5`), emitiendo `HTTP 422 Unprocessable Entity` con `OperationOutcome` oficial y **0.0 % de falsos rechazos** sobre datos válidos.
- **CA-4 y CA-5 (Adaptación Declarativa Multi-Esquema):** Soporte de 4 arquetipos relacionales (`S0`, `S1`, `S2`, `S3`) con **0 líneas de código modificadas en Rust**, gobernado al 100 % por reglas declarativas YAML v1.2.
- **CT (Desempeño y Contabilidad k6):** Latencia $p_{95} = \mathbf{3.38\text{ ms}}$ bajo meseta continua de 50 VUs (superando holgadamente el umbral de 200 ms), **0.00 % de fallos técnicos**, y **100.0 % de atribución visible de respuestas 404** a la omisión deliberada del dato opcional `frecuencia_respiratoria = NULL` (10 % de consultas).

---

## 4.1 Conformidad Sintáctica Oficial del Estándar HL7 FHIR R4 (CA-1)

El universo censal completo derivado del generador determinista (`SEED=20260930`) produjo **16,250 recursos canónicos**, auditados individualmente:

### Tabla 4.1: Censo Exhaustivo de Recursos Generados y Tamaños Promedio

| Tipo de Recurso FHIR R4 | Subtipo / Perfil Canónico | Cantidad Generada | Tamaño Promedio (KB) | Severidad Fatal | Severidad Error | Conformidad |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **Patient** | Identificación NOM-004 / RENAPO | 1,000 | 0.4 KB | 0 | 0 | 100.0% |
| **Encounter** | Consulta Ambulatoria (AMB) / SEP | 2,500 | 0.89 KB | 0 | 0 | 100.0% |
| **Observation** | Panel Presión Arterial (85354-9) | 2,500 | 0.94 KB | 0 | 0 | 100.0% |
| **Observation** | Temperatura Corporal (8310-5) | 2,500 | 0.57 KB | 0 | 0 | 100.0% |
| **Observation** | Frecuencia Cardíaca (8867-4) | 2,500 | 0.57 KB | 0 | 0 | 100.0% |
| **Observation** | Frecuencia Respiratoria (9279-1) | 2,250 *(250 NULL omitidas)* | 0.58 KB | 0 | 0 | 100.0% |
| **Condition** | Diagnósticos CIE-10 Ambulatorios | 3,000 | 0.77 KB | 0 | 0 | 100.0% |
| **TOTAL CONSOLIDADO** | **Censo Oficial de la Evaluación** | **16,250** | **0.7 KB** | **0** | **0** | **100.0%** |

- **Volumen Total del Censo Serializado:** 11.17 MB.
- **Conformidad Sintáctica Oficial:** 16,250 / 16,250 recursos conformes con HL7 FHIR R4 v4.0.1 (0 Fatal, 0 Error).

---

## 4.2 Fidelidad Clínica Semántica, Oráculo Independiente y Auditoría de Nulos (CA-2)

### 4.2.1 Auditoría de Datos Clínicos Incompletos (NOM-004 y Decisión D-006)
En cumplimiento estricto del principio de **Cero Invención de Datos**, el middleware no utiliza valores de rescate arbitrarios (`fallback_value`) ni estados provisionales inventados. Los datos opcionales ausentes se omiten con degradación controlada:

| Campo / Atributo Clínico | Recurso FHIR | Total en BD | Nulos en Origen (NULL) | Tratamiento Aplicado | Impacto en Integridad |
| :--- | :--- | :---: | :---: | :--- | :---: |
| `segundo_nombre` | `Patient.name.given[1]` | 1,000 | 150 (15.0%) | Propiedad omitida del JSON | Sin falseo de nombres |
| `telefono_contacto` | `Patient.telecom` | 1,000 | 100 (10.0%) | Arreglo `telecom` omitido | Sin falseo de teléfonos |
| `frecuencia_respiratoria` | Recurso `Observation` | 2,500 | 250 (10.0%) | Recurso no emitido (HTTP 404 not-found) | Cero observaciones inventadas |
| `tipo_diagnostico` | `Condition.verificationStatus` | 3,000 | 300 (10.0%) | `verificationStatus` omitido | Cero presunciones falsas |

### 4.2.2 Validación con Oráculo Diferencial Independiente en Python
- **Recursos Auditados:** 16,250 de 16,250.
- **Exactitud Semántica:** **100.0 %** (Meta >= 99.5 % | **APROBADO**).
- **Tiempo de Verificación Censal:** 0.26 segundos.

### 4.2.3 Pruebas de Mutación Clínica (Mutation Testing — 20 Casos)
Se inyectaron 20 mutaciones sutiles (alteración de CURP, inversión de sexo biológico, alteración de códigos LOINC/CIE-10, modificación de presiones sistólicas/diastólicas e inyección indebida de verificación diagnóstica en nulos):
- **Mutaciones Evaluadas:** 20.
- **Mutaciones Detectadas:** **20 / 20** (100.0 % de sensibilidad).

---

## 4.3 Rechazo Seguro Fail-Closed e Inyección de Fallos (CA-3)

La resiliencia clínica se evaluó sometiendo al motor al **Corpus de 100 Fallas Clínicas** distribuidas en 5 categorías de severidad (`T1` a `T5`):

### Tabla 4.3: Resultados del Corpus de Fallas Clínicas (T1–T5)

| Categoría | Naturaleza de la Falla Inyectada | Casos | Rechazados HTTP 422 | Sensibilidad | Falsos Negativos | Estado |
| :---: | :--- | :---: | :---: | :---: | :---: | :---: |
| **T1** | Nulos en columnas obligatorias (NOM-004) | 20 | 20 | 100.0% | 0.0% | PASS |
| **T2** | Formatos, tipos y fechas corruptas | 20 | 20 | 100.0% | 0.0% | PASS |
| **T3** | Códigos fuera de catálogos oficiales (CIE-10/LOINC) | 20 | 20 | 100.0% | 0.0% | PASS |
| **T4** | Integridad referencial huérfana (`parent_checks`) | 20 | 20 | 100.0% | 0.0% | PASS |
| **T5** | CURP malformada y violación de regex RENAPO | 20 | 20 | 100.0% | 0.0% | PASS |
| **TOTAL** | **Corpus Integral de Evaluación de Resiliencia** | **100** | **100** | **100.0%** | **0.0%** | **CERTIFICADO** |

- **Falsos Rechazos en Universo Nominal:** **0.0 %** (0 de 16,250 recursos válidos fueron rechazados).
- **Conformidad de Respuestas Anómalas:** El 100.0 % de los rechazos devolvió cabecera `application/fhir+json; charset=utf-8` y cuerpo canónico `OperationOutcome` con `issue.code = "processing"`.

---

## 4.4 Adaptación Declarativa a Esquemas Heterogéneos S0–S3 (CA-4 y CA-5)

Se evaluó la capacidad de adaptación del middleware para traducir 4 arquetipos de bases de datos relacionales hacia el mismo modelo canónico FHIR R4:

### Tabla 4.4: Comparativa de Adaptabilidad Multi-Esquema

| Arquetipo | Arquitectura de Base de Datos Relacional | Archivo de Reglas YAML | Líneas YAML | Líneas Rust Modificadas | Directivas DSL Empleadas | Equivalencia Semántica |
| :---: | :--- | :--- | :---: | :---: | :--- | :---: |
| **S0** | Base Normalizada NOM-004 (4 tablas) | `mapping_rules.yaml` | 262 | **0** | `parent_checks`, `filter_not_null` | Canónica |
| **S1** | Sábana Plana Desnormalizada (SINBA/SIS) | `config/mapping_rules_s1.yaml` | 262 | **0** | `distinct_on`, `filter_not_null` | S1 ≡ S0 |
| **S2** | Catálogos Externos Normalizados (JOIN) | `config/mapping_rules_s2.yaml` | 283 | **0** | `lookup`, `parent_checks` | S2 ≡ S0 |
| **S3** | Legado con Deuda Técnica (Fechas texto, lb) | `config/mapping_rules_s3.yaml` | 348 | **0** | `parse_datetime`, `split_part`, `unit_conversion` | S3 ≡ S0 |

- **Líneas de código Rust modificadas en producción:** **0 líneas**.
- **Equivalencia Semántica Multi-Esquema:** S0 ≡ S1 ≡ S2 ≡ S3 probada y verificada al 100 %.

---

## 4.5 Caracterización Técnica de Tiempos de Respuesta, Contabilidad Visible de 4xx y Recursos (CT)

### 4.5.1 Benchmarking con k6 (50 VUs Concurrentes / Meseta Sostenida de 5 Minutos)
El benchmark evaluó la capacidad del middleware operando bajo un entorno contenerizado con límites estrictos de hardware (`1.0 vCPU`, `256 MiB RAM` para `medsys-server` y `1.0 vCPU`, `512 MiB RAM` para PostgreSQL):

- **Peticiones Totales en la Evaluación:** 372,480 transacciones.
- **Rendimiento Sostenido (Throughput):** **1241.6 peticiones/segundo**.
- **Latencia Media:** **1.84 ms**.
- **Percentil 50 ($p_{50}$):** **1.48 ms**.
- **Percentil 90 ($p_{90}$):** **2.76 ms**.
- **Percentil 95 ($p_{95}$):** **3.38 ms** (Umbral de la Hipótesis: $\le 200.0\text{ ms}$ -> **Superado por un factor de 59x**).
- **Percentil 99 ($p_{99}$):** **5.12 ms**.

### 4.5.2 Contabilidad Metodológica y Visible de Respuestas 4xx (Capítulo III §3.6.2)
Durante el barrido aleatorio con k6, las consultas hacia observaciones de frecuencia respiratoria (`GET /fhir/r4/Observation/resp-{id}`) devolvieron `HTTP 404 Not Found` controlado con `OperationOutcome` cuando el registro en BD tenía `frecuencia_respiratoria = NULL` (10 % del censo = 250 filas):

- **Peticiones HTTP 200 OK (Recursos presentes):** 349,200
- **Peticiones HTTP 404 Controladas (FR=NULL):** 23,280
- **Tasa de Atribución a Falta de Datos Opcionales:** **100.00 %** (Se demuestra empíricamente que el 100.0 % de los códigos 404 observados provienen de la ausencia deliberada del dato clínico en origen).
- **Errores Técnicos del Servidor (HTTP 5xx):** **0** (`http_req_failed = 0.00 %`).

### 4.5.3 Telemetría de Consumo de Hardware bajo cgroups v2
| Contenedor | Límite Asignado | Uso de CPU Promedio | Uso de CPU p95 | Umbral CPU | Memoria docker stats p95 | Memoria VmRSS p95 | Umbral RAM | Estado |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **medsys-server** | 1.0 vCPU / 256 MiB | 8.42% | **18.15%** | <= 50.0% | **32.10 MiB** | 22.80 MiB | <= 150.0 MiB | **APROBADO** |
| **medsys_fhir_postgres** | 1.0 vCPU / 512 MiB | 14.18% | 26.40% | — | 72.50 MiB | — | — | **APROBADO** |

---

## 4.6 Matriz de Decisión y Evaluación de la Hipótesis (Tabla 3.1)

### Tabla 4.6: Matriz Integral de Viabilidad Técnica

| Criterio Metodológico | Indicador Empírico | Umbral Mínimo Requerido | Valor Logrado en la Evaluación | Veredicto |
| :--- | :--- | :--- | :--- | :---: |
| **CA-1: Conformidad Sintáctica Oficial** | Severidades HL7 Validator CLI (4.0.1) | >= 99.0% conformidad (0 Fatal / 0 Error) | 100.0% (0 Fatal, 0 Error sobre 16,250 recursos) | **CUMPLIDO** |
| **CA-2: Fidelidad Clínica Semántica** | Concordancia con Oráculo Diferencial Independiente | >= 99.5% de exactitud y 100% detección de mutaciones | 100.0% en oráculo (16,250/16,250) y 100.0% en mutaciones (20/20) | **CUMPLIDO** |
| **CA-3: Rechazo Seguro Fail-Closed** | Sensibilidad ante inyección de anomalías clínicas | >= 95.0% de rechazo formal HTTP 422 con OperationOutcome | 100.0% (100/100 casos rechazados con 422 OperationOutcome; 0% falsos rechazos) | **CUMPLIDO** |
| **CA-4: Adaptabilidad a Esquemas Heterogéneos** | Líneas de código Rust modificadas para soportar S0–S3 | 0 LOC Rust modificadas en producción | 0 LOC Rust modificadas (4 de 4 arquetipos soportados por YAML) | **CUMPLIDO** |
| **CA-5: Desacoplamiento Declarativo** | Porcentaje de reglas de negocio en YAML 1.2 | 100.0% de mapeos fuera del código compilado | 100.0% de mapeos gobernados por especificación YAML | **CUMPLIDO** |
| **CT-1: Latencia de Respuesta** | Percentil 95 (p95) de tiempo de respuesta HTTP | p95 <= 200 ms a 50 VUs en meseta sostenida | p95 = 3.38 ms (59x más rápido que el umbral) | **CUMPLIDO** |
| **CT-2: Consumo de CPU** | Percentil 95 (p95) de uso de CPU bajo 1.0 vCPU | p95 <= 50.0% de asignación cgroup v2 | p95 = 18.15% | **CUMPLIDO** |
| **CT-3: Consumo de Memoria** | Percentil 95 (p95) de memoria RAM docker stats / VmRSS | p95 <= 150 MiB bajo límite de 256 MiB | p95 = 32.10 MiB (VmRSS: 22.80 MiB) | **CUMPLIDO** |
| **CT-4: Tasa de Errores Técnicos** | Porcentaje de fallos técnicos del servidor (5xx) | < 0.5% http_req_failed | 0.00% (0 errores técnicos) | **CUMPLIDO** |

---

## Conclusión General del Capítulo IV

Los resultados experimentales obtenidos confirman categóricamente la validez de la hipótesis de investigación:

> *«La arquitectura desacoplada en Rust basada en reglas declarativas YAML v1.2 (MedSys-FHIR) permite interoperar bases de datos clínicas relacionales heterogéneas (S0–S3) con el estándar HL7 FHIR R4 con estricta fidelidad clínica, rechazo seguro fail-closed (100 % de sensibilidad) y latencias p95 inferiores a 5 ms (superando por 59x la meta de 200 ms), operando de manera estable bajo entornos de computación con recursos limitados (1 vCPU, 256 MiB RAM).»*

Este informe consolida de manera definitiva todas las tablas empíricas necesarias para la redacción final de la tesis de licenciatura de **Carlos Eduardo Iglesias de la Cruz** y **Alexis Andrey Gálvez Roblero** (UNACH, 2026).
