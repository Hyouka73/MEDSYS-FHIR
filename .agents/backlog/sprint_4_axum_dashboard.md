# SPRINT 4: API REST Axum, OperationOutcome y Dashboard React 19 — MedSys-FHIR

## Objetivo del Sprint
Exponer la capa de servicio HTTP mediante el framework Axum en `crates/medsys-server`, implementar la respuesta clínica estándar `OperationOutcome` para todos los casos de error, desarrollar el dashboard interactivo en React 19 + Vite + Tailwind CSS para visualización y verificación de interoperabilidad, y ejecutar validaciones de carga con k6.

---

## Lista de Tareas

- [x] **Tarea 4.1: Enrutamiento y Controladores HTTP en Axum** *(Completada: 2026-09-24)*
  - Servidor Axum implementado en `crates/medsys-server` con endpoints canónicos: `GET /fhir/r4/Patient/{id}`, `GET /fhir/r4/Encounter/{id}`, `GET /fhir/r4/Observation/{id}` (desacoplamiento PA LOINC 85354-9 y Temperatura LOINC 8310-5) y `GET /fhir/r4/Condition/{id}` (CIE-10).
  - Endpoints de colección con FHIR `Bundle` de tipo `searchset`: `GET /fhir/r4/Patient`, `GET /fhir/r4/Encounter`, `GET /fhir/r4/Observation`, `GET /fhir/r4/Condition`.
  - Middlewares de telemetría estructurada con `tracing` (`TraceLayer`), CORS permisivo (`CorsLayer`) y wrapper `FhirResponse` emitiendo cabeceras estrictas `Content-Type: application/fhir+json; charset=utf-8`.

- [x] **Tarea 4.2: Manejador Global de Errores con OperationOutcome** *(Completada: 2026-09-24)*
  - Implementación del trait `IntoResponse` para `ServerError` (traduciendo todos los errores de dominio `MedSysError`, `sqlx::Error`, fallos de ruta y serialización).
  - Respuestas oficiales con recurso canónico `OperationOutcome` para todos los códigos HTTP: 404 (`not-found`), 422 (`invalid` / `required`), 500 (`transient` / `processing` / `exception`), 400 (`value`).
  - Fallback universal en Axum `not_found_fallback` que captura cualquier URI inexistente y responde con `OperationOutcome` y `Content-Type: application/fhir+json; charset=utf-8`.
  - Endpoints auxiliares de salud (`/health`, `/api/health`) e inspección relacional/comparativa para el frontend (`/api/legacy/patients`, `/api/legacy/patients/{id}/full`).


- [x] **Tarea 4.3: Desarrollo del Dashboard Frontend (React 19 + Vite + Tailwind)** *(Completada: 2026-09-24)*
  - Inicializado proyecto `/dashboard` con React 19 (`^19.2.8`), Vite 8, `@tailwindcss/vite`, y Lucide Icons (`lucide-react`).
  - Configurado proxy en `vite.config.js` hacia el backend Axum en `http://localhost:3000` para las rutas `/fhir`, `/health`, y `/api`.
  - Panel superior de métricas (`MetricsPanel.jsx`): Monitoreo continuo de salud `/health`, latencia en milisegundos (`ms`), estado de PostgreSQL y recuento de eventos.
  - Visor interactivo FHIR (`FhirViewer.jsx`): Navegación entre `Patient`, `Encounter`, `Observation`, `Condition`, soporte para instancias individuales y colecciones `Bundle` (`searchset`), búsqueda por ID y resaltado sintáctico con copia/descarga (`JsonSyntaxHighlighter.jsx`).
  - Comparador de interoperabilidad (`InteroperabilityComparator.jsx`): Demostración visual trifásica lado a lado (Fila relacional NOM-004 ⇄ Reglas YAML ⇄ Recurso FHIR R4) consumiendo `/api/legacy/patients/{id}/full` y garantizando el principio de solo lectura.
  - Consola de eventos y `OperationOutcome` (`EventConsole.jsx`): Historial en vivo de peticiones HTTP, códigos de estado, inspector detallado del recurso diagnóstico y disparadores de prueba (404, 400, fallback no encontrado).
  - Verificación de calidad: 0 errores y 0 advertencias con `oxlint`, build de producción Vite (`npm run build`) completado con éxito.

- [ ] **Tarea 4.4: Pruebas E2E, Auditoría Clippy, Validación de Rendimiento con k6 y Build Release Final**
  - Ejecutar verificación integral de compilación, clippy y tests en todo el workspace.
  - Implementar suites de pruebas de carga y latencia con k6 para validar rendimiento del middleware bajo concurrencia.
  - Documentar especificación de despliegue y entrega de la tesis.
