---
name: verify-and-audit
description: Ejecuta la verificación empírica de tareas implementadas y audita la consistencia estructural del sistema Antigravity 2.0. Úsese antes de marcar cualquier tarea como [v] o [x].
---

# Skill: Verificación Empírica y Auditoría de Consistencia (`verify-and-audit`)

## Cuándo Usar
- Obligatoriamente en el paso 14 del ciclo **`continúa`** antes de pasar una tarea de `[~]` a `[v]` y `[x]`.
- Cuando el usuario invoca `/verify` o pide validar la configuración del proyecto.

## Procedimiento
1. **Validar la integridad de la configuración Antigravity 2.0:**
   ```bash
   python3 .agents/skills/verify-and-audit/scripts/validate_harness.py --root .
   ```
2. **Ejecutar los comandos de verificación propios del proyecto** (definidos en `.agents/rules/project-profile.md` y en la tarea activa de `BACKLOG.md`).
3. **Regla de Visibilidad de Errores Esperados (Pruebas de Resiliencia y Carga):**
   - Cuando una prueba evalúe respuestas de rechazo o ausencia de datos opcionales (como HTTP `404` por campos opcionales `NULL` o HTTP `422` por política *Fail-Closed*), verifica que el reporte mantenga **visibles y desglosados** esos conteos en contadores dedicados (demostrando que el 100 % de esos códigos `4xx` se debieron a ausencia/anomalía de datos en origen), separándolos de la tasa de fallos técnicos (`5xx`/timeouts) del servidor.
