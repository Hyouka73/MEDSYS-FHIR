#!/usr/bin/env python3
# ==============================================================================
# Generador Determinista del Corpus de 100 Fallas Clínicas (T1–T5)
# Tesis UNACH 2026 — MedSys-FHIR v1.2.0-eval
# ==============================================================================

import json
import os
import re

CURP_VALID = "ROMA900101HCSMNL09"
CURP_VALID_2 = "LOPE950512MCSPRN07"

cases = []

# ==============================================================================
# CATEGORÍA T1: Nulos y Cadenas Vacías en Campos Obligatorios (20 casos: T1-01 a T1-20)
# ==============================================================================
t1_definitions = [
    # Paciente (8 casos)
    ("T1-01", "Patient", "tbl_pacientes", "s0", "primer_nombre", None, "Paciente sin primer_nombre (NULL)",
     {"id_paciente": 9001, "curp": CURP_VALID, "primer_nombre": None, "segundo_nombre": "CARLOS", "apellido_paterno": "RODRIGUEZ", "apellido_materno": "MARTINEZ", "fecha_nacimiento": "1990-01-01", "sexo_biologico": "M", "telefono_contacto": "9611234567"}),
    ("T1-02", "Patient", "tbl_pacientes", "s0", "primer_nombre", "", "Paciente con primer_nombre vacío ('')",
     {"id_paciente": 9002, "curp": CURP_VALID, "primer_nombre": "", "segundo_nombre": None, "apellido_paterno": "GARCIA", "apellido_materno": "HERNANDEZ", "fecha_nacimiento": "1992-05-15", "sexo_biologico": "F", "telefono_contacto": "9617654321"}),
    ("T1-03", "Patient", "tbl_pacientes", "s0", "apellido_paterno", None, "Paciente sin apellido_paterno (NULL)",
     {"id_paciente": 9003, "curp": CURP_VALID, "primer_nombre": "JUAN", "segundo_nombre": None, "apellido_paterno": None, "apellido_materno": "PEREZ", "fecha_nacimiento": "1988-11-20", "sexo_biologico": "M", "telefono_contacto": None}),
    ("T1-04", "Patient", "tbl_pacientes", "s0", "apellido_paterno", "", "Paciente con apellido_paterno vacío ('')",
     {"id_paciente": 9004, "curp": CURP_VALID, "primer_nombre": "MARIA", "segundo_nombre": "ELENA", "apellido_paterno": "", "apellido_materno": "LOPEZ", "fecha_nacimiento": "1995-03-12", "sexo_biologico": "F", "telefono_contacto": "9612345678"}),
    ("T1-05", "Patient", "tbl_pacientes", "s0", "curp", None, "Paciente sin CURP (NULL)",
     {"id_paciente": 9005, "curp": None, "primer_nombre": "JOSE", "segundo_nombre": None, "apellido_paterno": "SANCHEZ", "apellido_materno": "DIAZ", "fecha_nacimiento": "1985-07-25", "sexo_biologico": "M", "telefono_contacto": "9613456789"}),
    ("T1-06", "Patient", "tbl_pacientes", "s0", "curp", "", "Paciente con CURP vacía ('')",
     {"id_paciente": 9006, "curp": "", "primer_nombre": "ANA", "segundo_nombre": "LUCIA", "apellido_paterno": "RAMIREZ", "apellido_materno": "GOMEZ", "fecha_nacimiento": "1993-09-30", "sexo_biologico": "F", "telefono_contacto": None}),
    ("T1-07", "Patient", "tbl_pacientes", "s0", "fecha_nacimiento", None, "Paciente sin fecha_nacimiento (NULL)",
     {"id_paciente": 9007, "curp": CURP_VALID, "primer_nombre": "PEDRO", "segundo_nombre": None, "apellido_paterno": "TORRES", "apellido_materno": "CASTRO", "fecha_nacimiento": None, "sexo_biologico": "M", "telefono_contacto": "9614567890"}),
    ("T1-08", "Patient", "tbl_pacientes", "s0", "sexo_biologico", None, "Paciente sin sexo_biologico (NULL)",
     {"id_paciente": 9008, "curp": CURP_VALID, "primer_nombre": "LUCIA", "segundo_nombre": "FERNANDA", "apellido_paterno": "FLORES", "apellido_materno": "MORALES", "fecha_nacimiento": "1991-12-05", "sexo_biologico": None, "telefono_contacto": "9615678901"}),

    # Consulta (6 casos)
    ("T1-09", "Encounter", "tbl_consultas", "s0", "id_paciente", None, "Consulta sin id_paciente (NULL)",
     {"id_consulta": 9009, "id_paciente": None, "fecha_hora_inicio": "2026-10-01 09:00:00", "fecha_hora_fin": "2026-10-01 09:30:00", "motivo_consulta": "Control prenatal", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T1-10", "Encounter", "tbl_consultas", "s0", "fecha_hora_inicio", None, "Consulta sin fecha_hora_inicio (NULL)",
     {"id_consulta": 9010, "id_paciente": 1, "fecha_hora_inicio": None, "fecha_hora_fin": "2026-10-01 10:30:00", "motivo_consulta": "Cefalea tensional", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T1-11", "Encounter", "tbl_consultas", "s0", "motivo_consulta", None, "Consulta sin motivo_consulta (NULL)",
     {"id_consulta": 9011, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 10:00:00", "fecha_hora_fin": "2026-10-01 10:30:00", "motivo_consulta": None, "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T1-12", "Encounter", "tbl_consultas", "s0", "motivo_consulta", "", "Consulta con motivo_consulta vacío ('')",
     {"id_consulta": 9012, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 11:00:00", "fecha_hora_fin": "2026-10-01 11:30:00", "motivo_consulta": "", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T1-13", "Encounter", "tbl_consultas", "s0", "estado_consulta", None, "Consulta sin estado_consulta (NULL)",
     {"id_consulta": 9013, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 12:00:00", "fecha_hora_fin": "2026-10-01 12:30:00", "motivo_consulta": "Revision general", "estado_consulta": None, "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T1-14", "Encounter", "tbl_consultas", "s0", "cedula_medico_tratante", None, "Consulta sin cedula_medico_tratante (NULL)",
     {"id_consulta": 9014, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 13:00:00", "fecha_hora_fin": "2026-10-01 13:30:00", "motivo_consulta": "Dolor abdominal", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": None, "nombre_medico": "DR. ROBERTO CRUZ"}),

    # Signos Vitales (4 casos)
    ("T1-15", "Observation", "tbl_signos_vitales", "s0", "presion_sistolica", None, "Presion arterial sin presion_sistolica (NULL)",
     {"id_signo": 9015, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": None, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T1-16", "Observation", "tbl_signos_vitales", "s0", "presion_diastolica", None, "Presion arterial sin presion_diastolica (NULL)",
     {"id_signo": 9016, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": None, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T1-17", "Observation", "tbl_signos_vitales", "s0", "temperatura_celsius", None, "Signos vitales sin temperatura_celsius (NULL)",
     {"id_signo": 9017, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": None, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "body_temperature"),
    ("T1-18", "Observation", "tbl_signos_vitales", "s0", "frecuencia_cardiaca", None, "Signos vitales sin frecuencia_cardiaca (NULL)",
     {"id_signo": 9018, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": None, "frecuencia_respiratoria": 18}, "heart_rate"),

    # Diagnosticos (2 casos)
    ("T1-19", "Condition", "tbl_diagnosticos", "s0", "codigo_cie10", None, "Diagnostico sin codigo_cie10 (NULL)",
     {"id_diagnostico": 9019, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": None, "descripcion_diagnostico": "Hipertension esencial", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
    ("T1-20", "Condition", "tbl_diagnosticos", "s0", "descripcion_diagnostico", None, "Diagnostico sin descripcion_diagnostico (NULL)",
     {"id_diagnostico": 9020, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "I10", "descripcion_diagnostico": None, "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
]

for item in t1_definitions:
    cid, rtype, tbl, arch, field, val, desc, data = item[:8]
    obs_type = item[8] if len(item) > 8 else None
    cases.append({
        "id": cid,
        "category": "T1",
        "title": desc,
        "description": f"Violación T1: Omisión de campo obligatorio '{field}' en {tbl}.",
        "table": tbl,
        "resource_type": rtype,
        "observation_type": obs_type,
        "archetype": arch,
        "fault_field": field,
        "injected_value": val,
        "data": data,
        "expected_status": 422,
        "expected_issue_code": "processing"
    })

# ==============================================================================
# CATEGORÍA T2: Formatos, Tipos y Fechas Inválidas (20 casos: T2-01 a T2-20)
# ==============================================================================
t2_definitions = [
    # Fechas imposibles o corruptas en Paciente (4 casos)
    ("T2-01", "Patient", "tbl_pacientes", "s0", "fecha_nacimiento", "2026-02-30", "Fecha de nacimiento con día imposible (30 de febrero)",
     {"id_paciente": 9021, "curp": CURP_VALID, "primer_nombre": "CARLOS", "segundo_nombre": None, "apellido_paterno": "LOPEZ", "apellido_materno": "GARCIA", "fecha_nacimiento": "2026-02-30", "sexo_biologico": "M", "telefono_contacto": "9611234567"}),
    ("T2-02", "Patient", "tbl_pacientes", "s0", "fecha_nacimiento", "1990-13-01", "Fecha de nacimiento con mes imposible (mes 13)",
     {"id_paciente": 9022, "curp": CURP_VALID, "primer_nombre": "MARIA", "segundo_nombre": None, "apellido_paterno": "HERNANDEZ", "apellido_materno": "PEREZ", "fecha_nacimiento": "1990-13-01", "sexo_biologico": "F", "telefono_contacto": "9612345678"}),
    ("T2-03", "Patient", "tbl_pacientes", "s0", "fecha_nacimiento", "FECHA_INVALIDA", "Fecha de nacimiento corrupta (texto no parseable)",
     {"id_paciente": 9023, "curp": CURP_VALID, "primer_nombre": "JORGE", "segundo_nombre": None, "apellido_paterno": "MARTINEZ", "apellido_materno": "SANCHEZ", "fecha_nacimiento": "FECHA_INVALIDA", "sexo_biologico": "M", "telefono_contacto": None}),
    ("T2-04", "Patient", "tbl_pacientes", "s0", "fecha_nacimiento", "01-99-1990", "Fecha de nacimiento con mes 99 en formato dd-mm-yyyy",
     {"id_paciente": 9024, "curp": CURP_VALID, "primer_nombre": "ANA", "segundo_nombre": "SOFIA", "apellido_paterno": "GARCIA", "apellido_materno": "LOPEZ", "fecha_nacimiento": "01-99-1990", "sexo_biologico": "F", "telefono_contacto": "9613456789"}),

    # Timestamps corruptos en Consultas (4 casos)
    ("T2-05", "Encounter", "tbl_consultas", "s0", "fecha_hora_inicio", "2026-04-31 10:00:00", "Timestamp con día imposible (31 de abril)",
     {"id_consulta": 9025, "id_paciente": 1, "fecha_hora_inicio": "2026-04-31 10:00:00", "fecha_hora_fin": "2026-04-30 10:30:00", "motivo_consulta": "Control prenatal", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T2-06", "Encounter", "tbl_consultas", "s0", "fecha_hora_inicio", "NO_DATETIME", "Timestamp corrupto (texto no datetime)",
     {"id_consulta": 9026, "id_paciente": 1, "fecha_hora_inicio": "NO_DATETIME", "fecha_hora_fin": "2026-10-01 10:30:00", "motivo_consulta": "Cefalea", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T2-07", "Encounter", "tbl_consultas", "s0", "fecha_hora_inicio", "2026-02-29 10:00:00", "Timestamp bisiesto inválido (29 feb 2026)",
     {"id_consulta": 9027, "id_paciente": 1, "fecha_hora_inicio": "2026-02-29 10:00:00", "fecha_hora_fin": "2026-02-28 10:30:00", "motivo_consulta": "Revision", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T2-08", "Encounter", "tbl_consultas", "s0", "fecha_hora_fin", "2026-13-01 10:00:00", "Timestamp de fin con mes 13",
     {"id_consulta": 9028, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 09:00:00", "fecha_hora_fin": "2026-13-01 10:00:00", "motivo_consulta": "Seguimiento", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),

    # Valores no numéricos en Signos Vitales (7 casos)
    ("T2-09", "Observation", "tbl_signos_vitales", "s0", "presion_sistolica", "ALTA", "Presion sistolica con texto no numérico ('ALTA')",
     {"id_signo": 9029, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": "ALTA", "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T2-10", "Observation", "tbl_signos_vitales", "s0", "presion_diastolica", "NORMAL", "Presion diastolica con texto no numérico ('NORMAL')",
     {"id_signo": 9030, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": "NORMAL", "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T2-11", "Observation", "tbl_signos_vitales", "s0", "temperatura_celsius", "FIEBRE", "Temperatura con texto no numérico ('FIEBRE')",
     {"id_signo": 9031, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": "FIEBRE", "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "body_temperature"),
    ("T2-12", "Observation", "tbl_signos_vitales", "s0", "temperatura_celsius", "37.5.8", "Temperatura con valor malformado ('37.5.8')",
     {"id_signo": 9032, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": "37.5.8", "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "body_temperature"),
    ("T2-13", "Observation", "tbl_signos_vitales", "s0", "frecuencia_cardiaca", "RAPIDA", "Frecuencia cardiaca no numérica ('RAPIDA')",
     {"id_signo": 9033, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": "RAPIDA", "frecuencia_respiratoria": 18}, "heart_rate"),
    ("T2-14", "Observation", "tbl_signos_vitales", "s0", "frecuencia_cardiaca", "80bpm", "Frecuencia cardiaca con sufijo no numérico ('80bpm')",
     {"id_signo": 9034, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": "80bpm", "frecuencia_respiratoria": 18}, "heart_rate"),
    ("T2-15", "Observation", "tbl_signos_vitales", "s0", "frecuencia_respiratoria", "VENTILANDO", "Frecuencia respiratoria no numérica ('VENTILANDO')",
     {"id_signo": 9035, "id_consulta": 1, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": "VENTILANDO"}, "respiratory_rate"),

    # S3 Fallas en split_part de Presion Arterial y unit_conversion (3 casos)
    ("T2-16", "Observation", "tbl_signos_s3", "s3", "presion_arterial_txt", "120/XX", "Presion arterial S3 con diastolica no numérica ('120/XX')",
     {"sig_id": 9036, "con_id": "1", "pac_id": "1", "fech_reg_txt": "01/10/2026 09:15:00", "presion_arterial_txt": "120/XX", "temp_c": 36.5, "fc": 75, "fr": 18, "peso_libras": "154.32"}, "blood_pressure_panel"),
    ("T2-17", "Observation", "tbl_signos_s3", "s3", "presion_arterial_txt", "YY/80", "Presion arterial S3 con sistolica no numérica ('YY/80')",
     {"sig_id": 9037, "con_id": "1", "pac_id": "1", "fech_reg_txt": "01/10/2026 09:15:00", "presion_arterial_txt": "YY/80", "temp_c": 36.5, "fc": 75, "fr": 18, "peso_libras": "154.32"}, "blood_pressure_panel"),
    ("T2-18", "Observation", "tbl_signos_s3", "s3", "presion_arterial_txt", "120-80", "Presion arterial S3 sin delimitador '/' ('120-80')",
     {"sig_id": 9038, "con_id": "1", "pac_id": "1", "fech_reg_txt": "01/10/2026 09:15:00", "presion_arterial_txt": "120-80", "temp_c": 36.5, "fc": 75, "fr": 18, "peso_libras": "154.32"}, "blood_pressure_panel"),

    # Fechas corruptas en Diagnostico (2 casos)
    ("T2-19", "Condition", "tbl_diagnosticos", "s0", "fecha_diagnostico", "2026-11-31", "Fecha de diagnostico imposible (31 de noviembre)",
     {"id_diagnostico": 9039, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "I10", "descripcion_diagnostico": "Hipertension", "fecha_diagnostico": "2026-11-31", "tipo_diagnostico": "CONFIRMADO"}),
    ("T2-20", "Condition", "tbl_diagnosticos", "s0", "fecha_diagnostico", "CORRUPT_TIMESTAMP", "Fecha de diagnostico corrupta ('CORRUPT_TIMESTAMP')",
     {"id_diagnostico": 9040, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "I10", "descripcion_diagnostico": "Hipertension", "fecha_diagnostico": "CORRUPT_TIMESTAMP", "tipo_diagnostico": "CONFIRMADO"}),
]

for item in t2_definitions:
    cid, rtype, tbl, arch, field, val, desc, data = item[:8]
    obs_type = item[8] if len(item) > 8 else None
    cases.append({
        "id": cid,
        "category": "T2",
        "title": desc,
        "description": f"Violación T2: Tipo de dato, formato o fecha corrupta en '{field}' de {tbl}.",
        "table": tbl,
        "resource_type": rtype,
        "observation_type": obs_type,
        "archetype": arch,
        "fault_field": field,
        "injected_value": val,
        "data": data,
        "expected_status": 422,
        "expected_issue_code": "processing"
    })

# ==============================================================================
# CATEGORÍA T3: Valores Fuera de Diccionario / Catálogo Normativo (20 casos: T3-01 a T3-20)
# ==============================================================================
t3_definitions = [
    # sexo_biologico fuera de M/F/I (6 casos)
    ("T3-01", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "X", "Sexo biológico no reconocido ('X')",
     {"id_paciente": 9041, "curp": CURP_VALID, "primer_nombre": "ALEX", "segundo_nombre": None, "apellido_paterno": "LOPEZ", "apellido_materno": "GARCIA", "fecha_nacimiento": "1990-01-01", "sexo_biologico": "X", "telefono_contacto": "9611234567"}),
    ("T3-02", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "DESCONOCIDO", "Sexo biológico texto no estándar ('DESCONOCIDO')",
     {"id_paciente": 9042, "curp": CURP_VALID, "primer_nombre": "PATRICIA", "segundo_nombre": None, "apellido_paterno": "HERNANDEZ", "apellido_materno": "PEREZ", "fecha_nacimiento": "1992-05-15", "sexo_biologico": "DESCONOCIDO", "telefono_contacto": "9612345678"}),
    ("T3-03", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "H", "Sexo biológico con clave H en lugar de M ('H')",
     {"id_paciente": 9043, "curp": CURP_VALID, "primer_nombre": "MIGUEL", "segundo_nombre": None, "apellido_paterno": "SANCHEZ", "apellido_materno": "DIAZ", "fecha_nacimiento": "1988-11-20", "sexo_biologico": "H", "telefono_contacto": "9613456789"}),
    ("T3-04", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "MUJER", "Sexo biológico texto libre ('MUJER')",
     {"id_paciente": 9044, "curp": CURP_VALID, "primer_nombre": "MARIA", "segundo_nombre": "ELENA", "apellido_paterno": "GARCIA", "apellido_materno": "LOPEZ", "fecha_nacimiento": "1995-03-12", "sexo_biologico": "MUJER", "telefono_contacto": "9614567890"}),
    ("T3-05", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "Z", "Sexo biológico clave inexistente ('Z')",
     {"id_paciente": 9045, "curp": CURP_VALID, "primer_nombre": "ROBERTO", "segundo_nombre": None, "apellido_paterno": "TORRES", "apellido_materno": "CASTRO", "fecha_nacimiento": "1985-07-25", "sexo_biologico": "Z", "telefono_contacto": "9615678901"}),
    ("T3-06", "Patient", "tbl_pacientes", "s0", "sexo_biologico", "NB", "Sexo biológico no binario no contemplado en NOM-004 ('NB')",
     {"id_paciente": 9046, "curp": CURP_VALID, "primer_nombre": "SAM", "segundo_nombre": None, "apellido_paterno": "RAMIREZ", "apellido_materno": "GOMEZ", "fecha_nacimiento": "1993-09-30", "sexo_biologico": "NB", "telefono_contacto": "9616789012"}),

    # estado_consulta fuera de FINALIZADA/EN_CURSO/CANCELADA (6 casos)
    ("T3-07", "Encounter", "tbl_consultas", "s0", "estado_consulta", "PENDIENTE_REVISION", "Estado de consulta fuera de catálogo ('PENDIENTE_REVISION')",
     {"id_consulta": 9047, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 09:00:00", "fecha_hora_fin": "2026-10-01 09:30:00", "motivo_consulta": "Revision", "estado_consulta": "PENDIENTE_REVISION", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T3-08", "Encounter", "tbl_consultas", "s0", "estado_consulta", "ABIERTO", "Estado de consulta fuera de catálogo ('ABIERTO')",
     {"id_consulta": 9048, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 10:00:00", "fecha_hora_fin": "2026-10-01 10:30:00", "motivo_consulta": "Cefalea", "estado_consulta": "ABIERTO", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T3-09", "Encounter", "tbl_consultas", "s0", "estado_consulta", "CERRADO", "Estado de consulta fuera de catálogo ('CERRADO')",
     {"id_consulta": 9049, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 11:00:00", "fecha_hora_fin": "2026-10-01 11:30:00", "motivo_consulta": "Dolor", "estado_consulta": "CERRADO", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T3-10", "Encounter", "tbl_consultas", "s0", "estado_consulta", "EN_ESPERA", "Estado de consulta fuera de catálogo ('EN_ESPERA')",
     {"id_consulta": 9050, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 12:00:00", "fecha_hora_fin": "2026-10-01 12:30:00", "motivo_consulta": "Control", "estado_consulta": "EN_ESPERA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T3-11", "Encounter", "tbl_consultas", "s0", "estado_consulta", "SUSPENDIDA", "Estado de consulta fuera de catálogo ('SUSPENDIDA')",
     {"id_consulta": 9051, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 13:00:00", "fecha_hora_fin": "2026-10-01 13:30:00", "motivo_consulta": "Gripe", "estado_consulta": "SUSPENDIDA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T3-12", "Encounter", "tbl_consultas", "s0", "estado_consulta", "NO_ASISTIO", "Estado de consulta fuera de catálogo ('NO_ASISTIO')",
     {"id_consulta": 9052, "id_paciente": 1, "fecha_hora_inicio": "2026-10-01 14:00:00", "fecha_hora_fin": "2026-10-01 14:30:00", "motivo_consulta": "Vacuna", "estado_consulta": "NO_ASISTIO", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),

    # tipo_diagnostico presente pero fuera de CONFIRMADO/PRESUNTIVO (8 casos)
    ("T3-13", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "DESCARTADO_MANUAL", "Tipo de diagnóstico fuera de catálogo ('DESCARTADO_MANUAL')",
     {"id_diagnostico": 9053, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "I10", "descripcion_diagnostico": "Hipertension", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "DESCARTADO_MANUAL"}),
    ("T3-14", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "SOSPECHOSO", "Tipo de diagnóstico fuera de catálogo ('SOSPECHOSO')",
     {"id_diagnostico": 9054, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "E11", "descripcion_diagnostico": "Diabetes", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "SOSPECHOSO"}),
    ("T3-15", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "DESCARTADO", "Tipo de diagnóstico fuera de catálogo ('DESCARTADO')",
     {"id_diagnostico": 9055, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "J00", "descripcion_diagnostico": "Rinofaringitis", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "DESCARTADO"}),
    ("T3-16", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "PRELIMINAR", "Tipo de diagnóstico fuera de catálogo ('PRELIMINAR')",
     {"id_diagnostico": 9056, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "K21", "descripcion_diagnostico": "Reflujo", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "PRELIMINAR"}),
    ("T3-17", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "DIFERENCIAL", "Tipo de diagnóstico fuera de catálogo ('DIFERENCIAL')",
     {"id_diagnostico": 9057, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "M54.5", "descripcion_diagnostico": "Lumbago", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "DIFERENCIAL"}),
    ("T3-18", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "CRONICO", "Tipo de diagnóstico fuera de catálogo ('CRONICO')",
     {"id_diagnostico": 9058, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "N39.0", "descripcion_diagnostico": "Infeccion urinaria", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CRONICO"}),
    ("T3-19", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "DEFINITIVO", "Tipo de diagnóstico fuera de catálogo ('DEFINITIVO')",
     {"id_diagnostico": 9059, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "J45", "descripcion_diagnostico": "Asma", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "DEFINITIVO"}),
    ("T3-20", "Condition", "tbl_diagnosticos", "s0", "tipo_diagnostico", "CURADO", "Tipo de diagnóstico fuera de catálogo ('CURADO')",
     {"id_diagnostico": 9060, "id_consulta": 1, "id_paciente": 1, "codigo_cie10": "A09", "descripcion_diagnostico": "Gastroenteritis", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CURADO"}),
]

for item in t3_definitions:
    cid, rtype, tbl, arch, field, val, desc, data = item
    cases.append({
        "id": cid,
        "category": "T3",
        "title": desc,
        "description": f"Violación T3: Valor '{val}' en '{field}' no está contemplado en el diccionario YAML.",
        "table": tbl,
        "resource_type": rtype,
        "archetype": arch,
        "fault_field": field,
        "injected_value": val,
        "data": data,
        "expected_status": 422,
        "expected_issue_code": "processing"
    })

# ==============================================================================
# CATEGORÍA T4: Llaves Foráneas Padres Rotas / Registros Huérfanos (20 casos: T4-01 a T4-20)
# ==============================================================================
t4_definitions = [
    # Consultas con id_paciente inexistente (5 casos)
    ("T4-01", "Encounter", "tbl_consultas", "s0", "id_paciente", 99991, "Consulta huérfana: id_paciente 99991 no existe",
     {"id_consulta": 9061, "id_paciente": 99991, "fecha_hora_inicio": "2026-10-01 09:00:00", "fecha_hora_fin": "2026-10-01 09:30:00", "motivo_consulta": "Control", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T4-02", "Encounter", "tbl_consultas", "s0", "id_paciente", 99992, "Consulta huérfana: id_paciente 99992 no existe",
     {"id_consulta": 9062, "id_paciente": 99992, "fecha_hora_inicio": "2026-10-01 10:00:00", "fecha_hora_fin": "2026-10-01 10:30:00", "motivo_consulta": "Cefalea", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T4-03", "Encounter", "tbl_consultas", "s0", "id_paciente", 99993, "Consulta huérfana: id_paciente 99993 no existe",
     {"id_consulta": 9063, "id_paciente": 99993, "fecha_hora_inicio": "2026-10-01 11:00:00", "fecha_hora_fin": "2026-10-01 11:30:00", "motivo_consulta": "Revision", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T4-04", "Encounter", "tbl_consultas", "s0", "id_paciente", 99994, "Consulta huérfana: id_paciente 99994 no existe",
     {"id_consulta": 9064, "id_paciente": 99994, "fecha_hora_inicio": "2026-10-01 12:00:00", "fecha_hora_fin": "2026-10-01 12:30:00", "motivo_consulta": "Dolor", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),
    ("T4-05", "Encounter", "tbl_consultas", "s0", "id_paciente", 99995, "Consulta huérfana: id_paciente 99995 no existe",
     {"id_consulta": 9065, "id_paciente": 99995, "fecha_hora_inicio": "2026-10-01 13:00:00", "fecha_hora_fin": "2026-10-01 13:30:00", "motivo_consulta": "Gripe", "estado_consulta": "FINALIZADA", "cedula_medico_tratante": "12345678", "nombre_medico": "DR. ROBERTO CRUZ"}),

    # Signos Vitales con id_consulta inexistente (5 casos)
    ("T4-06", "Observation", "tbl_signos_vitales", "s0", "id_consulta", 88881, "Signos vitales huérfanos: id_consulta 88881 no existe",
     {"id_signo": 9066, "id_consulta": 88881, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T4-07", "Observation", "tbl_signos_vitales", "s0", "id_consulta", 88882, "Signos vitales huérfanos: id_consulta 88882 no existe",
     {"id_signo": 9067, "id_consulta": 88882, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "body_temperature"),
    ("T4-08", "Observation", "tbl_signos_vitales", "s0", "id_consulta", 88883, "Signos vitales huérfanos: id_consulta 88883 no existe",
     {"id_signo": 9068, "id_consulta": 88883, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "heart_rate"),
    ("T4-09", "Observation", "tbl_signos_vitales", "s0", "id_consulta", 88884, "Signos vitales huérfanos: id_consulta 88884 no existe",
     {"id_signo": 9069, "id_consulta": 88884, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "respiratory_rate"),
    ("T4-10", "Observation", "tbl_signos_vitales", "s0", "id_consulta", 88885, "Signos vitales huérfanos: id_consulta 88885 no existe",
     {"id_signo": 9070, "id_consulta": 88885, "id_paciente": 1, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),

    # Signos Vitales con id_paciente inexistente (5 casos)
    ("T4-11", "Observation", "tbl_signos_vitales", "s0", "id_paciente", 77771, "Signos vitales huérfanos: id_paciente 77771 no existe",
     {"id_signo": 9071, "id_consulta": 1, "id_paciente": 77771, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),
    ("T4-12", "Observation", "tbl_signos_vitales", "s0", "id_paciente", 77772, "Signos vitales huérfanos: id_paciente 77772 no existe",
     {"id_signo": 9072, "id_consulta": 1, "id_paciente": 77772, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "body_temperature"),
    ("T4-13", "Observation", "tbl_signos_vitales", "s0", "id_paciente", 77773, "Signos vitales huérfanos: id_paciente 77773 no existe",
     {"id_signo": 9073, "id_consulta": 1, "id_paciente": 77773, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "heart_rate"),
    ("T4-14", "Observation", "tbl_signos_vitales", "s0", "id_paciente", 77774, "Signos vitales huérfanos: id_paciente 77774 no existe",
     {"id_signo": 9074, "id_consulta": 1, "id_paciente": 77774, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "respiratory_rate"),
    ("T4-15", "Observation", "tbl_signos_vitales", "s0", "id_paciente", 77775, "Signos vitales huérfanos: id_paciente 77775 no existe",
     {"id_signo": 9075, "id_consulta": 1, "id_paciente": 77775, "fecha_registro": "2026-10-01 09:15:00", "presion_sistolica": 120, "presion_diastolica": 80, "temperatura_celsius": 36.5, "frecuencia_cardiaca": 75, "frecuencia_respiratoria": 18}, "blood_pressure_panel"),

    # Diagnosticos con id_consulta o id_paciente inexistente (5 casos)
    ("T4-16", "Condition", "tbl_diagnosticos", "s0", "id_consulta", 66661, "Diagnóstico huérfano: id_consulta 66661 no existe",
     {"id_diagnostico": 9076, "id_consulta": 66661, "id_paciente": 1, "codigo_cie10": "I10", "descripcion_diagnostico": "Hipertension", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
    ("T4-17", "Condition", "tbl_diagnosticos", "s0", "id_consulta", 66662, "Diagnóstico huérfano: id_consulta 66662 no existe",
     {"id_diagnostico": 9077, "id_consulta": 66662, "id_paciente": 1, "codigo_cie10": "E11", "descripcion_diagnostico": "Diabetes", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
    ("T4-18", "Condition", "tbl_diagnosticos", "s0", "id_consulta", 66663, "Diagnóstico huérfano: id_consulta 66663 no existe",
     {"id_diagnostico": 9078, "id_consulta": 66663, "id_paciente": 1, "codigo_cie10": "J00", "descripcion_diagnostico": "Rinofaringitis", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
    ("T4-19", "Condition", "tbl_diagnosticos", "s0", "id_paciente", 55551, "Diagnóstico huérfano: id_paciente 55551 no existe",
     {"id_diagnostico": 9079, "id_consulta": 1, "id_paciente": 55551, "codigo_cie10": "K21", "descripcion_diagnostico": "Reflujo", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
    ("T4-20", "Condition", "tbl_diagnosticos", "s0", "id_paciente", 55552, "Diagnóstico huérfano: id_paciente 55552 no existe",
     {"id_diagnostico": 9080, "id_consulta": 1, "id_paciente": 55552, "codigo_cie10": "M54.5", "descripcion_diagnostico": "Lumbago", "fecha_diagnostico": "2026-10-01", "tipo_diagnostico": "CONFIRMADO"}),
]

for item in t4_definitions:
    cid, rtype, tbl, arch, field, val, desc, data = item[:8]
    obs_type = item[8] if len(item) > 8 else None
    cases.append({
        "id": cid,
        "category": "T4",
        "title": desc,
        "description": f"Violación T4: Integridad referencial activa rota. Registro padre inexistente con {field}={val}.",
        "table": tbl,
        "resource_type": rtype,
        "observation_type": obs_type,
        "archetype": arch,
        "fault_field": field,
        "injected_value": val,
        "data": data,
        "expected_status": 422,
        "expected_issue_code": "processing"
    })

# ==============================================================================
# CATEGORÍA T5: Sintaxis de Identificadores y Regex CURP Inválida (20 casos: T5-01 a T5-20)
# ==============================================================================
t5_definitions = [
    ("T5-01", "ROMA900101HCSMNL0", "CURP de longitud insuficiente (17 caracteres)"),
    ("T5-02", "ROMA900101HCSMNL099", "CURP de longitud excesiva (19 caracteres)"),
    ("T5-03", "ROMA900101HCSMNL0@", "CURP con carácter especial '@' al final"),
    ("T5-04", "roma900101hcsmnl09", "CURP en minúsculas (debe ser mayúsculas)"),
    ("T5-05", "1234900101HCSMNL09", "CURP con dígitos en prefijo de 4 letras"),
    ("T5-06", "ROM9900101HCSMNL09", "CURP con dígito en prefijo alfabético"),
    ("T5-07", "ROMA901301HCSMNL09", "CURP con mes 13 en fecha YYMMDD"),
    ("T5-08", "ROMA900001HCSMNL09", "CURP con mes 00 en fecha YYMMDD"),
    ("T5-09", "ROMA900132HCSMNL09", "CURP con día 32 en fecha YYMMDD"),
    ("T5-10", "ROMA900100HCSMNL09", "CURP con día 00 en fecha YYMMDD"),
    ("T5-11", "ROMA900101ZCSMNL09", "CURP con sexo inválido 'Z' (solo H, M, X)"),
    ("T5-12", "ROMA9001011CSMNL09", "CURP con dígito '1' en posición de sexo"),
    ("T5-13", "ROMA900101HXXMNL09", "CURP con entidad federativa inexistente 'XX'"),
    ("T5-14", "ROMA900101H99MNL09", "CURP con entidad federativa numérica '99'"),
    ("T5-15", "ROMA900101HCSAEI09", "CURP con vocales 'AEI' en lugar de 3 consonantes internas"),
    ("T5-16", "ROMA900101HCSMNA09", "CURP con vocal 'A' en consonante interna 3"),
    ("T5-17", "ROMA900101HCSMNL0A", "CURP con letra 'A' en último dígito verificador"),
    ("T5-18", "ROMA 900101HCSMNL0", "CURP con espacio en blanco intercalado"),
    ("T5-19", "ROMA-900101-HCS-09", "CURP con guiones de separación no permitidos"),
    ("T5-20", "ROMA900101HCSMNL!!", "CURP con signos de exclamación al final"),
]

for idx, (cid, bad_curp, desc) in enumerate(t5_definitions, start=1):
    pid = 9080 + idx
    cases.append({
        "id": cid,
        "category": "T5",
        "title": desc,
        "description": f"Violación T5: CURP '{bad_curp}' no cumple con la expresión regular oficial de RENAPO (18 caracteres).",
        "table": "tbl_pacientes",
        "resource_type": "Patient",
        "archetype": "s0",
        "fault_field": "curp",
        "injected_value": bad_curp,
        "data": {
            "id_paciente": pid,
            "curp": bad_curp,
            "primer_nombre": "VALENTINA",
            "segundo_nombre": None,
            "apellido_paterno": "MENDEZ",
            "apellido_materno": "CASTILLO",
            "fecha_nacimiento": "1990-01-01",
            "sexo_biologico": "F",
            "telefono_contacto": "9618765432"
        },
        "expected_status": 422,
        "expected_issue_code": "processing"
    })

# ==============================================================================
# Generación de Archivos de Salida
# ==============================================================================
output_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "tests", "fault_corpus")
os.makedirs(output_dir, exist_ok=True)

json_path = os.path.join(output_dir, "fault_corpus_100.json")
sql_path = os.path.join(output_dir, "fault_corpus_100.sql")
readme_path = os.path.join(output_dir, "README.md")

# 1. Guardar fault_corpus_100.json
with open(json_path, "w", encoding="utf-8") as f:
    json.dump(cases, f, indent=2, ensure_ascii=False)

print(f"[OK] Generado {json_path} con {len(cases)} casos.")

# 2. Guardar fault_corpus_100.sql
with open(sql_path, "w", encoding="utf-8") as f:
    f.write("-- =============================================================================\n")
    f.write("-- Corpus de Inyección de 100 Fallas Clínicas (T1–T5) — MedSys-FHIR v1.2.0-eval\n")
    f.write("-- Tesis UNACH 2026: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero\n")
    f.write("-- =============================================================================\n\n")
    f.write("CREATE SCHEMA IF NOT EXISTS fault_corpus;\n\n")

    # Copias de estructura sin FK restrictivas para permitir la inyección aislada de pruebas
    f.write("-- Estructuras de prueba en esquema aislado fault_corpus\n")
    f.write("CREATE TABLE IF NOT EXISTS fault_corpus.tbl_pacientes (\n")
    f.write("    id_paciente SERIAL PRIMARY KEY,\n")
    f.write("    curp VARCHAR(18),\n")
    f.write("    primer_nombre VARCHAR(50),\n")
    f.write("    segundo_nombre VARCHAR(50),\n")
    f.write("    apellido_paterno VARCHAR(50),\n")
    f.write("    apellido_materno VARCHAR(50),\n")
    f.write("    fecha_nacimiento TEXT,\n")
    f.write("    sexo_biologico VARCHAR(20),\n")
    f.write("    telefono_contacto VARCHAR(20)\n")
    f.write(");\n\n")

    f.write("CREATE TABLE IF NOT EXISTS fault_corpus.tbl_consultas (\n")
    f.write("    id_consulta SERIAL PRIMARY KEY,\n")
    f.write("    id_paciente INTEGER,\n")
    f.write("    fecha_hora_inicio TEXT,\n")
    f.write("    fecha_hora_fin TEXT,\n")
    f.write("    motivo_consulta TEXT,\n")
    f.write("    estado_consulta VARCHAR(30),\n")
    f.write("    cedula_medico_tratante VARCHAR(20),\n")
    f.write("    nombre_medico VARCHAR(100)\n")
    f.write(");\n\n")

    f.write("CREATE TABLE IF NOT EXISTS fault_corpus.tbl_signos_vitales (\n")
    f.write("    id_signo SERIAL PRIMARY KEY,\n")
    f.write("    id_consulta INTEGER,\n")
    f.write("    id_paciente INTEGER,\n")
    f.write("    fecha_registro TEXT,\n")
    f.write("    presion_sistolica TEXT,\n")
    f.write("    presion_diastolica TEXT,\n")
    f.write("    temperatura_celsius TEXT,\n")
    f.write("    frecuencia_cardiaca TEXT,\n")
    f.write("    frecuencia_respiratoria TEXT\n")
    f.write(");\n\n")

    f.write("CREATE TABLE IF NOT EXISTS fault_corpus.tbl_diagnosticos (\n")
    f.write("    id_diagnostico SERIAL PRIMARY KEY,\n")
    f.write("    id_consulta INTEGER,\n")
    f.write("    id_paciente INTEGER,\n")
    f.write("    codigo_cie10 VARCHAR(10),\n")
    f.write("    descripcion_diagnostico TEXT,\n")
    f.write("    fecha_diagnostico TEXT,\n")
    f.write("    tipo_diagnostico VARCHAR(30)\n")
    f.write(");\n\n")

    f.write("-- Inserción determinista de los 100 casos anómalos etiquetados\n")
    for case in cases:
        tbl = case["table"]
        cid = case["id"]
        title = case["title"]
        data = case["data"]
        f.write(f"-- [{cid}] {title}\n")

        # Generar INSERT según tabla
        if "pacientes" in tbl:
            cols = ["id_paciente", "curp", "primer_nombre", "segundo_nombre", "apellido_paterno", "apellido_materno", "fecha_nacimiento", "sexo_biologico", "telefono_contacto"]
            target_t = "fault_corpus.tbl_pacientes"
        elif "consultas" in tbl:
            cols = ["id_consulta", "id_paciente", "fecha_hora_inicio", "fecha_hora_fin", "motivo_consulta", "estado_consulta", "cedula_medico_tratante", "nombre_medico"]
            target_t = "fault_corpus.tbl_consultas"
        elif "signos" in tbl:
            if case["archetype"] == "s3":
                cols = ["id_signo", "id_consulta", "id_paciente", "fecha_registro", "presion_sistolica", "presion_diastolica", "temperatura_celsius", "frecuencia_cardiaca", "frecuencia_respiratoria"]
                target_t = "fault_corpus.tbl_signos_vitales"
                data = {
                    "id_signo": data["sig_id"],
                    "id_consulta": int(data["con_id"]),
                    "id_paciente": int(data["pac_id"]),
                    "fecha_registro": data["fech_reg_txt"],
                    "presion_sistolica": data.get("presion_arterial_txt"),
                    "presion_diastolica": None,
                    "temperatura_celsius": str(data["temp_c"]),
                    "frecuencia_cardiaca": str(data["fc"]),
                    "frecuencia_respiratoria": str(data["fr"]),
                }
            else:
                cols = ["id_signo", "id_consulta", "id_paciente", "fecha_registro", "presion_sistolica", "presion_diastolica", "temperatura_celsius", "frecuencia_cardiaca", "frecuencia_respiratoria"]
                target_t = "fault_corpus.tbl_signos_vitales"
        elif "diagnosticos" in tbl:
            cols = ["id_diagnostico", "id_consulta", "id_paciente", "codigo_cie10", "descripcion_diagnostico", "fecha_diagnostico", "tipo_diagnostico"]
            target_t = "fault_corpus.tbl_diagnosticos"

        vals = []
        for c in cols:
            v = data.get(c)
            if v is None:
                vals.append("NULL")
            elif isinstance(v, (int, float)):
                vals.append(str(v))
            else:
                clean_v = str(v).replace("'", "''")
                vals.append(f"'{clean_v}'")

        f.write(f"INSERT INTO {target_t} ({', '.join(cols)}) VALUES ({', '.join(vals)});\n\n")

print(f"[OK] Generado {sql_path} con 100 sentencias SQL.")

# 3. Guardar README.md
with open(readme_path, "w", encoding="utf-8") as f:
    f.write("# Corpus de Inyección de 100 Fallas Clínicas (`T1`–`T5`) — MedSys-FHIR\n\n")
    f.write("Este directorio contiene el corpus experimental determinista de **100 casos de prueba anómalos** ")
    f.write("diseñados para la evaluación cuantitativa de los Objetivos Específicos 2 y 3 (Capítulo III, §3.2.3 y Tabla 1) ")
    f.write("de la tesis de licenciatura de Carlos Eduardo Iglesias de la Cruz y Alexis Andrey Gálvez Roblero (UNACH, 2026).\n\n")
    f.write("## 1. Taxonomía de las 5 Categorías de Falla (`T1`–`T5`)\n\n")
    f.write("| Categoría | Código | Casos | Causa Raíz Simulada | Comportamiento Normativo Esperado |\n")
    f.write("|-----------|--------|-------|---------------------|-----------------------------------|\n")
    f.write("| **T1** | `T1-01` a `T1-20` | 20 | Campos críticos obligatorios nulos o vacíos (`primer_nombre`, `curp`, `fecha_nacimiento`, `presion_sistolica`, etc.) | Rechazo HTTP 422 `OperationOutcome` (`code = \"processing\"`) |\n")
    f.write("| **T2** | `T2-01` a `T2-20` | 20 | Tipos de datos corruptos, cadenas no numéricas y fechas imposibles (ej. 30 de febrero, mes 13, `ALTA`, `120/XX`) | Rechazo HTTP 422 `OperationOutcome` (`code = \"processing\"`) |\n")
    f.write("| **T3** | `T3-01` a `T3-20` | 20 | Valores categóricos fuera de catálogo/diccionario (`sexo = \"X\"`, `estado = \"ABIERTO\"`, `tipo = \"DESCARTADO\"`) | Rechazo HTTP 422 `OperationOutcome` (`code = \"processing\"`) |\n")
    f.write("| **T4** | `T4-01` a `T4-20` | 20 | Llaves foráneas rotas y registros huérfanos (`id_paciente = 99991`, `id_consulta = 88881`) | Rechazo HTTP 422 `OperationOutcome` (`code = \"processing\"`) |\n")
    f.write("| **T5** | `T5-01` a `T5-20` | 20 | Identificadores y sintaxis CURP violando la regex oficial de RENAPO (17/19 car., caracteres inválidos, mes 13, sexo Z) | Rechazo HTTP 422 `OperationOutcome` (`code = \"processing\"`) |\n\n")
    f.write("## 2. Archivos del Corpus\n\n")
    f.write("- `fault_corpus_100.json`: Catálogo estructurado en JSON con los 100 casos etiquetados individualmente con sus metadatos y payload de prueba.\n")
    f.write("- `fault_corpus_100.sql`: Script DDL y DML para inyectar los 100 casos en un esquema aislado de PostgreSQL (`fault_corpus`).\n")
    f.write("- `README.md`: Este documento de contexto técnico.\n\n")
    f.write("## 3. Criterio de Aprobación Experimental\n\n")
    f.write("1. **Sensibilidad de Rechazo:** 100 de 100 casos anómalos (100.0 %) deben ser rechazados de manera cerrada (*Fail-Closed*) con HTTP 422 `OperationOutcome` y código de issue `processing`.\n")
    f.write("2. **Especificidad:** Cero falsos rechazos (0.0 %) sobre los 9,000 registros válidos del universo censal.\n")

print(f"[OK] Generado {readme_path}.")
