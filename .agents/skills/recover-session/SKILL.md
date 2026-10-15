---
name: recover-session
description: Recupera una ejecución interrumpida por límite de cuota, cierre de sesión, conflicto de edición o fallo repetido (circuit breaker). Úsese cuando exista una tarea en [/] incompleta o tras 3 fallos consecutivos.
---

# Skill: Recuperación de Sesión e Interrupciones (`recover-session`)

## Cuándo Usar
- Al iniciar con **`continúa`** cuando `STATE.md` muestra que la sesión anterior se interrumpió con una tarea en estado `[/]`.
- Cuando un comando de compilación o prueba falla 3 veces seguidas (*Circuit Breaker*).
- Cuando se reanuda tras alcanzar un límite temporal de cuota de Google AI Pro.

## Procedimiento
1. Ejecuta `git status` y `git diff --stat` para inspeccionar exactamente qué archivos quedaron modificados a mitad de camino.
2. Verifica si los cambios parciales compilan o pasan pruebas:
   - Si son válidos y avanzan hacia el objetivo de la tarea `[/]`, consérvalos y completa únicamente la parte faltante.
   - Si dejaron un archivo sintácticamente corrupto o truncado, repara ese archivo sin borrar archivos sanos.
3. Si la interrupción se debió a 3 fallos del mismo enfoque técnico:
   - Documenta en `DECISIONS.md` por qué falló el intento previo.
   - Si existe una alternativa técnica viable dentro del alcance del plan maestro, aplícala.
   - Si falta una herramienta externa, permiso o definición del usuario, marca la tarea como `[!]` en `BACKLOG.md`, regístralo en `STATE.md` y explica claramente al usuario qué se necesita para desbloquearla.
