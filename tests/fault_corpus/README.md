# Corpus de Inyección de 100 Fallas Clínicas (`T1`–`T5`) — MedSys-FHIR

Este directorio contiene el corpus experimental determinista de **100 casos de prueba anómalos** diseñados para la evaluación cuantitativa de los Objetivos Específicos 2 y 3 (Capítulo III, §3.2.3 y Tabla 1) de la tesis de licenciatura de Carlos Eduardo Iglesias de la Cruz y Alexis Andrey Gálvez Roblero (UNACH, 2026).

## 1. Taxonomía de las 5 Categorías de Falla (`T1`–`T5`)

| Categoría | Código | Casos | Causa Raíz Simulada | Comportamiento Normativo Esperado |
|-----------|--------|-------|---------------------|-----------------------------------|
| **T1** | `T1-01` a `T1-20` | 20 | Campos críticos obligatorios nulos o vacíos (`primer_nombre`, `curp`, `fecha_nacimiento`, `presion_sistolica`, etc.) | Rechazo HTTP 422 `OperationOutcome` (`code = "processing"`) |
| **T2** | `T2-01` a `T2-20` | 20 | Tipos de datos corruptos, cadenas no numéricas y fechas imposibles (ej. 30 de febrero, mes 13, `ALTA`, `120/XX`) | Rechazo HTTP 422 `OperationOutcome` (`code = "processing"`) |
| **T3** | `T3-01` a `T3-20` | 20 | Valores categóricos fuera de catálogo/diccionario (`sexo = "X"`, `estado = "ABIERTO"`, `tipo = "DESCARTADO"`) | Rechazo HTTP 422 `OperationOutcome` (`code = "processing"`) |
| **T4** | `T4-01` a `T4-20` | 20 | Llaves foráneas rotas y registros huérfanos (`id_paciente = 99991`, `id_consulta = 88881`) | Rechazo HTTP 422 `OperationOutcome` (`code = "processing"`) |
| **T5** | `T5-01` a `T5-20` | 20 | Identificadores y sintaxis CURP violando la regex oficial de RENAPO (17/19 car., caracteres inválidos, mes 13, sexo Z) | Rechazo HTTP 422 `OperationOutcome` (`code = "processing"`) |

## 2. Archivos del Corpus

- `fault_corpus_100.json`: Catálogo estructurado en JSON con los 100 casos etiquetados individualmente con sus metadatos y payload de prueba.
- `fault_corpus_100.sql`: Script DDL y DML para inyectar los 100 casos en un esquema aislado de PostgreSQL (`fault_corpus`).
- `README.md`: Este documento de contexto técnico.

## 3. Criterio de Aprobación Experimental

1. **Sensibilidad de Rechazo:** 100 de 100 casos anómalos (100.0 %) deben ser rechazados de manera cerrada (*Fail-Closed*) con HTTP 422 `OperationOutcome` y código de issue `processing`.
2. **Especificidad:** Cero falsos rechazos (0.0 %) sobre los 9,000 registros válidos del universo censal.
