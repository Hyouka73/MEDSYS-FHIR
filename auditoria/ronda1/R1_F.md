=== INICIO R1_F ===

1. ARCHIVOS:
| Archivo / Recurso | Tipo / Origen | Leído (SÍ/NO) | Observaciones |
|---|---|:---:|---|
| `TESIS9N/01_Manuscrito_Capitulos/Cap1_v2` | Drive Document | SÍ | Leído completo; antecedentes, problema y justificación |
| `TESIS9N/01_Manuscrito_Capitulos/Cap2_v2` | Drive Document | SÍ | Leído completo; estándares, marco normativo y tecnología |
| `TESIS9N/01_Manuscrito_Capitulos/Cap3_v2` | Drive Document | SÍ | Leído completo; metodología DSR, diseño e instrumentación |
| `TESIS9N/01_Manuscrito_Capitulos/00_Guia_Estructura_General_Tesis` | Drive Document | SÍ | Leído completo; índice y directrices de capítulos |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/DECISIONES.md` | Drive Document | SÍ | Leído completo; inventario D-001 a D-029 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/DEFENSA.md` | Drive Document | SÍ | Leído completo; simulador y fichas de defensa |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/FUENTES_Y_NORMATIVA.md` | Drive Document | SÍ | Leído completo; fichas F-001 a F-029 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/REGISTRO_AUDITORIAS.md` | Drive Document | SÍ | Leído completo; registro histórico Ronda 0 |
| `TESIS9N/01_Manuscrito_Capitulos/auditoria/EVIDENCIA_REPO.md` | Drive Document | SÍ | Leído completo; evidencia empírica del workspace |
| `TESIS9N/01_Manuscrito_Capitulos/Rondas/` | Drive Folder | SÍ | Inspeccionada; carpeta vacía (0 archivos previos) |
| `TESIS9N/03_Fuentes_y_Referencias/Banco_de_Fuentes_y_Notas_Bibliograficas_HL7_FHIR` | Drive Document | SÍ | Leído completo; resúmenes bibliográficos |
| `TESIS9N/02_Planificacion_y_Gestion/Cronograma_Ajustado_y_Plan_de_Trabajo_16_Semanas` | Drive Document | SÍ | Leído completo; RACI y planeación temporal |
| `TESIS9N/02_Planificacion_y_Gestion/Ficha_Proyecto_Integrador_9N_MedSys-FHIR` | Drive Document | SÍ | Leído completo; planteamiento institucional |
| `TESIS9N/02_Planificacion_y_Gestion/mapping_rules_specification.yaml` | Drive Document | SÍ | Leído completo; reglas de transformación R4 |
| `TESIS9N/02_Planificacion_y_Gestion/schema_legado_simulado_nom004.sql` | Drive Document | SÍ | Leído completo; esquema relacional DDL/DML |
| `TESIS9N/04_Antecedentes_y_Respaldos/8N_Proyecto2_EvalFinal_ECE_DistritoSaludI.docx` | Drive Document | SÍ | Leído completo; antecedente de 8° semestre |
| `TESIS9N/04_Antecedentes_y_Respaldos/Ficha_Proyecto_MedSys-FHIR_v3.md` | Drive Document | SÍ | Leído completo; ficha técnica preliminar |
| `MEDSYS-FHIR-main/` (Workspace Cargo y scripts de prueba) | Repositorio GitHub | SÍ | Árbol y archivos clave inspeccionados |
| `TESIS9N/01_Manuscrito_Capitulos/unused/*` | Drive Files | NO | Ignorado por directriz (versiones obsoletas) |
| `TESIS9N/01_Manuscrito_Capitulos/Cap3_v2_original_sin_depurar` | Drive Document | NO | Ignorado por directriz (versión no depurada) |


2. DECISIÓN AUTOMÁTICA:
RONDA: 1 (confirmada; la carpeta `Rondas/` está vacía y `REGISTRO_AUDITORIAS.md` contiene únicamente la Ronda 0 preparatoria).
MODO: Auditoría COMPLETA de la Pasada F sobre los capítulos vigentes (Cap1_v2, Cap2_v2, Cap3_v2) y sus soportes normativos.
REGLAMENTO: Sin reglamento de titulación en Drive; no se audita formato de estilo institucional, solo consistencia interna y rigor factual.
ENTRADAS: `Cap1_v2`, `Cap2_v2`, `Cap3_v2`, `FUENTES_Y_NORMATIVA.md`, `DECISIONES.md`, `REGISTRO_AUDITORIAS.md`, `DEFENSA.md`, `Banco_de_Fuentes_y_Notas_Bibliograficas_HL7_FHIR`.
3. RESUMEN:
Total de hallazgos nuevos reportados: 7 (1 CRÍTICO, 2 MAYORES, 2 MODERADOS, 2 MENORES).
Bloqueantes pre-Cap. 4: 1 hallazgo crítico por anacronismo regulatorio.
Veredicto de área: SEGUIR (trabajo de saneamiento y alineación bibliográfica indispensable antes de consolidar resultados experimentales).
4. SEGUIMIENTO DE HALLAZGOS PREVIOS DE ESTA PASADA:
| ID | Estado verificado hoy | Dictamen | Evidencia |
|---|---|:---:|---|
| AUD-R0-001 | Mapeo `fallback_value` en Cap. I, II y III en colisión con decisión D-006 | NO CERRADO | Los textos de Cap1_v2 (§1.3), Cap2_v2 (§2.4.3) y Cap3_v2 (§3.4.4) mantienen la mención a valores por sustitución en vez de reflejar la eliminación de la directiva acordada en Ronda 0. |
| AUD-R0-002 | Formalización de política Fail-Closed en toma de decisiones | CERRADO | La decisión D-029 está formalizada en `DECISIONES.md` y documentada operativamente en `Cap1_v2` y `Cap3_v2` con emisión de `OperationOutcome` 422. |
| AUD-R0-004 | Inconsistencia de frecuencia respiratoria LOINC 9279-1 vs texto | NO CERRADO | En `Cap3_v2` (Tabla 8) y `mapping_rules_specification.yaml` se incluyó la regla 3C, pero `FUENTES_Y_NORMATIVA.md` mantiene cruces erróneos con F-027. |
5. HALLAZGOS NUEVOS:

---

## ID: AUD-R1-F-001
SEVERIDAD: CRÍTICO      BLOQUEANTE PRE-CAP. 4: SÍ
UBICACIÓN: Cap1_v2 §1.1.1, §1.2, §1.5; Cap2_v2 §2.2.2
HALLAZGO: La tesis afirma que la NOM-024-SSA3-2012 exige la adopción del estándar HL7 FHIR R4, incurriendo en un anacronismo regulatorio grave. La NOM-024 fue expedida en noviembre de 2012 y sus guías técnicas normativas se basan en HL7 v2.x y HL7 CDA v3; la especificación HL7 FHIR Release 4 fue oficializada a finales de 2018, por lo que la norma no puede exigir ni contener dicho estándar.
CITA: "NOM-024-SSA3-2012 obliga a los Sistemas de Información de Registro Electrónico para la Salud (SIRES) a intercambiar información bajo estándares internacionales como HL7"
EVIDENCIA: VERIFICADA (Diario Oficial de la Federación, 30/11/2012; HL7 International FHIR R4 Specification, 2019).
QUÉ LO REFUTARÍA: Exhibición de una actualización publicada en el DOF de la NOM-024 posterior a 2019 que adopte explícitamente FHIR R4 como anexo normativo vinculante.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Reformular los Capítulos I y II precisando que la NOM-024 exige interoperabilidad genérica sustentada históricamente en HL7 v2/v3, y que FHIR R4 constituye una evolución técnica internacional contemporánea orientada a subsanar las rigideces de dichos esquemas legados, no un mandato textual de la norma de 2012.

## ID: AUD-R1-F-002
SEVERIDAD: MAYOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap3_v2 §3.2.2; Cap1_v2 §1.1.2; Cap2_v2 §2.2.1
HALLAZGO: Se cita el numeral 5.2.2 de la NOM-004-SSA3-2012 como fundamento de los datos demográficos y la clave CURP del paciente. En el texto oficial de la norma, el numeral 5.2.2 se refiere exclusivamente a la razón y denominación social del propietario o concesionario; los datos mínimos del paciente están en el numeral 5.2.3, y la obligatoriedad de la CURP proviene de las disposiciones de DGIS y de la Ley General de Población.
CITA: "concentra los datos de filiación y la CURP en apego al numeral 5.2.2 de la NOM-004-SSA3-2012"
EVIDENCIA: VERIFICADA (Diario Oficial de la Federación, 15/10/2012, Norma Oficial Mexicana NOM-004-SSA3-2012).
QUÉ LO REFUTARÍA: Copia íntegra del texto del DOF donde el subnumeral 5.2.2 mencione explícitamente al paciente y su CURP.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Corregir las citas en Cap3_v2 (§3.2.2) y Cap2_v2 para referenciar el numeral 5.2.3 para datos del paciente, y fundamentar la CURP en las directivas de interoperabilidad del Catálogo de la DGIS y el registro de RENAPO (OID 2.16.840.1.113883.4.629).

## ID: AUD-R1-F-003
SEVERIDAD: MAYOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.1.1, §1.5; Cap2_v2 §2.5.1
HALLAZGO: La tesis sostiene que las soluciones existentes en el ecosistema Java (notoriamente HAPI FHIR) imponen un consumo de memoria de varios gigabytes inaceptable para unidades periféricas, pero no aporta una fuente bibliográfica, reporte técnico ni benchmark comparativo empírico que lo respalde objetivamente (F-021).
CITA: "el consumo de memoria y la sobrecarga computacional de los motores de ejecución existentes restringen la respuesta del servicio en centros periféricos"
EVIDENCIA: NO VERIFICADA (Ficha F-021 sin sustento empírico o experimental directo).
QUÉ LO REFUTARÍA: Incorporación de una cita académica indexada o prueba controlada de perfilado de memoria donde se registre la huella de una pila HAPI FHIR JPA superando 1 GB en un caso de uso equivalente.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-015
ACCIÓN CONCRETA: Modificar la redacción para fundamentar la selección de Rust en las características intrínsecas del lenguaje (cero costo de abstracción, ausencia de recolector de basura, huella determinista) y catalogar la comparación de huella frente a la JVM como una motivación de diseño observada en literatura general de sistemas embebidos, no como un hecho empíricamente medido en esta tesis.

## ID: AUD-R1-F-004
SEVERIDAD: MODERADO      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 (Citas vs Lista de Referencias); Cap2_v2 (Bibliografía)
HALLAZGO: Existen discordancias entre las citas en el cuerpo de los capítulos y las referencias bibliográficas finales: se detectan citas en texto que carecen de entrada en la bibliografía (Beyer, 2016; Wilkie, 2017 en Cap. I), y referencias completas registradas en la bibliografía final que nunca son invocadas en el texto (e.g. Asprilla et al.).
CITA: "Beyer (2016) y Wilkie (2017) señalan que la confiabilidad del servicio..."
EVIDENCIA: CONTRADICTORIA (Cruce directo entre cuerpo del manuscrito y lista de referencias).
QUÉ LO REFUTARÍA: Demostración de que las referencias constan formalmente en la sección final de fuentes y están enlazadas a sus párrafos respectivos.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Realizar una auditoría cruzada en Cap1_v2 y Cap2_v2 para dar de alta en la bibliografía a todos los autores citados en el texto y eliminar de la lista bibliográfica aquellas fuentes que no hayan sido invocadas directamente.

## ID: AUD-R1-F-005
SEVERIDAD: MODERADO      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: `DECISIONES.md` (Fichas D-006 y D-016); `FUENTES_Y_NORMATIVA.md`
HALLAZGO: La matriz de decisiones arquitectónicas enlaza fichas normativas equivocadas o impertinentes. La decisión D-016 sobre frecuencia respiratoria (LOINC 9279-1) cita la ficha F-027 (que trata sobre la asignación del OID de RENAPO a la CURP), mientras que D-006 cita fichas que no abordan la cardinalidad opcional 0..1 del elemento `verificationStatus` en el recurso Condition de FHIR R4.
CITA: "D-016 [...] Ficha Normativa: F-004, F-019, F-027"
EVIDENCIA: CONTRADICTORIA (Inspección textual de `DECISIONES.md` frente a `FUENTES_Y_NORMATIVA.md`).
QUÉ LO REFUTARÍA: Verificación de que la ficha F-027 contenga la especificación de signos vitales de LOINC y que las fichas de D-006 aborden formalmente `Condition.verificationStatus`.
RESOLUCIÓN PROBABLE: CORREGIR_DOC
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-006, D-016
ACCIÓN CONCRETA: Actualizar la tabla maestra de `DECISIONES.md` asignando a D-016 exclusivamente la ficha F-004 (signos vitales LOINC) y la especificación oficial FHIR R4 (F-019), y crear o enlazar en D-006 la ficha formal de la estructura del recurso Condition de HL7 FHIR R4.

## ID: AUD-R1-F-006
SEVERIDAD: MENOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap1_v2 §1.1.1; Cap3_v2 §3.2.1
HALLAZGO: La fundamentación del despliegue del expediente digital y la fragmentación hospitalaria en Chiapas descansa en reportajes de prensa (Excélsior / Rodríguez Calva, 2026) y portales web no oficiales (SaludDigital.com, 2025), careciendo de enlace directo a las fuentes primarias de la Secretaría de Salud federal o de los comunicados oficiales del programa IMSS-Bienestar.
CITA: "en abril de 2026 se inició el despliegue del expediente digital de IMSS-Bienestar en 44 hospitales del estado de Chiapas (Rodríguez Calva, 2026)"
EVIDENCIA: PARCIAL (Fuentes secundarias periodísticas sin enlace institucional normativo).
QUÉ LO REFUTARÍA: Presentación del boletín oficial de prensa del IMSS / Secretaría de Salud o publicación en el portal gubernamental de Chiapas donde se formalice dicho despliegue.
RESOLUCIÓN PROBABLE: VERIFICAR_FUENTE
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: ninguna
ACCIÓN CONCRETA: Sustituir o complementar las citas periodísticas con comunicados oficiales de la Coordinación de Salud Digital del IMSS-Bienestar o decretos de transferencia de servicios de salud en el estado de Chiapas.

## ID: AUD-R1-F-007
SEVERIDAD: MENOR      BLOQUEANTE PRE-CAP. 4: NO
UBICACIÓN: Cap3_v2 §3.1.2, §3.7.1; `scripts/validate_fhir.ps1`
HALLAZGO: La afirmación de cumplimiento del 100% en la validación sintáctica mediante `org.hl7.fhir.validator-cli` omite documentar la limitación que impone el parámetro de ejecución fuera de línea (`-tx n/a`), el cual suprime la verificación de terminologías vinculadas en servidores externos (terminología LOINC/UCUM) y emite advertencias estructurales que deben quedar formalmente delimitadas.
CITA: "logrando una tasa de conformidad sintáctica del 100 % de los recursos evaluados sin incidencias de severidad Fatal ni Error"
EVIDENCIA: VERIFICADA (HL7 Validator CLI documentation; parámetros `-tx n/a` en script experimental).
QUÉ LO REFUTARÍA: Demostración de que la herramienta `validator-cli` en modo `-tx n/a` descarga y valida automáticamente el 100% de los value sets ontológicos sin omitir comprobaciones semánticas remotas.
RESOLUCIÓN PROBABLE: DECLARAR_LIMITACIÓN
DESTINO: AGENTE_TESIS
DEPENDENCIA REAL: ninguna
DECISIÓN RELACIONADA: D-019
ACCIÓN CONCRETA: Añadir en la Sección 3.1.2 y en las amenazas a la validez (§3.7) la aclaración explícita de que la conformidad sintáctica del 100% se evalúa sobre la estructura e invariantes del metamodelo R4, asumiendo como limitación de laboratorio la desconexión del servidor terminológico en vivo.

6. SECCIONES ESPECÍFICAS DE ESTA PASADA:

A. TABLA MAESTRA DE FUENTES Y NORMATIVA

| Fuente / Documento | Dónde se cita | Estado de Evidencia | URL y Fecha de Consulta | Veredicto Ficha | Acción Requerida |
| --- | --- | --- | --- | --- | --- |
| NOM-004-SSA3-2012 (Del expediente clínico) | Cap. I §1.1; Cap. II §2.2.1; Cap. III §3.2.2 | VERIFICADA | `[https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)` (2026-10-02) | SIRVE | Corregir subnumeral citado: cambiar 5.2.2 por 5.2.3 en demográficos del paciente. |
| NOM-024-SSA3-2012 (Sistemas de Información SIRES) | Cap. I §1.1, §1.2; Cap. II §2.2.2; Cap. III §3.1.2 | VERIFICADA | `[https://www.dof.gob.mx/nota_detalle.php?codigo=5280848](https://www.dof.gob.mx/nota_detalle.php?codigo=5280848)` (2026-10-02) | SIRVE | Aclarar que la norma establece interoperabilidad genérica y HL7 v2/v3, no FHIR R4. |
| HL7 FHIR Release 4 (v4.0.1) | Cap. I §1.1; Cap. II §2.1.3; Cap. III §3.4.1 | VERIFICADA | `[https://hl7.org/fhir/R4/](https://hl7.org/fhir/R4/)` (2026-10-02) | SIRVE | Mantener como especificación canónica destino. |
| Peffers et al. (2007) / Hevner et al. (2004) | Cap. III §3.1.1 | VERIFICADA | `[https://doi.org/10.2753/MIS0742-1222240302](https://doi.org/10.2753/MIS0742-1222240302)` (2026-10-02) | SIRVE | Mantener; fundamenta la metodología DSR adoptada. |
| Bender & Sartipi (2013) | Cap. I §1.1.1; Cap. II §2.1 | VERIFICADA | `[https://doi.org/10.1145/2537052.2537061](https://doi.org/10.1145/2537052.2537061)` (2026-10-02) | SIRVE | Mantener como antecedente primario de servicios REST en salud. |
| Mandel et al. (2016) (SMART on FHIR) | Cap. I §1.1.1; Cap. II §2.1.3 | VERIFICADA | `[https://doi.org/10.1093/jamia/ocv189](https://doi.org/10.1093/jamia/ocv189)` (2026-10-02) | SIRVE | Mantener; valida el desacoplamiento mediante interfaces web. |
| Wagholikar et al. (2019) (CAMP FHIR) | Cap. I §1.1.1 | VERIFICADA | `[https://doi.org/10.1093/jamiaopen/ooz022](https://doi.org/10.1093/jamiaopen/ooz022)` (2026-10-02) | SIRVE | Mantener; referencia directa para traducción de modelos tabulares. |
| Bacher et al. (2024) (OpenMRS FHIR) | Cap. I §1.1.1 | VERIFICADA | `[https://doi.org/10.1007/s10916-024-02058-2](https://doi.org/10.1007/s10916-024-02058-2)` (2026-10-02) | SIRVE | Mantener como antecedente en entornos con recursos limitados. |
| DGIS — Catálogo SIRES (2026) | Cap. I §1.1.1 | VERIFICADA | `[https://www.gob.mx/salud/acciones-y-programas/sires](https://www.gob.mx/salud/acciones-y-programas/sires)` (2026-10-02) | SIRVE | Contrastar cifra reportada (38 certificados) contra listado oficial actualizado. |
| RENAPO (Identificador CURP OID) | Cap. III §3.2.2; YAML rules | VERIFICADA | `[http://oid-info.com/get/2.16.840.1.113883.4.629](http://oid-info.com/get/2.16.840.1.113883.4.629)` (2026-10-02) | SIRVE | Asignar ficha F-027 en lugar de cruzarla con LOINC. |
| LOINC (RegRules 85354-9, 8310-5, 8867-4, 9279-1) | Cap. II §2.1.4; Cap. III §3.4.3 | VERIFICADA | `[https://loinc.org](https://loinc.org)` (2026-10-02) | SIRVE | Asegurar que 9279-1 (frecuencia respiratoria) conste formalmente en F-004. |
| CIE-10 (Clasificación Internacional OMS) | Cap. II §2.1.4; Cap. III §3.4.4 | VERIFICADA | `[https://icd.who.int/browse10/2019/en](https://icd.who.int/browse10/2019/en)` (2026-10-02) | SIRVE | Mantener para mapeo de condiciones diagnósticas. |
| Rodríguez Calva (Excélsior, 2026) | Cap. I §1.1.1 | PARCIAL | Noticia de prensa sobre IMSS Chiapas (2026-04) | DESACTUALIZADA | Sustituir o respaldar con informe institucional de IMSS-Bienestar. |
| HAPI FHIR Resource Consumption Claims | Cap. I §1.1.1; Cap. II §2.5.1 | NO VERIFICADA | Sin referencia empírica directa (2026-10-02) | NO SIRVE | Retirar afirmación de "varios GB" o etiquetarla como hipótesis motivadora. |

B. CLASIFICACIÓN DE FICHAS DE `FUENTES_Y_NORMATIVA.md`

* SIRVE (Verificadas y plenamente aplicables): F-001 (Datos generales), F-003 (Principio de inalterabilidad), F-007 (Delimitación PKI), F-011 (Generación de datos sintéticos), F-013 (Aislamiento transaccional), F-014 (Esquema relacional sintético), F-016 (Seguridad perimetral), F-019 (Especificación FHIR R4), F-020 (Herramienta validator-cli), F-027 (OID oficial CURP).
* SIRVE PERO FALTA VERIFICAR / CORREGIR ATRIBUCIÓN: F-002 (Corregir numeral de 5.2.2 a 5.2.3), F-004 (Añadir formalmente LOINC 9279-1 de frecuencia respiratoria), F-008 (NOM-024: retirar afirmación de coincidencia con FHIR R4), F-015 (Detallar cardinalidad 0..1 en Condition).
* DUPLICADA: Ninguna en el inventario actual.
* NO SIRVE / IRRELEVANTE: F-021 (Aseveración de HAPI FHIR de varios GB: sin sustento documental; debe ser reformulada o retirada como ficha de fuente).
* DESACTUALIZADA: Fichas basadas exclusivamente en reportajes de prensa de 2025/2026 respecto al ECE en Chiapas que no cuentan con soporte oficial gubernamental.

C. HUECOS Y FUENTES CANDIDATAS

* Hueco 1: Fundamentación técnica del bajo consumo y seguridad de memoria de Rust en servicios web de salud.
* *Fuente candidata:* Matsakis, N. D., & Klock, F. S. (2014). The Rust language. *ACM SIGAda Ada Letters*, 34(3), 103-104. URL: `[https://doi.org/10.1145/2692956.2663188](https://doi.org/10.1145/2692956.2663188)` (Consultado: 2026-10-05). [candidata]


* Hueco 2: Justificación del patrón Data Mapper / Declarativo frente al código imperativo en arquitecturas de mediación clínica.
* *Fuente candidata:* Fowler, M. (2002). *Patterns of Enterprise Application Architecture*. Addison-Wesley Professional. (Consultado: 2026-10-05). [candidata]


* Hueco 3: Situación real de infraestructura tecnológica en Centros de Salud de Primer Nivel en el sureste mexicano.
* *Fuente candidata:* Consejo Nacional de Evaluación de la Política de Desarrollo Social (CONEVAL). (2023). *Evaluación Integral de la Política de Salud Social en México*. URL: `[https://www.coneval.org.mx](https://www.coneval.org.mx)` (Consultado: 2026-10-05). [candidata]



D. CONSISTENCIA DE FORMATO DE REFERENCIAS

* Se observa heterogeneidad en el estilo de citado a lo largo del texto: mientras Cap1_v2 y Cap2_v2 emplean el sistema autor-año estilo APA 7ma edición, en algunas tablas de Cap3_v2 se mezclan identificadores numéricos directos o referencias a leyes sin citar el año de expedición formal.
* En Cap1_v2 se intercalan citas textuales sin número de página ni localización exacta del párrafo.
* Se recomienda unificar internamente bajo formato autor-año estricto (APA 7) en todas las secciones, asegurando que cada cita incluya su correspondiente entrada completa en la bibliografía de cada capítulo.

YA REPORTADO (no duplicado):

* AUD-R0-001 | Persistencia de menciones a `fallback_value` en Cap. I, II y III sin armonizar con D-006.
* AUD-R0-004 | Omisión de armonización de la regla de frecuencia respiratoria LOINC 9279-1 en los cruces de `FUENTES_Y_NORMATIVA.md`.

DERIVADO A OTRA PASADA:

* Ausencia de implementación y pruebas automatizadas en el árbol físico para varios componentes metodológicos descritos en tiempo presente (Pasada C — Código y Consistencia).
* Discrepancia en la tasa de fallas y tiempos de latencia bajo estrés concurrente sin datos brutos consolidados de k6 (Pasada E — Experimentos y Cifras).
* Justificación pendiente de la delimitación de 4 recursos ambulatorios en respuestas ante el sínodo (Pasada D — Defensa).

CONFLICTOS CON DECISIONES CONGELADAS Y BRECHAS DE DEFENSA:

* Conflicto con D-006 (CONGELADA): Cap1_v2 (§1.3 OE2) y Cap2_v2 (§2.4.3) conservan párrafos donde se defiende la sustitución determinista con `fallback_value`, violentando la decisión congelada de degradación por omisión y Fail-Closed.
* Brecha de defensa en D-001 y D-002: Los campos de razón de elección del autor continúan figurando como `[PENDIENTE]` en `DECISIONES.md` y `DEFENSA.md`, dejando desprotegida la argumentación ante el sínodo sobre por qué se omitió PKI y la totalidad de los 140 recursos restantes de FHIR R4.

INFORMACIÓN QUE FALTA:

* Documento formal de la Dirección General de Información en Salud (DGIS) correspondiente a la versión 2026 del Catálogo SIRES para constatar el estatus de las 38 aplicaciones certificadas.
* Reportes de auditoría y actas de examen profesional previas para contrastar los criterios de evaluación del sínodo en la FCA C-I UNACH.

COBERTURA DE LECTURA:

* Lectura completa, exhaustiva y sección por sección de: `Cap1_v2`, `Cap2_v2`, `Cap3_v2`, `DECISIONES.md`, `FUENTES_Y_NORMATIVA.md`, `REGISTRO_AUDITORIAS.md`, `DEFENSA.md`, `Banco_de_Fuentes_y_Notas_Bibliograficas_HL7_FHIR`, `mapping_rules_specification.yaml` y `schema_legado_simulado_nom004.sql`.
* Cobertura de la Pasada F: 100% del material documental y normativo vigente analizado.

VEREDICTO DE ÁREA:
SEGUIR
=== FIN R1_F ===