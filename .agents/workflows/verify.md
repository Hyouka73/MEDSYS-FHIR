---
description: Ejecuta la validación del harness Antigravity 2.0 y la suite de pruebas empíricas del proyecto.
---

# Workflow de Verificación y Auditoría (`/verify`)

1. Ejecuta `python3 .agents/skills/verify-and-audit/scripts/validate_harness.py --root .`.
2. Ejecuta los comandos de prueba y calidad configurados en `.agents/rules/project-profile.md`.
3. Reporta los resultados reales obtenidos sin modificar código salvo solicitud explícita.
