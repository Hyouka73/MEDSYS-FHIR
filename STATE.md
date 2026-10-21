# ESTADO PERSISTENTE DEL PROYECTO (`STATE.md`)

## Checkpoint vigente — 2026-10-20, 23:16 America/Mexico_City

**EVALUACIÓN REABIERTA; DETENIDA POR INSTRUCCIÓN «termina».** El cierre 40/40 que sigue debajo es un antecedente histórico, no el dictamen experimental vigente. CA-1–CA-5 y CT permanecen pendientes de verificación primaria; ningún hallazgo R5 se cierra en esta sesión.

- Inicio: commit `18b434a692f8ab28a400f6ccab64c5d52e6bc183`, árbol limpio, ninguna etiqueta local. No se creó v1.2.0-eval ni se hizo commit/push.
- Lectura efectiva en Drive: registro, paquete de prompts, capítulos III y IV. Guardadas copias textuales en `auditoria/evidencias/2026-10-20-reverificacion/`. No se modificó ningún documento en Drive.
- Se recuperaron umbrales originales y regla de decisión; véase `ACCESO_Y_BASELINE.md` en esa carpeta.
- Cambios en curso: dos pruebas de regresión añadidas en `crates/medsys-db/src/dynamic_repo.rs`; no se certifica su aprobación ni corrección productiva. Inspeccionar diff antes de reanudar.
- Entorno: Windows 11, Rust/Cargo 1.98.1, Python 3.13.1, Java 21.0.10. Docker Desktop 4.37.1/CLI 27.4.0 no pudo levantar motor Linux. Log informa fallo al retirar socket `userAnalyticsOtlpHttp.sock` por acceso del sistema; no se borraron archivos ni volúmenes para intentar repararlo.
- Sin PostgreSQL/HTTP aislado listo, no se ejecutaron censos Rust S0–S3, validador oficial, corpus HTTP ni CT. Metadatos de releases en `tools/experimental/bin/` no equivalen a herramientas descargadas ni corridas.
- Siguiente acción al recibir «continúa»: recuperar sesión, inspeccionar cambios parciales y resultados de subagentes, ejecutar regresiones baseline, corregir los defectos, resolver motor Docker con medidas reversibles o preparar PostgreSQL aislado alternativo; después completar evidencias antes de editar celdas N/V.
- Pendiente externo académico: determinación/documento de la instancia competente sobre trámite ético institucional. No se afirmó su resolución.

### Entrega para revisión

El usuario autorizó posteriormente crear un commit local de este checkpoint. Es un avance parcial, no una corrección terminada ni una evaluación aprobada. Incluye fuentes documentales, estado, metadatos de herramientas y dos regresiones aún sin resultado acreditado. No se realiza push. Para el agente revisor: leer este checkpoint, contrastar el diff del commit y continuar por las regresiones baseline antes de implementar cambios productivos o cerrar hallazgos. No tomar los bloques históricos inferiores como estado experimental vigente.

---

## Metadatos de Control Operativo
- **Estado del Sistema:** `ACTIVE`
- **Plan Maestro de Referencia (Solo Lectura):** `.agents/backlog/sprint_10_automatizacion_experimentos_cap4.md` y `auditoria/DECISIONES.md`
- **Fase / Sprint Activo:** Sprint 10 (Completado) — Todos los Sprints (1 al 10) culminados al 100%
- **Progreso Global:** `40 / 40 tareas completadas (100.0%)`
- **Tarea en Curso (`[/]`):** Ninguna (Proyecto y Evaluación Experimental Finalizados)
- **Última Tarea Cerrada (`[x]`):** Tarea 10.4: Verificación Final del Informe `auditoria/RESULTADOS_CAPITULO_IV.md` y Cierre 40/40
- **Siguiente Tarea Inmediata:** Ninguna. Batería experimental del Capítulo IV culminada y verificada.
- **Estado de Verificación / Build:** `ALL_TESTS_PASSING` (91/91 pruebas Rust, clippy 0 warnings, oráculo 100%, corpus 100%)

---

## Registro de la Última Iteración
- **Archivos Modificados/Creados:**
  - `scripts/generate_data.py`: exportación determinista de `valid_resp_ids.json` (2,250) y `omitted_resp_ids.json` (250).
  - `tests/k6/load_test.js`: contadores diagnósticos visibles y separación de 404 por datos ausentes vs 5xx.
  - `tests/k6/resilience_and_errors_test.js` y `tests/k6/smoke_test.js`: `http.expectedStatuses(200, 400, 404, 422)` y métricas de rechazo.
  - `crates/medsys-server/src/handlers/patient.rs`, `encounter.rs`, `observation.rs`, `condition.rs`: enrutamiento dinámico multi-esquema (S0–S3) con `medsys_db::row_to_legacy_*`.
  - `crates/medsys-server/tests/e2e_interoperability.rs`: test E2E de enrutamiento multi-arquetipo S1–S3.
  - `scripts/run_all_cap4_experiments.py`: orquestador maestro automatizado de las 6 secciones experimentales.
  - `output/resultados_cap4.json` y `auditoria/RESULTADOS_CAPITULO_IV.md`: resultados consolidados oficiales.
- **Delegaciones Activas:** Ninguna.
- **Bloqueos (`[!]`):** Ninguno.

---

## Cómo Continuar
El sistema se encuentra 100 % completado y verificado empíricamente. Para re-evaluar o consultar el estado:
> `python scripts/run_all_cap4_experiments.py`


