# INVENTARIO Y VERIFICACIÓN DE FUENTES Y NORMATIVA (FUENTES\_Y\_NORMATIVA.md) — MedSys-FHIR

Este archivo inventaría toda afirmación presente en los Capítulos I, II y III de la tesis que invoque una norma oficial, ley, artículo, numeral, estándar internacional o fuente externa con carácter de obligación, prohibición, alcance o dato empírico clave.

---

## 1\. Reglas de Gestión del Inventario

1. **Regla Antifabricación de Textos Normativos:** El campo *Texto textual de la fuente* solo puede llenarse con transcripciones literales extraídas de fuentes oficiales (Diario Oficial de la Federación, Cámara de Diputados, Secretaría de Salud / DGIS, HL7, LOINC y sitios oficiales de editores académicos). Queda estrictamente prohibido redactarlo de memoria o inferirlo.  
2. **Estados de Verificación:**  
   - **VERIFICADA CONTRA TEXTO OFICIAL:** La afirmación ha sido contrastada directamente contra el texto del documento oficial publicado y se constata coincidencia material o paráfrasis fiel.  
   - **DISCREPANCIA:** El numeral, artículo o fracción citado no existe, dice otra cosa, no contiene el dato atribuido o la tesis incurre en anacronismo o atribución errónea.  
   - **NO VERIFICADA — SIN ACCESO:** El documento oficial no pudo ser localizado o consultado en repositorios oficiales, documentando la justificación de los intentos realizados.  
   - **FUENTE SECUNDARIA:** Solo se localizó una fuente secundaria (blog, resumen, reportaje de prensa o artículo de terceros no oficial), manteniéndose la ficha como NO VERIFICADA.  
3. **Criterios de Auditoría:** Para cada ficha se registra la URL oficial exacta, fecha de consulta (2026-10-02), versión del documento (fecha de publicación o última reforma), naturaleza de la disposición (OBLIGACIÓN / PROHIBICIÓN / RECOMENDACIÓN / DEFINICIÓN / DATO EMPÍRICO), sujeto obligado y análisis de ámbito y vigencia temporal.

---

## 2\. Inventario de Fichas Normativas y Fuentes Externas

### F-001 — NOM-004-SSA3-2012: Datos generales mínimos del expediente clínico

- **Afirmación en la tesis:** "todo expediente clínico debe integrar datos generales mínimos que identifiquen al establecimiento y al paciente, entre ellos nombre completo, sexo y fecha de nacimiento o edad" (\[TESIS: Cap II §2.2.1\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012; entró en vigor a los 60 días naturales posteriores (Norma Oficial Mexicana vigente, sin modificaciones posteriores).  
- **Localización exacta en la fuente:** Numeral 5.2 (y subnumerales 5.2.1 a 5.2.4).  
- **Texto textual de la fuente:**

>   
> "5.2 Todo expediente clínico, deberá tener los siguientes datos generales: 5.2.1 Tipo, nombre y domicilio del establecimiento y en su caso, nombre de la institución a la que pertenece; 5.2.2 En su caso, la razón y denominación social del propietario o concesionario; 5.2.3 Nombre, sexo, edad y domicilio del paciente; y 5.2.4 Los demás que señalen las disposiciones sanitarias."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Personal del área de la salud y establecimientos prestadores de servicios de atención médica de los sectores público, social y privado, incluidos los consultorios (numeral 2).  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (la tesis modela datos de consulta externa en un establecimiento de salud público en México bajo la norma vigente de 2012).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-002 — NOM-004-SSA3-2012: Identificación demográfica del paciente

- **Afirmación en la tesis:** "concentra los datos de filiación y la CURP en apego al numeral 5.2.2 de la NOM-004-SSA3-2012" (\[TESIS: Cap III §3.2.2\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 5.2.2 (en contraste con 5.2.3).  
- **Texto textual de la fuente:**

>   
> "5.2.2 En su caso, la razón y denominación social del propietario o concesionario;" "5.2.3 Nombre, sexo, edad y domicilio del paciente; y"  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Integración de expedientes clínicos en unidades de atención médica en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ en cuanto a materia asistencial, pero con error de cita y atribución normativa.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS ERRÓNEA.  
- **Estado:** DISCREPANCIA.  
- **Análisis de discrepancia:**  
1. El numeral 5.2.2 citado por la tesis regula exclusivamente la razón y denominación social del propietario o concesionario del establecimiento médico. Los datos del paciente (nombre, sexo, edad y domicilio) corresponden al numeral 5.2.3.  
2. La NOM-004-SSA3-2012 no contempla ni menciona la Clave Única de Registro de Población (CURP) en el numeral 5.2.2 ni en ningún otro numeral de su texto normativo oficial. La obligatoriedad de la CURP como identificador único en registros electrónicos en salud proviene de la NOM-024-SSA3-2012, numeral 6.5.1.

---

### F-003 — NOM-004-SSA3-2012: Requisitos formales de notas médicas y firma

- **Afirmación en la tesis:** "las notas médicas deben asentar fecha, hora, nombre completo y firma autógrafa, electrónica o digital de quien las elabora, sujetándose estas dos últimas a las disposiciones aplicables" (\[TESIS: Cap II §2.2.1\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 5.10.  
- **Texto textual de la fuente:**

>   
> "5.10 Todas las notas en el expediente clínico deberán contener fecha, hora y nombre completo de quien la elabora, así como la firma autógrafa, electrónica o digital, en su caso; estas dos últimas se sujetarán a las disposiciones jurídicas aplicables."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Personal médico y profesionales de salud que elaboran notas médicas en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-004 — NOM-004-SSA3-2012: Registro obligatorio de constantes vitales en consulta externa

- **Afirmación en la tesis:** "El numeral 6.1.2 señala la obligación de registrar la exploración física y las constantes fisiológicas (tensión arterial, frecuencia cardíaca, frecuencia respiratoria y temperatura)" (\[TESIS: Cap II §2.2.1\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 6.1.2.  
- **Texto textual de la fuente:**

>   
> "6.1.2 Exploración física.- Deberá tener como mínimo: habitus exterior, signos vitales (temperatura, tensión arterial, frecuencia cardiaca y respiratoria), peso y talla, así como, datos de cabeza, cuello, tórax, abdomen, miembros y genitales o específicamente la información que corresponda a la materia del odontólogo, psicólogo, nutriólogo y otros profesionales de la salud; y"  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Personal médico y profesionales del área de la salud que elaboran historias clínicas y notas en consulta general y de especialidad.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ. Se comprueba que el texto oficial lista textualmente la frecuencia respiratoria entre los signos vitales mínimos ("signos vitales (temperatura, tensión arterial, frecuencia cardiaca y respiratoria)").  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-005 — NOM-004-SSA3-2012: Diagnósticos clínicos y problemas de salud

- **Afirmación en la tesis:** "el numeral 6.1.4 regula el registro de los diagnósticos clínicos o problemas de salud identificados" (\[TESIS: Cap II §2.2.1\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 6.1.4.  
- **Texto textual de la fuente:**

>   
> "6.1.4 Diagnósticos o problemas clínicos;"  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Elaboración de historias clínicas en consulta general y de especialidad en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-006 — NOM-004-SSA3-2012: Ámbito general de obligatoriedad jurídica

- **Afirmación en la tesis:** "la NOM-004-SSA3-2012 es obligatoria para todo establecimiento de atención médica del país" (\[TESIS: Cap I §1.5\]).  
- **Fuente citada:** Secretaría de Salud (2012a). Norma Oficial Mexicana NOM-004-SSA3-2012, Del expediente clínico. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5272787](https://dof.gob.mx/nota_detalle_popup.php?codigo=5272787)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 15 de octubre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 2 (Campo de aplicación).  
- **Texto textual de la fuente:**

>   
> "2. Campo de aplicación. Esta norma, es de observancia obligatoria para el personal del área de la salud y los establecimientos prestadores de servicios de atención médica de los sectores público, social y privado, incluidos los consultorios."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Todo el territorio mexicano y todos los prestadores asistenciales de salud públicos, sociales y privados.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-007 — NOM-024-SSA3-2012: Definición normativa de Firma Electrónica Avanzada

- **Afirmación en la tesis:** "define a la Firma Electrónica Avanzada como el mecanismo que garantiza la autoría e integridad del documento clínico, vinculando al firmante con el contenido asentado" (\[TESIS: Cap II §2.2.2\]).  
- **Fuente citada:** Secretaría de Salud (2012b). Norma Oficial Mexicana NOM-024-SSA3-2012, Sistemas de información de registro electrónico para la salud. Intercambio de información en salud. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle.php?codigo=5280847\&fecha=30/11/2012](https://dof.gob.mx/nota_detalle.php?codigo=5280847&fecha=30/11/2012)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 30 de noviembre de 2012 (Norma Oficial Mexicana vigente, sin modificaciones posteriores).  
- **Localización exacta en la fuente:** Numeral 3.22.  
- **Texto textual de la fuente:**

>   
> "3.22 Firma Electrónica Avanzada.- Es el conjunto de datos y caracteres que permite la identificación del firmante, que ha sido creada por medios electrónicos bajo su exclusivo control, de manera que está vinculada únicamente al mismo y a los datos a los que se refiere, lo que permite que sea detectable cualquier modificación ulterior de éstos, la cual produce los mismos efectos jurídicos que la firma autógrafa."  
> 

- **Tipo:** DEFINICIÓN.  
- **A quién obliga / ámbito de aplicación:** Sistemas de Información de Registro Electrónico para la Salud (SIRES) en el Sistema Nacional de Salud.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-008 — NOM-024-SSA3-2012: Estándares tecnológicos para la interoperabilidad en salud

- **Afirmación en la tesis:** "establece que las plataformas considerarán los estándares HL7 CDA, HL7 V3 y XML, así como los estándares probados que determine la Secretaría de Salud" (\[TESIS: Cap II §2.2.2\]); y en Cap I §1.1.2: "alineándose formalmente con el estándar internacional HL7 FHIR Release 4 (HL7 International, 2019)".  
- **Fuente citada:** Secretaría de Salud (2012b). Norma Oficial Mexicana NOM-024-SSA3-2012. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle.php?codigo=5280847\&fecha=30/11/2012](https://dof.gob.mx/nota_detalle.php?codigo=5280847&fecha=30/11/2012)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 30 de noviembre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 6.1.3.1.  
- **Texto textual de la fuente:**

>   
> "6.1.3.1 De acuerdo al alcance del intercambio de información, se pueden considerar los estándares: HL7 CDA, HL7 V3, XML y/o el estándar probado que determine la Secretaría a través de la DGIS de acuerdo a las Guías y Formatos que se emitan para tal fin."  
> 

- **Tipo:** RECOMENDACIÓN / OBLIGACIÓN CONDICIONAL.  
- **A quién obliga / ámbito de aplicación:** Establecimientos del Sistema Nacional de Salud y desarrolladores de SIRES.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** PARCIALMENTE.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS Y AFIRMACIÓN DISCREPANTE.  
- **Estado:** DISCREPANCIA.  
- **Análisis de discrepancia:**  
1. La norma NO menciona en ningún numeral o apéndice el estándar HL7 FHIR ni su Release 4 (el estándar FHIR R4 fue publicado en 2018/2019, mientras que la norma data de 2012).  
2. Los estándares contemplados expresamente en el texto son HL7 CDA, HL7 V3, XML y los estándares probados que determine la Secretaría a través de la DGIS.  
3. Si bien en Cap II §2.2.2 la tesis aclara acertadamente que la NOM-024 no menciona expresamente a FHIR por razones cronológicas, en Cap I §1.1.2 afirma taxativamente que las disposiciones normativas (NOM-004 y NOM-024) exigen el intercambio homogéneo "alineándose formalmente con el estándar internacional HL7 FHIR Release 4", lo cual contradice el texto normativo vigente.

---

### F-009 — NOM-024-SSA3-2012: Remisión a la Ley de Firma Electrónica Avanzada

- **Afirmación en la tesis:** "la NOM-024-SSA3-2012 (numerales 3.22 y 9.6) remite a la Ley de Firma Electrónica Avanzada (2012) para dotar a los registros digitales de los mismos efectos jurídicos que la firma autógrafa" (\[TESIS: Cap II §2.2.4\]).  
- **Fuente citada:** Secretaría de Salud (2012b). Norma Oficial Mexicana NOM-024-SSA3-2012. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle.php?codigo=5280847\&fecha=30/11/2012](https://dof.gob.mx/nota_detalle.php?codigo=5280847&fecha=30/11/2012)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 30 de noviembre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numerales 3.22 (Definiciones), 6.6.5 (Especificaciones) y 9.6 (Bibliografía).  
- **Texto textual de la fuente:**

>   
> "3.22 Firma Electrónica Avanzada.- Es el conjunto de datos y caracteres que permite la identificación del firmante, que ha sido creada por medios electrónicos bajo su exclusivo control, de manera que está vinculada únicamente al mismo y a los datos a los que se refiere, lo que permite que sea detectable cualquier modificación ulterior de éstos, la cual produce los mismos efectos jurídicos que la firma autógrafa." "6.6.5 Con fines de intercambio de información entre Prestadores de Servicios de Salud los SIRES deben implementar mecanismos de autenticación, de cifrado y de firma electrónica avanzada de acuerdo a las disposiciones jurídicas, Guías y Formatos aplicables." "9.6 Ley de Firma Electrónica Avanzada." (asiento dentro del capítulo 9 "Bibliografía").  
> 

- **Tipo:** DEFINICIÓN / BIBLIOGRAFÍA.  
- **A quién obliga / ámbito de aplicación:** Registros clínicos electrónicos en el Sistema Nacional de Salud.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ en el fondo legal, pero atribuyendo mandato normativo a un asiento bibliográfico.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS CON CITA INCORRECTA.  
- **Estado:** DISCREPANCIA.  
- **Análisis de discrepancia:** El numeral 9.6 de la NOM-024-SSA3-2012 es únicamente una referencia bibliográfica que lista el título "9.6 Ley de Firma Electrónica Avanzada." sin redacción preceptiva. La equivalencia jurídica respecto a la firma autógrafa se encuentra incorporada en la definición misma del numeral 3.22 de la norma y en el artículo 7 de la LFEA, mientras que la obligación operativa de incorporarla para intercambio interinstitucional se encuentra en el numeral 6.6.5.

---

### F-010 — NOM-024-SSA3-2012: Ámbito de observabilidad obligatoria

- **Afirmación en la tesis:** "la NOM-024-SSA3-2012 es de observancia obligatoria para los establecimientos del Sistema Nacional de Salud que adopten un SIRES" (\[TESIS: Cap I §1.5\]).  
- **Fuente citada:** Secretaría de Salud (2012b). Norma Oficial Mexicana NOM-024-SSA3-2012. Diario Oficial de la Federación.  
- **URL de consulta:** [https\://dof.gob.mx/nota\_detalle.php?codigo=5280847\&fecha=30/11/2012](https://dof.gob.mx/nota_detalle.php?codigo=5280847&fecha=30/11/2012)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 30 de noviembre de 2012 (Norma Oficial Mexicana vigente).  
- **Localización exacta en la fuente:** Numeral 1.2 (Campo de aplicación).  
- **Texto textual de la fuente:**

>   
> "1.2 Esta Norma es de observancia obligatoria en todo el territorio nacional para todos los establecimientos que presten servicios de atención médica que formen parte del Sistema Nacional de Salud que adopten un Sistema de Información de Registro Electrónico para la Salud, así como para aquellas personas físicas o morales que dentro del territorio nacional cuenten indistintamente con los derechos de propiedad, uso, autoría, distribución y/o comercialización de dichos Sistemas; en ambos casos, en términos de la presente Norma y de las disposiciones jurídicas aplicables."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Establecimientos de salud públicos, sociales y privados que operen un SIRES, así como desarrolladores y distribuidores de software en salud en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-011 — LGPDPPSO (2025): Calificación de datos de salud como personales sensibles

- **Afirmación en la tesis:** "los datos relativos a la salud presente y futura son clasificados como datos personales sensibles (artículo 3 fracción X)" (\[TESIS: Cap I §1.5; Cap II §2.2.3\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025 (Edición Vespertina); última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículo 3 fracción X.  
- **Texto textual de la fuente:**

>   
> "Artículo 3\. Para los efectos de la presente Ley se entenderá por: ... X. Datos personales sensibles: Aquellos que se refieran a la esfera más íntima de su titular, o cuya utilización indebida pueda dar origen a discriminación o conlleve un riesgo grave para éste. De manera enunciativa más no limitativa, se consideran sensibles los datos personales que puedan revelar aspectos como origen racial o étnico, estado de salud presente o futuro, información genética, creencias religiosas, filosóficas y morales, afiliación sindical, opiniones políticas, preferencia sexual;"  
> 

- **Tipo:** DEFINICIÓN.  
- **A quién obliga / ámbito de aplicación:** Sujetos obligados del sector público (autoridades, dependencias y entidades de salud pública federales, estatales y municipales).  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (las unidades de IMSS-Bienestar y el Distrito de Salud I son sujetos obligados bajo la ley vigente).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-012 — LGPDPPSO (2025): Autoridad garante nacional (Secretaría Anticorrupción y Buen Gobierno)

- **Afirmación en la tesis:** "bajo la tutela de la Secretaría Anticorrupción y Buen Gobierno (artículo 3 fracción XXVI)" (\[TESIS: Cap I §1.5; Cap II §2.2.3\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículo 3 fracción XXVI.  
- **Texto textual de la fuente:**

>   
> "XXVI. Secretaría: Secretaría Anticorrupción y Buen Gobierno;"  
> 

- **Tipo:** DEFINICIÓN.  
- **A quién obliga / ámbito de aplicación:** Órgano garante y regulador del cumplimiento de protección de datos personales en el sector público federal mexicano.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (actualizado formalmente conforme a la nueva legislación de 2025 que sustituyó las funciones tutelares del INAI en la Administración Pública Federal).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-013 — LGPDPPSO (2025): Requisito de consentimiento expreso para datos sensibles

- **Afirmación en la tesis:** "el tratamiento de datos sensibles por parte de las unidades públicas exige consentimiento expreso conforme al artículo 7" (\[TESIS: Cap I §1.5; Cap II §2.2.3\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículo 7\.  
- **Texto textual de la fuente:**

>   
> "Artículo 7\. Por regla general no podrán tratarse datos personales sensibles, salvo que se cuente con el consentimiento expreso de la persona titular o, en su defecto, se trate de los casos establecidos en el artículo 16 de esta Ley."  
> 

- **Tipo:** OBLIGACIÓN / PROHIBICIÓN GENERAL CON EXCEPCIÓN.  
- **A quién obliga / ámbito de aplicación:** Todo sujeto obligado que efectúe tratamiento de datos personales sensibles.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-014 — LGPDPPSO (2025): Excepción de consentimiento para atención médica y diagnóstico

- **Afirmación en la tesis:** "exceptuándose en su artículo 16 fracción VII cuando sea indispensable para la atención médica, prevención o diagnóstico asistencial" (\[TESIS: Cap I §1.5; Cap II §2.2.3\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículo 16 fracción VII.  
- **Texto textual de la fuente:**

>   
> "Artículo 16\. El responsable no estará obligado a recabar el consentimiento de la persona titular para el tratamiento de sus datos personales en los siguientes casos: ... VII. Cuando los datos personales sean necesarios para efectuar un tratamiento para la prevención, diagnóstico o la prestación de asistencia sanitaria;"  
> 

- **Tipo:** EXCEPCIÓN LEGAL A OBLIGACIÓN DE CONSENTIMIENTO.  
- **A quién obliga / ámbito de aplicación:** Servicios médicos y asistenciales del sector público.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-015 — LGPDPPSO (2025): Principio de proporcionalidad y minimización de datos

- **Afirmación en la tesis:** "el tratamiento debe observar strictly el principio de proporcionalidad (artículos 10 y 19)" (\[TESIS: Cap I §1.5; Cap II §2.2.3; Cap III §3.1.5 num. 7, §3.2.2\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículos 10 y 19\.  
- **Texto textual de la fuente:**

>   
> "Artículo 10\. El responsable deberá observar los principios de licitud, finalidad, lealtad, consentimiento, calidad, proporcionalidad, información y responsabilidad en el tratamiento de datos personales." "Artículo 19\. El responsable sólo deberá tratar los datos personales que resulten adecuados, relevantes y estrictamente necesarios para la finalidad que justificó su obtención."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Sujetos obligados en el diseño, persistencia y tratamiento de bases de datos y mediadores de interoperabilidad.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (justifica la omisión del atributo unidad\_medica y el tratamiento opcional de telefono\_contacto sin persistencia innecesaria).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-016 — LGPDPPSO (2025): Medidas de seguridad técnicas y administrativas

- **Afirmación en la tesis:** "instrumentar medidas de seguridad técnicas y administrativas (artículos 25 y 26)" (\[TESIS: Cap I §1.5; Cap II §2.2.3; Cap III §3.1.5 num. 8, §3.3.1\]).  
- **Fuente citada:** Ley General de Protección de Datos Personales en Posesión de Sujetos Obligados (LGPDPPSO). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LGPDPPSO.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículos 25 y 26\.  
- **Texto textual de la fuente:**

>   
> "Artículo 25\. Con independencia del tipo de sistema en el que se encuentren los datos personales o el tipo de tratamiento que se efectúe, el responsable deberá establecer y mantener las medidas de seguridad de carácter administrativo, físico y técnico para la protección de los datos personales contra daño, pérdida, alteración, destrucción o su uso, acceso o tratamiento no autorizado, así como garantizar su confidencialidad, integridad y disponibilidad." "Artículo 26\. Las medidas de seguridad adoptadas por el responsable deberán considerar: I. El riesgo inherente a los datos personales tratados; II. La sensibilidad de los datos personales tratados; III. El desarrollo tecnológico..."  
> 

- **Tipo:** OBLIGACIÓN.  
- **A quién obliga / ámbito de aplicación:** Responsables del tratamiento de datos personales en el sector público.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (justifica la delegación perimetral de seguridad hacia un API Gateway y el confinamiento del middleware en subred privada no enrutable).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-017 — LFPDPPP (2025): Ámbito privado de protección de datos personales

- **Afirmación en la tesis:** "En el ámbito privado, la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP, 2025\) regula todo tratamiento de datos personales por parte de personas físicas o morales, abarcando su recolección, uso y transferencia" (\[TESIS: Cap II §2.2.3\]).  
- **Fuente citada:** Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP). Nueva Ley DOF 20-03-2025; reforma DOF 14-11-2025.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LFPDPPP.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LFPDPPP.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 20 de marzo de 2025; última reforma DOF 14 de noviembre de 2025\.  
- **Localización exacta en la fuente:** Artículos 1 y 2\.  
- **Texto textual de la fuente:**

>   
> "Artículo 1\. La presente Ley es de orden público y de observancia general en todo el territorio nacional y tiene por objeto la protección de los datos personales en posesión de los particulares, con la finalidad de regular su tratamiento legítimo, controlado e informado, a efecto de garantizar la privacidad y el derecho a la autodeterminación informativa de las personas." "Artículo 2\. Son sujetos regulados por esta Ley, los particulares sean personas físicas o morales de carácter privado que lleven a cabo el tratamiento de datos personales, con excepción de..."  
> 

- **Tipo:** DEFINICIÓN / ÁMBITO DE APLICACIÓN.  
- **A quién obliga / ámbito de aplicación:** Personas físicas y morales de carácter privado que tratan datos personales en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (distinción del marco privado frente al régimen de sujetos obligados de los CESSA públicos).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-018 — Ley de Firma Electrónica Avanzada (2012): Equivalencia funcional de la firma digital

- **Afirmación en la tesis:** "remite a la Ley de Firma Electrónica Avanzada (2012) para dotar a los registros digitales de los mismos efectos jurídicos que la firma autógrafa" (\[TESIS: Cap II §2.2.4\]).  
- **Fuente citada:** Ley de Firma Electrónica Avanzada (LFEA). Publicada en el DOF el 11 de enero de 2012; última reforma DOF 20-05-2021.  
- **URL de consulta:** [https\://www\.diputados.gob.mx/LeyesBiblio/pdf/LFEA.pdf](https://www.diputados.gob.mx/LeyesBiblio/pdf/LFEA.pdf)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicada en el DOF el 11 de enero de 2012; texto vigente con reforma del 20 de mayo de 2021\.  
- **Localización exacta en la fuente:** Artículo 7\.  
- **Texto textual de la fuente:**

>   
> "Artículo 7\. La firma electrónica avanzada podrá ser utilizada en documentos electrónicos y, en su caso, en mensajes de datos. Los documentos electrónicos y los mensajes de datos que cuenten con firma electrónica avanzada producirán los mismos efectos que los presentados con firma autógrafa y, en consecuencia, tendrán el mismo valor probatorio que las disposiciones aplicables les otorgan a éstos."  
> 

- **Tipo:** DEFINICIÓN / EQUIVALENCIA JURÍDICA.  
- **A quién obliga / ámbito de aplicación:** Comunicaciones y actos jurídicos electrónicos del orden federal en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-019 — HL7 FHIR Release 4 (v4.0.1, 2019): Estados normativos y madurez FMM

- **Afirmación en la tesis:** "Los recursos Patient y Observation alcanzaron el nivel normativo (FMM N), proveyendo estabilidad futura y compatibilidad en sus definiciones base (HL7 International, 2019). En contraste, el recurso Condition se ubica en nivel FMM 3 y Encounter en nivel FMM 2, ambos bajo la categoría de uso de prueba (Trial Use) sujeta a revisiones incrementales" (\[TESIS: Cap II §2.1.3\]).  
- **Fuente citada:** HL7 International (2019, 1 de noviembre). FHIR Release 4 (R4) v4.0.1.  
- **URL de consulta:**  
  - Patient: [https\://hl7.org/fhir/R4/patient.html](https://hl7.org/fhir/R4/patient.html)  
  - Observation: [https\://hl7.org/fhir/R4/observation.html](https://hl7.org/fhir/R4/observation.html)  
  - Condition: [https\://hl7.org/fhir/R4/condition.html](https://hl7.org/fhir/R4/condition.html)  
  - Encounter: [https\://hl7.org/fhir/R4/encounter.html](https://hl7.org/fhir/R4/encounter.html)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** FHIR Release 4 (v4.0.1, oficializada el 1 de noviembre de 2019).  
- **Localización exacta en la fuente:** Encabezados de especificación y tablas de estructura de cada recurso.  
- **Texto textual de la fuente:**  
  - `patient.html`: "Maturity Level: Normative" | "Standards Status: Normative"  
  - `observation.html`: "Maturity Level: Normative" | "Standards Status: Normative"  
  - `condition.html`: "Maturity Level: 3" | "Standards Status: Trial Use"  
  - `encounter.html`: "Maturity Level: 2" | "Standards Status: Trial Use"  
- **Tipo:** DEFINICIÓN TÉCNICA / DATO FORMAL.  
- **A quién obliga / ámbito de aplicación:** Especificación técnica formal internacional de FHIR R4.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-020 — HL7 FHIR Release 4: Prohibición de elementos con valor nulo explícito

- **Afirmación en la tesis:** "especificación normativa de HL7 FHIR Release 4, que prohíbe emitir atributos explícitos con valor nulo o cadenas vacías" (\[TESIS: Cap III §3.4.2\]).  
- **Fuente citada:** HL7 International (2019). FHIR Release 4 (v4.0.1).  
- **URL de consulta:** [https\://hl7.org/fhir/R4/datatypes.html](https://hl7.org/fhir/R4/datatypes.html) (Sección 2.24.0.1) y [https\://hl7.org/fhir/R4/json.html](https://hl7.org/fhir/R4/json.html) (Sección 2.23.2).  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** FHIR Release 4 (v4.0.1, 1 de noviembre de 2019).  
- **Localización exacta en la fuente:** `datatypes.html` sección 2.24.0.1 y `json.html` sección 2.23.2.  
- **Texto textual de la fuente:**

>   
> `https://hl7.org/fhir/R4/datatypes.html`: "When the value is missing, and there are no extensions, the element is not represented at all. This means that in xml, attributes are never present with a length of 0 (value=""), and properties are never a 0 length string or null in JSON ("name" : "" is not valid)." `https://hl7.org/fhir/R4/json.html`: "In JSON, an element or property with a value of null SHALL NOT be present unless it is associated with an extension"  
> 

- **Tipo:** PROHIBICIÓN TÉCNICA NORMATIVA.  
- **A quién obliga / ámbito de aplicación:** Cualquier motor o biblioteca de serialización JSON conforme con HL7 FHIR R4.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (sustenta la directiva optional: true y la omisión de claves ante valores relacionales nulos).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-021 — HAPI FHIR: Consumo de memoria física residente (RSS) en la JVM

- **Afirmación en la tesis:** "HAPI FHIR, construidas sobre la máquina virtual de Java (JVM), imponen una huella de memoria física residente (RSS) de varios gigabytes" (\[TESIS: Cap I §1.1.2\]) y "2 a 4+ GB de RAM (Bacher et al., 2024; HAPI FHIR, 2026)" (\[TESIS: Cap II §2.4.4 Tabla 3\]).  
- **Fuente citada:** HAPI FHIR (2026); Bacher et al. (2024).  
- **URL de consulta:** [https\://hapifhir.io/](https://hapifhir.io/) y [https\://pmc.ncbi.nlm.nih.gov/articles/PMC11141833/](https://pmc.ncbi.nlm.nih.gov/articles/PMC11141833/)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Documentación oficial en línea de HAPI FHIR (2026) y publicación de Bacher et al. (2024).  
- **Localización exacta en la fuente:** No localizada en las fuentes citadas.  
- **Texto textual de la fuente:** No existe ninguna declaración o especificación en el sitio oficial `hapifhir.io` ni en el artículo de Bacher et al. (2024) que determine que HAPI FHIR requiera de forma nativa "2 a 4+ GB de RAM" para operar como servidor Plain Server/Façade.  
- **Tipo:** DATO EMPÍRICO.  
- **A quién obliga / ámbito de aplicación:** Servidores FHIR basados en Java.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ en cuanto al problema general de memoria de la JVM, pero el dato numérico carece de atribución oficial válida.  
- **Estado:** DISCREPANCIA.  
- **Análisis de discrepancia:** La tesis atribuye el rango cuantitativo de "2 a 4+ GB de RAM" a la documentación oficial de HAPI FHIR (2026) y a Bacher et al. (2024). Sin embargo, la documentación oficial de HAPI FHIR no publica dicho requisito de memoria para arquitecturas de mediación sin persistencia interna, y el estudio de Bacher et al. (2024) no midió ni reportó cifras de consumo de memoria en gigabytes para HAPI FHIR.

---

### F-022 — Bacher et al. (2024): Sobrecarga computacional de motores FHIR sobre OpenMRS

- **Afirmación en la tesis:** "el consumo de memoria y la sobrecarga computacional de los motores de ejecución existentes restringen la respuesta del servicio en centros periféricos con equipamiento austero" (\[TESIS: Cap I §1.1.1; Cap II §2.4.4\]).  
- **Fuente citada en la tesis:** Bacher, L., Kasthurirathne, S. N., Purkayastha, S., y Grannis, S. J. (2024). FHIRing up OpenMRS: Architecture, implementation and real-world use-cases in low- and middle-income countries. Applied Clinical Informatics, 15(3), 570–579. [https\://doi.org/10.1055/a-2325-7815](https://doi.org/10.1055/a-2325-7815).  
- **URL de consulta:** [https\://pmc.ncbi.nlm.nih.gov/articles/PMC11141833/](https://pmc.ncbi.nlm.nih.gov/articles/PMC11141833/)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** AMIA Joint Summits on Translational Science Proceedings 2024; 2024: 162–171 (publicado en línea el 23 de mayo de 2024).  
- **Localización exacta en la fuente:** PMCID: PMC11141833, Sección Discussion, p. 169\.  
- **Texto textual de la fuente:**

>   
> "We have also identified several areas where the overheads of FHIR and the existing tools can be a major limitation in environments with limited infrastructure and bandwidth." "The conversion of OpenMRS data to FHIR Resources in bulk is a time-consuming process and the resulting JSON representations of the FHIR resources were relatively large, both of which made the pipeline slower than required."  
> 

- **Tipo:** ARTÍCULO CIENTÍFICO / DATO EMPÍRICO.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ en cuanto a contenido temático.  
- **Estado:** DISCREPANCIA.  
- **Análisis de discrepancia:** El contenido del artículo apoya conceptualmente las afirmaciones sobre sobrecarga y limitaciones de infraestructura con módulos FHIR en entornos de bajos recursos. No obstante, los metadatos bibliográficos citados en la tesis son erróneos:  
1. Autores reales: I. Bacher, M. Goodrich, A. Kimaina, M. Seaton, G. Faulkenberry, S. Vaish, J. Flowers, H. S. Fraser (la tesis cita a Bacher, L., Kasthurirathne, S. N., Purkayastha, S., Grannis, S. J.).  
2. Revista y datos de publicación reales: AMIA Joint Summits on Translational Science Proceedings, 2024, 162–171; PMCID: PMC11141833 (la tesis cita Applied Clinical Informatics 15(3): 570–579, con un DOI 10.1055/a-2325-7815 no registrado).

---

### F-023 — DGIS (2026): Estado oficial de certificaciones SIRES bajo NOM-024

- **Afirmación en la tesis:** "hacia septiembre de 2026 el catálogo oficial de la Dirección General de Información en Salud (2026) registró 38 certificados (14 con alcance mixto público-privado y 24 privados)... y ninguno pertenece al ámbito estrictamente público" (\[TESIS: Cap I §1.1.1; Cap II §2.2.5\]).  
- **Fuente citada:** Dirección General de Información en Salud (DGIS, 21 de septiembre de 2026). Catálogo oficial de SIRES Certificados en la NOM-024-SSA3-2012. Secretaría de Salud.  
- **URL de consulta:** [http\://www\.dgis.salud.gob.mx/contenidos/intercambio/sires\_certificacion\_gobmx.html](http://www.dgis.salud.gob.mx/contenidos/intercambio/sires_certificacion_gobmx.html)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Catálogo oficial en línea al 21 de septiembre de 2026\.  
- **Localización exacta en la fuente:** Catálogo web oficial.  
- **Texto textual de la fuente:**

>   
> "De conformidad con la NOM-024-SSA3-2012, numeral 7.5.2 el cual señala que, para obtener el certificado de cumplimiento con la norma en mención se debe contar con un Dictamen de Verificación Satisfactorio; así como, cumplir con la documentación de la información técnica requerida y las disposiciones jurídicas aplicables; a continuación, se enlistan los Sistemas de Información de Registro Electrónico para la Salud (SIRES) que han cumplido con lo dispuesto en el Procedimiento de Evaluación de la Conformidad y cuentan con un Certificado VIGENTE en la NOM-024-SSA3-2012, de acuerdo al alcance especificado: NOTA: Un sistema cuyo ámbito de aplicación es el Sector Público es susceptible de ser implementado en el Sector Privado. Sin embargo, un sistema cuyo ámbito de aplicación es el Sector Privado solo será susceptible de ser implementado en el Sector Público cuando su grado de cumplimiento con respecto a las variables asociadas a las Guías de Intercambio Aplicables sea del 100%." \[El catálogo oficial enlista 38 certificados emitidos a proveedores privados: 14 con alcance "Sector Público y Privado" / "Sector Público, Privado y Social", y 24 con alcance exclusivo "Sector Privado"; 0 certificados corresponden a software de titularidad estrictamente pública\].  
> 

- **Tipo:** DATO EMPÍRICO OFICIAL.  
- **A quién obliga / ámbito de aplicación:** Padrón federal informativo de software en salud en México.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-024 — Salud Digital (2021): Diagnóstico de adopción de estándares en México

- **Afirmación en la tesis:** "hacia septiembre de 2020 el país disponía únicamente de tres certificaciones vigentes de Sistemas de Información de Registro Electrónico para la Salud (SIRES) ante la autoridad federal" (\[TESIS: Cap I §1.1.1; Cap II §2.2.5\]).  
- **Fuente citada:** Salud Digital (2021, 7 de diciembre). Estado de situación en la adopción del estándar HL7/FHIR en México.  
- **URL de consulta:** [https\://saluddigital.com/big-data/estado-de-situacion-en-la-adopcion-del-estandar-hl7-fhir-en-mexico/](https://saluddigital.com/big-data/estado-de-situacion-en-la-adopcion-del-estandar-hl7-fhir-en-mexico/)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Reportaje periodístico digital publicado el 7 de diciembre de 2021\.  
- **Localización exacta en la fuente:** Artículo en portal web.  
- **Texto textual de la fuente:**

>   
> "Asimismo, tras el impacto de la pandemia, actualmente solo existen tres certificaciones en México para la Certificación de Sistema de Información en Salud."  
> 

- **Tipo:** DATO EMPÍRICO.  
- **A quién obliga / ámbito de aplicación:** Cobertura de prensa del Foro de Salud Digital RECAINSA 2021\.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ en cuanto a contexto.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL DEL ARTÍCULO DE TERCEROS.  
- **Estado:** FUENTE SECUNDARIA (Se mantiene como NO VERIFICADA conforme a las reglas del usuario por no provenir de repositorio gubernamental ni editor científico oficial).

---

### F-025 — Morales-Rocha et al. (2020): Modelo del dominio de ECE mediante KMoS-RE

- **Afirmación en la tesis:** "estructuraron un modelo del dominio de expediente clínico electrónico mediante el método KMoS-RE a partir del corpus normativo de la Secretaría de Salud (NOM-004 y NOM-024), proporcionando un marco formal para el diseño y experimentación de soluciones arquitectónicas en salud sin requerir la intervención de sistemas en producción" (\[TESIS: Cap I §1.1.1; Cap III §3.2.1\]).  
- **Fuente citada:** Morales-Rocha, V. M., Olmos-Sánchez, K. M., Hernández-Hernández, J. I., & Barbosa-Ramírez, F. (2020). Marco Técnico de Referencia para la Interoperabilidad de Expedientes Clínicos Electrónicos: Modelo del Dominio. Revista Mexicana de Ingeniería Biomédica, 41(1), 105–116. [https\://doi.org/10.17488/rmib.41.1.8](https://doi.org/10.17488/rmib.41.1.8).  
- **URL de consulta:** [https\://www\.scielo.org.mx/scielo.php?script=sci\_arttext\&pid=S0188-95322020000100105](https://www.scielo.org.mx/scielo.php?script=sci_arttext&pid=S0188-95322020000100105)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicación en Rev. Mex. Ing. Bioméd., vol. 41, no. 1, ene./abr. 2020\.  
- **Localización exacta en la fuente:** Páginas 105, 107 y 115\.  
- **Texto textual de la fuente:**

>   
> "En su forma general, las fases que constituyen el proceso KMoS-RE son: la modelación del dominio, la modelación del sistema y el desarrollo de la especificación de requisitos. La primera fase permite formalizar las propiedades del dominio, como es el caso de los conceptos, atributos y las relaciones existentes entre éstos... Esto constituye la primera fase del proyecto, que tiene como finalidad desarrollar el marco de referencia... El marco técnico de referencia en el que se está trabajando, a pesar de estar enfocado en el cumplimiento de la normatividad mexicana, puede servir como base para el diseño de marcos técnicos de referencia para expedientes clínicos electrónicos en otros países."  
> 

- **Tipo:** ARTÍCULO CIENTÍFICO / METODOLÓGICO.  
- **A quién obliga / ámbito de aplicación:** Ingeniería de requisitos e interoperabilidad en salud digital mexicana.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (antecedente metodológico para modelar esquemas sin intervenir bases en producción).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-026 — IMSS (2026) / Rodríguez Calva (2026): Despliegue de expediente digital en Chiapas

- **Afirmación en la tesis:** "en abril de 2026 se inició el despliegue del expediente digital de IMSS-Bienestar en 44 hospitales del estado de Chiapas (Rodríguez Calva, 2026)... el Instituto Mexicano del Seguro Social (2026) reportó que la puesta en marcha del expediente clínico electrónico en hospitales públicos de Chiapas proyectó optimizar aproximadamente el 35 % del tiempo de gestión clínica para reorientarlo a la atención directa" (\[TESIS: Cap I §1.1.1, §1.5\]).  
- **Fuentes citadas:**  
  1. Instituto Mexicano del Seguro Social (19 de abril de 2026). Comunicado No. 212/2026: "Secretaría de Salud, IMSS e IMSS Bienestar, arrancan en Chiapas componente de digitalización del Servicio Universal de Salud".  
  2. Rodríguez Calva, P. (19 de abril de 2026). "En Chiapas arranca servicio universal de salud con expedientes digitales". Excélsior.  
- **URL de consulta:** [http\://www\.imss.gob.mx/prensa/archivo/202604/212](http://www.imss.gob.mx/prensa/archivo/202604/212) y [https\://www\.excelsior.com.mx/nacional/chiapas-arranca-servicio-universal-salud-con-expedientes-digitales](https://www.excelsior.com.mx/nacional/chiapas-arranca-servicio-universal-salud-con-expedientes-digitales)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Publicaciones oficiales y periodísticas del 19 de abril de 2026\.  
- **Localización exacta en la fuente:** Comunicado oficial No. 212/2026 del IMSS y nota de Excélsior.  
- **Texto textual de la fuente:**

>   
> "El Servicio Universal de Salud en Chiapas incluyó la puesta en marcha del expediente clínico electrónico en los 44 hospitales de IMSS Bienestar que operan en la entidad. Su operación permitirá dar seguimiento integral a cada paciente, evitar la repetición innecesaria de estudios, reducir procesos administrativos y aumentar el tiempo efectivo de atención médica. Al respecto, el director general de IMSS Bienestar, Alejandro Svarch Pérez, informó que esta herramienta tecnológica permitirá agilizar la atención médica. 'Aproximadamente el 35% del tiempo de la gestión clínica se destina ahora a la atención directa de las y los pacientes, y no a procesos administrativos', detalló."  
> 

- **Tipo:** DATO EMPÍRICO OFICIAL.  
- **A quién obliga / ámbito de aplicación:** Red hospitalaria de segundo nivel de IMSS-Bienestar en Chiapas.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ (justificación contextual de la brecha en el primer nivel asistencial ambulatorio).  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

### F-027 — Algoritmo RENAPO: Sintaxis de CURP y OID nacional

- **Afirmación en la tesis:** "Las claves CURP se estructuraron conforme al algoritmo oficial de 18 caracteres de la Secretaría de Gobernación" (\[TESIS: Cap III §3.2.3\]) asociando el OID `urn:oid:2.16.840.1.113883.4.629` (Registro Nacional de Población, RENAPO).  
- **Fuentes citadas:**  
  1. Secretaría de Gobernación (SEGOB). Instructivo Normativo para la Asignación de la Clave Única de Registro de Población (DOF 18-jun-2018; modificación DOF 18-oct-2021).  
  2. Dirección General de Información en Salud (DGIS) / HL7 International. Catálogo de OID's Registrados en Salud.  
- **URL de consulta:**  
  - Instructivo DOF: [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5526717](https://dof.gob.mx/nota_detalle_popup.php?codigo=5526717) y [https\://dof.gob.mx/nota\_detalle\_popup.php?codigo=5632965](https://dof.gob.mx/nota_detalle_popup.php?codigo=5632965)  
  - Catálogo OID DGIS: [http\://www\.dgis.salud.gob.mx/contenidos/intercambio/consultaoid\_gobmx.html](http://www.dgis.salud.gob.mx/contenidos/intercambio/consultaoid_gobmx.html)  
- **Fecha de consulta:** 2026-10-02.  
- **Versión del documento:** Instructivo Normativo vigente (DOF 18/06/2018, reformado 18/10/2021) y Catálogo OID DGIS vigente.  
- **Localización exacta en la fuente:** DOF 18-jun-2018 Anexo 1 numeral 1; Catálogo OID DGIS entrada CURP.  
- **Texto textual de la fuente:**

>   
> `DOF 18-jun-2018 (Instructivo Normativo CURP)`: "1. Características de la CURP. 1.1. Composición: Alfanumérica; 1.2. Longitud: 18 caracteres; 1.3. Naturaleza: Biunívoca; 1.4. Universal: Se asigna a todas las personas que conforman la población, y 1.5. Verificable: En su estructura existen elementos que permiten comprobar si fue conformada correctamente o no, como fecha de nacimiento, sexo, entidad federativa de nacimiento y las primeras cuatro posiciones de la clave..." `Catálogo OID DGIS`: "2.16.840.1.113883.4.629 \- CURP. Clave Única de Registro de Población."  
> 

- **Tipo:** ESPECIFICACIÓN NORMATIVA Y TÉCNICA OFICIAL.  
- **A quién obliga / ámbito de aplicación:** Padrón nacional de población en México y registro de derechohabientes.  
- **¿La tesis la usa dentro de ese ámbito y de su vigencia temporal?:** SÍ.  
- **¿La tesis cita textual o parafrasea?:** PARÁFRASIS FIEL.  
- **Estado:** VERIFICADA CONTRA TEXTO OFICIAL.

---

## 3\. Síntesis Estadística de la Verificación

| Estado | Total de Fichas | Fichas Comprendidas |
| :---- | :---: | :---- |
| **VERIFICADA CONTRA TEXTO OFICIAL** | 21 | F-001, F-003, F-004, F-005, F-006, F-007, F-010, F-011, F-012, F-013, F-014, F-015, F-016, F-017, F-018, F-019, F-020, F-023, F-025, F-026, F-027 |
| **DISCREPANCIA** | 5 | F-002, F-008, F-009, F-021, F-022 |
| **FUENTE SECUNDARIA** | 1 | F-024 |
| **NO VERIFICADA — SIN ACCESO** | 0 | Ninguna (todas las fuentes oficiales fueron consultadas exitosamente) |
| **Total** | 27 | F-001 a F-027 |

