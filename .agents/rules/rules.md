# REGLAS DEL SISTEMA Y DESARROLLO — MedSys-FHIR

## 1. Contexto y Objetivos del Proyecto
MedSys-FHIR es un middleware de interoperabilidad en salud desarrollado en Rust para la tesis de licenciatura de Carlos Eduardo Iglesias de la Cruz y Alexis Andrey Gálvez Roblero (UNACH).
- Propósito: Traducir estructuras de datos de un esquema relacional legado simulado (PostgreSQL 16 en Docker) hacia 4 recursos clave del estándar HL7 FHIR Release 4 (R4): Patient, Encounter, Observation y Condition.
- Enfoque: Arquitectura desacoplada mediante patrones Adapter, Facade y Data Mapper, empleando reglas declarativas dinámicas en YAML (mapping_rules.yaml).
- Restricción Ético-Legal: El sistema legado es un arquetipo sintético deducido de la NOM-004-SSA3-2012 y NOM-024-SSA3-2012. No se opera sobre datos de pacientes reales por apego a la LFPDPPP (2025).

## 2. Reglas del Dominio Clínico y Normativo
1. Conformidad Estricta con HL7 FHIR R4:
   - Todo recurso emitido debe satisfacer el esquema oficial de FHIR R4 sin omitir elementos requeridos (e.g., status, resourceType, id).
   - Prohibido inventar propiedades o estructuras JSON personalizadas fuera de la especificación FHIR R4.
2. Identificadores Oficiales y Terminologías:
   - Pacientes: El identificador nacional oficial en México es la CURP, modelada con system "urn:oid:2.16.840.1.113883.4.629" y use "official".
   - Encuentros: Código de clase ambulatoria obligatorio "AMB" bajo el sistema "http://terminology.hl7.org/CodeSystem/v3-ActCode". Cédula del médico tratante identificada bajo "http://cedulaprofesional.sep.gob.mx".
   - Signos Vitales (Observation): Deben estructurarse bajo la categoría "vital-signs" y codificación LOINC (Presión sistólica: 8480-6, Presión diastólica: 8462-4, Temperatura: 8310-5 con unidad "Cel").
   - Diagnósticos (Condition): Codificados bajo el catálogo internacional CIE-10 con system "http://hl7.org/fhir/sid/icd-10".
3. Manejo Oficial de Excepciones Clínicas:
   - Cuando una consulta no exista (HTTP 404), ocurra un error de validación (HTTP 422) o falle una conexión (HTTP 500), la API NUNCA debe devolver un JSON genérico como {"error": "msg"}.
   - Obligatorio responder con el recurso canónico OperationOutcome de HL7 FHIR, detallando severity ("error"), code ("not-found" / "processing") y diagnostics en español técnico.

## 3. Reglas de Codificación en Rust
1. Seguridad de Memoria y Manejo de Errores:
   - Prohibido terminantemente el uso de unwrap() o expect() en código de producción. Toda función falible debe retornar Result<T, MedSysError>.
   - Emplear thiserror para la definición tipada de errores internos y anyhow exclusivamente en pruebas unitarias e inicialización de CLI.
   - Definir un enum de error centralizado que implemente IntoResponse de Axum para convertir automáticamente errores en recursos OperationOutcome con su respectivo código HTTP.
2. Concurrencia y Runtime Asíncrono:
   - Uso de tokio como runtime multihilo con modelo de concurrencia cooperativa sin bloqueo (async/await).
   - Operaciones de I/O de red y base de datos deben ser no bloqueantes. Para cómputo pesado de serialización masiva, emplear tokio::task::spawn_blocking.
3. Persistencia Asíncrona con SQLx:
   - Todo acceso a PostgreSQL debe realizarse mediante sqlx::PgPool configurado con timeout y límite de conexiones.
   - Cero consultas con cadenas concatenadas. Todas las sentencias SQL deben ser parametrizadas con placeholders ($1, $2) para prevenir inyecciones SQL.
   - Acceso a base de datos de solo lectura: El middleware no realiza INSERT, UPDATE ni DELETE sobre tablas legadas (tbl_pacientes, tbl_consultas, etc.).
4. Mapeo y Serialización:
   - Deserializar mapping_rules.yaml usando serde_yaml hacia structs tipados que validen el esquema en tiempo de inicio.
   - Utilizar el crate helios-fhir con la feature flag "r4" activada para modelar los recursos.
   - Serializar los recursos finales con serde_json emitiendo cabeceras Content-Type: application/fhir+json.

## 4. Reglas de Arquitectura y Estructura del Repositorio
1. Monorepo Cargo Workspace:
   - /crates/medsys-core: Lógica pura de dominio, reglas YAML, parser y modelos helios-fhir. Sin dependencias de HTTP ni base de datos.
   - /crates/medsys-db: Adaptador de persistencia con SQLx y consultas parametrizadas.
   - /crates/medsys-server: API REST en Axum, middleware de telemetría y enrutamiento.
   - /dashboard: Frontend en React 19 + Vite + Tailwind CSS.
   - /docker: docker-compose.yml con PostgreSQL 16 y scripts SQL de inicialización.
2. Principio de Responsabilidad Única y Desacoplamiento:
   - El dominio (medsys-core) no debe conocer los detalles del motor de base de datos ni los frameworks web.

## 5. Reglas de Continuidad y Autonomía de Agentes
1. Protocolo de Inicio: Antes de escribir código, leer obligatoriamente STATE.md y BACKLOG.md.
2. Protocolo de Validación: Cada tarea de código debe verificarse ejecutando cargo check y cargo test. Si las pruebas fallan, la tarea NO está completada.
3. Protocolo de Cierre: Al completar una tarea, actualizar inmediatamente STATE.md con los archivos modificados y marcar el checkbox [x] correspondiente en BACKLOG.md.
