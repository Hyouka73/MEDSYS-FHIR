---
trigger: always_on
globs: ["**/*"]
---

# Regla 00: Núcleo de Orquestación y Continuidad (`continúa`)

1. **Invariante de Continuidad:** Ninguna sesión comienza desde cero. Antes de proponer o ejecutar cualquier acción, lee `STATE.md` y `BACKLOG.md`.
2. **Inmutabilidad del Plan Maestro Original:** El documento `.md` de plan maestro creado por el usuario es una **fuente primaria de solo lectura**. Está prohibido sobrescribirlo, truncarlo o borrarle secciones durante la inicialización o ejecución, salvo que el usuario lo pida explícitamente. Toda adaptación operativa se vuelca en `STATE.md`, `BACKLOG.md`, `DECISIONS.md` y `.agents/rules/project-profile.md`.
3. **Grep & Inspect, Don't Guess:** Nunca asumas que un requisito descrito en el plan maestro ya está implementado en el repositorio, ni inventes nombres de funciones, rutas o paquetes. Verifica siempre con inspección real de archivos y comandos de prueba.
4. **Máquina de Estados Estricta en `BACKLOG.md`:**
   - `[ ]` Pendiente
   - `[/]` En progreso
   - `[~]` Implementada (pendiente de validación empírica)
   - `[v]` Verificada (con salida real de pruebas/comandos)
   - `[x]` Completada (verificada + registrada en `STATE.md`)
   - `[!]` Bloqueada (requiere decisión o insumo del usuario)
   *Prohibido pasar de `[ ]` o `[/]` directamente a `[x]` sin pasar por la verificación real `[v]`.*
