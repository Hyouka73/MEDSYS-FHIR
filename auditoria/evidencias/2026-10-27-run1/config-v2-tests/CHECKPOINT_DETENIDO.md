# Variante unidad_medica/serviceType — detenida por instrucción del usuario

Fecha de checkpoint UTC: 2026-10-28T03:01:31.892656+00:00

## Trabajo efectivamente realizado

Sólo se inspeccionaron fuentes, tesis guardada, estándar HL7 y bases sintéticas mediante SELECT de solo lectura. No se crearon configuraciones variantes ni se inició su ensayo. No hubo compilación Rust, cambios en crates/**, cambios en config/** originales, escritura en bases, ni servidores propios iniciados para esta tarea.

- `crates/medsys-core/src/engine/transform.rs:350–354`: `transform_encounter` produce `serviceType.text` cuando `LegacyConsulta.unidad_medica` está presente.
- `crates/medsys-db/src/dynamic_repo.rs:789`: el adaptador reconoce `unidad_medica`, `unidad` y `clinica` como campo opcional.
- `crates/medsys-db/src/repository/consultas.rs:13–38`: el repositorio tradicional S0 sí proyectaba `unidad_medica`.
- Los cuatro YAML originales de `config/` no contienen una regla que proyecte `unidad_medica` en Encounter. Los cuatro DDL de esquemas sí contienen la columna.
- `scripts/oracle_verifier.py:265–266`: el oráculo vigente espera `serviceType.text` cuando la unidad fuente está presente. No se modificó ese oráculo.
- La copia del Capítulo III, `auditoria/evidencias/2026-10-20-reverificacion/cap3-fuente.txt:431`, dice que unidad_medica no se proyecta en Patient; no constituye una prohibición de preservarla en Encounter.

## Observación SQL ya realizada

Con usuario de solo lectura `medsys_ro`, host `127.0.0.1:55432`, se consultaron las bases `medsys_eval_v2_s0` a `medsys_eval_v2_s3`. En las tablas Encounter correspondientes (`tbl_consultas`, `tbl_sabana_consulta_externa`, `tbl_consultas_s2`, `tbl_consultas_s3`) se observaron, para cada esquema, 2,500 filas, 2,500 valores unidad_medica presentes y 0 NULL. PK: id_consulta salvo S3 con con_id.

Muestras observadas por esquema: ID 1 e ID 2 = CESSA Tuxtla Poniente; ID 3 = Hospital Básico Comunitario Berriozábal. Esta inspección de fuente no es una comparación HTTP ni un resultado CA-2.

Consulta reproducible (adaptar tabla y PK según esquema):

```sql
SELECT count(*), count(unidad_medica),
       count(*) FILTER (WHERE unidad_medica IS NULL)
FROM tbl_consultas;
SELECT id_consulta, unidad_medica FROM tbl_consultas
WHERE unidad_medica IS NOT NULL ORDER BY id_consulta LIMIT 3;
SELECT id_consulta, unidad_medica FROM tbl_consultas
WHERE unidad_medica IS NULL ORDER BY id_consulta LIMIT 3;
```

## Reserva semántica

Se consultó la definición oficial [HL7 FHIR R4 Encounter](https://hl7.org/fhir/R4/encounter-definitions.html#Encounter.serviceType). serviceType categoriza el servicio proporcionado (por ejemplo una especialidad); los valores observados identifican establecimientos. Por tanto, la propuesta hacia serviceType.text reproduciría el contrato del constructor y del oráculo existentes, pero no demuestra que una unidad médica equivalga semánticamente a un tipo de servicio. Un futuro diseño de Organization/serviceProvider o Location requiere una decisión adicional, fuera de esta copia acotada.

## Variante propuesta, aún NO creada

Copiar cada YAML original a `output/evaluation/20261027/config-v2/` y añadir únicamente al bloque Encounter:

```yaml
      - source_column: "unidad_medica"
        target_path: "serviceType.text"
        optional: true
```

No introducir una constante o reemplazo. Mantener las configuraciones originales y el binario congelado EA7E... intactos. Verificar equivalencia estructural salvo esa regla, hashes antes/después, parseo mediante arranque del mismo ejecutable y muestras HTTP antes/después en puertos separados de 3310–3313 y 3340–3343. No compilar Rust ni declarar CA-2 aprobada por muestras.

## Pendientes al detenerse

1. Crear las cuatro copias YAML y sus hashes/diffs.
2. Verificar parseo y allowlist con el ejecutable congelado, sin compilar.
3. Ejecutar muestras HTTP nominales reales por esquema y comparar todo el JSON además de serviceType.
4. Conservar peticiones, respuestas, logs, identidad de base/configuración/binario y códigos de salida.
5. El caso de origen NULL no existe en las cuatro bases v2 observadas. No fue ensayado por HTTP. El coordinador autorizó considerar después una copia aislada o fixture unitaria con inyección/restauración, sin modificar v2 mientras corre la evaluación original.

No hay procesos propios pendientes de esta tarea. No se modificó Drive.
