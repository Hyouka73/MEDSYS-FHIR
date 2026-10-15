---
name: init-from-plan
description: Inicializa o adapta cualquier proyecto nuevo o existente a partir de un archivo Markdown (.md) de plan maestro sin sobrescribir el documento original. Úsese al comenzar un proyecto o cuando STATE.md indique estado UNINITIALIZED.
---

# Skill: Inicialización y Adaptación desde Plan Maestro (`init-from-plan`)

## Cuándo Usar
- Cuando `STATE.md` tiene `Estado del Sistema: UNINITIALIZED`.
- Cuando el usuario instala este paquete en un repositorio nuevo o existente y escribe **`continúa`** o `/init`.
- Cuando se agrega o actualiza un documento `.md` de plan maestro en el proyecto.

## Procedimiento Paso a Paso
1. **Ejecutar el descubridor de plan y escáner de repositorio (Caja Negra):**
   ```bash
   python3 .agents/skills/init-from-plan/scripts/discover_plan.py --root . --json
   ```
   *(No leas el código fuente del script a menos que falle; usa su salida JSON).*
2. **Analizar el Plan Maestro Detectado (Solo Lectura):**
   - Lee el archivo `.md` identificado por `discover_plan.py` como plan maestro.
   - **REGLA CRÍTICA:** Jamás modifiques ni sobrescribas el archivo de plan maestro original del usuario.
3. **Detectar el Tipo de Proyecto y Stack Real:**
   - Revisa en la salida de `discover_plan.py` los lenguajes, frameworks, archivos existentes y suites de pruebas presentes en el repositorio.
4. **Configurar `.agents/rules/project-profile.md`:**
   - Registra el nombre del proyecto, la ruta del plan maestro original, el arquetipo de proyecto, los subagentes activos pertinentes y los comandos exactos de compilación/linter/test.
5. **Construir o Reconciliar `BACKLOG.md` y `DECISIONS.md`:**
   - Extrae todas las fases y tareas del plan maestro.
   - Si el proyecto ya tiene código implementado, verifica cada ítem contra los archivos existentes: marca como `[x]` únicamente lo que ya exista y pase sus pruebas; marca como `[~]` lo que exista en código pero carezca de verificación; y deja en `[ ]` todo lo pendiente.
   - Registra las decisiones técnicas del plan y cualquier supuesto de adaptación en `DECISIONS.md`.
6. **Activar `STATE.md`:**
   - Cambia `Estado del Sistema` a `ACTIVE`, señala la primera tarea pendiente `[ ]` y continúa inmediatamente con su ejecución.
