---
trigger: always_on
globs: ["**/*"]
---

# Regla 03: Optimización de Contexto, Cuotas y Suscripción Google AI Pro

1. **Higiene de Contexto (Anti-Context Bloat):**
   - No leas archivos masivos completos (logs de miles de líneas, volcados SQL de varios megabytes o `node_modules/` / `target/`). Usa los scripts de las skills (`.agents/skills/*/scripts/*.py`) con `--help` o búsquedas dirigidas (`grep`, `head`, `tail`) como cajas negras.
   - Pasa a cada subagente únicamente el resumen relevante y las rutas de los archivos que necesita tocar, nunca el historial entero de la conversación.
2. **Selección Eficiente de Modelos (cuando la superficie permita elegirlos):**
   - Usa modelos rápidos (`flash` / `inherit`) para exploración de archivos, ejecución de pruebas rutinarias y actualización de `STATE.md`.
   - Reserva el razonamiento profundo (`pro` / `inherit` con esfuerzo alto) para arquitectura, depuración de fallos complejos (`architect-researcher`, `reviewer-auditor`) y reconciliación inicial del plan maestro.
3. **Resiliencia ante Límites de Cuota (Checkpoint Incremental):**
   - Actualiza `STATE.md` y `BACKLOG.md` inmediatamente después de verificar cada tarea individual (`[x]`), no al final de 10 tareas juntas. Así, si se alcanza un límite temporal de cuota de Google AI Pro o se cierra la ventana, al escribir **`continúa`** en una nueva sesión se retoma exactamente desde el último punto guardado con cero trabajo perdido.
