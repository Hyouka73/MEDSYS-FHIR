# FLUJO DE TRABAJO AUTÓNOMO (WORKFLOW) — MedSys-FHIR

## Descripción General
Este workflow define el protocolo de ejecución autónoma, incremental y sin pérdida de contexto para el desarrollo del middleware MedSys-FHIR. Permite al usuario iniciar el trabajo con una instrucción simple ("Comienza", "Continúa") y garantiza que, incluso si la sesión se interrumpe o se traslada a otro entorno o chat, el sistema mantenga su memoria histórica, estado actual y trazabilidad del backlog.

---

## 1. El Ciclo de Ejecución Autónomo (Loop de Desarrollo)

Cada sesión de desarrollo sigue un ciclo estricto de 6 fases:

```
[1. INGESTA DE ESTADO] ---> [2. ASIGNACIÓN ORQUESTADOR] ---> [3. DESARROLLO (CODER)]
          ^                                                               |
          |                                                               v
[6. CONSOLIDACIÓN MEMORIA] <-- [5. ACTUALIZACIÓN BACKLOG] <-- [4. VALIDACIÓN (QA)]
```

### Fase 1: Ingesta de Contexto y Estado Actual
- El agente lee obligatoriamente dos archivos raíz:
  1. STATE.md: Revisa qué tarea se completó en el último turno, cuáles son los archivos modificados y el bloqueo actual (si existe).
  2. BACKLOG.md: Identifica las tareas pendientes marcadas con [ ] dentro del Sprint activo.
- Si no hay bloqueos, localiza la primera tarea pendiente con estado [ ].

### Fase 2: Planificación del Turno (Rol: Orquestador)
- El Orquestador descompone la tarea seleccionada en subtareas atómicas ejecutables (e.g., crear struct, implementar trait, escribir test).
- Confirma que los archivos requeridos existan y que no haya incompatibilidades con las reglas del sistema (.agents/rules/rules.md).
- Asigna la ejecución técnica al Agente Desarrollador.

### Fase 3: Ejecución Técnica y Código (Rol: Desarrollador)
- El Desarrollador implementa exclusivamente lo necesario para resolver la subtarea activa.
- Regla de oro: No tocar archivos ajenos a la tarea en curso.
- Sigue las convenciones de Rust: fuertemente tipado, sin unwrap(), con manejo de errores vía MedSysError.
- En caso de tocar mapeos, verifica el archivo mapping_rules_specification.yaml.
- En caso de base de datos, valida contra schema_legado_simulado_nom004.sql.

### Fase 4: Auditoría y Verificación de Calidad (Rol: Validador)
- El Validador inspecciona los cambios y ejecuta la suite de verificación técnica:
  1. Compilación estática: cargo check
  2. Formato y buenas prácticas: cargo clippy -- -D warnings
  3. Pruebas unitarias e integración: cargo test
  4. Si aplica a FHIR: Valida que los JSON generados cumplan con la sintaxis de helios-fhir R4.
- Si alguna verificación falla: Devuelve el control al Desarrollador indicando la traza del error específica.
- Si todas las verificaciones pasan: Otorga visto bueno para cierre.

### Fase 5: Actualización del Backlog (Rol: Gestor de Backlog)
- El Gestor de Backlog modifica BACKLOG.md cambiando el estado de la tarea completada de [ ] a [x].
- Añade una breve nota técnica con la fecha y hora de finalización (e.g., "[x] Tarea 1.2: Structs de reglas implementados - Verificado con test unitario").

### Fase 6: Consolidación de Memoria (Rol: Gestor de Memoria)
- Se actualiza STATE.md registrando:
  - Última tarea completada.
  - Lista de archivos creados o editados en la iteración.
  - Siguiente tarea pendiente inmediata del backlog.
  - Estado del build (PASS/FAIL).
- Yield controlado: Si el usuario solicitó trabajar por etapas, el agente se detiene aquí y reporta brevemente el avance. Si se ordenó ejecución continua, el Orquestador inicia el siguiente ciclo.

---

## 2. Comandos Rápidos del Usuario (Vocabulario de Control)

El sistema responde a comandos concisos y deterministas:

| Comando | Acción del Sistema |
| :--- | :--- |
| **Comienza** | Inicializa el entorno, verifica Docker/Rust, lee el Sprint 1 de BACKLOG.md y arranca la primera tarea. |
| **Continúa** | Lee STATE.md, recupera el hilo exacto donde se quedó y ejecuta la siguiente tarea pendiente. |
| **Valida** | Invoca al Agente Validador para ejecutar cargo test, clippy y validar la integridad del backlog. |
| **Estado** | Genera un resumen ejecutivo del Sprint actual: tareas hechas [x], en progreso [/] y pendientes [ ]. |
| **Pausa** | Guarda el estado actual en STATE.md, limpia procesos en ejecución y queda listo para reanudar otro día. |

---

## 3. Protocolo de Reanudación Cross-Session (Otro Día / Otro Chat)
Cuando inicies una nueva sesión de chat o abras el proyecto después de varios días:
1. No necesitas pegar explicaciones largas ni todo el contexto.
2. Solo dile al nuevo agente:
   > "Lee .agents/rules/rules.md, .agents/workflows/dev-workflow.md, STATE.md y BACKLOG.md. Continúa con la siguiente tarea."
3. El agente inspeccionará los archivos de estado y comenzará a programar inmediatamente sin perder el hilo.
