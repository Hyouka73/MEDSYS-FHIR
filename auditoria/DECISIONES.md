# BASE DE CONOCIMIENTO DE AUDITORÍA — DECISIONES (MedSys-FHIR)

## Este archivo contiene el inventario exhaustivo de decisiones técnicas, arquitectónicas, metodológicas y de alcance del proyecto MedSys-FHIR. Constituye el artefacto permanente de referencia para todas las rondas de auditoría de la tesis de licenciatura en Ingeniería en Desarrollo y Tecnologías de Software (UNACH).

## D-001 — Alcance de recursos clínicos delimitado a 4 tipos básicos de consulta ambulatoria

- **Estado:** CONGELADA  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** El alcance funcional de interoperabilidad clínica del middleware se delimita exclusivamente a cuatro recursos de HL7 FHIR Release 4 (Patient, Encounter, Observation y Condition), complementados con el recurso técnico OperationOutcome para la gestión estructurada de errores.  
- **Contexto / problema que resuelve:** La especificación FHIR R4 cuenta con más de 140 recursos. Para evaluar la viabilidad de la arquitectura de mediación en una tesis de licenciatura sin dispersar el esfuerzo en la totalidad del estándar, se requiere acotar un núcleo representativo del acto médico ambulatorio en unidades de primer nivel (CESSA).  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "no contempla el desarrollo de módulos de gestión administrativa, facturación ni agendas de citas" (\[TESIS: Cap I §1.6\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Implementar recursos documentales (DocumentReference, Composition), hospitalarios o de procedencia (Provenance).  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Se sacrifica la capacidad de representar hospitalización, cirugías, recetas farmacéuticas (MedicationRequest), citas previas y trazabilidad jurídica de autoría (Provenance).  
- **Limitación a declarar en la tesis:** El middleware es un prototipo representativo de la consulta externa ambulatoria y no un servidor FHIR clínico de propósito general ni un sistema hospitalario integral.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.3, §1.4, §1.6; Cap II §2.1.4 Tabla 1; Cap III §3.1.2, §3.1.5 num. 4, §3.3.3\]. Cita: "cubren el subconjunto mínimo para representar el acto médico ambulatorio" (\[TESIS: Cap II §2.7 Tabla 6\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 define exactamente Patient, Encounter, Observation y Condition; STATE.md §1 num. 2\].  
- **Objetivo o requisito al que sirve:** Objetivo General y Objetivo Específico 3\.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-002 — Firma electrónica avanzada, sellado de tiempo y PKI fuera del alcance

- **Estado:** NO VERIFICADO  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La implementación de infraestructura de clave pública (PKI), la emisión o validación de certificados digitales de la Secretaría de Economía/SAT, el sellado de tiempo y la validación en tiempo de ejecución de Firma Electrónica Avanzada quedan formalmente excluidas del alcance del prototipo, omitiéndose el recurso Provenance.  
- **Contexto / problema que resuelve:** Implementar una autoridad certificadora o validar firmas digitales de acuerdo con la Ley de Firma Electrónica Avanzada (LFEA) introduce una enorme complejidad criptográfica e institucional que desborda el alcance de un middleware de mediación sintáctica y semántica.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "incorporar firmas digitales basadas en infraestructura de clave pública (PKI)... quedan fuera del alcance" (\[TESIS: Cap II §2.2.4\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Implementar firmas digitales locales simuladas, HMAC simétrico o incrustar firmas dummy en el recurso Provenance.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Los documentos clínicos serializados en JSON FHIR carecen de validez jurídica para su aceptación legal y documental obligatoria en transferencias interinstitucionales según la NOM-004 y NOM-024.  
- **Limitación a declarar en la tesis:** Los recursos generados por MedSys-FHIR son técnicamente conformes con FHIR R4 pero no poseen eficacia legal ni validez probatoria ante terceros al carecer de firma digital avanzada.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6(b); Cap II §2.1.4, §2.2.4; Cap III §3.1.5 num. 2, §3.6.1 Tabla 10\]. Cita: "omitiéndose el modelado del recurso Provenance; estos mecanismos se reconocen indispensables... pero quedan delimitados como trabajo futuro" (\[TESIS: Cap III §3.1.5\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml no define mappings para Provenance; no hay utilidades PKI en el código\].  
- **Objetivo o requisito al que sirve:** Delimitación formal del alcance del proyecto.  
- **Conflictos detectados con otras partes:** Tensión regulatoria frente a la NOM-004-SSA3-2012 (numeral 5.10) y NOM-024-SSA3-2012 (numeral 3.22 y 9.6) que exigen firma electrónica para el expediente clínico electrónico.  
- **Consecuencias pendientes:** Declarar explícitamente la falta de validez jurídica en la defensa y en las conclusiones de la tesis.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Contrastada contra texto oficial de NOM-004 numeral 5.10 \[F-003\], NOM-024 numerales 3.22 \[F-007\] y 9.6 \[F-009\], y LFEA art. 7 \[F-018\]).

---

## D-003 — Autenticación, autorización perimetral y seguridad delegadas al API Gateway

- **Estado:** CONGELADA  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La autenticación de clientes, control de acceso basado en roles (RBAC), terminación TLS, mitigación de ataques volumétricos (rate limiting) y trazas de auditoría de seguridad se delegan conceptualmente a una pasarela perimetral (API Gateway / reverse proxy), asumiendo un modelo de Defensa en Profundidad y arquitectura SOA para entornos de producción.  
- **Contexto / problema que resuelve:** En un entorno asistencial regulado por los artículos 25 y 26 de la LGPDPPSO, los datos de salud son sensibles. Ejecutar verificaciones criptográficas pesadas y consultas a proveedores de identidad (IdP) dentro del motor en Rust fragmentaría la cohesión del servicio y generaría sobrecarga de memoria y latencia.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Implementar validación nativa de OAuth 2.0 / JWT dentro del núcleo de MedSys-FHIR" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Implementar middleware de autenticación HTTP Basic o mTLS directo en Axum.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** El microservicio MedSys-FHIR por sí mismo es completamente inseguro si se expone fuera de su subred privada; depende al 100% de la infraestructura perimetral circundante.  
- **Limitación a declarar en la tesis:** En el banco de pruebas de laboratorio, el API Gateway NO fue instanciado; las pruebas con k6 evaluaron el microservicio directamente sin seguridad para aislar el rendimiento intrínseco. En producción asistencial, la pasarela perimetral agregará una sobrecarga de latencia criptográfica obligatoria.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6(f); Cap III §3.1.5 num. 8, §3.3.1 Tabla 5 y texto\]. Cita: "la protección perimetral, la mitigación de ataques... y la autenticación de clientes se delegan conceptualmente a un API Gateway" (\[TESIS: Cap III §3.3.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: No existe configuración de API Gateway (Kong, Envoy, Nginx) ni docker-compose que lo instancie en el repo\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 2 (diseño arquitectónico desacoplado).  
- **Conflictos detectados con otras partes:** Brecha entre la arquitectura de referencia prescrita para producción (aislamiento en subred privada) y el montaje del laboratorio (conexión directa sin pasarela).  
- **Consecuencias pendientes:** Defender ante el sínodo por qué se midió sin API Gateway y cuantificar la limitación en el presupuesto total de latencia.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Alineada con LGPDPPSO arts. 25 y 26 \[F-016\]).

---

## D-004 — El middleware no implementa OAuth2 ni inspección de JWT directamente

- **Estado:** CONGELADA  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** El middleware MedSys-FHIR no incorpora dentro de su código fuente en Rust bibliotecas de decodificación o validación de tokens JWT, negociación OAuth 2.0 ni gestión de sesiones de usuario.  
- **Contexto / problema que resuelve:** Mantener el núcleo de transformación con una huella de memoria física estrictamente contenida (RSS ≤ 150 MB) y latencias deterministas en el percentil 95 (p95 ≤ 200 ms), evitando llamadas síncronas bloqueantes a endpoints JWKS externos.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "introduciría dependencias de red de bloqueo potencial, generaría picos de latencia en el percentil 95 (p95)" (\[TESIS: Cap III §3.3.1\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Validar claims de JWT de manera local y desconectada con clave pública precargada.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Incapacidad de validar identidades de usuarios o roles dentro de la lógica de transformación de datos.  
- **Limitación a declarar en la tesis:** El middleware es un componente interno de backend que debe operar exclusivamente en redes aisladas no enrutables detrás de un proxy autenticador.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6(f); Cap III §3.1.5 num. 8, §3.3.1\]. Cita: "El middleware MedSys-FHIR no incorpora de forma nativa lógica de autenticación ni autorización" (\[TESIS: Cap III §3.1.5 num. 8\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml y .agents/backlog/ no contienen referencias a librerías JWT ni flujos OAuth2\].  
- **Objetivo o requisito al que sirve:** Cumplimiento de la hipótesis (§1.4) en cuanto a RSS ≤ 150 MB y p95 ≤ 200 ms.  
- **Conflictos detectados con otras partes:** Ninguno. Es el corolario directo de D-003.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-005 — Configuración de reglas de mapeo desacopladas en formato YAML 1.2

- **Estado:** CONGELADA  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Las reglas de correspondencia estructural, sintáctica y semántica entre el modelo relacional legado y los recursos HL7 FHIR R4 se especifican en un archivo declarativo externo en formato YAML versión 1.2 (mapping\_rules.yaml v1.1.0).  
- **Contexto / problema que resuelve:** Evitar el acoplamiento rígido de sentencias y transformaciones codificadas en duro (*hardcoded*) dentro del binario en Rust, permitiendo auditar y ajustar reglas sin recompilar el código fuente.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Mapeo codificado de forma estática en el código fuente de la aplicación" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas citadas en la tesis:* "la elección de YAML (versión 1.2) sobre JSON para definir las reglas de mapeo responde a su mayor legibilidad humana y a su capacidad nativa para incorporar comentarios" (\[TESIS: Cap III §3.4.1\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Emplear el DSL formal FHIR Mapping Language (FML) o esquemas XML.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Ligera sobrecarga de CPU/memoria durante el parseo e inicialización en arranque; expresividad limitada a transformaciones predefinidas en el motor.  
- **Limitación a declarar en la tesis:** El motor YAML no genera uniones SQL arbitrarias ni lógica imperativa compleja; solo aplica mapeos deterministas sobre consultas SQL predefinidas.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.3 Obj. 2, §1.4; Cap II §2.4.3, §2.7 Tabla 6; Cap III §3.1.3, §3.3.1 Tabla 5, §3.4.1\]. Cita: "su mayor legibilidad humana y a su capacidad nativa para incorporar comentarios explicativos mediante el carácter \#" (\[TESIS: Cap III §3.4.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 en la raíz del repositorio\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 2 e Hipótesis de investigación.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-006 — Gestión de datos incompletos sin alteración de datos clínicos (Degradación por omisión)

- **Estado:** NO VERIFICADO  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** El middleware opera bajo el principio rector: "El middleware transforma; no corrige ni completa. Nunca sustituye un dato ausente por un valor inventado". La gestión se estructura en dos niveles:  
  - **Nivel 1 (Omisión / Degradación por omisión):** Si un atributo relacional secundario u opcional es NULL en la base de datos, la clave se omite completamente del payload JSON del recurso mediante la directiva optional: true (coherente con D-025 y la regla normativa de FHIR R4 \[F-020\]).  
  - **Nivel 2 (Rechazo / Fail-Closed):** Si un identificador indispensable o juicio clínico normativo está ausente, o si un valor relacional no es reconocido por un diccionario terminológico, la transacción se interrumpe y se rechaza mediante el recurso canónico OperationOutcome (ver D-029).  
  - **Eliminación de fallback\_value:** La directiva fallback\_value se elimina completamente del diseño arquitectónico. Sustituir un valor por defecto como "provisional" ante un diagnóstico no clasificado implicaría afirmar una certeza diagnóstica que ningún médico asentó, alterando la verdad clínica del expediente.  
  - **Comportamiento en Condition.verificationStatus:** Si tbl\_diagnosticos.tipo\_diagnostico es NULL, se omite el elemento en el recurso (permitido por la cardinalidad 0..1 de FHIR R4 \[F-019\]); si contiene un valor desconocido fuera del catálogo permitido, se aplica Fail-Closed.  
  - **Reconceptualización terminológica:** El término "degradación elegante" se conserva estrictamente bajo el concepto técnico de "degradación por omisión".  
- **Contexto / problema que resuelve:** En los repositorios clínicos de primer nivel de atención existen campos nulos o incompletos. Se debía resolver el dilema bioético y técnico entre la continuidad operativa del servicio y la fidelidad documental exigida por la NOM-004-SSA3-2012 y la LGPDPPSO.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "descarta de forma categórica la invención o síntesis automatizada de variables biológicas, constantes vitales o juicios diagnósticos" (\[TESIS: Cap III §3.4.2\]).  
  - *Alternativas descartadas en revisión:* "provee un valor de respaldo determinista ante valores nulos en campos no críticos de clasificación operativa... asigna de forma determinista el código de respaldo 'provisional'" (\[TESIS: Cap III §3.4.2\]) \-\> DESCARTADA por delegación del autor al contravenir la no alteración del acto médico.  
- **Razón de la elección \[AUTOR\]:** ASESOR TÉCNICO: Delegada a asesor técnico; un middleware no puede asumir el rol médico de clasificar diagnósticos ni inventar datos clínicos; se adopta degradación por omisión.  
- **Qué se sacrifica:** Se sacrifica la emisión de recursos en tuplas relacionales que presenten inconsistencias en juicios clínicos o códigos no contemplados en diccionarios, aumentando la tasa de errores controlados (HTTP 422 con OperationOutcome).  
- **Limitación a declarar en la tesis:** El middleware rechaza registros relacionales corruptos o con códigos terminológicos fuera de norma, requiriendo que los sistemas legados corrijan la calidad del dato en su origen.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.3 Obj. 2; Cap II §2.4.3; Cap III §3.4.2 Tabla 7 y texto, §3.4.4 Tabla 8\]. Cita de la formulación previa: "ante valor nulo se aplica fallback\_value: 'provisional'" (\[TESIS: Cap III §3.4.4 Tabla 8\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 líneas 178–187 NO implementa fallback\_value; solo define dictionary para CONFIRMADO y PRESUNTIVO\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 2, Principio de proporcionalidad y calidad de la LGPDPPSO (arts. 10 y 19 \[F-015\]), y numeral 6.1.4 de NOM-004 \[F-005\].  
- **Conflictos detectados con otras partes:** CONTRADICE EVIDENCIA en el texto de la tesis actual. Cap. I §1.3 (OE2), Cap. II §2.4.3, Cap. III §3.4.2 (Tabla 7 y párrafos) y §3.4.4 (Tabla 8\) afirman que existe fallback\_value en el middleware.  
- **Consecuencias pendientes:**  
1. Registrar como pendiente de ajuste en el texto: Cap. I §1.3 (eliminar mención de fallback\_value en OE2), Cap. II §2.4.3 (eliminar fallback\_value y reformular como degradación por omisión), Cap. III §3.4.2 Tabla 7 (eliminar directiva fallback\_value), Cap. III §3.4.2 párrafos (eliminar inyección de "provisional"), Cap. III §3.4.4 Tabla 8 (actualizar política de nulidad).  
2. Verificar esquema DDL: En schema\_legado\_simulado\_nom004.sql línea 52, tipo\_diagnostico está definido como VARCHAR(20) DEFAULT 'CONFIRMADO' CHECK (tipo\_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')). Al no tener NOT NULL, en PostgreSQL un NULL explícito pasaría el check, pero por defecto siempre inserta 'CONFIRMADO'. Si se garantiza NOT NULL en el DDL, el caso nulo ni siquiera se presenta en la base de datos.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Normas de soporte verificadas en F-005, F-015 y F-020; pendiente verificar la armonización textual en Capítulos I, II y III donde persiste la mención de fallback\_value).

---

## D-007 — Marco metodológico sustentado en el paradigma Design Science Research (Hevner / Peffers)

- **Estado:** CONFIRMADA POR EL AUTOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La investigación se estructura rigurosamente bajo el paradigma de la Ciencia del Diseño (Design Science Research, DSR) formulado por Hevner et al. (2004) y operacionalizado por Peffers et al. (2007). Los umbrales de rendimiento y calidad técnica definidos en el Capítulo I §1.4 constituyen los objetivos cuantitativos del artefacto de software; las pruebas empíricas de laboratorio en contenedores Docker representan la fase formal de evaluación tecnológica del ciclo DSR; y la hipótesis de investigación formaliza el criterio riguroso de aceptación o rechazo del artefacto.  
- **Contexto / problema que resuelve:** En proyectos de desarrollo de software para tesis de ingeniería, se requiere justificar la validez científica y metodológica del desarrollo del artefacto frente a la investigación empírica tradicional que exige grupos de control experimentales.  
- **Alternativas consideradas:**  
- *Alternativas citadas en la tesis:* "no constituye un experimento factorial clásico con grupo de control, sino la evaluación sistemática... frente a criterios predefinidos" (\[TESIS: Cap III §3.1.1\]).  
- *Alternativas inferidas por el agente (no confirmadas):* Estudio cualitativo de caso clínico o investigación descriptiva documental pura.  
- **Razón de la elección \[AUTOR\]:** ASESOR TÉCNICO: Delegada a asesor técnico; alineación epistemológica y metodológica entre el método científico y la ingeniería de software bajo el paradigma DSR.  
- **Qué se sacrifica:** Se sacrifica la evaluación de impacto clínico directo sobre pacientes reales o flujos de trabajo humanos en centros de salud.  
- **Limitación a declarar en la tesis:** La evaluación empírica se restringe al entorno de laboratorio bajo condiciones controladas y simulación computacional de carga, sin medir adopción organizativa real.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.1 a §1.4; Cap II §2.7; Cap III §3.1.1 a §3.1.3\]. Cita: "se sustenta en el paradigma de la ciencia del diseño en sistemas de información (Design Science Research, DSR)" (\[TESIS: Cap III §3.1.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md, .agents/backlog/overview.md alinean el backlog con las metas de la tesis\].  
- **Objetivo o requisito al que sirve:** Marco metodológico general de la investigación.  
- **Conflictos detectados con otras partes:** Cap. III §3.1 cita a Hevner y Peffers y describe las fases generales en la Tabla 2, pero no incluye un mapeo tabular o textual explícito y exhaustivo que relacione las 6 etapas del modelo de Peffers et al. (2007) con cada sección de la tesis.  
- **Consecuencias pendientes:** Reportar la necesidad de incorporar en Cap. III §3.1 una correspondencia explícita paso a paso de las etapas de Peffers (Problema, Objetivos, Diseño/Desarrollo, Demostración, Evaluación, Comunicación).  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-008 — Protocolo de evaluación cuantitativa con k6, docker stats y HL7 Validator

- **Estado:** CONGELADA  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La contrastación de la hipótesis se basa exclusivamente en tres instrumentos formales: k6 para métricas de red bajo el método RED (latencia p95 ≤ 200 ms y tasa de éxito ≥ 99.5%), docker stats sobre el subsistema cgroups de Docker para telemetría de recursos (memoria física residente RSS ≤ 150 MB y CPU ≤ 50%), y la herramienta oficial org.hl7.fhir.validator-cli para conformidad sintáctica (100% sin incidencias Fatal ni Error).  
- **Contexto / problema que resuelve:** Asegurar la objetividad de las mediciones de rendimiento y validez del estándar evitando métricas subjetivas o herramientas intrusivas de instrumentación en código que alteren la latencia basal.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "desaconseja el empleo del promedio aritmético... adopta el percentil 95 (p95) como indicador principal" (\[TESIS: Cap II §2.6.1\]).  
  - *Alternativas citadas en la tesis:* "FHIRPath... no constituye por sí mismo un validador sintáctico integral del esquema ni de los tipos de datos" (\[TESIS: Cap II §2.6.3\]).  
  - *Alternativas citadas en la tesis:* "Se erradica por completo la ejecución nativa en el anfitrión (cargo run)" (\[TESIS: Cap III §3.6.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Uso de Apache JMeter, Locust o inspección de memoria en Rust con jemallocator.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** No se mide latencia a nivel de funciones internas individuales (profiling intrusivo / tracing distributed tipo Jaeger/OpenTelemetry); se mide la transacción HTTP de extremo a extremo.  
- **Limitación a declarar en la tesis:** docker stats bajo WSL2 reporta el consumo de memoria a nivel cgroup, incluyendo páginas activas del kernel, lo que constituye una cota superior conservadora del consumo real.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.3 Obj. 3, §1.4; Cap II §2.6.1 a §2.6.3; Cap III §3.1.2 Tabla 1, §3.1.4, §3.2.4, §3.6.2\]. Cita: "k6 es la fuente exclusiva para las métricas de red y rendimiento... docker stats es la fuente para el consumo" (\[TESIS: Cap III §3.1.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Mención en .agents/backlog/sprint\_4\_axum\_dashboard.md y validator.md; scripts no encontrados físicamente en el entorno\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 3 e Hipótesis de investigación.  
- **Conflictos detectados con otras partes:** Ausencia física de los scripts ejecutables en el árbol de archivos local disponible.  
- **Consecuencias pendientes:** Verificar y asegurar la existencia física de los scripts de prueba en el repositorio final.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Conformidad sintáctica normada por HL7 FHIR R4 verificada en F-019 y F-020).

---

## D-009 — Selección del lenguaje Rust (edición 2021\) frente a lenguajes con Garbage Collector o C/C++

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** El middleware se desarrolla en el lenguaje de sistemas Rust (edición 2021\) para garantizar seguridad de memoria estática sin recolector de basura, predictibilidad de latencia determinista y bajo consumo de memoria física en hardware austero.  
- **Contexto / problema que resuelve:** En servidores con hardware limitado (1 vCPU y menos de 1 GB de RAM disponible) en los CESSA, los entornos gestionados con recolector de basura (JVM, Go) saturan la memoria volátil o causan pausas no deterministas.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Java/C\# (pausas de GC y consumo de memoria); C/C++ (riesgo de vulnerabilidades de memoria)" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas citadas en la tesis:* "Go... depende de un recolector de basura en tiempo de ejecución" (\[TESIS: Cap II §2.5.1\]).  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Curva de aprendizaje más pronunciada para el desarrollador y mayores tiempos de compilación del código fuente.  
- **Limitación a declarar en la tesis:** El diseño experimental de la tesis no aísla estadísticamente el efecto causal de Rust frente a otros lenguajes mediante un benchmark comparativo multiproyecto.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.4, §1.5; Cap II §2.5.1, §2.5.2 Tabla 4; Cap III §3.1.2, §3.2.4 Tabla 4, §3.3.1 Tabla 5\]. Cita: "Seguridad de memoria estática sin recolector de basura, latencia determinista p95 y baja huella RSS" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md, .agents/rules/rules.md prescriben Rust 2021\].  
- **Objetivo o requisito al que sirve:** Objetivo General e Hipótesis de investigación.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-010 — Selección del crate helios-fhir (Release 4\) para el modelo canónico tipado en Rust

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Adopción del crate helios-fhir v0.2 parametrizado para la versión HL7 FHIR Release 4 como base de los tipos canónicos fuertemente tipados y la serialización JSON mediante Serde.  
- **Contexto / problema que resuelve:** Modelar las especificaciones y reglas sintácticas de FHIR R4 en Rust sin incurrir en inconsistencias tipadas ni en la sobrecarga de generar estructuras manualmente.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "fhir-sdk (expansión de macros de múltiples versiones); modelado manual de estructuras" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas citadas en la tesis:* "descartándose el modelado manual debido a la complejidad de los tipos normativos" (\[TESIS: Cap II §2.5.3\]).  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Dependencia de la evolución comunitaria del crate en crates.io y de su estabilidad de API.  
- **Limitación a declarar en la tesis:** La fidelidad del modelo canónico en memoria depende de la cobertura y corrección de las definiciones tipadas en el crate de terceros.  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.5.3 Tabla 5; Cap III §3.3.1 Tabla 5, §3.3.2\]. Cita: "Soporte modular para FHIR Release 4 vía feature flags y serialización tipada en Serde" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md Sprint 2 cita helios-fhir v0.2; .agents/backlog/sprint\_2\_helios\_fhir.md\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 3\.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Alineada con especificación normativa de FHIR R4 \[F-019, F-020\]).

---

## D-011 — Selección de PostgreSQL 16 como motor de persistencia relacional simulado

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Se utiliza PostgreSQL 16 (imagen oficial postgres:16-alpine) para albergar el esquema relacional sintético que representa a los sistemas clínicos legados.  
- **Contexto / problema que resuelve:** Proveer un entorno de persistencia robusto, reproducible y estandarizado con transacciones ACID de solo lectura representativo de los sistemas relacionales hospitalarios.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "SQLite (sin concurrencia cliente-servidor); motores comerciales privativos" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* MySQL / MariaDB, Microsoft SQL Server u Oracle Database.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Requiere adaptar conectores y sintaxis SQL en caso de interconectar el middleware con otros motores RDBMS legados (e.g., SQL Server o MySQL) en centros de salud reales.  
- **Limitación a declarar en la tesis:** La evaluación de rendimiento se acota al motor PostgreSQL 16; no se midieron variaciones de latencia con otros motores de bases de datos.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6; Cap II §2.3.2; Cap III §3.2.2 Tabla 3, §3.2.4 Tabla 4, §3.3.1 Tabla 5\]. Cita: "Motor estandarizado del modelo relacional con transacciones ACID de solo lectura" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
- **Evidencia en el repo:** \[EVIDENCIA: schema\_legado\_simulado\_nom004.sql implementa DDL y datos en sintaxis PostgreSQL\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 1 y Objetivo Específico 3\.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-012 — Aislamiento transaccional en modo REPEATABLE READ READ ONLY

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Todas las conexiones del pool de extracción SQL del middleware se configuran de manera forzada en modo transaccional REPEATABLE READ READ ONLY.  
- **Contexto / problema que resuelve:** Asegurar que si la reconstrucción de un recurso clínico requiere consultas sucesivas, estas observen una instantánea idéntica y consistente de la base de datos sin interferir con la captura clínica médica concurrente ni adquirir bloqueos exclusivos.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "no adquieren bloqueos exclusivos sobre las tablas operativas" (\[TESIS: Cap II §2.3.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Nivel de aislamiento READ COMMITTED (expone a lecturas no repetibles) o SERIALIZABLE (sobrecarga de cerrojos).  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Riesgo potencial de errores de serialización si una transacción concurrente realiza modificaciones masivas simultáneas durante la lectura.  
- **Limitación a declarar en la tesis:** La garantía transaccional asume que la base de datos subyacente soporta instantáneas MVCC eficientes sin degradación severa de memoria en el servidor relacional.  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.3.2; Cap III §3.3.1\]. Cita: "configura sus conexiones en modo transaccional REPEATABLE READ READ ONLY, garantizando una instantánea de datos consistente" (\[TESIS: Cap II §2.3.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Mención en especificaciones de persistencia en .agents/backlog/sprint\_3\_sqlx\_docker.md\].  
- **Objetivo o requisito al que sirve:** Preservar la estabilidad e integridad de los sistemas legados en operación.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-013 — Alcance funcional exclusivo de lectura por ID (GET /fhir/r4/{Resource}/{id})

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** El contrato de interfaz RESTful del middleware implementa exclusivamente la operación de lectura individual por identificador primario (read), omitiendo mutaciones (POST, PUT, DELETE), búsqueda por parámetros (\_search) y el endpoint de metadatos (/fhir/r4/metadata).  
- **Contexto / problema que resuelve:** Acotar el problema de investigación a la viabilidad de la transformación y serialización sintáctica/semántica en tiempo real, evitando construir un servidor FHIR transaccional completo con motor de búsqueda e indexación.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "el prototipo no implementa operaciones de búsqueda por parámetros (\_search), mutaciones de datos (POST, PUT, DELETE)" (\[TESIS: Cap III §3.1.5 num. 4\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Implementar búsqueda FHIR básica por identificador CURP (/Patient?identifier=...).  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Clientes externos no pueden consultar catálogos de pacientes por nombre o fecha, ni registrar o actualizar registros a través del middleware.  
- **Limitación a declarar en la tesis:** La solución no permite ingesta ni actualización de información clínica; opera estrictamente como una fachada (*façade*) de lectura.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6; Cap III §3.1.5 num. 4, §3.3.3\]. Cita: "La interacción con los recursos se restringe exclusivamente a la lectura por identificador (GET /fhir/r4/{Resource}/{id})" (\[TESIS: Cap III §3.1.5 num. 4\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md, .agents/backlog/sprint\_4\_axum\_dashboard.md solo definen endpoints GET por id\].  
- **Objetivo o requisito al que sirve:** Delimitación de alcance del Objetivo Específico 3\.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-014 — Uso exclusivo de datos clínicos sintéticos con semilla fija (20260930)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** La base de datos relacional se puebla con un conjunto de 9,000 registros sintéticos (1,000 pacientes, 2,500 consultas, 2,500 signos vitales, 3,000 diagnósticos) generados algorítmicamente en Python con semilla pseudoaleatoria fija 20260930\.  
- **Contexto / problema que resuelve:** Cumplir con la protección legal estricta de datos personales sensibles (LGPDPPSO y LFPDPPP) ante la ausencia de un convenio de colaboración técnica y confidencialidad con las autoridades del Distrito de Salud I, garantizando reproducibilidad exacta.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "El proyecto no interactúa con expedientes clínicos reales ni accede a infraestructura en producción" (\[TESIS: Cap III §3.1.5 num. 1\]).  
  - *Alternativas citadas en la tesis:* "ausencia de un convenio formal de colaboración técnica y confidencialidad suscrito entre la institución académica y las autoridades sanitarias" (\[TESIS: Cap III §3.2.1\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Anonimizar o desasociar expedientes reales de una clínica privada.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Los datos no presentan la totalidad de imperfecciones o dispersión de textos libres no estructurados de notas clínicas reales en campo.  
- **Limitación a declarar en la tesis:** Los resultados demuestran viabilidad sobre un esquema sintético controlado; no constituyen una validación sobre expedientes desestructurados en centros hospitalarios reales.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.6; Cap II §2.2.3; Cap III §3.1.5 num. 1, §3.2.1, §3.2.3\]. Cita: "Todos los experimentos se desarrollan exclusivamente mediante datos sintéticos en un ambiente de laboratorio local contenerizado" (\[TESIS: Cap III §3.1.5 num. 1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: schema\_legado\_simulado\_nom004.sql incluye datos sintéticos insertados de prueba\].  
- **Objetivo o requisito al que sirve:** Cumplimiento normativo de protección de datos (LGPDPPSO arts. 3 fracc. X, 7, 25 y 26 \[F-011, F-013, F-016\]) y Objetivo Específico 1\.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Fundamento de protección de datos personales sensibles verificado contra texto oficial \[F-011, F-013, F-016\]).

---

## D-015 — Diseño experimental sin línea base comparativa directa (Evaluación de artefacto único)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La investigación no efectúa una comparación experimental directa lado a lado contra una plataforma existente (e.g., HAPI FHIR) ni contra una variante con mapeo en código; se evalúa el comportamiento de un artefacto único frente a umbrales cuantitativos predefinidos de aceptación técnica.  
- **Contexto / problema que resuelve:** En el marco de DSR, la validación se centra en verificar si el artefacto satisface los requisitos de desempeño y conformidad especificados para resolver la problemática planteada.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "ausencia de una línea base comparativa directa (como HAPI FHIR o una variante de mapeo codificado)" (\[TESIS: Cap III §3.1.1\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Implementar dos versiones de MedSys-FHIR (una en Rust y otra en Python/Node.js) para contrastar causalmente el lenguaje.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Imposibilidad de demostrar estadísticamente superioridad causal relativa directa de MedSys-FHIR sobre HAPI FHIR en el mismo banco de pruebas.  
- **Limitación a declarar en la tesis:** Se asume como limitación metodológica explícita que los resultados certifican la viabilidad técnica del artefacto frente a criterios de aceptación, pero no prueban superioridad empírica directa sobre alternativas existentes.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.1, §3.1.5 num. 7, §3.6.1 Tabla 10\]. Cita: "circunscribiendo el análisis a verificar si el artefacto satisface los requisitos de latencia, uso de recursos y conformidad" (\[TESIS: Cap III §3.1.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: No existen suites de benchmarking comparativo contra HAPI FHIR en el repositorio\].  
- **Objetivo o requisito al que sirve:** Formulación metodológica de la Hipótesis (§1.4).  
- **Conflictos detectados con otras partes:** Ninguno; se encuentra explícitamente delimitado en el texto.  
- **Consecuencias pendientes:** Defender ante el sínodo por qué un diseño de artefacto único es válido bajo DSR.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Articulada con la evaluación de antecedentes empíricos de HAPI FHIR y Bacher et al. \[F-021, F-022\]).

---

## D-016 — Inclusión de frecuencia respiratoria en el mapeo de Observation (LOINC 9279-1, UCUM /min)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** Se incluye formalmente la frecuencia respiratoria en las reglas de transformación hacia recursos FHIR Observation (categoría vital-signs \[F-019\], código LOINC 9279-1 \[F-027\], unidad UCUM /min \[F-019\]). Razones del autor:  
  - El dato existe y se persiste en la base de datos de origen (tbl\_signos\_vitales.frecuencia\_respiratoria).  
  - Omitirlo en el mapeo implica pérdida deliberada de información clínica almacenada, lo cual contradice el principio de integridad y no alteración de D-006.  
  - El argumento previo de la tesis sobre "evitar redundancia técnica sobre un tercer escalar" es metodológicamente débil.  
  - No introduce ningún recurso FHIR nuevo; reutiliza el arquetipo escalar simple de Observation ya implementado para temperatura y frecuencia cardíaca.  
- **Contexto / problema que resuelve:** Completar el conjunto de constantes vitales de la consulta médica ambulatoria presentes en la persistencia relacional.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "delimitación de ingeniería para verificar ambos caminos estructurales del motor sin introducir redundancia funcional" (\[TESIS: Cap III §3.1.5 num. 7\]) \-\> DESCARTADA en revisión por el autor.  
  - *Alternativas citadas en la tesis:* "exclusión de la frecuencia respiratoria en la serialización FHIR de esta versión" (\[TESIS: Cap III §3.1.5 num. 7\]) \-\> DESCARTADA en revisión por el autor.  
- **Razón de la elección \[AUTOR\]:** ASESOR TÉCNICO: El dato existe y se persiste en tbl\_signos\_vitales; omitirlo vulnera el principio de no alteración y fidelidad de los datos clínicos.  
- **Qué se sacrifica:** Incrementa el volumen total de recursos Observation emitidos y validados por el validador normativo de HL7 (de 7,500 observaciones esperadas a 10,000 observaciones).  
- **Limitación a declarar en la tesis:** Numeral 6.1.2 de la NOM-004-SSA3-2012 verificado contra texto oficial del DOF (F-004), el cual lista expresamente la frecuencia respiratoria entre los signos vitales obligatorios; perfil normativo de FHIR R4 y LOINC 9279-1 con unidad UCUM /min verificados contra especificación oficial (F-019, F-020, F-027).  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.2.1; Cap III §3.1.5 num. 7, §3.2.2 Tabla 3, §3.4.4 Tabla 8\].  
- **Evidencia en el repo:** \[EVIDENCIA: schema\_legado\_simulado\_nom004.sql línea 38 define frecuencia\_respiratoria INT NULL; mapping\_rules\_specification.yaml v1.1.0 actualmente NO contiene su mapping\].  
- **Objetivo o requisito al que sirve:** Fidelidad del dato clínico e interoperabilidad completa de signos vitales.  
- **Conflictos detectados con otras partes:**  
1. mapping\_rules\_specification.yaml v1.1.0 no tiene la regla para frecuencia respiratoria.  
2. Cap. II §2.2.1 y Cap. III §3.1.5, Tabla 3 y Tabla 8 justifican la exclusión por "redundancia"; deben ser actualizados para reflejar su inclusión.  
3. Alteración del conteo de recursos: 2,500 filas de tbl\_signos\_vitales generaban 3 Observations cada una (7,500). Con frecuencia respiratoria generarán 4 Observations (10,000 Observations en total), modificando el total de recursos exportados de 14,000 a 16,500.  
- **Consecuencias pendientes:**  
- Actualizar mapping\_rules\_specification.yaml agregando el subtipo respiratory\_rate con LOINC 9279-1 (verificado en loinc.org y FHIR R4 Vital Signs Profile \[F-004, F-019, F-027\]).  
- Actualizar el texto en Cap. II §2.2.1, Cap. III §3.1.5, Tabla 3 y Tabla 8\.  
- Ajustar el conteo de recursos en el protocolo de exportación y validación.  
- Verificar numeral 6.1.2 de NOM-004 contra texto oficial adjunto: VERIFICADO (F-004).  
- **Última verificación:** Ronda 0 / 2026-10-02 (Verificado contra DOF NOM-004 numeral 6.1.2 \[F-004\] y FHIR R4 Observation Vital Signs Profile \[F-019, F-027\]).

---

## D-017 — Validación de conformidad sintáctica mediante muestreo censal exhaustivo (--all, 100% de registros)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** La validación de conformidad con org.hl7.fhir.validator-cli se ejecuta de forma censal sobre el 100% de las instancias clínicas persistidas en la base de datos (9,000 registros relacionales), descartando muestreos aleatorios reducidos.  
- **Contexto / problema que resuelve:** Evitar sesgos de representatividad estadística o pasar por alto errores sintácticos en casos borde al validar muestras pequeñas (e.g., 50 recursos frente a miles).  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "en lugar de una muestra reducida" (\[TESIS: Cap III §3.1.4\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Validar una muestra probabilística de 100 recursos.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Mayor tiempo de procesamiento y exportación de archivos JSON en disco para la fase de validación fuera de línea.  
- **Limitación a declarar en la tesis:** El muestreo censal exhaustivo valida la totalidad de los datos sintéticos generados en laboratorio, pero no sustituye la variabilidad infinita de un entorno de producción abierto.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.4, §3.6.2\]. Cita: "extrae el 100 % de las instancias persistidas mediante el parámetro \--all (muestreo censal exhaustivo)" (\[TESIS: Cap III §3.1.4\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Mención en .agents/backlog/sprint\_4\_axum\_dashboard.md y validator.md\].  
- **Objetivo o requisito al que sirve:** Validez de conclusión y contrastación de la Hipótesis (§1.4).  
- **Conflictos detectados con otras partes:** El script scripts/export\_fhir\_samples.py no existe físicamente en el entorno local.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-018 — Aislamiento estricto de pruebas de resiliencia del escenario de carga nominal sostenida

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Las peticiones orientadas a evaluar escenarios de error (IDs inexistentes HTTP 404 o parámetros malformados HTTP 400\) se ejecutan en scripts independientes (resilience\_test.js), manteniéndose estrictamente separadas de la prueba de carga nominal sostenida a 50 VU (load\_test.js).  
- **Contexto / problema que resuelve:** Prevenir que las respuestas 404 o 400 distorsionen artificialmente la tasa de error basal (http\_req\_failed) o sesguen la distribución de latencia (p95) de la operación nominal en meseta.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "sin sesgar ni distorsionar las métricas basales de disponibilidad (http\_req\_failed) ni los percentiles de latencia" (\[TESIS: Cap III §3.1.4\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Mezclar un porcentaje (e.g., 5%) de tráfico erróneo dentro de la misma batería de k6.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** La prueba de carga nominal no evalúa el comportamiento del sistema bajo ataques simultáneos de tráfico malicioso o erróneo concurrente.  
- **Limitación a declarar en la tesis:** Las métricas de latencia p95 reportan la operación exitosa sobre registros existentes; la capacidad de manejo de errores concurrentes masivos se evalúa de forma segregada.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.4\]. Cita: "se desacoplan categóricamente de la prueba nominal y se ejecutan en scripts de resiliencia independientes (resilience\_test.js)" (\[TESIS: Cap III §3.1.4\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Mención en especificaciones de testing del backlog\].  
- **Objetivo o requisito al que sirve:** Rigor metodológico en la medición de variables dependientes (Tabla 1).  
- **Conflictos detectados con otras partes:** Scripts no encontrados físicamente en el entorno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-019 — Validación de conformidad sintáctica fuera de línea con opción \-tx n/a

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** La validación con org.hl7.fhir.validator-cli se realiza desconectada de internet mediante el argumento \-tx n/a, tolerando advertencias (*Warnings*) por falta de servidor de terminologías remoto y fijando el umbral de aceptación en cero incidencias Fatal y cero Error.  
- **Contexto / problema que resuelve:** Evitar fallas de validación debidas a latencias de red, caídas o indisponibilidad de servidores ontológicos internacionales públicos de terminología FHIR durante la ejecución de pruebas de laboratorio.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "desactiva la conexión a servidores de terminología remotos" (\[TESIS: Cap III §3.6.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Desplegar un servidor de terminologías local (e.g., Ontoserver) contenerizado.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** No se valida en tiempo de ejecución si los códigos LOINC o CIE-10 están activos o vigentes en una ontología viva en red.  
- **Limitación a declarar en la tesis:** La prueba oficial certifica conformidad estructural y sintáctica de esquema; no constituye una validación semántica en tiempo real contra un servidor ontológico FHIR.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.4, §3.1.5 num. 6, §3.6.2\]. Cita: "las advertencias (Warnings) por ausencia de servidor de terminologías remoto (-tx n/a) son esperadas y no invalidan" (\[TESIS: Cap III §3.1.4\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Mención en scripts y guías de validación en backlog\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 3 e Hipótesis (§1.4).  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-020 — Pila asíncrona Tokio \+ Axum \+ SQLx (Consultas parametrizadas seguras sin ORM pesado)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** La arquitectura técnica interna se construye sobre el runtime asíncrono Tokio, el framework web ligero Axum y el cliente de base de datos SQLx con consultas SQL parametrizadas de solo lectura, descartando capas ORM pesadas.  
- **Contexto / problema que resuelve:** Manejar concurrencia masiva no bloqueante con mínimo consumo de hilos del SO y eliminar la sobrecarga de memoria de los ORMs tradicionales.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Frameworks síncronos o con intermediarios ORM pesados de abstracción excesiva" (\[TESIS: Cap III §3.3.1 Tabla 5\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Actix-web con ORM Diesel o SeaORM.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Requiere escribir y mantener consultas SQL en texto plano y manejar tipos de vida asíncronos rigurosos en Rust.  
- **Limitación a declarar en la tesis:** El acoplamiento a SQLx demanda adaptar las consultas SQL en caso de migrar a motores no compatibles.  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.4.2; Cap III §3.1.3 Tabla 2, §3.3.1 Tabla 5\]. Cita: "runtime asíncrono Tokio... framework Axum... conector SQLx para ejecutar consultas parametrizadas de solo lectura sin sobrecarga de ORM" (\[TESIS: Cap III §3.3.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md, .agents/rules/rules.md prescriben este stack\].  
- **Objetivo o requisito al que sirve:** Requisitos no funcionales de rendimiento y bajo consumo.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-021 — Panel de observabilidad técnica (React 19 \+ Tailwind CSS) restringido a diagnóstico de ingeniería

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** DIRECTO  
- **Decisión:** El panel web desarrollado en React 19 y Tailwind CSS se delimita exclusivamente como una consola técnica de supervisión de salud (/health) e inspección visual semántica de correspondencia para ingenieros y auditores (SRE/DevOps), descartando pruebas de usabilidad clínica (SUS).  
- **Contexto / problema que resuelve:** Proporcionar visibilidad operativa del middleware sin incurrir en desviación de alcance (*scope creep*) hacia el desarrollo de un Expediente Clínico Electrónico de punto de atención.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "descarta justificadamente la aplicación de pruebas de usabilidad clínica o instrumentos de interacción humana —tales como cuestionarios SUS—" (\[TESIS: Cap III §3.5\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Evaluar el dashboard con médicos mediante cuestionarios TAM o SUS.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** No se genera evidencia sobre la experiencia de usuario de personal médico asistencial.  
- **Limitación a declarar en la tesis:** La interfaz de observabilidad no es un software médico ni apto para consulta clínica directa con pacientes.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.3 Tabla 2, §3.5\]. Cita: "consola técnica de soporte diagnóstico para perfiles de ingeniería de software, investigación y administración de sistemas" (\[TESIS: Cap III §3.5\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Carpeta dashboard/ en workspace; mención en STATE.md\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 3 (panel de observabilidad técnica).  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-022 — Confinamiento estricto de recursos en Docker cgroups v2 (1.0 vCPU y 256 MB RAM) bajo WSL2

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** El middleware se evalúa confinado obligatoriamente dentro de un contenedor Docker restringido a 1.0 vCPU (cpus: 1.0, cuota CFS 100,000 µs / 100,000 µs) y 256 MB de RAM (mem\_limit: 256m) sobre WSL2, prohibiendo la ejecución nativa en el anfitrión.  
- **Contexto / problema que resuelve:** Emular de forma realista el techo de capacidad de servidores austeros de centros de salud periféricos (CESSA) operando sobre una laptop anfitriona moderna (AMD Ryzen 7 5700U de 8 núcleos/16 hilos y 13.8 GB RAM).  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Se erradica por completo la ejecución nativa en el anfitrión (cargo run)" (\[TESIS: Cap III §3.6.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Desplegar en un servidor bare-metal físico antiguo real.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** WSL2 y Docker introducen una ligera sobrecarga de virtualización en llamadas al sistema de red local.  
- **Limitación a declarar en la tesis:** Amenaza a la validez externa y de constructo: el planificador CFS restringe la cuota de CPU, pero no neutraliza las ventajas microarquitectónicas del anfitrión (bus DDR4 de doble canal y caché L3 compartida de 8 MB).  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.2.4 Tabla 4, §3.6.1 Tabla 10, §3.6.2\]. Cita: "aplicó un confinamiento forzado mediante el subsistema de grupos de control de Linux (cgroups v2) integrado en Docker" (\[TESIS: Cap III §3.6.1\]).  
- **Evidencia en el repo:** \[EVIDENCIA: Especificaciones de despliegue en backlog\].  
- **Objetivo o requisito al que sirve:** Validez interna y de constructo del diseño experimental.  
- **Conflictos detectados con otras partes:** Ninguno; limitación metodológica explícitamente reconocida en §3.6.1.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-023 — Semántica de validación estricta al arranque (*Fail-Fast*) con panic\! ante YAML inválido

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** El motor de mapeo valida exhaustivamente el archivo mapping\_rules.yaml durante la fase de inicialización en arranque; si detecta errores de sintaxis, tipos incompatibles o claves no reconocidas, aborta inmediatamente emitiendo un pánico controlado (panic\!).  
- **Contexto / problema que resuelve:** Prevenir que el middleware entre en servicio en un estado de configuración indeterminado o corrupto que provoque fallos silenciosos o generación de recursos malformados durante la atención de solicitudes.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "si el archivo YAML contiene inconsistencias... aborta su inicio emitiendo un pánico controlado (panic\!)" (\[TESIS: Cap III §3.4.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Carga perezosa con degradación o ignorar directivas no reconocidas con advertencias en log.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** El servicio no arranca si hay un error menor en el YAML; requiere intervención del administrador.  
- **Limitación a declarar en la tesis:** El arranque requiere un archivo YAML 100% válido y verificado previamente.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.4.2\]. Cita: "El motor aplica una semántica de validación estricta al arranque (Fail-Fast)" (\[TESIS: Cap III §3.4.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: .agents/rules/rules.md instruye validación al arranque\].  
- **Objetivo o requisito al que sirve:** Estabilidad y confiabilidad operativa.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-024 — Modelado de la identificación nacional mediante CURP con OID oficial de RENAPO

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** DIRECTO  
- **Decisión:** La identificación oficial del paciente se estructura en Patient.identifier\[0\] asignando la clave CURP de 18 caracteres bajo el sistema formal OID urn:oid:2.16.840.1.113883.4.629 (Registro Nacional de Población, RENAPO).  
- **Contexto / problema que resuelve:** Garantizar la interoperabilidad nacional unívoca del paciente conforme a las directrices de salud digital en México y el catálogo de OID de la DGIS.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* NO CITABLE.  
  - *Alternativas inferidas por el agente (no confirmadas):* Usar URLs locales de la clínica o el número de seguridad social (NSS) como identificador principal.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** La CURP debe existir y cumplir con la sintaxis de 18 caracteres; registros sin CURP no pueden ser procesados como identificador nacional oficial.  
- **Limitación a declarar en la tesis:** La prueba verifica la sintaxis de 18 caracteres; no consulta en línea el padrón de RENAPO.  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.1.4; Cap III §3.2.2 Tabla 3, §3.4.3, §3.4.4 Tablas 8 y 9\]. Cita: "asociando el valor de la Clave Única de Registro de Población (CURP) al espacio de nombres oficial del RENAPO" (\[TESIS: Cap II §2.1.4\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 línea 21: system: "urn:oid:2.16.840.1.113883.4.629"\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 1 y apego a la normatividad técnica nacional (DGIS OID catalogue e Instructivo Normativo CURP \[F-027\]; corregir atribución indebida a NOM-004 detectada en AUD-R0-009 / F-002).  
- **Conflictos detectados con otras partes:** Discrepancia en Cap III §3.2.2 que atribuye la CURP al numeral 5.2.2 de la NOM-004-SSA3-2012 (ver AUD-R0-009).  
- **Consecuencias pendientes:** Corregir en la tesis la referencia normativa de la CURP, vinculándola a la NOM-024 numeral 6.5.1 y al catálogo OID de la DGIS.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Verificado contra catálogo oficial de OIDs de DGIS y DOF Instructivo CURP \[F-027\]).

---

## D-025 — Atributo telefono\_contacto tratado como telecomunicación opcional (optional: true)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** DIRECTO  
- **Decisión:** La columna relacional telefono\_contacto se mapea a telecom\[0\].value como un dato secundario opcional mediante la directiva optional: true, instruyendo al serializador a omitir completamente la clave en el JSON si el valor en la base de datos es NULL.  
- **Contexto / problema que resuelve:** Respetar la especificación normativa de HL7 FHIR Release 4 que prohíbe emitir atributos explícitos con valor nulo ("telecom": null) o cadenas vacías (verificado en F-020), y apegarse al principio de proporcionalidad y minimización de la LGPDPPSO (arts. 10 y 19 \[F-015\]).  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "prohíbe emitir atributos explícitos con valor nulo o cadenas vacías" (\[TESIS: Cap III §3.4.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Exigir teléfono como obligatorio o colocar cadenas dummy tipo "Sin teléfono".  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Ningún sacrificio clínico; se evita emitir JSON inválido ante pacientes sin número telefónico registrado.  
- **Limitación a declarar en la tesis:** Ninguna.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.1.5 num. 7, §3.2.2, §3.4.2, §3.4.3\]. Cita: "telefono\_contacto como un dato de telecomunicación secundario opcional... en apego al principio de proporcionalidad y minimización" (\[TESIS: Cap III §3.2.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 líneas 37–40: optional: true\].  
- **Objetivo o requisito al que sirve:** Conformidad sintáctica FHIR y cumplimiento de LGPDPPSO.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Verificado contra regla normativa de FHIR R4 \[F-020\] y LGPDPPSO arts. 10 y 19 \[F-015\]).

---

## D-026 — Fusión determinista de apellidos en name\[0\].family con supresión de espacios residuales

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** DIRECTO  
- **Decisión:** Los campos apellido\_paterno y apellido\_materno de tbl\_pacientes se concatenan en un único elemento estructurado name\[0\].family mediante combine\_with y el formato "{apellido\_paterno} {apellido\_materno}". Si el materno es NULL, se asigna exclusivamente el paterno suprimiendo espacios en blanco residuales.  
- **Contexto / problema que resuelve:** Representar fielmente el sistema de doble apellido tradicional mexicano e hispanoamericano dentro de la estructura estándar de FHIR R4 sin generar espacios en blanco que provoquen observaciones en validación sintáctica.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* NO CITABLE.  
  - *Alternativas inferidas por el agente (no confirmadas):* Mapear solo el apellido paterno o usar extensiones no estándar para separar apellidos.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Se fusionan los dos apellidos en una sola cadena en lugar de diferenciarlos en extensiones FHIR específicas.  
- **Limitación a declarar en la tesis:** En sistemas receptores que requieran estricta distinción entre paterno y materno, se requeriría una extensión FHIR específica para México.  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.4.2, §3.4.3, §3.4.4 Tabla 8\]. Cita: "concatena los apellidos paterno y materno en el atributo estructurado name\[0\].family... suprimiendo espacios en blanco residuales" (\[TESIS: Cap III §3.4.3\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 líneas 27–30\].  
- **Objetivo o requisito al que sirve:** Identificación del paciente conforme a la NOM-004 numeral 5.2.3 \[F-001\].  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02.

---

## D-027 — Transformación atómica 1:1 de tuplas de tbl\_diagnosticos a recursos independientes Condition

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** DIRECTO  
- **Decisión:** Cada fila extraída de la tabla relacional tbl\_diagnosticos se transforma de manera atómica e independiente en un recurso Condition autónomo (relación 1:1 fila-a-recurso), prescindiendo de estructuras de arreglos dinámicos anidados en memoria.  
- **Contexto / problema que resuelve:** Optimizar el acceso y construcción en memoria en tiempo constante O(1), preservando la granularidad y naturaleza atómica de los recursos clínicos de FHIR R4.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "Esta correspondencia directa hace innecesario el uso de arreglos dinámicos anidados para la multiplicidad diagnóstica" (\[TESIS: Cap III §3.4.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Agrupar todos los diagnósticos de una consulta dentro de una única estructura o Bundle.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Si un paciente tiene múltiples diagnósticos en una consulta, el cliente debe recuperar cada recurso Condition por su identificador individual.  
- **Limitación a declarar en la tesis:** El prototipo evalúa la lectura por identificador de recurso; no implementa endpoints de búsqueda agrupada por consulta (/Condition?encounter=...).  
- **Evidencia en la tesis:** \[TESIS: Cap III §3.4.2, §3.4.4 Tabla 8\]. Cita: "la relación entre las tuplas... y los recursos FHIR es de naturaleza estrictamente fila-a-recurso (1:1)" (\[TESIS: Cap III §3.4.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: mapping\_rules\_specification.yaml v1.1.0 líneas 169–172: primary\_key: "id\_diagnostico"\].  
- **Objetivo o requisito al que sirve:** Eficiencia computacional y simplicidad arquitectónica.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Alineada con especificación de Condition en FHIR R4 \[F-019\]).

---

## D-028 — Adopción canónica de OperationOutcome para todos los códigos HTTP de error (400, 404, 422, 500\)

- **Estado:** BORRADOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** UNA PREGUNTA  
- **Decisión:** Todas las respuestas de fallo del middleware (HTTP 400 Bad Request, 404 Not Found, 422 Unprocessable Entity y 500 Internal Server Error) retornan obligatoriamente el recurso canónico OperationOutcome serializado en JSON con sus niveles de severidad y código de diagnóstico normativo.  
- **Contexto / problema que resuelve:** Asegurar la consistencia estricta del contrato de interfaz RESTful de HL7 FHIR, garantizando que los clientes interoperables puedan interpretar sintáctica y semánticamente el motivo de la falla en formato nativo del estándar.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* NO CITABLE.  
  - *Alternativas inferidas por el agente (no confirmadas):* Retornar errores en formato RFC 7807 (Problem Details) o texto plano.  
- **Razón de la elección \[AUTOR\]:** PENDIENTE.  
- **Qué se sacrifica:** Ligero incremento en el tamaño de la carga útil del mensaje de error en comparación con respuestas simples en texto plano.  
- **Limitación a declarar en la tesis:** Ninguna; constituye una mejor práctica internacional en FHIR RESTful APIs.  
- **Evidencia en la tesis:** \[TESIS: Cap II §2.1.4; Cap III §3.1.4, §3.3.3\]. Cita: "Para el tratamiento de fallos, el middleware adopta una política de errores homogénea basada en el recurso canónico OperationOutcome" (\[TESIS: Cap III §3.3.3\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md §1 num. 4; .agents/backlog/sprint\_4\_axum\_dashboard.md\].  
- **Objetivo o requisito al que sirve:** Resiliencia y apego a la especificación oficial de HL7 FHIR R4.  
- **Conflictos detectados con otras partes:** Ninguno.  
- **Consecuencias pendientes:** Ninguna.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Alineada con especificación de OperationOutcome en FHIR R4 \[F-019\]).

---

## D-029 — Política Fail-Closed ante identificadores o juicios clínicos indispensables ausentes o no mapeables

- **Estado:** CONFIRMADA POR EL AUTOR  
- **Confirmación del autor:** CONFIRMADA  
- **Nivel:** COMPLETA  
- **Decisión:** El middleware aplica una política estricta de fallo seguro (*Fail-Closed*): ante la ausencia de un identificador normativo indispensable (e.g., CURP) o un juicio clínico obligatorio (e.g., código CIE-10), o cuando un valor relacional de la base de datos no es reconocido por los diccionarios de traducción terminológica, la transacción se interrumpe y se rechaza emitiendo un recurso OperationOutcome con severidad error o fatal (HTTP 422), previniendo la emisión de historiales sintéticamente falseados.  
- **Contexto / problema que resuelve:** La disyuntiva entre mantener el servicio respondiendo datos incompletos/falseados o proteger la fidelidad jurídica y bioética del expediente clínico electrónico.  
- **Alternativas consideradas:**  
  - *Alternativas citadas en la tesis:* "previniendo la emisión de historiales sintéticamente falseados" (\[TESIS: Cap I §1.1.2\]).  
  - *Alternativas inferidas por el agente (no confirmadas):* Degradación con cadenas vacías, valores aleatorios o emitir recursos FHIR incompletos que violen la cardinalidad mínima del estándar.  
- **Razón de la elección \[AUTOR\]:** ASESOR TÉCNICO: Delegada a asesor técnico; un sistema de salud no puede asumir datos clínicos inventados ni tolerar inconsistencias no mapeables; se aplica rechazo controlado Fail-Closed.  
- **Qué se sacrifica:** Disponibilidad de lectura en registros que presenten datos relacionales corruptos o códigos fuera de catálogo.  
- **Limitación a declarar en la tesis:** La política Fail-Closed exige una adecuada calidad del dato en los repositorios relacionales legados; registros corruptos no son reparados automáticamente por el middleware.  
- **Evidencia en la tesis:** \[TESIS: Cap I §1.1.2, §1.3 OE2; Cap II §2.1.4, §2.4.3; Cap III §3.1.4, §3.3.3\]. Cita: "una política estricta de fallo seguro (Fail-Closed) que rechace transacciones cuando falten identificadores o juicios clínicos indispensables" (\[TESIS: Cap I §1.1.2\]).  
- **Evidencia en el repo:** \[EVIDENCIA: STATE.md, .agents/rules/rules.md prescriben manejo estructurado con OperationOutcome\].  
- **Objetivo o requisito al que sirve:** Objetivo Específico 2 y Principio de Fidelidad Clínica.  
- **Conflictos detectados con otras partes:** En versiones previas del Capítulo III y Capítulo II, la política Fail-Closed coexistía con la directiva fallback\_value en diagnósticos; al eliminarse fallback\_value (D-006), Fail-Closed asume el control estricto de cualquier valor fuera de catálogo.  
- **Consecuencias pendientes:** Armonizar Cap. II §2.4.3 y Cap. III §3.4.2 con esta definición formal unificada.  
- **Última verificación:** Ronda 0 / 2026-10-02 (Respaldada por NOM-004 numerales 5.10 y 6.1.4 \[F-003, F-005\] y LGPDPPSO arts. 10 y 19 \[F-015\]).