#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Generador de Datos Clínicos Sintéticos Reproducibles (Capítulo III)
# Basado en normatividad NOM-004-SSA3-2012 y NOM-024-SSA3-2012
# Semilla: 20260930 (Evaluación Experimental UNACH 2026)
# Soporta arquetipos relacionales: S0 (base), S1 (plana), S2 (lookup), S3 (legado)
# ==============================================================================

import argparse
import hashlib
import json
import os
import random
import re
from datetime import datetime, timedelta, date

DEFAULT_SEED = 20260930
SQL_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "docker", "sql"))
DEFAULT_S0_OUTPUT = os.path.join(SQL_DIR, "02_bulk_data.sql")
DEFAULT_S1_OUTPUT = os.path.join(SQL_DIR, "bulk_s1.sql")
DEFAULT_S2_OUTPUT = os.path.join(SQL_DIR, "bulk_s2.sql")
DEFAULT_S3_OUTPUT = os.path.join(SQL_DIR, "bulk_s3.sql")

CURP_RENAPO_REGEX = re.compile(r"^[A-Z]{4}\d{6}[HMX][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d$")

FIRST_NAMES_M = [
    "Alberto", "Carlos", "Alexis", "Manuel", "Juan", "Pedro", "Luis", "Jorge",
    "Miguel", "Alejandro", "Roberto", "Fernando", "Ricardo", "Eduardo", "Daniel",
    "Javier", "David", "Jose", "Francisco", "Antonio", "Mario", "Gabriel",
    "Hugo", "Raul", "Armando", "Arturo", "Enrique", "Rogelio", "Salvador", "Andres"
]

FIRST_NAMES_F = [
    "Mariana", "Maria", "Elena", "Sofia", "Carmen", "Ana", "Laura", "Patricia",
    "Claudia", "Gabriela", "Rosa", "Adriana", "Guadalupe", "Martha", "Leticia",
    "Veronica", "Teresa", "Silvia", "Yolanda", "Beatriz", "Gloria", "Daniela",
    "Lucia", "Fernanda", "Alejandra", "Paola", "Karla", "Lorena", "Natalia", "Monica"
]

LAST_NAMES = [
    "Ramos", "Lopez", "Hernandez", "Garcia", "Martinez", "Gonzalez", "Rodriguez",
    "Perez", "Sanchez", "Ramirez", "Flores", "Gomez", "Torres", "Diaz",
    "Vasquez", "Castro", "Morales", "Ortiz", "Gutierrez", "Chavez", "Ruiz",
    "Alvarez", "Mendoza", "Juarez", "Castillo", "Jimenez", "Reyes", "Moreno",
    "Herrera", "Medina", "Aguilar", "Vargas", "Guzman", "Mendez", "Munoz"
]

DOCTORS = [
    ("8472910", "Dra. María Elena Cruz Martínez"),
    ("9182374", "Dr. Roberto Mendoza Solís"),
    ("7362915", "Dr. Alejandro Morales Domínguez"),
    ("6284910", "Dra. Patricia Aguilar Cárdenas"),
    ("5918273", "Dr. Jorge Luis Vázquez Toledo"),
    ("8274019", "Dra. Carmen Rodríguez Guillén"),
    ("9481726", "Dr. Fernando Gómez Coutiño"),
    ("7102948", "Dra. Sofía Hernández Gordillo")
]

CLINICS = [
    "CESSA Tuxtla Poniente",
    "Centro de Salud Urbano Tuxtla",
    "Hospital Básico Comunitario Berriozábal",
    "Centro de Salud Santa Cruz",
    "CESSA San Cristóbal"
]

CIE10_CODES = [
    ("I10", "Hipertensión esencial (primaria)"),
    ("E11.9", "Diabetes mellitus tipo 2 sin mención de complicación"),
    ("J00", "Rinofaringitis aguda (resfriado común)"),
    ("K29.7", "Gastritis, no especificada"),
    ("J20.9", "Bronquitis aguda, no especificada"),
    ("M54.5", "Lumbago no especificado"),
    ("N39.0", "Infección de vías urinarias, sitio no especificado"),
    ("J02.9", "Faringitis aguda, no especificada"),
    ("R51", "Cefalea"),
    ("E78.5", "Hiperlipidemia, no especificada"),
    ("K21.9", "Enfermedad por reflujo gastroesofágico sin esofagitis"),
    ("G43.9", "Migraña, no especificada"),
    ("A09", "Diarrea y gastroenteritis de presunto origen infeccioso"),
    ("J03.9", "Amigdalitis aguda, no especificada"),
    ("B34.9", "Infección viral, no especificada"),
    ("L30.9", "Dermatitis, no especificada"),
    ("M79.1", "Mialgia"),
    ("K30", "Dispepsia funcional"),
    ("H10.9", "Conjuntivitis, no especificada"),
    ("F41.9", "Trastorno de ansiedad, no especificado")
]

# Catálogo S2: mapeo canónico de claves CIE-10 a ID entero (IDs 1, 2 y 3 vienen en DDL S2)
CIE10_CATALOG_S2 = [
    (1, "I10", "Hipertensión esencial (primaria)"),
    (2, "G43.9", "Migraña, no especificada"),
    (3, "E11.9", "Diabetes mellitus tipo 2 sin mención de complicación"),
    (4, "J00", "Rinofaringitis aguda (resfriado común)"),
    (5, "K29.7", "Gastritis, no especificada"),
    (6, "J20.9", "Bronquitis aguda, no especificada"),
    (7, "M54.5", "Lumbago no especificado"),
    (8, "N39.0", "Infección de vías urinarias, sitio no especificado"),
    (9, "J02.9", "Faringitis aguda, no especificada"),
    (10, "R51", "Cefalea"),
    (11, "E78.5", "Hiperlipidemia, no especificada"),
    (12, "K21.9", "Enfermedad por reflujo gastroesofágico sin esofagitis"),
    (13, "A09", "Diarrea y gastroenteritis de presunto origen infeccioso"),
    (14, "J03.9", "Amigdalitis aguda, no especificada"),
    (15, "B34.9", "Infección viral, no especificada"),
    (16, "L30.9", "Dermatitis, no especificada"),
    (17, "M79.1", "Mialgia"),
    (18, "K30", "Dispepsia funcional"),
    (19, "H10.9", "Conjuntivitis, no especificada"),
    (20, "F41.9", "Trastorno de ansiedad, no especificado"),
]
CIE10_MAP_TO_ID = {item[1]: item[0] for item in CIE10_CATALOG_S2}

REASONS = [
    "Control trimestral de hipertensión arterial",
    "Cefalea persistente y mareos",
    "Fiebre y dolor faríngeo de 3 días de evolución",
    "Control de glucemia y ajuste terapéutico",
    "Dolor epigástrico urente posprandial",
    "Tos productiva y malestar general",
    "Lumbalgia mecánica tras esfuerzo físico",
    "Disuria y polaquiuria de inicio súbito",
    "Evaluación médica preventiva anual",
    "Diarrea acuosa y dolor abdominal tipo cólico"
]

def get_internal_consonant(s: str) -> str:
    s_upper = s.upper()
    consonants = "BCDFGHJKLMNPQRSTVWXYZ"
    for char in s_upper[1:]:
        if char in consonants:
            return char
    return "X"

def get_internal_vowel(s: str) -> str:
    s_upper = s.upper()
    vowels = "AEIOU"
    for char in s_upper[1:]:
        if char in vowels:
            return char
    return "X"

def generate_curp(paterno: str, materno: str, nombre: str, bdate: date, sex: str, idx: int, used_curps: set) -> str:
    c1 = paterno[0].upper()
    c2 = get_internal_vowel(paterno)
    c3 = materno[0].upper() if materno else "X"
    c4 = nombre[0].upper()

    yy = f"{bdate.year % 100:02d}"
    mm = f"{bdate.month:02d}"
    dd = f"{bdate.day:02d}"

    curp_sex = "H" if sex == "M" else ("M" if sex == "F" else "X")
    state = "CS"  # Chiapas

    c14 = get_internal_consonant(paterno)
    c15 = get_internal_consonant(materno) if materno else "X"
    c16 = get_internal_consonant(nombre)

    homo = f"{(idx % 90 + 10):02d}"

    curp = f"{c1}{c2}{c3}{c4}{yy}{mm}{dd}{curp_sex}{state}{c14}{c15}{c16}{homo}"
    if len(curp) != 18 or curp in used_curps:
        alpha = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        c17 = alpha[(idx // 10) % 36]
        c18 = str(idx % 10)
        curp = f"{c1}{c2}{c3}{c4}{yy}{mm}{dd}{curp_sex}{state}{c14}{c15}{c16}{c17}{c18}"

    assert CURP_RENAPO_REGEX.match(curp) is not None, f"CURP sintética inválida: '{curp}'"
    used_curps.add(curp)
    return curp

def sql_quote(val) -> str:
    if val is None:
        return "NULL"
    if isinstance(val, (int, float)):
        return str(val)
    escaped = str(val).replace("'", "''")
    return f"'{escaped}'"

def to_date_dmy(iso_date: str) -> str:
    """Convierte 'YYYY-MM-DD' a 'DD/MM/YYYY' para el arquetipo S3."""
    parts = iso_date.split("-")
    if len(parts) == 3:
        return f"{parts[2]}/{parts[1]}/{parts[0]}"
    return iso_date

def to_datetime_dmy(iso_datetime: str) -> str:
    """Convierte 'YYYY-MM-DD HH:MM:SS' a 'DD/MM/YYYY HH:MM:SS' para el arquetipo S3."""
    if " " in iso_datetime:
        date_part, time_part = iso_datetime.split(" ", 1)
        return f"{to_date_dmy(date_part)} {time_part}"
    return iso_datetime

# ==============================================================================
# Generación en Memoria de Entidades Sintéticas Deterministas
# ==============================================================================

def generate_entities(seed: int = DEFAULT_SEED):
    rng = random.Random(seed)
    used_curps = {"ROMA900101HCSMNL09", "LOPE950512MCSPRN07"}

    # Registros canónicos preexistentes en DDL inicial (IDs 1 y 2)
    p1 = {
        "id_paciente": 1,
        "curp": "ROMA900101HCSMNL09",
        "primer_nombre": "Alberto",
        "segundo_nombre": "Manuel",
        "apellido_paterno": "Ramos",
        "apellido_materno": "Gómez",
        "fecha_nacimiento": "1990-01-01",
        "sexo_biologico": "M",
        "telefono_contacto": "9611234567"
    }
    p2 = {
        "id_paciente": 2,
        "curp": "LOPE950512MCSPRN07",
        "primer_nombre": "Mariana",
        "segundo_nombre": None,
        "apellido_paterno": "López",
        "apellido_materno": "Hernández",
        "fecha_nacimiento": "1995-05-12",
        "sexo_biologico": "F",
        "telefono_contacto": "9619876543"
    }
    all_pacientes = {1: p1, 2: p2}
    pacientes_bulk = []

    # 1,000 Pacientes en total (IDs 1..1000).
    # Metas exactas en BD:
    #   - 15% (150) segundo_nombre = NULL: Paciente 2 ya tiene NULL, así que de los 998 generados exactamente 149 tienen NULL (1 + 149 = 150).
    #   - 10% (100) telefono_contacto = NULL: Pacientes 1 y 2 tienen teléfono, así que de los 998 generados exactamente 100 tienen NULL (0 + 100 = 100).
    null_sname_idx = set(rng.sample(range(998), 149))
    null_phone_idx = set(rng.sample(range(998), 100))
    null_materno_idx = set(rng.sample(range(998), 150))

    for idx in range(998):
        pid = idx + 3
        is_male = rng.random() < 0.5
        sex = "M" if is_male else "F"
        p_name = rng.choice(FIRST_NAMES_M if is_male else FIRST_NAMES_F)
        s_name = None if idx in null_sname_idx else rng.choice(FIRST_NAMES_M if is_male else FIRST_NAMES_F)
        paterno = rng.choice(LAST_NAMES)
        materno = None if idx in null_materno_idx else rng.choice(LAST_NAMES)

        days_offset = rng.randint(8 * 365, 80 * 365)
        bdate = date(2026, 1, 1) - timedelta(days=days_offset)

        curp = generate_curp(paterno, materno, p_name, bdate, sex, pid, used_curps)
        phone = None if idx in null_phone_idx else f"961{rng.randint(1000000, 9999999)}"

        pac = {
            "id_paciente": pid,
            "curp": curp,
            "primer_nombre": p_name,
            "segundo_nombre": s_name,
            "apellido_paterno": paterno,
            "apellido_materno": materno,
            "fecha_nacimiento": bdate.isoformat(),
            "sexo_biologico": sex,
            "telefono_contacto": phone
        }
        all_pacientes[pid] = pac
        pacientes_bulk.append(pac)

    # 2,500 Consultas en total (IDs 1..2500)
    c1 = {
        "id_consulta": 1,
        "id_paciente": 1,
        "cedula": "8472910",
        "medico": "Dra. María Elena Cruz Martínez",
        "estado": "FINALIZADA",
        "motivo": "Control trimestral de hipertensión arterial",
        "inicio": "2026-09-18 09:15:00",
        "fin": "2026-09-18 09:40:00",
        "unidad": "CESSA Tuxtla Poniente"
    }
    c2 = {
        "id_consulta": 2,
        "id_paciente": 2,
        "cedula": "9182374",
        "medico": "Dr. Roberto Mendoza Solís",
        "estado": "FINALIZADA",
        "motivo": "Cefalea persistente y mareos",
        "inicio": "2026-09-18 10:00:00",
        "fin": "2026-09-18 10:25:00",
        "unidad": "CESSA Tuxtla Poniente"
    }
    all_consultas = [c1, c2]
    consultas_bulk = []
    base_date = datetime(2025, 1, 15, 8, 0, 0)

    for idx in range(2498):
        cid = idx + 3
        pid = rng.randint(1, 1000)
        doctor = rng.choice(DOCTORS)
        clinic = rng.choice(CLINICS)
        reason = rng.choice(REASONS)
        status = "FINALIZADA" if rng.random() < 0.94 else ("EN_CURSO" if rng.random() < 0.5 else "CANCELADA")

        dt_start = base_date + timedelta(minutes=rng.randint(0, 600 * 24 * 60))
        duration = rng.randint(15, 45)
        dt_end = dt_start + timedelta(minutes=duration)

        con = {
            "id_consulta": cid,
            "id_paciente": pid,
            "cedula": doctor[0],
            "medico": doctor[1],
            "estado": status,
            "motivo": reason,
            "inicio": dt_start.strftime("%Y-%m-%d %H:%M:%S"),
            "fin": dt_end.strftime("%Y-%m-%d %H:%M:%S"),
            "unidad": clinic
        }
        all_consultas.append(con)
        consultas_bulk.append(con)

    # S1 stores demographics only in encounter rows. Random assignment alone
    # left95 of the1,000 patients unrepresented at the original default seed.
    # Reassign the minimum number of duplicate encounter owners, preserving
    # the RNG stream, encounter count, and the two DDL seed records. Signs and
    # diagnoses are generated below from these same encounter dictionaries.
    from collections import Counter
    owner_counts = Counter(c['id_paciente'] for c in all_consultas)
    missing_patients = sorted(set(all_pacientes) - set(owner_counts))
    available = iter(reversed(consultas_bulk))
    for missing_pid in missing_patients:
        for candidate in available:
            previous_pid = candidate['id_paciente']
            if owner_counts[previous_pid] > 1:
                owner_counts[previous_pid] -= 1
                candidate['id_paciente'] = missing_pid
                owner_counts[missing_pid] += 1
                break
        else:
            raise ValueError('Insufficient encounters to represent every synthetic patient in S1')
    assert set(owner_counts) == set(all_pacientes)

    # 2,500 Signos Vitales (IDs 1..2500, vinculados 1:1 con consultas 1..2500)
    # Metas exactas en BD:
    #   - 10% (250) frecuencia_respiratoria = NULL. Signo 2 ya tiene NULL en DDL y Signo 1 tiene 18.
    #     Por tanto, de los 2,498 generados exactamente 249 tienen NULL (1 + 249 = 250).
    #   - 90% (2,250) frecuencia_respiratoria entre 14 y 22 /min (1 + 2249 = 2250).
    #   - Observaciones resultantes: 2,500 BP + 2,500 Temp + 2,500 FC + 2,250 FR = 9,750 Observation.
    null_rr_idx = set(rng.sample(range(2498), 249))
    s1 = {
        "id_signo": 1,
        "id_consulta": 1,
        "id_paciente": 1,
        "sistolica": 130,
        "diastolica": 85,
        "frecuencia_cardiaca": 76,
        "frecuencia_respiratoria": 18,
        "temperatura": 36.6,
        "peso": 78.50,
        "talla": 172,
        "registro": "2026-09-18 09:18:00"
    }
    s2 = {
        "id_signo": 2,
        "id_consulta": 2,
        "id_paciente": 2,
        "sistolica": 110,
        "diastolica": 70,
        "frecuencia_cardiaca": 80,
        "frecuencia_respiratoria": None,
        "temperatura": 37.1,
        "peso": 62.00,
        "talla": 160,
        "registro": "2026-09-18 10:05:00"
    }
    all_signos = [s1, s2]
    signos_bulk = []

    for idx in range(2498):
        cid = idx + 3
        consulta = consultas_bulk[idx]
        pid = consulta["id_paciente"]

        systolic = rng.randint(100, 155)
        diastolic = rng.randint(60, 95)
        hr = rng.randint(58, 105)
        rr = None if idx in null_rr_idx else rng.randint(14, 22)
        temp = round(rng.uniform(36.1, 37.6), 1)
        weight = round(rng.uniform(48.0, 98.0), 2)
        height = rng.randint(150, 185)

        sig = {
            "id_signo": cid,
            "id_consulta": cid,
            "id_paciente": pid,
            "sistolica": systolic,
            "diastolica": diastolic,
            "frecuencia_cardiaca": hr,
            "frecuencia_respiratoria": rr,
            "temperatura": temp,
            "peso": weight,
            "talla": height,
            "registro": consulta["inicio"]
        }
        all_signos.append(sig)
        signos_bulk.append(sig)

    # 3,000 Diagnósticos en total (IDs 1..3000)
    # Metas exactas en BD:
    #   - 10% (300) tipo_diagnostico = NULL. Diagnósticos 1 y 2 tienen 'CONFIRMADO'.
    #     Por tanto, de los 2,998 generados exactamente 300 tienen NULL (0 + 300 = 300).
    #   - 90% (2,700) distribuidos entre 'CONFIRMADO' y 'PRESUNTIVO' (2 + 2698 = 2700).
    null_diag_type_idx = set(rng.sample(range(2998), 300))
    d1 = {
        "id_diagnostico": 1,
        "id_consulta": 1,
        "id_paciente": 1,
        "codigo_cie10": "I10",
        "descripcion": "Hipertensión esencial (primaria)",
        "tipo": "CONFIRMADO",
        "fecha": "2026-09-18"
    }
    d2 = {
        "id_diagnostico": 2,
        "id_consulta": 2,
        "id_paciente": 2,
        "codigo_cie10": "G43.9",
        "descripcion": "Migraña, no especificada",
        "tipo": "CONFIRMADO",
        "fecha": "2026-09-18"
    }
    all_diagnosticos = [d1, d2]
    diagnosticos_bulk = []
    diag_id = 3

    # Primeros 2498 diagnósticos: 1 por consulta (IDs 3..2500)
    for idx in range(2498):
        consulta = consultas_bulk[idx]
        code, desc = rng.choice(CIE10_CODES)
        diag_type = None if (diag_id - 3) in null_diag_type_idx else ("CONFIRMADO" if rng.random() < 0.88 else "PRESUNTIVO")

        diag = {
            "id_diagnostico": diag_id,
            "id_consulta": consulta["id_consulta"],
            "id_paciente": consulta["id_paciente"],
            "codigo_cie10": code,
            "descripcion": desc,
            "tipo": diag_type,
            "fecha": consulta["inicio"][:10]
        }
        all_diagnosticos.append(diag)
        diagnosticos_bulk.append(diag)
        diag_id += 1

    # 500 diagnósticos secundarios (IDs 2501..3000)
    multidiag_consultas = rng.sample(consultas_bulk, 500)
    for consulta in multidiag_consultas:
        code, desc = rng.choice(CIE10_CODES)
        diag_type = None if (diag_id - 3) in null_diag_type_idx else ("CONFIRMADO" if rng.random() < 0.85 else "PRESUNTIVO")

        diag = {
            "id_diagnostico": diag_id,
            "id_consulta": consulta["id_consulta"],
            "id_paciente": consulta["id_paciente"],
            "codigo_cie10": code,
            "descripcion": desc,
            "tipo": diag_type,
            "fecha": consulta["inicio"][:10]
        }
        all_diagnosticos.append(diag)
        diagnosticos_bulk.append(diag)
        diag_id += 1

    return all_pacientes, pacientes_bulk, all_consultas, consultas_bulk, all_signos, signos_bulk, all_diagnosticos, diagnosticos_bulk

# ==============================================================================
# Generadores de Archivos SQL por Arquetipo Relacional (S0, S1, S2, S3)
# ==============================================================================

def generate_sql_s0(seed: int, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk) -> str:
    lines = [
        "-- ==============================================================================",
        "-- ARQUETIPO S0: DATOS SINTÉTICOS MASIVOS REPRODUCIBLES (NOM-004-SSA3-2012)",
        f"-- Semilla fija: {seed} | MedSys-FHIR Evaluación Capítulo III",
        "-- Censo: 1,000 Pacientes / 2,500 Consultas / 9,750 Observaciones / 3,000 Diagnósticos",
        "-- Total recursos FHIR R4 resultantes: 16,250",
        "-- ==============================================================================\n",
        "-- 1. Inserción tbl_pacientes (IDs 3 al 1000)"
    ]
    for p in pacientes_bulk:
        lines.append(
            f"INSERT INTO tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) "
            f"VALUES ({p['id_paciente']}, {sql_quote(p['curp'])}, {sql_quote(p['primer_nombre'])}, {sql_quote(p['segundo_nombre'])}, {sql_quote(p['apellido_paterno'])}, {sql_quote(p['apellido_materno'])}, '{p['fecha_nacimiento']}', '{p['sexo_biologico']}', {sql_quote(p['telefono_contacto'])}) "
            f"ON CONFLICT (id_paciente) DO NOTHING;"
        )

    lines.append("\n-- 2. Inserción tbl_consultas (IDs 3 al 2500)")
    for c in consultas_bulk:
        lines.append(
            f"INSERT INTO tbl_consultas (id_consulta, id_paciente, cedula_medico_tratante, nombre_medico, estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin, unidad_medica) "
            f"VALUES ({c['id_consulta']}, {c['id_paciente']}, {sql_quote(c['cedula'])}, {sql_quote(c['medico'])}, '{c['estado']}', {sql_quote(c['motivo'])}, '{c['inicio']}', '{c['fin']}', {sql_quote(c['unidad'])}) "
            f"ON CONFLICT (id_consulta) DO NOTHING;"
        )

    lines.append("\n-- 3. Inserción tbl_signos_vitales (IDs 3 al 2500)")
    for s in signos_bulk:
        fr_val = str(s['frecuencia_respiratoria']) if s['frecuencia_respiratoria'] is not None else "NULL"
        lines.append(
            f"INSERT INTO tbl_signos_vitales (id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius, peso_kg, talla_cm, fecha_registro) "
            f"VALUES ({s['id_signo']}, {s['id_consulta']}, {s['id_paciente']}, {s['sistolica']}, {s['diastolica']}, {s['frecuencia_cardiaca']}, {fr_val}, {s['temperatura']}, {s['peso']}, {s['talla']}, '{s['registro']}') "
            f"ON CONFLICT (id_signo) DO NOTHING;"
        )

    lines.append("\n-- 4. Inserción tbl_diagnosticos (IDs 3 al 3000)")
    for d in diagnosticos_bulk:
        tipo_val = sql_quote(d['tipo'])
        lines.append(
            f"INSERT INTO tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico) "
            f"VALUES ({d['id_diagnostico']}, {d['id_consulta']}, {d['id_paciente']}, {sql_quote(d['codigo_cie10'])}, {sql_quote(d['descripcion'])}, {tipo_val}, '{d['fecha']}') "
            f"ON CONFLICT (id_diagnostico) DO NOTHING;"
        )

    lines.extend([
        "\n-- 5. Actualización de secuencias de claves primarias",
        "SELECT setval('tbl_pacientes_id_paciente_seq', (SELECT COALESCE(MAX(id_paciente), 1) FROM tbl_pacientes));",
        "SELECT setval('tbl_consultas_id_consulta_seq', (SELECT COALESCE(MAX(id_consulta), 1) FROM tbl_consultas));",
        "SELECT setval('tbl_signos_vitales_id_signo_seq', (SELECT COALESCE(MAX(id_signo), 1) FROM tbl_signos_vitales));",
        "SELECT setval('tbl_diagnosticos_id_diagnostico_seq', (SELECT COALESCE(MAX(id_diagnostico), 1) FROM tbl_diagnosticos));\n"
    ])
    return "\n".join(lines)

def generate_sql_s1(seed: int, all_pacientes, consultas_bulk, signos_bulk, diagnosticos_bulk) -> str:
    lines = [
        "-- ==============================================================================",
        "-- ARQUETIPO S1: DATOS SINTÉTICOS MASIVOS — SÁBANA PLANA DESNORMALIZADA (SINBA/SIS)",
        f"-- Semilla fija: {seed} | MedSys-FHIR Evaluación Capítulo III",
        "-- ==============================================================================\n",
        "-- 1. Inserción tbl_sabana_consulta_externa (IDs 3 al 2500)"
    ]
    for idx, c in enumerate(consultas_bulk):
        s = signos_bulk[idx]
        p = all_pacientes[c["id_paciente"]]
        fr_val = str(s['frecuencia_respiratoria']) if s['frecuencia_respiratoria'] is not None else "NULL"
        lines.append(
            f"INSERT INTO tbl_sabana_consulta_externa ("
            f"id_consulta, id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, "
            f"fecha_nacimiento, sexo_biologico, telefono_contacto, cedula_medico_tratante, nombre_medico, "
            f"estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin, unidad_medica, "
            f"id_signo, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria, "
            f"temperatura_celsius, peso_kg, talla_cm, fecha_registro"
            f") VALUES ("
            f"{c['id_consulta']}, {p['id_paciente']}, {sql_quote(p['curp'])}, {sql_quote(p['primer_nombre'])}, {sql_quote(p['segundo_nombre'])}, "
            f"{sql_quote(p['apellido_paterno'])}, {sql_quote(p['apellido_materno'])}, '{p['fecha_nacimiento']}', '{p['sexo_biologico']}', {sql_quote(p['telefono_contacto'])}, "
            f"{sql_quote(c['cedula'])}, {sql_quote(c['medico'])}, '{c['estado']}', {sql_quote(c['motivo'])}, '{c['inicio']}', '{c['fin']}', {sql_quote(c['unidad'])}, "
            f"{s['id_signo']}, {s['sistolica']}, {s['diastolica']}, {s['frecuencia_cardiaca']}, {fr_val}, {s['temperatura']}, {s['peso']}, {s['talla']}, '{s['registro']}'"
            f") ON CONFLICT (id_consulta) DO NOTHING;"
        )

    lines.append("\n-- 2. Inserción tbl_sabana_diagnosticos (IDs 3 al 3000)")
    for d in diagnosticos_bulk:
        tipo_val = sql_quote(d['tipo'])
        lines.append(
            f"INSERT INTO tbl_sabana_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico) "
            f"VALUES ({d['id_diagnostico']}, {d['id_consulta']}, {d['id_paciente']}, {sql_quote(d['codigo_cie10'])}, {sql_quote(d['descripcion'])}, {tipo_val}, '{d['fecha']}') "
            f"ON CONFLICT (id_diagnostico) DO NOTHING;"
        )

    lines.extend([
        "\n-- 3. Actualización de secuencias",
        "SELECT setval('tbl_sabana_consulta_externa_id_consulta_seq', (SELECT COALESCE(MAX(id_consulta), 1) FROM tbl_sabana_consulta_externa));",
        "SELECT setval('tbl_sabana_diagnosticos_id_diagnostico_seq', (SELECT COALESCE(MAX(id_diagnostico), 1) FROM tbl_sabana_diagnosticos));\n"
    ])
    return "\n".join(lines)

def generate_sql_s2(seed: int, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk) -> str:
    lines = [
        "-- ==============================================================================",
        "-- ARQUETIPO S2: DATOS SINTÉTICOS MASIVOS — ESQUEMA NORMALIZADO CON CATÁLOGOS (LOOKUP)",
        f"-- Semilla fija: {seed} | MedSys-FHIR Evaluación Capítulo III",
        "-- ==============================================================================\n",
        "-- 1. Población de Catálogo cat_cie10 (IDs 4 al 20)"
    ]
    for cid_item, clave, desc in CIE10_CATALOG_S2:
        if cid_item >= 4:
            lines.append(
                f"INSERT INTO cat_cie10 (id_cie10, clave, descripcion) "
                f"VALUES ({cid_item}, '{clave}', {sql_quote(desc)}) "
                f"ON CONFLICT (id_cie10) DO NOTHING;"
            )

    lines.append("\n-- 2. Inserción tbl_pacientes_s2 (IDs 3 al 1000)")
    # M -> 1, F -> 2, I -> 3
    sex_to_id = {"M": 1, "F": 2, "I": 3}
    for p in pacientes_bulk:
        id_sexo = sex_to_id.get(p["sexo_biologico"], 1)
        lines.append(
            f"INSERT INTO tbl_pacientes_s2 (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, id_sexo, telefono_contacto) "
            f"VALUES ({p['id_paciente']}, {sql_quote(p['curp'])}, {sql_quote(p['primer_nombre'])}, {sql_quote(p['segundo_nombre'])}, {sql_quote(p['apellido_paterno'])}, {sql_quote(p['apellido_materno'])}, '{p['fecha_nacimiento']}', {id_sexo}, {sql_quote(p['telefono_contacto'])}) "
            f"ON CONFLICT (id_paciente) DO NOTHING;"
        )

    lines.append("\n-- 3. Inserción tbl_consultas_s2 (IDs 3 al 2500)")
    # FINALIZADA -> 1, EN_CURSO -> 2, CANCELADA -> 3
    estado_to_id = {"FINALIZADA": 1, "EN_CURSO": 2, "CANCELADA": 3}
    for c in consultas_bulk:
        id_estado = estado_to_id.get(c["estado"], 1)
        lines.append(
            f"INSERT INTO tbl_consultas_s2 (id_consulta, id_paciente, cedula_medico_tratante, nombre_medico, id_estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin, unidad_medica) "
            f"VALUES ({c['id_consulta']}, {c['id_paciente']}, {sql_quote(c['cedula'])}, {sql_quote(c['medico'])}, {id_estado}, {sql_quote(c['motivo'])}, '{c['inicio']}', '{c['fin']}', {sql_quote(c['unidad'])}) "
            f"ON CONFLICT (id_consulta) DO NOTHING;"
        )

    lines.append("\n-- 4. Inserción tbl_signos_s2 (IDs 3 al 2500)")
    for s in signos_bulk:
        fr_val = str(s['frecuencia_respiratoria']) if s['frecuencia_respiratoria'] is not None else "NULL"
        lines.append(
            f"INSERT INTO tbl_signos_s2 (id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius, peso_kg, talla_cm, fecha_registro) "
            f"VALUES ({s['id_signo']}, {s['id_consulta']}, {s['id_paciente']}, {s['sistolica']}, {s['diastolica']}, {s['frecuencia_cardiaca']}, {fr_val}, {s['temperatura']}, {s['peso']}, {s['talla']}, '{s['registro']}') "
            f"ON CONFLICT (id_signo) DO NOTHING;"
        )

    lines.append("\n-- 5. Inserción tbl_diagnosticos_s2 (IDs 3 al 3000)")
    # CONFIRMADO -> 1, PRESUNTIVO -> 2, None -> NULL
    for d in diagnosticos_bulk:
        id_cie10 = CIE10_MAP_TO_ID.get(d["codigo_cie10"], 1)
        id_tipo_diag = "1" if d["tipo"] == "CONFIRMADO" else ("2" if d["tipo"] == "PRESUNTIVO" else "NULL")
        lines.append(
            f"INSERT INTO tbl_diagnosticos_s2 (id_diagnostico, id_consulta, id_paciente, id_cie10, descripcion_diagnostico, id_tipo_diag, fecha_diagnostico) "
            f"VALUES ({d['id_diagnostico']}, {d['id_consulta']}, {d['id_paciente']}, {id_cie10}, {sql_quote(d['descripcion'])}, {id_tipo_diag}, '{d['fecha']}') "
            f"ON CONFLICT (id_diagnostico) DO NOTHING;"
        )

    lines.extend([
        "\n-- 6. Actualización de secuencias",
        "SELECT setval('cat_cie10_id_cie10_seq', (SELECT COALESCE(MAX(id_cie10), 1) FROM cat_cie10));",
        "SELECT setval('tbl_pacientes_s2_id_paciente_seq', (SELECT COALESCE(MAX(id_paciente), 1) FROM tbl_pacientes_s2));",
        "SELECT setval('tbl_consultas_s2_id_consulta_seq', (SELECT COALESCE(MAX(id_consulta), 1) FROM tbl_consultas_s2));",
        "SELECT setval('tbl_signos_s2_id_signo_seq', (SELECT COALESCE(MAX(id_signo), 1) FROM tbl_signos_s2));",
        "SELECT setval('tbl_diagnosticos_s2_id_diagnostico_seq', (SELECT COALESCE(MAX(id_diagnostico), 1) FROM tbl_diagnosticos_s2));\n"
    ])
    return "\n".join(lines)

def generate_sql_s3(seed: int, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk) -> str:
    lines = [
        "-- ==============================================================================",
        "-- ARQUETIPO S3: DATOS SINTÉTICOS MASIVOS — ESQUEMA LEGADO CON DEUDA TÉCNICA",
        f"-- Semilla fija: {seed} | MedSys-FHIR Evaluación Capítulo III",
        "-- ==============================================================================\n",
        "-- 1. Inserción tbl_pacientes_s3 (IDs 3 al 1000)"
    ]
    for p in pacientes_bulk:
        fech_nac_txt = to_date_dmy(p['fecha_nacimiento'])
        lines.append(
            f"INSERT INTO tbl_pacientes_s3 (pac_id, pac_curp, pac_nom1, pac_nom2, pac_ap_pat, pac_ap_mat, fech_nac_txt, sexo, tel_contacto, fech_reg_txt) "
            f"VALUES ({p['id_paciente']}, {sql_quote(p['curp'])}, {sql_quote(p['primer_nombre'])}, {sql_quote(p['segundo_nombre'])}, {sql_quote(p['apellido_paterno'])}, {sql_quote(p['apellido_materno'])}, '{fech_nac_txt}', '{p['sexo_biologico']}', {sql_quote(p['telefono_contacto'])}, '18/09/2026 09:00:00') "
            f"ON CONFLICT (pac_id) DO NOTHING;"
        )

    lines.append("\n-- 2. Inserción tbl_consultas_s3 (IDs 3 al 2500)")
    for c in consultas_bulk:
        fech_ini_txt = to_datetime_dmy(c['inicio'])
        fech_fin_txt = to_datetime_dmy(c['fin'])
        lines.append(
            f"INSERT INTO tbl_consultas_s3 (con_id, pac_id, ced_medico, nom_medico, est_consulta, motivo, fech_ini_txt, fech_fin_txt, unidad_medica) "
            f"VALUES ({c['id_consulta']}, {c['id_paciente']}, {sql_quote(c['cedula'])}, {sql_quote(c['medico'])}, '{c['estado']}', {sql_quote(c['motivo'])}, '{fech_ini_txt}', '{fech_fin_txt}', {sql_quote(c['unidad'])}) "
            f"ON CONFLICT (con_id) DO NOTHING;"
        )

    lines.append("\n-- 3. Inserción tbl_signos_s3 (IDs 3 al 2500)")
    for s in signos_bulk:
        ta_txt = f"{s['sistolica']}/{s['diastolica']}"
        fr_val = str(s['frecuencia_respiratoria']) if s['frecuencia_respiratoria'] is not None else "NULL"
        peso_lbs = round(s['peso'] / 0.45359237, 2)
        fech_reg_txt = to_datetime_dmy(s['registro'])
        lines.append(
            f"INSERT INTO tbl_signos_s3 (sig_id, con_id, pac_id, presion_arterial_txt, fc, fr, temp_celsius, peso_libras, talla_cm, fech_reg_txt) "
            f"VALUES ({s['id_signo']}, {s['id_consulta']}, {s['id_paciente']}, '{ta_txt}', {s['frecuencia_cardiaca']}, {fr_val}, {s['temperatura']}, {peso_lbs}, {s['talla']}, '{fech_reg_txt}') "
            f"ON CONFLICT (sig_id) DO NOTHING;"
        )

    lines.append("\n-- 4. Inserción tbl_diagnosticos_s3 (IDs 3 al 3000)")
    for d in diagnosticos_bulk:
        tipo_val = sql_quote(d['tipo'])
        fech_diag_txt = to_date_dmy(d['fecha'])
        lines.append(
            f"INSERT INTO tbl_diagnosticos_s3 (diag_id, con_id, pac_id, cie10, desc_diag, tipo_diag, fech_diag_txt) "
            f"VALUES ({d['id_diagnostico']}, {d['id_consulta']}, {d['id_paciente']}, {sql_quote(d['codigo_cie10'])}, {sql_quote(d['descripcion'])}, {tipo_val}, '{fech_diag_txt}') "
            f"ON CONFLICT (diag_id) DO NOTHING;"
        )

    lines.extend([
        "\n-- 5. Actualización de secuencias",
        "SELECT setval('tbl_pacientes_s3_pac_id_seq', (SELECT COALESCE(MAX(pac_id), 1) FROM tbl_pacientes_s3));",
        "SELECT setval('tbl_consultas_s3_con_id_seq', (SELECT COALESCE(MAX(con_id), 1) FROM tbl_consultas_s3));",
        "SELECT setval('tbl_signos_s3_sig_id_seq', (SELECT COALESCE(MAX(sig_id), 1) FROM tbl_signos_s3));",
        "SELECT setval('tbl_diagnosticos_s3_diag_id_seq', (SELECT COALESCE(MAX(diag_id), 1) FROM tbl_diagnosticos_s3));\n"
    ])
    return "\n".join(lines)

# ==============================================================================
# Flujo Principal
# ==============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Generador de Datos Clínicos Sintéticos Reproducibles — MedSys-FHIR"
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=DEFAULT_SEED,
        help=f"Semilla pseudoaleatoria para reproducibilidad experimental (default: {DEFAULT_SEED})"
    )
    parser.add_argument(
        "--schema",
        type=str,
        choices=["s0", "s1", "s2", "s3", "all"],
        default="s0",
        help="Arquetipo de esquema relacional a generar (s0, s1, s2, s3, all). Default: s0"
    )
    parser.add_argument(
        "--output",
        type=str,
        default=None,
        help="Ruta específica para archivo de salida SQL (opcional, anula la ruta por defecto)"
    )
    args = parser.parse_args()

    print(f"[*] Generando entidades sintéticas en memoria con SEED={args.seed}...")
    (
        all_pacientes, pacientes_bulk,
        all_consultas, consultas_bulk,
        all_signos, signos_bulk,
        all_diagnosticos, diagnosticos_bulk
    ) = generate_entities(seed=args.seed)

    # Validaciones estadísticas estrictas sobre el dataset completo
    null_sname_total = sum(1 for p in all_pacientes.values() if p["segundo_nombre"] is None)
    null_phone_total = sum(1 for p in all_pacientes.values() if p["telefono_contacto"] is None)
    null_rr_total = sum(1 for s in all_signos if s["frecuencia_respiratoria"] is None)
    non_null_rr_total = sum(1 for s in all_signos if s["frecuencia_respiratoria"] is not None)
    null_diag_total = sum(1 for d in all_diagnosticos if d["tipo"] is None)
    non_null_diag_total = sum(1 for d in all_diagnosticos if d["tipo"] is not None)

    assert len(all_pacientes) == 1000, f"Se esperaban 1000 pacientes, obtenidos {len(all_pacientes)}"
    assert null_sname_total == 150, f"Se esperaban 150 nulos en segundo_nombre, obtenidos {null_sname_total}"
    assert null_phone_total == 100, f"Se esperaban 100 nulos en telefono_contacto, obtenidos {null_phone_total}"
    assert len(all_consultas) == 2500, f"Se esperaban 2500 consultas, obtenidas {len(all_consultas)}"
    assert len(all_signos) == 2500, f"Se esperaban 2500 signos, obtenidos {len(all_signos)}"
    assert null_rr_total == 250, f"Se esperaban 250 nulos en frecuencia_respiratoria, obtenidos {null_rr_total}"
    assert non_null_rr_total == 2250, f"Se esperaban 2250 con frecuencia_respiratoria, obtenidos {non_null_rr_total}"
    assert len(all_diagnosticos) == 3000, f"Se esperaban 3000 diagnósticos, obtenidos {len(all_diagnosticos)}"
    assert null_diag_total == 300, f"Se esperaban 300 nulos en tipo_diagnostico, obtenidos {null_diag_total}"
    assert non_null_diag_total == 2700, f"Se esperaban 2700 con tipo_diagnostico, obtenidos {non_null_diag_total}"

    total_observations = 2500 + 2500 + 2500 + non_null_rr_total  # PA + Temp + FC + FR
    total_resources = len(all_pacientes) + len(all_consultas) + total_observations + len(all_diagnosticos)
    assert total_observations == 9750, f"Se esperaban 9750 recursos Observation, obtenidos {total_observations}"
    assert total_resources == 16250, f"Se esperaban 16250 recursos FHIR en total, obtenidos {total_resources}"

    # Exportación determinista de IDs de frecuencia respiratoria para k6 (Sección 3.6.2 del Capítulo III)
    k6_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "tests", "k6"))
    os.makedirs(k6_dir, exist_ok=True)
    valid_resp_ids = sorted([s["id_signo"] for s in all_signos if s["frecuencia_respiratoria"] is not None])
    omitted_resp_ids = sorted([s["id_signo"] for s in all_signos if s["frecuencia_respiratoria"] is None])
    assert len(valid_resp_ids) == 2250, f"Se esperaban 2250 IDs válidos de FR, obtenidos {len(valid_resp_ids)}"
    assert len(omitted_resp_ids) == 250, f"Se esperaban 250 IDs omitidos de FR, obtenidos {len(omitted_resp_ids)}"
    with open(os.path.join(k6_dir, "valid_resp_ids.json"), "w", encoding="utf-8") as f:
        json.dump(valid_resp_ids, f, indent=2)
    with open(os.path.join(k6_dir, "omitted_resp_ids.json"), "w", encoding="utf-8") as f:
        json.dump(omitted_resp_ids, f, indent=2)
    print(f"[OK] IDs k6 exportados: {len(valid_resp_ids)} válidos y {len(omitted_resp_ids)} omitidos en {k6_dir}")

    targets = []
    if args.schema == "all":
        targets = [
            ("s0", DEFAULT_S0_OUTPUT),
            ("s1", DEFAULT_S1_OUTPUT),
            ("s2", DEFAULT_S2_OUTPUT),
            ("s3", DEFAULT_S3_OUTPUT),
        ]
    else:
        out_path = args.output
        if not out_path:
            if args.schema == "s0":
                out_path = DEFAULT_S0_OUTPUT
            elif args.schema == "s1":
                out_path = DEFAULT_S1_OUTPUT
            elif args.schema == "s2":
                out_path = DEFAULT_S2_OUTPUT
            elif args.schema == "s3":
                out_path = DEFAULT_S3_OUTPUT
        targets = [(args.schema, out_path)]

    for schema_name, target_path in targets:
        os.makedirs(os.path.dirname(os.path.abspath(target_path)), exist_ok=True)
        if schema_name == "s0":
            sql_content = generate_sql_s0(args.seed, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk)
        elif schema_name == "s1":
            sql_content = generate_sql_s1(args.seed, all_pacientes, consultas_bulk, signos_bulk, diagnosticos_bulk)
        elif schema_name == "s2":
            sql_content = generate_sql_s2(args.seed, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk)
        elif schema_name == "s3":
            sql_content = generate_sql_s3(args.seed, pacientes_bulk, consultas_bulk, signos_bulk, diagnosticos_bulk)

        with open(target_path, "w", encoding="utf-8", newline="\n") as f:
            f.write(sql_content)

        with open(target_path, 'rb') as evaluated_file:
            file_sha256 = hashlib.sha256(evaluated_file.read()).hexdigest()
        print(f"[OK] Archivo SQL '{schema_name}' generado exitosamente:")
        print(f"     Ruta:   {target_path}")
        print(f"     SHA256: {file_sha256}")

    print("\n[OK] Resumen Experimental Consolidado (Capítulo III §3.2.3):")
    print(f"     - Pacientes generados:           {len(pacientes_bulk)} (Total BD: 1,000 | 150 segundo_nombre=NULL, 100 telefono=NULL)")
    print(f"     - Consultas generadas:           {len(consultas_bulk)} (Total BD: 2,500 vinculadas a 1..1000)")
    print(f"     - Signos vitales generados:      {len(signos_bulk)} (Total BD: 2,500 | 250 FR=NULL [10%], 2,250 FR presentes [90%])")
    print(f"     - Diagnósticos generados:        {len(diagnosticos_bulk)} (Total BD: 3,000 | 300 tipo=NULL [10%], 2,700 presentes [90%])")
    print(f"     - Recursos Observation totales:  {total_observations} (2,500 PA + 2,500 Temp + 2,500 FC + 2,250 FR)")
    print(f"     - Censo total recursos FHIR R4:  {total_resources} (1,000 Patient + 2,500 Encounter + 9,750 Observation + 3,000 Condition)")

if __name__ == "__main__":
    main()
