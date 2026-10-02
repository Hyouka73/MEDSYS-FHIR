#!/usr/bin/env python3
# ==============================================================================
# MedSys-FHIR: Generador de Datos Clínicos Sintéticos Reproducibles (Capítulo III)
# Basado en normatividad NOM-004-SSA3-2012 y NOM-024-SSA3-2012
# Semilla: 20260930 (Evaluación Experimental UNACH 2026)
# ==============================================================================

import argparse
import os
import random
from datetime import datetime, timedelta, date

DEFAULT_SEED = 20260930
DEFAULT_OUTPUT_FILE = os.path.join(os.path.dirname(__file__), "..", "docker", "sql", "02_bulk_data.sql")
SEED = DEFAULT_SEED
OUTPUT_FILE = DEFAULT_OUTPUT_FILE

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

used_curps = {"ROMA900101HCSNN01", "LOPE950512MCSNN02"}

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

def generate_curp(paterno: str, materno: str, nombre: str, bdate: date, sex: str, idx: int) -> str:
    c1 = paterno[0].upper()
    c2 = get_internal_vowel(paterno)
    c3 = materno[0].upper() if materno else "X"
    c4 = nombre[0].upper()
    
    yy = f"{bdate.year % 100:02d}"
    mm = f"{bdate.month:02d}"
    dd = f"{bdate.day:02d}"
    
    curp_sex = "H" if sex == "M" else ("M" if sex == "F" else "X")
    state = "CS" # Chiapas
    
    c14 = get_internal_consonant(paterno)
    c15 = get_internal_consonant(materno) if materno else "X"
    c16 = get_internal_consonant(nombre)
    
    homo = f"{(idx % 90 + 10):02d}"
    
    curp = f"{c1}{c2}{c3}{c4}{yy}{mm}{dd}{curp_sex}{state}{c14}{c15}{c16}{homo}"
    if len(curp) != 18 or curp in used_curps:
        # Fallback safe uniqueness
        alpha = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        curp = f"{c1}{c2}{c3}{c4}{yy}{mm}{dd}{curp_sex}{state}{c14}{c15}{c16}{alpha[(idx // 36) % 36]}{alpha[idx % 36]}"
    
    used_curps.add(curp)
    return curp

def main(seed: int = DEFAULT_SEED, output_path: str = DEFAULT_OUTPUT_FILE):
    random.seed(seed)
    used_curps.clear()
    used_curps.update({"ROMA900101HCSNN01", "LOPE950512MCSNN02"})
    print(f"[*] Generando datos sintéticos reproducibles con SEED={seed}...")
    
    pacientes = []
    # Pacientes 1 y 2 ya existen en schema_legado_simulado_nom004.sql
    for pid in range(3, 1001):
        is_male = random.random() < 0.5
        sex = "M" if is_male else "F"
        p_name = random.choice(FIRST_NAMES_M if is_male else FIRST_NAMES_F)
        s_name = random.choice(FIRST_NAMES_M if is_male else FIRST_NAMES_F) if random.random() < 0.4 else None
        paterno = random.choice(LAST_NAMES)
        materno = random.choice(LAST_NAMES) if random.random() < 0.85 else None
        
        # Fecha de nacimiento entre 1945 y 2018 (edad 8 a 81)
        days_offset = random.randint(8 * 365, 80 * 365)
        bdate = date(2026, 1, 1) - timedelta(days=days_offset)
        
        curp = generate_curp(paterno, materno, p_name, bdate, sex, pid)
        phone = f"961{random.randint(1000000, 9999999)}"
        
        pacientes.append({
            "id_paciente": pid,
            "curp": curp,
            "primer_nombre": p_name,
            "segundo_nombre": s_name,
            "apellido_paterno": paterno,
            "apellido_materno": materno,
            "fecha_nacimiento": bdate.isoformat(),
            "sexo_biologico": sex,
            "telefono_contacto": phone
        })

    # Consultas: IDs 3 a 2500 (total 2500 consultas)
    consultas = []
    base_date = datetime(2025, 1, 15, 8, 0, 0)
    
    for cid in range(3, 2501):
        # Distribución de pacientes (algunos tienen varias consultas)
        pid = random.randint(1, 1000)
        doctor = random.choice(DOCTORS)
        clinic = random.choice(CLINICS)
        reason = random.choice(REASONS)
        status = "FINALIZADA" if random.random() < 0.94 else ("EN_CURSO" if random.random() < 0.5 else "CANCELADA")
        
        dt_start = base_date + timedelta(minutes=random.randint(0, 600 * 24 * 60))
        duration = random.randint(15, 45)
        dt_end = dt_start + timedelta(minutes=duration)
        
        consultas.append({
            "id_consulta": cid,
            "id_paciente": pid,
            "cedula": doctor[0],
            "medico": doctor[1],
            "estado": status,
            "motivo": reason,
            "inicio": dt_start.strftime("%Y-%m-%d %H:%M:%S"),
            "fin": dt_end.strftime("%Y-%m-%d %H:%M:%S"),
            "unidad": clinic
        })

    # Signos Vitales: 2500 filas (IDs 3 a 2500, vinculados 1:1 con consultas 3 a 2500)
    signos = []
    for cid in range(3, 2501):
        consulta = consultas[cid - 3]
        pid = consulta["id_paciente"]
        
        systolic = random.randint(100, 155)
        diastolic = random.randint(60, 95)
        hr = random.randint(58, 105)
        rr = random.randint(14, 22)
        temp = round(random.uniform(36.1, 37.6), 1)
        weight = round(random.uniform(48.0, 98.0), 2)
        height = random.randint(150, 185)
        
        signos.append({
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
        })

    # Diagnósticos: 3000 filas (IDs 3 a 3000)
    # Las primeras 2498 filas corresponden 1:1 a las consultas 3 a 2500.
    # Las siguientes 500 filas son segundos diagnósticos de consultas seleccionadas al azar.
    diagnosticos = []
    diag_id = 3
    
    # 1 por consulta (IDs 3 a 2500)
    for cid in range(3, 2501):
        consulta = consultas[cid - 3]
        pid = consulta["id_paciente"]
        code, desc = random.choice(CIE10_CODES)
        diag_type = "CONFIRMADO" if random.random() < 0.88 else "PRESUNTIVO"
        diag_date = consulta["inicio"][:10]
        
        diagnosticos.append({
            "id_diagnostico": diag_id,
            "id_consulta": cid,
            "id_paciente": pid,
            "codigo_cie10": code,
            "descripcion": desc,
            "tipo": diag_type,
            "fecha": diag_date
        })
        diag_id += 1

    # 500 diagnósticos secundarios (IDs 2501 a 3000)
    multidiag_consultas = random.sample(consultas, 500)
    for consulta in multidiag_consultas:
        cid = consulta["id_consulta"]
        pid = consulta["id_paciente"]
        code, desc = random.choice(CIE10_CODES)
        diag_type = "CONFIRMADO" if random.random() < 0.85 else "PRESUNTIVO"
        diag_date = consulta["inicio"][:10]
        
        diagnosticos.append({
            "id_diagnostico": diag_id,
            "id_consulta": cid,
            "id_paciente": pid,
            "codigo_cie10": code,
            "descripcion": desc,
            "tipo": diag_type,
            "fecha": diag_date
        })
        diag_id += 1

    output_dir = os.path.dirname(output_path)
    if output_dir:
        os.makedirs(output_dir, exist_ok=True)
    
    with open(output_path, "w", encoding="utf-8") as f:
        f.write("-- ==============================================================================\n")
        f.write("-- DATOS SINTÉTICOS MASIVOS REPRODUCIBLES (1,000 Pacientes / 2,500 Consultas)\n")
        f.write(f"-- Semilla fija: {seed} | MedSys-FHIR Evaluación Capítulo III\n")
        f.write("-- ==============================================================================\n\n")
        
        # Inserción tbl_pacientes
        f.write("-- 1. Inserción tbl_pacientes (IDs 3 al 1000)\n")
        for p in pacientes:
            sec_val = f"'{p['segundo_nombre']}'" if p['segundo_nombre'] else "NULL"
            mat_val = f"'{p['apellido_materno']}'" if p['apellido_materno'] else "NULL"
            f.write(
                f"INSERT INTO tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) "
                f"VALUES ({p['id_paciente']}, '{p['curp']}', '{p['primer_nombre']}', {sec_val}, '{p['apellido_paterno']}', {mat_val}, '{p['fecha_nacimiento']}', '{p['sexo_biologico']}', '{p['telefono_contacto']}') "
                f"ON CONFLICT (id_paciente) DO NOTHING;\n"
            )
        f.write("\n")
        
        # Inserción tbl_consultas
        f.write("-- 2. Inserción tbl_consultas (IDs 3 al 2500)\n")
        for c in consultas:
            f.write(
                f"INSERT INTO tbl_consultas (id_consulta, id_paciente, cedula_medico_tratante, nombre_medico, estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin, unidad_medica) "
                f"VALUES ({c['id_consulta']}, {c['id_paciente']}, '{c['cedula']}', '{c['medico']}', '{c['estado']}', '{c['motivo']}', '{c['inicio']}', '{c['fin']}', '{c['unidad']}') "
                f"ON CONFLICT (id_consulta) DO NOTHING;\n"
            )
        f.write("\n")
        
        # Inserción tbl_signos_vitales
        f.write("-- 3. Inserción tbl_signos_vitales (IDs 3 al 2500)\n")
        for s in signos:
            f.write(
                f"INSERT INTO tbl_signos_vitales (id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius, peso_kg, talla_cm, fecha_registro) "
                f"VALUES ({s['id_signo']}, {s['id_consulta']}, {s['id_paciente']}, {s['sistolica']}, {s['diastolica']}, {s['frecuencia_cardiaca']}, {s['frecuencia_respiratoria']}, {s['temperatura']}, {s['peso']}, {s['talla']}, '{s['registro']}') "
                f"ON CONFLICT (id_signo) DO NOTHING;\n"
            )
        f.write("\n")
        
        # Inserción tbl_diagnosticos
        f.write("-- 4. Inserción tbl_diagnosticos (IDs 3 al 3000)\n")
        for d in diagnosticos:
            f.write(
                f"INSERT INTO tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico) "
                f"VALUES ({d['id_diagnostico']}, {d['id_consulta']}, {d['id_paciente']}, '{d['codigo_cie10']}', '{d['descripcion']}', '{d['tipo']}', '{d['fecha']}') "
                f"ON CONFLICT (id_diagnostico) DO NOTHING;\n"
            )
        f.write("\n")
        
        # Actualización de secuencias
        f.write("-- 5. Actualización de secuencias de claves primarias\n")
        f.write("SELECT setval('tbl_pacientes_id_paciente_seq', (SELECT COALESCE(MAX(id_paciente), 1) FROM tbl_pacientes));\n")
        f.write("SELECT setval('tbl_consultas_id_consulta_seq', (SELECT COALESCE(MAX(id_consulta), 1) FROM tbl_consultas));\n")
        f.write("SELECT setval('tbl_signos_vitales_id_signo_seq', (SELECT COALESCE(MAX(id_signo), 1) FROM tbl_signos_vitales));\n")
        f.write("SELECT setval('tbl_diagnosticos_id_diagnostico_seq', (SELECT COALESCE(MAX(id_diagnostico), 1) FROM tbl_diagnosticos));\n")

    print(f"[OK] Archivo SQL generado con exito: {output_path}")
    print(f"    - Pacientes generados: {len(pacientes)} (Total acumulado: 1,000)")
    print(f"    - Consultas generadas: {len(consultas)} (Total acumulado: 2,500)")
    print(f"    - Signos vitales generados: {len(signos)} (Total acumulado: 2,500)")
    print(f"    - Diagnosticos generados: {len(diagnosticos)} (Total acumulado: 3,000)")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generador de Datos Clínicos Sintéticos Reproducibles (MedSys-FHIR)"
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=DEFAULT_SEED,
        help="Semilla pseudoaleatoria para reproducibilidad de datos sintéticos"
    )
    parser.add_argument(
        "--output",
        type=str,
        default=DEFAULT_OUTPUT_FILE,
        help="Ruta del archivo SQL de salida"
    )
    args = parser.parse_args()
    main(seed=args.seed, output_path=args.output)
