# ARQUITECTURA DEL SISTEMA UNIVERSAL DE ORQUESTACIÓN — Google Antigravity 2.0

## 1. Diseño Basado en las Convenciones Oficiales de Antigravity 2.0 (Octubre 2026)

Esta arquitectura separa estrictamente **cuatro niveles de responsabilidad** conforme al runtime real de Google Antigravity 2.0:

1. **Coordinador Maestro (`AGENTS.md` / `GEMINI.md` + `.gemini/config.json`):**
   - El agente del hilo principal funge como *Master Orchestrator*. No delega su propio rol de coordinación a otro coordinador (evitando doble sobrecarga de tokens).
   - Ejecuta el ciclo determinista de 17 pasos al recibir **`continúa`**.
2. **Reglas de Activación Constante (`.agents/rules/*.md` con `trigger: always_on`):**
   - Diseñadas para ocupar menos de 3,500 tokens en total (muy por debajo del presupuesto de 20,000 tokens de reglas introducido en Antigravity v2.17.0), evitando desplazar del contexto a las skills, subagentes y herramientas MCP.
   - Separan las 4 reglas universales inmutables (`00` a `03`) de la capa de adaptación específica del proyecto (`project-profile.md`).
3. **Subagentes Especializados (`.agents/agents/<nombre>/agent.md`):**
   - Descubiertos nativamente en `.agents/agents/{agent_name}/agent.md` con frontmatter oficial (`name`, `description`, `kind: local`, `model: inherit`, `mainAgent: true`, `subagent: true`).
   - Invocados bajo un **Contrato de Delegación Explícito** con alcance estricto de archivos (`Allowed Files`) y soporte para el método *Direct Injection Proxy* (`TypeName: "self"`), permitiendo hasta 3 subagentes en paralelo cuando sus conjuntos de archivos son disjuntos ($S_1 \cap S_2 = \emptyset$).
4. **Skills Procedimentales con Scripts Caja Negra (`.agents/skills/<nombre>/SKILL.md`):**
   - Siguen el estándar de divulgación progresiva (*progressive disclosure*) de Antigravity 2.0: el agente solo ve `name` y `description` al inicio, y carga el cuerpo de `SKILL.md` o ejecuta sus scripts en `scripts/` bajo demanda.

---

## 2. Mejoras Clave frente al Sistema `.agents` Anterior (`v1.1.0`)

| Dimensión | Sistema Anterior (`v1.1.0`) | Nueva Arquitectura Universal Antigravity 2.0 (`v2.0.0`) |
| :--- | :--- | :--- |
| **Acoplamiento al Proyecto** | Reglas, agentes y workflows tenían hardcodeado Rust, Axum, PostgreSQL 16, NOM-004 y `MedSys-FHIR`. | **100 % Universal:** El núcleo es agnóstico al lenguaje y dominio; la especificidad vive aislada en `.agents/rules/project-profile.md`. |
| **Estructura de Subagentes** | Archivos planos `.agents/agents/developer.md` sin frontmatter YAML oficial de Antigravity 2.0. | Estructura oficial `.agents/agents/<rol>/agent.md` con frontmatter `kind: local`, `model: inherit`, `subagent: true` y contrato formal de delegación. |
| **Skills Nativas (`SKILL.md`)** | No existía carpeta `.agents/skills/` ni scripts ejecutables de apoyo. | 6 skills completas en `.agents/skills/<skill>/SKILL.md` con scripts Python de caja negra (`discover_plan.py`, `reconcile_status.py`, `validate_harness.py`). |
| **Inicialización desde `.md`** | Exigía crear manualmente `STATE.md` y los archivos de sprint antes de empezar. | Descubre automáticamente cualquier plan `.md` del usuario sin sobrescribirlo y genera el backlog operativo en el primer **`continúa`**. |
| **Granularidad de Estado** | Solo `[ ]`, `[/]` y `[x]` (permitía marcar `[x]` sin evidencia separada de verificación). | 6 estados formales (`[ ]`, `[/]`, `[~]` Implementada, `[v]` Verificada con evidencia real, `[x]` Completada, `[!]` Bloqueada). |
| **Configuración AGY v2.17+** | No incluía `.gemini/config.json` ni `AGENTS.md` / `GEMINI.md` sincronizados. | Incluye `.gemini/config.json`, `AGENTS.md` y `GEMINI.md` bajo el patrón *Single Source of Truth*. |
