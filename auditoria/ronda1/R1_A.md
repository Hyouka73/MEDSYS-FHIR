=== INICIO R1_A ===

1. ARCHIVOS:
| Archivo / Recurso | Tipo / Origen | Leído (SÍ/NO) | Observaciones |
|---|---|:---:|---|
| `TESIS9N/01_Manuscrito_Capitulos/Cap1_v2` | Drive Document | SÍ | Leído completo; planteamiento, pregunta, objetivos, hipótesis, justificación y alcance |
| `TESIS9N/01_Manuscrito_Capitulos/Cap2_v2` | Drive Document | SÍ | Leído completo; marco conceptual, normativo y tecnología |
| `TESIS9N/01_Manuscrito_Capitulos/Cap3_v2` | Drive Document | SÍ | Leído completo; metodología DSR, diseño experimental y métricas |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/DECISIONES.md` | Drive Document | SÍ | Leído vía registro; decisiones D-001 a D-029 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/REGISTRO_AUDITORIAS.md` | Drive Document | SÍ | Leído vía registro; auditorías históricas Ronda 0 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/FUENTES_Y_NORMATIVA.md` | Drive Document | SÍ | Leído vía registro; fichas F-001 a F-029 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/DEFENSA.md` | Drive Document | SÍ | Leído vía registro; fichas argumentales |
| `TESIS9N/01_Manuscrito_Capitulos/Rondas/R1_F.md` | Drive Document | SÍ | Leído completo; auditoría normativa de Ronda 1 |
| `TESIS9N/02_Planificacion_y_Gestion/Ficha_Proyecto_Integrador_9N_MedSys-FHIR` | Drive Document | SÍ | Leído completo; planteamiento institucional |
| `TESIS9N/02_Planificacion_y_Gestion/Cronograma_Ajustado_y_Plan_de_Trabajo_16_Semanas` | Drive Document | SÍ | Leído completo; matriz RACI y fases |
| `TESIS9N/02_Planificacion_y_Gestion/mapping_rules_specification.yaml` | Drive Document | SÍ | Leído completo; especificación YAML |
| `TESIS9N/02_Planificacion_y_Gestion/schema_legado_simulado_nom004.sql` | Drive Document | SÍ | Leído completo; DDL relacional sintético |
| `MEDSYS-FHIR-main/` | Repositorio GitHub | NO | No requerido para Pasada A (hilo lógico y metodología) |
| `TESIS9N/01_Manuscrito_Capitulos/unused/*` | Drive Files | NO | Ignorado por directriz (versiones obsoletas) |
| `TESIS9N/01_Manuscrito_Capitulos/Cap3_v2_original_sin_depurar` | Drive Document | NO | Ignorado por directriz (versión previa no depurada) |


2. DECISIÓN AUTOMÁTICA:
RONDA: 1 (confirmada; la carpeta `Rondas/` contiene únicamente `R1_F.md` y `REGISTRO_AUDITORIAS.md` contiene la Ronda 0).
MODO: Auditoría COMPLETA de la Pasada A sobre la coherencia lógica de los capítulos vigentes (`Cap1_v2`, `Cap2_v2`, `Cap3_v2`).
REGLAMENTO: Sin reglamento de titulación en Drive; no se audita formato de estilo institucional.
ENTRADAS: `Cap1_v2`, `Cap2_v2`, `Cap3_v2`, `DECISIONES.md`, `REGISTRO_AUDITORIAS.md`, `FUENTES_Y_NORMATIVA.md`, `R1_F.md`, `Ficha_Proyecto_Integrador_9N_MedSys-FHIR`.


3. RESUMEN:
Total de hallazgos nuevos reportados: 6 (0 CRÍTICOS, 3 MAYORES, 3 MODERADOS).
Bloqueantes pre-Cap. 4: 1 hallazgo mayor bloqueante por ausencia de operacionalización de la política Fail-Closed en la hipótesis.
Veredicto de área: SEGUIR (ronda 1; requiere subsanar inconsistencias en el hilo lógico y variables metodológicas antes de registrar mediciones en Cap. IV).
4. SEGUIMIENTO DE HALLAZGOS PREVIOS DE ESTA PASADA:
| ID | Estado verificado hoy | Dictamen | Evidencia |
|---|---|:---:|---|
| AUD-R0-001 | Mapeo `fallback_value` en Cap. I, II y III en colisión con decisión D-006 | NO CERRADO | En `Cap1_v2` (§1.3, OE2) persiste la directiva textual `fallback_value`, contraviniendo D-006. |
| AUD-R0-002 | Formalización de política Fail-Closed en toma de decisiones | CERRADO | La decisión D-029 está documentada en `DECISIONES.md` y articulada conceptualmente en `Cap1_v2` (§1.1.2). |
5. HALLAZGOS NUEVOS:

---

## ID: AUD-R1-A-001
SEVERIDAD: MAYOR      BLOQUEANTE PRE-CAP. 4: SÍ
UBICACIÓN: Cap1_v2 §1.3 (OE2); Cap1_v2 §1.4; Cap3_v2 §3.1.2
HALLAZGO: La hipótesis de investigación no operacionaliza métricas ni umbrales cuantitativos de aceptación para el Objetivo Específico 2 respecto al comportamiento Fail-Closed y la emisión de OperationOutcome ante registros anómalos.
CITA: "logrando una tasa de conformidad sintáctica del 100 % de los recursos evaluados sin incidencias de severidad Fatal ni Error"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: La incorporación formal en la hipótesis (§1.4) y en la Tabla 1 de Cap. III de un umbral verificable de rechazo e integridad (e.g. 100 % de emisiones `OperationOutcome` con HTTP 422 ante transacciones con datos mandatorios faltantes).
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-029
ACCIÓN CONCRETA: Añadir a la formulación de la hipótesis (§1.4) y a la Tabla 1 de operacionalización metodológica (§3.1.2) el umbral de aceptación cuantitativo para el flujo de fallo seguro (Fail-Closed) mediante `OperationOutcome`.

## ID: AUD-R1-A-002
SEVERIDAD: MAYOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.1.2; Cap1_v2 §1.4; Cap3_v2 §3.1.2
HALLAZGO: Existe una contradicción no articulada en la caracterización de los recursos de hardware: el problema describe equipos con 1-2 núcleos y menos de 1 GB libre para procesos auxiliares, pero la hipótesis y el experimento fijan el límite en 1 vCPU y 256 MB de RAM sin justificarlo como cota de estrés inferior.
CITA: "entornos de 1 o 2 núcleos virtuales y márgenes de memoria RAM disponible inferiores a 1 GB para procesos auxiliares"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: Una cláusula metodológica explícita que justifique que el contenedor Docker se limitó a 256 MB y 1 vCPU deliberadamente como escenario de estrés extremo de borde frente al margen disponible real.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-003
ACCIÓN CONCRETA: Armonizar la redacción en §1.1.2 y §1.4 explicitando que el límite de 256 MB en Docker constituye una prueba de estrés de borde más exigente que la cota de 1 GB observada en el hardware de campo.

## ID: AUD-R1-A-003
SEVERIDAD: MODERADO      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.1.2; Cap1_v2 §1.2; Cap1_v2 §1.4
HALLAZGO: La clave CURP y la correspondencia con el identificador oficial nacional se exigen taxativamente en la hipótesis de investigación (§1.4) y en las variables de Cap. III, sin haber sido introducidas ni problematizadas en el planteamiento ni en la pregunta de investigación.
CITA: "realizando la asignación de códigos terminológicos (LOINC y CIE-10) y del identificador oficial nacional (CURP) conforme a las reglas declarativas"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: Que el planteamiento del problema (§1.1.2) o la pregunta central (§1.2) fundamenten la resolución unívoca de la identidad del paciente mediante la clave CURP bajo el marco legal mexicano.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Integrar en el planteamiento del problema (§1.1.2) y acotar en la pregunta central (§1.2) el requisito de resolución del identificador nacional de pacientes (CURP).

## ID: AUD-R1-A-004
SEVERIDAD: MODERADO      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.2; Cap1_v2 §1.3 (OE3); Cap3_v2 §3.6
HALLAZGO: El panel de observabilidad técnica y el endpoint `/health` forman parte integral del Objetivo Específico 3 sin derivar del problema de investigación ni de la pregunta central, distorsionando el foco de evaluación del middleware.
CITA: "integrando un panel de observabilidad técnica con propósito exclusivo de supervisión operativa del servicio (/health)"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: Que el desarrollo de componentes de interfaz visual o telemetría esté fundamentado en la pregunta de investigación o en el planteamiento del problema.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-012
ACCIÓN CONCRETA: Subordinar la redacción del panel en OE3 (§1.3) a un papel estrictamente instrumental de soporte experimental para la toma de métricas, evitando situarlo como un entregable clínico de interoperabilidad.

## ID: AUD-R1-A-005
SEVERIDAD: MAYOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap3_v2 §3.1.1; Cap3_v2 §3.7
HALLAZGO: La fundamentación en Design Science Research omite estructurar formalmente las seis etapas del modelo de Peffers et al. (2007) y clasificar las cuatro categorías canónicas de amenazas a la validez conforme a Wohlin et al. (2012).
CITA: "se enmarca epistemológicamente en el paradigma de la ciencia del diseño en sistemas de información (Design Science Research, DSR), formulado por Hevner et al. (2004) y metodológicamente estructurado por Peffers et al. (2007)"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: La inclusión en Cap. III de una tabla de correspondencia entre las fases del proceso DSR y la tesis, acompañada de un desglose formal de las amenazas interna, externa, de constructo y de conclusión.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Incorporar en §3.1.1 una matriz de trazabilidad entre las 6 actividades DSR (Peffers et al., 2007) y los capítulos del informe, y desglosar formalmente en §3.7 las cuatro amenazas a la validez según Wohlin et al. (2012).

## ID: AUD-R1-A-006
SEVERIDAD: MODERADO      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.3 (OE1); Cap3_v2 §3.2.1
HALLAZGO: El Objetivo Específico 1 compromete un análisis normativo y del estado de adopción tomando como caso de estudio el Distrito de Salud I, pero el manuscrito no reporta instrumentos de recolección de campo (encuestas, entrevistas o visitas técnicas) en dicha jurisdicción.
CITA: "tomando como caso de estudio contextual la red de atención del Distrito de Salud I en Tuxtla Gutiérrez, Chiapas"
EVIDENCIA: VERIFICADA
QUÉ LO REFUTARÍA: La presentación en Cap. III de instrumentos metodológicos aplicados directamente en las clínicas del Distrito de Salud I.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-001
ACCIÓN CONCRETA: Ajustar la redacción de OE1 (§1.3) y §3.2.1 precisando que el Distrito de Salud I funge como un arquetipo contextual referencial de análisis documental, y no como un caso de estudio con intervención empírica in situ.

6. SECCIONES ESPECÍFICAS DE ESTA PASADA:

A. MATRIZ OBJETIVO - HIPÓTESIS - CRITERIO DE EVALUACIÓN

| Problema abordado | Objetivo | Hipótesis vinculada | Criterio de aceptación técnica | Instrumento / Dónde se define |
| --- | --- | --- | --- | --- |
| Falta de adopción normativa en el primer nivel asistencial | OE1: Analizar marco normativo y adopción en Distrito de Salud I | No operacionalizada en hipótesis empírica | Revisión documental y diagnóstico del marco NOM-004/NOM-024 | Análisis documental (§1.1, §2.2) [HUECO: sin métrica de contraste] |
| Inviabilidad de migración de BD y rigidez de mapeo manual | OE2: Diseñar arquitectura de mediación con reglas YAML desacopladas | Hipótesis General (parcial): Reglas declarativas y tipado en Rust | Transformación sin alterar BD legada; degradación estructural | `mapping_rules.yaml` / Cap3_v2 §3.4 [HUECO: Fail-Closed no medido en hipótesis] |
| Sobrecarga de cómputo y desajuste sintáctico en borde asistencial | OE3: Implementar y evaluar prototipo funcional para 4 recursos FHIR | Hipótesis General: Viabilidad técnica, conformidad y rendimiento | 1) Conformidad sintáctica: 100 % (0 Fatal/Error)<br>

<br>2) Éxito HTTP $\ge 99.5\,\%$<br>

<br>3) Latencia: $p95 \le 200\text{ ms}$ (50 VU)<br>

<br>4) Memoria: $\text{RSS} \le 150\text{ MB}$<br>

<br>5) CPU: $\le 50\,\%$ de 1 vCPU | 1) `org.hl7.fhir.validator-cli` (OpenJDK 21)<br>

<br>2) k6 (`http_req_failed`)<br>

<br>3) k6 (`http_req_duration`)<br>

<br>4) Docker daemon (`docker stats`)<br>

<br>5) Docker daemon (`docker stats`)<br>

<br>Definido en §1.4 y Tabla 1 (§3.1.2) |
| Carencia de mediador ligero para consulta externa ambulatoria | OG: Desarrollar e investigar middleware FHIR R4 en Rust para esquema legado | Hipótesis General íntegra | Cumplimiento simultáneo de los 5 umbrales cuantitativos de aceptación | Suite k6, HL7 Validator y Docker stats en entorno local (§1.4) |

B. ANÁLISIS DEL HILO CONDUCTOR

* Eslabón Problema $\rightarrow$ Pregunta: Coherente. El problema plantea la inviabilidad de migrar bases relacionales en el primer nivel y la pesadez de plataformas como HAPI FHIR, lo que conduce a preguntar si una solución ligera en Rust con reglas declarativas es viable.
* Eslabón Pregunta $\rightarrow$ Objetivo General: Coherente. El objetivo general formaliza el desarrollo y la evaluación de dicha viabilidad en laboratorio.
* Eslabón Objetivos Específicos $\rightarrow$ Hipótesis: Discontinuo. La hipótesis concentra exclusivamente la evaluación experimental del prototipo (OE3), omitiendo la evaluación empírica del comportamiento de rechazo (Fail-Closed) formulado en OE2.
* Eslabón Hipótesis $\rightarrow$ Metodología: Coherente en herramientas, pero con divergencia en la caracterización de los límites de memoria de hardware (1 GB en problema vs 256 MB en banco de pruebas).

C. EVALUACIÓN DE ELEMENTOS NO DERIVADOS

* Clave CURP: Aparece de forma repentina en la hipótesis (§1.4) como entregable normativo sin haberse establecido previamente como variable clave en la formulación del problema (§1.1.2) ni en la pregunta (§1.2).
* Panel de Observabilidad y endpoint `/health`: Aparecen incorporados en OE3 (§1.3). Su desarrollo en React 18 no responde a la pregunta de investigación central del middleware de interoperabilidad y debe ser catalogado como soporte instrumental de medición.

D. FALSIBILIDAD Y OPERACIONALIZACIÓN DE LA HIPÓTESIS

* La hipótesis cumple con el criterio de falsabilidad de Popper: define cinco umbrales numéricos prefijados que determinan su rechazo si cualquiera de ellos falla.
* Deficiencia identificada: La hipótesis está sesgada al camino feliz (éxito HTTP $\ge 99.5\,\%$, recursos válidos). No incorpora una condición de falsabilidad para evaluar si el sistema falla de manera segura (Fail-Closed) emitiendo `OperationOutcome` cuando los datos clínicos requeridos son inconsistentes.

E. CONSISTENCIA DE DELIMITACIONES Y HARDWARE

* Se detecta inconsistencia entre §1.1.2 (equipos con márgenes de RAM inferiores a 1 GB) y §1.4 / §3.1.2 (contenedor limitado a 1 vCPU y 256 MB). Debe aclararse que el contenedor utiliza 256 MB como límite superior de prueba de estrés, no como la memoria total del servidor físico.
* Delimitación de alcance consistente: En §1.6 se declara de manera nítida que la investigación abarca solo 4 recursos FHIR en PostgreSQL 16 local con datos sintéticos, excluyendo despliegues en producción, pacientes reales y PKI.

F. EVALUACIÓN DE LA DELIMITACIÓN SIN API GATEWAY

* En §1.6 se delimita y fundamenta con solidez que los mecanismos perimetrales (OAuth 2.0, inspección de JWT) se delegan a un API Gateway conforme a los principios de Arquitectura Orientada a Servicios (SOA) y responsabilidad única. Esta delimitación es técnicamente válida y protege la validez interna del experimento al aislar el consumo intrínseco del middleware.

G. AMENAZAS A LA VALIDEZ (Wohlin et al., 2012)

* Validez Interna: Parcialmente abordada. El entorno local en Docker aísla variables externas de red, pero la falta de aleatorización en los tiempos de llegada de solicitudes en k6 podría inducir sesgos de concurrencia artificial.
* Validez Externa: Crítica. El uso de un esquema sintético homogéneo no garantiza el comportamiento frente a esquemas relacionales reales altamente desnormalizados o con corrupción histórica de datos.
* Validez de Constructo: Débil. Evaluar la interoperabilidad estructural mediante el validador con bandera `-tx n/a` (modo desconectado de terminologías) restringe la verificación a la sintaxis del metamodelo, omitiendo la validación semántica ontológica de códigos LOINC/CIE-10.
* Validez de Conclusión: Limitada. No se contemplan repeticiones con cálculo de varianza o intervalos de confianza estadísticos, basándose en percentiles brutos de una única corrida de carga sostenida.

H. CORRESPONDENCIA CON EL PROCESO DSR (Peffers et al., 2007)

1. Identificación del problema y motivación: Desarrollada en Cap. I (§1.1).
2. Definición de objetivos de una solución: Formulada en Cap. I (§1.3 y §1.4).
3. Diseño y desarrollo: Detallada en Cap. II y Cap. III (§3.3, §3.4, §3.5).
4. Demostración: Planteada como el prototipo en Docker (Cap. IV).
5. Evaluación: Diseñada mediante k6, HL7 Validator y Docker stats (Cap. IV).
6. Comunicación: Materializada en el manuscrito de tesis (Caps. I a V).

* Observación: Aunque las 6 actividades existen en el proyecto, en §3.1.1 no están formalizadas en una estructura metodológica explícita.

YA REPORTADO (no duplicado):

* AUD-R0-001 | Persistencia de la directiva `fallback_value` en Cap1_v2 §1.3 (OE2) en conflicto con D-006.
* AUD-R1-F-001 | Anacronismo regulatorio en Cap1_v2 y Cap2_v2 sobre la obligatoriedad de FHIR en la NOM-024 (reportado en Pasada F).
* AUD-R1-F-007 | Limitación del validador con parámetro `-tx n/a` sin servidor terminológico vivo (reportado en Pasada F).

DERIVADO A OTRA PASADA:

* Citas en texto desalineadas con la bibliografía en Cap. I (Pasada F: Fuentes y Normativa).
* Verificación de la tasa de fallas y scripts k6 frente a la meta de latencia (Pasada E: Experimentos y Cifras).
* Auditoría de implementación física de endpoints y tipado en Rust (Pasada B: Arquitectura y Código).

CONFLICTOS CON DECISIONES CONGELADAS Y BRECHAS DE DEFENSA:

* Conflicto con D-006 (CONGELADA): Cap1_v2 (§1.3 OE2) mantiene textualmente la directiva `fallback_value` pese a su eliminación técnica formal.
* Brecha de defensa en D-001 y D-002: Las justificaciones de exclusión de PKI y selección de los 4 recursos continúan marcadas como `[PENDIENTE]` en la matriz de decisiones.

INFORMACIÓN QUE FALTA:

* Datos consolidados de las pruebas de laboratorio de k6 y validación FHIR para verificar la factibilidad de los capítulos IV y V.

COBERTURA DE LECTURA:

* Lectura completa de: `Cap1_v2`, `Cap2_v2`, `Cap3_v2`, `Ficha_Proyecto_Integrador_9N_MedSys-FHIR`, `Cronograma_Ajustado_y_Plan_de_Trabajo_16_Semanas`, `mapping_rules_specification.yaml`, `schema_legado_simulado_nom004.sql` y `R1_F.md`.
* Cobertura de la Pasada A: 100 % del marco lógico y metodológico analizado.

VEREDICTO DE ÁREA:
SEGUIR
=== FIN R1_A ===