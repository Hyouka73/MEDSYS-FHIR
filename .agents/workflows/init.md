---
description: Inicializa o adapta el proyecto leyendo el archivo .md de plan maestro sin modificarlo y generando STATE.md, BACKLOG.md, DECISIONS.md y project-profile.md.
---

# Workflow de Inicialización desde Plan Maestro (`/init`)

Sigue íntegramente las instrucciones de `.agents/skills/init-from-plan/SKILL.md`:
1. Ejecuta `python3 .agents/skills/init-from-plan/scripts/discover_plan.py --root . --json`.
2. Lee el documento `.md` de plan maestro detectado sin alterarlo.
3. Inspecciona los archivos existentes en el repositorio.
4. Genera o reconcilia `.agents/rules/project-profile.md`, `BACKLOG.md`, `DECISIONS.md` y `STATE.md`.
5. Ejecuta `python3 .agents/skills/verify-and-audit/scripts/validate_harness.py --root .` para confirmar que la configuración quedó íntegra.
