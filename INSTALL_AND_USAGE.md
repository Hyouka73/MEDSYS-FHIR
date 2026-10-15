# GUÍA DE INSTALACIÓN, INICIALIZACIÓN Y USO — Sistema Universal Antigravity 2.0

## 1. Qué Archivos Copiar y Dónde

### A. Instalación por Proyecto (Recomendada y Autocontenida)
Copia el contenido de la raíz de este paquete directamente en la raíz de tu repositorio:
- `AGENTS.md` y `GEMINI.md` (Puntos de entrada en la raíz)
- `.gemini/config.json` (Configuración de proyecto para Antigravity 2.0 v2.17+)
- `.agents/` (`rules/`, `agents/`, `skills/`, `workflows/`)
- `STATE.md`, `BACKLOG.md` y `DECISIONS.md`

### B. Instalación Global Opcional en tu Estación de Trabajo
Si deseas que las skills y subagentes estén disponibles globalmente en todas tus carpetas dentro de Google Antigravity 2.0 sin copiarlos cada vez:
- Copia `.agents/skills/*` en `~/.gemini/config/skills/` (o `~/.gemini/antigravity-cli/skills/` para `agy` CLI).
- Copia `.agents/agents/*` en `~/.gemini/config/agents/`.
*(Aun usando instalación global, conserva siempre `AGENTS.md`, `GEMINI.md`, `STATE.md`, `BACKLOG.md`, `DECISIONS.md` y `.agents/rules/project-profile.md` en la raíz de cada proyecto para mantener el estado aislado por repositorio).*

---

## 2. Cómo Inicializar un Proyecto Nuevo desde un Único Archivo `.md`

1. Coloca tu archivo de plan integral (por ejemplo `PLAN_PROYECTO.md`, `SPEC.md`, `MASTER_PLAN.md` o cualquier nombre `.md`) en la raíz de tu proyecto.
2. Extrae este ZIP en la raíz del proyecto.
3. Abre Google Antigravity 2.0 en esa carpeta y escribe únicamente:
   > **`continúa`**
4. El Coordinador detectará que `STATE.md` está en `UNINITIALIZED`, ejecutará `discover_plan.py` para encontrar tu `.md` **sin modificarlo ni reescribirlo**, extraerá las fases, requisitos, decisiones y criterios de aceptación hacia `BACKLOG.md`, `DECISIONS.md` y `.agents/rules/project-profile.md`, y comenzará a ejecutar la primera tarea.

---

## 3. Cómo Adaptar un Proyecto Existente (con Código o Avance Previo)

1. Extrae el paquete en la raíz del repositorio existente (si ya tienes un `STATE.md` y `BACKLOG.md` que deseas conservar, mantenlos o usa el adaptador preconfigurado de `examples/02_existing_project_adaptation_medsys_fhir/`).
2. Escribe en Antigravity 2.0:
   > **`continúa`**
3. El sistema inspeccionará el código real existente, reconciliará el plan contra lo ya implementado (marcando `[x]` solo lo que tenga pruebas pasando y `[~]` lo que requiera verificación) y retomará desde la primera brecha pendiente.

---

## 4. Flujo de Trabajo Diario y Recuperación ante Interrupciones

- **Para avanzar automáticamente:** Escribe **`continúa`** (o `/continue`).
- **Para ver el estado sin modificar nada:** Escribe `/status`.
- **Para hacer una consulta sin tocar archivos:** Escribe `/ask <tu pregunta>`.
- **Para auditar la configuración y correr pruebas:** Escribe `/verify`.
- **Si se agota temporalmente la cuota de Google AI Pro o cierras la aplicación:** No pierdes nada. Cada tarea completada queda guardada en `STATE.md` y `BACKLOG.md`. Cuando vuelvas a abrir Antigravity y escribas **`continúa`**, la skill `recover-session` e `reconcile-state` retomarán exactamente donde se quedó.
