---
trigger: always_on
globs: ["**/*"]
---

# Regla 02: Contrato de Delegación y Concurrencia Segura de Subagentes

1. **Cuándo Delegar vs. Ejecutar Directamente:**
   - Ejecuta directamente en el hilo principal solo tareas atómicas de coordinación, lectura inicial, cambios de configuración menores ($< 30$ líneas en 1 archivo) o consolidación final de `STATE.md`.
   - Delega a los subagentes de `.agents/agents/{rol}/agent.md` las tareas de diseño detallado (`architect-researcher`), implementación de módulos (`implementer`), creación/ejecución de suites de prueba (`tester-qa`), auditoría de calidad/seguridad (`reviewer-auditor`) y documentación extensa (`documenter-integrator`).
2. **Mecanismo de Invocación en Antigravity 2.0:**
   - En superficies que soportan subagentes nativos o `invoke_subagent`, lee primero `.agents/agents/{rol}/agent.md` e inyecta sus instrucciones junto con el **Contrato de Delegación** completo (`.agents/skills/delegate-subagent/resources/delegation_contract_template.md`).
   - Si la plataforma requiere `TypeName: "self"` para que el subagente herede permisos de escritura y ejecución de terminal, establece siempre `TypeName: "self"` y `Role: "<Nombre del Subagente>"`.
   - Si en una sesión o superficie concreta la invocación paralela de subagentes está restringida, el Coordinador ejecuta secuencialmente el rol adoptando explícitamente el contrato y las restricciones del archivo `agent.md` correspondiente.
3. **Regla de Oro de Paralelismo (Cero Colisión de Archivos):**
   - Máximo **3 subagentes simultáneos**.
   - Dos subagentes solo pueden ejecutarse en paralelo si sus listas de archivos permitidos (`Allowed Files to Modify`) son **estrictamente disjuntas** ($S_1 \cap S_2 = \emptyset$). Si dos tareas tocan el mismo archivo (por ejemplo `Cargo.toml`, `package.json` o un enrutador central), **secuéncialas obligatoriamente**.
