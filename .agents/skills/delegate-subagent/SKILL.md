---
name: delegate-subagent
description: Construye y ejecuta contratos explícitos de delegación hacia los subagentes de .agents/agents/, garantizando aislamiento de archivos en ejecución paralela y verificación de entregables.
---

# Skill: Delegación Estructurada a Subagentes (`delegate-subagent`)

## Cuándo Usar
- Siempre que el Coordinador Maestro delegue diseño, implementación, pruebas, auditoría o documentación a uno o hasta 3 subagentes en paralelo.

## Árbol de Decisión de Selección de Subagente
- **¿Falta desglosar un plan `.md` o actualizar dependencias del backlog?** $\rightarrow$ `planner-analyst`
- **¿Se requiere diseñar cómo resolver un módulo complejo o investigar el código?** $\rightarrow$ `architect-researcher`
- **¿Se van a escribir o modificar archivos de código, SQL, YAML o scripts?** $\rightarrow$ `implementer`
- **¿Se van a correr suites de pruebas, oráculos, corpus de fallos o benchmarks?** $\rightarrow$ `tester-qa`
- **¿Se requiere auditar calidad, seguridad o consistencia antes de cerrar una fase?** $\rightarrow$ `reviewer-auditor`
- **¿Se van a redactar reportes de resultados, capítulos o consolidar `STATE.md`?** $\rightarrow$ `documenter-integrator`

## Procedimiento de Lanzamiento en Antigravity 2.0
1. Lee la definición del subagente seleccionado en `.agents/agents/<nombre>/agent.md`.
2. Rellena el contrato de delegación siguiendo la plantilla `.agents/skills/delegate-subagent/resources/delegation_contract_template.md`.
3. **Verificación de Aislamiento Paralelo:** Si vas a lanzar 2 o 3 subagentes `implementer` simultáneamente, comprueba que sus listas `Allowed Files to Modify` no compartan ningún archivo.
4. Lanza el subagente (usando la herramienta nativa de subagentes de Antigravity 2.0 con `TypeName: "self"` y `Role: "<nombre>"`).
5. Al recibir la respuesta del subagente, no des por cerrada la tarea sin comprobar que los archivos existen y pasan el comando de verificación.
