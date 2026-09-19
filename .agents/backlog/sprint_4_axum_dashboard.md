# SPRINT 4: API REST Axum, OperationOutcome y Dashboard React 19 — MedSys-FHIR

## Objetivo del Sprint
Exponer la capa de servicio HTTP mediante el framework Axum en `crates/medsys-server`, implementar la respuesta clínica estándar `OperationOutcome` para todos los casos de error, y desarrollar el dashboard interactivo en React 19 + Vite + Tailwind CSS para visualización y verificación de interoperabilidad.

---

## Lista de Tareas

- [ ] **Tarea 4.1: Enrutamiento y Controladores HTTP en Axum**
  - Implementar servidor Axum y endpoints REST FHIR (`GET /Patient/{id}`, `GET /Encounter/{id}`, `GET /Observation/{id}`, `GET /Condition/{id}`).
  - Integrar middlewares de logging/telemetría con `tracing`.

- [ ] **Tarea 4.2: Manejador Global de Errores con OperationOutcome**
  - Implementar trait `IntoResponse` para `MedSysError`.
  - Asegurar que errores 404, 422 y 500 emitan invariablemente el recurso canónico `OperationOutcome` con cabecera `Content-Type: application/fhir+json`.

- [ ] **Tarea 4.3: Desarrollo del Dashboard Frontend (React 19 + Vite + Tailwind)**
  - Inicializar `/dashboard` con Vite, React 19, Tailwind CSS y Lucide Icons.
  - Implementar visualizador de recursos FHIR, comparador entre fila SQL legada y recurso FHIR generado, y consola de inspección clínica.

- [ ] **Tarea 4.4: Pruebas E2E, Auditoría Clippy y Build Release Final**
  - Ejecutar verificación integral de compilación, clippy y tests en todo el workspace.
  - Documentar especificación de despliegue y entrega de la tesis.
