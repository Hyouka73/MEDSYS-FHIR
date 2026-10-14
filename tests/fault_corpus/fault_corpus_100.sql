-- =============================================================================
-- Corpus de Inyección de 100 Fallas Clínicas (T1–T5) — MedSys-FHIR v1.2.0-eval
-- Tesis UNACH 2026: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
-- =============================================================================

CREATE SCHEMA IF NOT EXISTS fault_corpus;

-- Estructuras de prueba en esquema aislado fault_corpus
CREATE TABLE IF NOT EXISTS fault_corpus.tbl_pacientes (
    id_paciente SERIAL PRIMARY KEY,
    curp VARCHAR(18),
    primer_nombre VARCHAR(50),
    segundo_nombre VARCHAR(50),
    apellido_paterno VARCHAR(50),
    apellido_materno VARCHAR(50),
    fecha_nacimiento TEXT,
    sexo_biologico VARCHAR(20),
    telefono_contacto VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS fault_corpus.tbl_consultas (
    id_consulta SERIAL PRIMARY KEY,
    id_paciente INTEGER,
    fecha_hora_inicio TEXT,
    fecha_hora_fin TEXT,
    motivo_consulta TEXT,
    estado_consulta VARCHAR(30),
    cedula_medico_tratante VARCHAR(20),
    nombre_medico VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS fault_corpus.tbl_signos_vitales (
    id_signo SERIAL PRIMARY KEY,
    id_consulta INTEGER,
    id_paciente INTEGER,
    fecha_registro TEXT,
    presion_sistolica TEXT,
    presion_diastolica TEXT,
    temperatura_celsius TEXT,
    frecuencia_cardiaca TEXT,
    frecuencia_respiratoria TEXT
);

CREATE TABLE IF NOT EXISTS fault_corpus.tbl_diagnosticos (
    id_diagnostico SERIAL PRIMARY KEY,
    id_consulta INTEGER,
    id_paciente INTEGER,
    codigo_cie10 VARCHAR(10),
    descripcion_diagnostico TEXT,
    fecha_diagnostico TEXT,
    tipo_diagnostico VARCHAR(30)
);

-- Inserción determinista de los 100 casos anómalos etiquetados
-- [T1-01] Paciente sin primer_nombre (NULL)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9001, 'ROMA900101HCSMNL09', NULL, 'CARLOS', 'RODRIGUEZ', 'MARTINEZ', '1990-01-01', 'M', '9611234567');

-- [T1-02] Paciente con primer_nombre vacío ('')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9002, 'ROMA900101HCSMNL09', '', NULL, 'GARCIA', 'HERNANDEZ', '1992-05-15', 'F', '9617654321');

-- [T1-03] Paciente sin apellido_paterno (NULL)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9003, 'ROMA900101HCSMNL09', 'JUAN', NULL, NULL, 'PEREZ', '1988-11-20', 'M', NULL);

-- [T1-04] Paciente con apellido_paterno vacío ('')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9004, 'ROMA900101HCSMNL09', 'MARIA', 'ELENA', '', 'LOPEZ', '1995-03-12', 'F', '9612345678');

-- [T1-05] Paciente sin CURP (NULL)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9005, NULL, 'JOSE', NULL, 'SANCHEZ', 'DIAZ', '1985-07-25', 'M', '9613456789');

-- [T1-06] Paciente con CURP vacía ('')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9006, '', 'ANA', 'LUCIA', 'RAMIREZ', 'GOMEZ', '1993-09-30', 'F', NULL);

-- [T1-07] Paciente sin fecha_nacimiento (NULL)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9007, 'ROMA900101HCSMNL09', 'PEDRO', NULL, 'TORRES', 'CASTRO', NULL, 'M', '9614567890');

-- [T1-08] Paciente sin sexo_biologico (NULL)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9008, 'ROMA900101HCSMNL09', 'LUCIA', 'FERNANDA', 'FLORES', 'MORALES', '1991-12-05', NULL, '9615678901');

-- [T1-09] Consulta sin id_paciente (NULL)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9009, NULL, '2026-10-01 09:00:00', '2026-10-01 09:30:00', 'Control prenatal', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T1-10] Consulta sin fecha_hora_inicio (NULL)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9010, 1, NULL, '2026-10-01 10:30:00', 'Cefalea tensional', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T1-11] Consulta sin motivo_consulta (NULL)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9011, 1, '2026-10-01 10:00:00', '2026-10-01 10:30:00', NULL, 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T1-12] Consulta con motivo_consulta vacío ('')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9012, 1, '2026-10-01 11:00:00', '2026-10-01 11:30:00', '', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T1-13] Consulta sin estado_consulta (NULL)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9013, 1, '2026-10-01 12:00:00', '2026-10-01 12:30:00', 'Revision general', NULL, '12345678', 'DR. ROBERTO CRUZ');

-- [T1-14] Consulta sin cedula_medico_tratante (NULL)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9014, 1, '2026-10-01 13:00:00', '2026-10-01 13:30:00', 'Dolor abdominal', 'FINALIZADA', NULL, 'DR. ROBERTO CRUZ');

-- [T1-15] Presion arterial sin presion_sistolica (NULL)
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9015, 1, 1, '2026-10-01 09:15:00', NULL, 80, 36.5, 75, 18);

-- [T1-16] Presion arterial sin presion_diastolica (NULL)
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9016, 1, 1, '2026-10-01 09:15:00', 120, NULL, 36.5, 75, 18);

-- [T1-17] Signos vitales sin temperatura_celsius (NULL)
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9017, 1, 1, '2026-10-01 09:15:00', 120, 80, NULL, 75, 18);

-- [T1-18] Signos vitales sin frecuencia_cardiaca (NULL)
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9018, 1, 1, '2026-10-01 09:15:00', 120, 80, 36.5, NULL, 18);

-- [T1-19] Diagnostico sin codigo_cie10 (NULL)
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9019, 1, 1, NULL, 'Hipertension esencial', '2026-10-01', 'CONFIRMADO');

-- [T1-20] Diagnostico sin descripcion_diagnostico (NULL)
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9020, 1, 1, 'I10', NULL, '2026-10-01', 'CONFIRMADO');

-- [T2-01] Fecha de nacimiento con día imposible (30 de febrero)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9021, 'ROMA900101HCSMNL09', 'CARLOS', NULL, 'LOPEZ', 'GARCIA', '2026-02-30', 'M', '9611234567');

-- [T2-02] Fecha de nacimiento con mes imposible (mes 13)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9022, 'ROMA900101HCSMNL09', 'MARIA', NULL, 'HERNANDEZ', 'PEREZ', '1990-13-01', 'F', '9612345678');

-- [T2-03] Fecha de nacimiento corrupta (texto no parseable)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9023, 'ROMA900101HCSMNL09', 'JORGE', NULL, 'MARTINEZ', 'SANCHEZ', 'FECHA_INVALIDA', 'M', NULL);

-- [T2-04] Fecha de nacimiento con mes 99 en formato dd-mm-yyyy
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9024, 'ROMA900101HCSMNL09', 'ANA', 'SOFIA', 'GARCIA', 'LOPEZ', '01-99-1990', 'F', '9613456789');

-- [T2-05] Timestamp con día imposible (31 de abril)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9025, 1, '2026-04-31 10:00:00', '2026-04-30 10:30:00', 'Control prenatal', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T2-06] Timestamp corrupto (texto no datetime)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9026, 1, 'NO_DATETIME', '2026-10-01 10:30:00', 'Cefalea', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T2-07] Timestamp bisiesto inválido (29 feb 2026)
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9027, 1, '2026-02-29 10:00:00', '2026-02-28 10:30:00', 'Revision', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T2-08] Timestamp de fin con mes 13
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9028, 1, '2026-10-01 09:00:00', '2026-13-01 10:00:00', 'Seguimiento', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T2-09] Presion sistolica con texto no numérico ('ALTA')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9029, 1, 1, '2026-10-01 09:15:00', 'ALTA', 80, 36.5, 75, 18);

-- [T2-10] Presion diastolica con texto no numérico ('NORMAL')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9030, 1, 1, '2026-10-01 09:15:00', 120, 'NORMAL', 36.5, 75, 18);

-- [T2-11] Temperatura con texto no numérico ('FIEBRE')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9031, 1, 1, '2026-10-01 09:15:00', 120, 80, 'FIEBRE', 75, 18);

-- [T2-12] Temperatura con valor malformado ('37.5.8')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9032, 1, 1, '2026-10-01 09:15:00', 120, 80, '37.5.8', 75, 18);

-- [T2-13] Frecuencia cardiaca no numérica ('RAPIDA')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9033, 1, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 'RAPIDA', 18);

-- [T2-14] Frecuencia cardiaca con sufijo no numérico ('80bpm')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9034, 1, 1, '2026-10-01 09:15:00', 120, 80, 36.5, '80bpm', 18);

-- [T2-15] Frecuencia respiratoria no numérica ('VENTILANDO')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9035, 1, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 'VENTILANDO');

-- [T2-16] Presion arterial S3 con diastolica no numérica ('120/XX')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9036, 1, 1, '01/10/2026 09:15:00', '120/XX', NULL, '36.5', '75', '18');

-- [T2-17] Presion arterial S3 con sistolica no numérica ('YY/80')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9037, 1, 1, '01/10/2026 09:15:00', 'YY/80', NULL, '36.5', '75', '18');

-- [T2-18] Presion arterial S3 sin delimitador '/' ('120-80')
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9038, 1, 1, '01/10/2026 09:15:00', '120-80', NULL, '36.5', '75', '18');

-- [T2-19] Fecha de diagnostico imposible (31 de noviembre)
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9039, 1, 1, 'I10', 'Hipertension', '2026-11-31', 'CONFIRMADO');

-- [T2-20] Fecha de diagnostico corrupta ('CORRUPT_TIMESTAMP')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9040, 1, 1, 'I10', 'Hipertension', 'CORRUPT_TIMESTAMP', 'CONFIRMADO');

-- [T3-01] Sexo biológico no reconocido ('X')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9041, 'ROMA900101HCSMNL09', 'ALEX', NULL, 'LOPEZ', 'GARCIA', '1990-01-01', 'X', '9611234567');

-- [T3-02] Sexo biológico texto no estándar ('DESCONOCIDO')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9042, 'ROMA900101HCSMNL09', 'PATRICIA', NULL, 'HERNANDEZ', 'PEREZ', '1992-05-15', 'DESCONOCIDO', '9612345678');

-- [T3-03] Sexo biológico con clave H en lugar de M ('H')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9043, 'ROMA900101HCSMNL09', 'MIGUEL', NULL, 'SANCHEZ', 'DIAZ', '1988-11-20', 'H', '9613456789');

-- [T3-04] Sexo biológico texto libre ('MUJER')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9044, 'ROMA900101HCSMNL09', 'MARIA', 'ELENA', 'GARCIA', 'LOPEZ', '1995-03-12', 'MUJER', '9614567890');

-- [T3-05] Sexo biológico clave inexistente ('Z')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9045, 'ROMA900101HCSMNL09', 'ROBERTO', NULL, 'TORRES', 'CASTRO', '1985-07-25', 'Z', '9615678901');

-- [T3-06] Sexo biológico no binario no contemplado en NOM-004 ('NB')
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9046, 'ROMA900101HCSMNL09', 'SAM', NULL, 'RAMIREZ', 'GOMEZ', '1993-09-30', 'NB', '9616789012');

-- [T3-07] Estado de consulta fuera de catálogo ('PENDIENTE_REVISION')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9047, 1, '2026-10-01 09:00:00', '2026-10-01 09:30:00', 'Revision', 'PENDIENTE_REVISION', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-08] Estado de consulta fuera de catálogo ('ABIERTO')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9048, 1, '2026-10-01 10:00:00', '2026-10-01 10:30:00', 'Cefalea', 'ABIERTO', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-09] Estado de consulta fuera de catálogo ('CERRADO')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9049, 1, '2026-10-01 11:00:00', '2026-10-01 11:30:00', 'Dolor', 'CERRADO', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-10] Estado de consulta fuera de catálogo ('EN_ESPERA')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9050, 1, '2026-10-01 12:00:00', '2026-10-01 12:30:00', 'Control', 'EN_ESPERA', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-11] Estado de consulta fuera de catálogo ('SUSPENDIDA')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9051, 1, '2026-10-01 13:00:00', '2026-10-01 13:30:00', 'Gripe', 'SUSPENDIDA', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-12] Estado de consulta fuera de catálogo ('NO_ASISTIO')
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9052, 1, '2026-10-01 14:00:00', '2026-10-01 14:30:00', 'Vacuna', 'NO_ASISTIO', '12345678', 'DR. ROBERTO CRUZ');

-- [T3-13] Tipo de diagnóstico fuera de catálogo ('DESCARTADO_MANUAL')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9053, 1, 1, 'I10', 'Hipertension', '2026-10-01', 'DESCARTADO_MANUAL');

-- [T3-14] Tipo de diagnóstico fuera de catálogo ('SOSPECHOSO')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9054, 1, 1, 'E11', 'Diabetes', '2026-10-01', 'SOSPECHOSO');

-- [T3-15] Tipo de diagnóstico fuera de catálogo ('DESCARTADO')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9055, 1, 1, 'J00', 'Rinofaringitis', '2026-10-01', 'DESCARTADO');

-- [T3-16] Tipo de diagnóstico fuera de catálogo ('PRELIMINAR')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9056, 1, 1, 'K21', 'Reflujo', '2026-10-01', 'PRELIMINAR');

-- [T3-17] Tipo de diagnóstico fuera de catálogo ('DIFERENCIAL')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9057, 1, 1, 'M54.5', 'Lumbago', '2026-10-01', 'DIFERENCIAL');

-- [T3-18] Tipo de diagnóstico fuera de catálogo ('CRONICO')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9058, 1, 1, 'N39.0', 'Infeccion urinaria', '2026-10-01', 'CRONICO');

-- [T3-19] Tipo de diagnóstico fuera de catálogo ('DEFINITIVO')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9059, 1, 1, 'J45', 'Asma', '2026-10-01', 'DEFINITIVO');

-- [T3-20] Tipo de diagnóstico fuera de catálogo ('CURADO')
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9060, 1, 1, 'A09', 'Gastroenteritis', '2026-10-01', 'CURADO');

-- [T4-01] Consulta huérfana: id_paciente 99991 no existe
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9061, 99991, '2026-10-01 09:00:00', '2026-10-01 09:30:00', 'Control', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T4-02] Consulta huérfana: id_paciente 99992 no existe
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9062, 99992, '2026-10-01 10:00:00', '2026-10-01 10:30:00', 'Cefalea', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T4-03] Consulta huérfana: id_paciente 99993 no existe
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9063, 99993, '2026-10-01 11:00:00', '2026-10-01 11:30:00', 'Revision', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T4-04] Consulta huérfana: id_paciente 99994 no existe
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9064, 99994, '2026-10-01 12:00:00', '2026-10-01 12:30:00', 'Dolor', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T4-05] Consulta huérfana: id_paciente 99995 no existe
INSERT INTO fault_corpus.tbl_consultas (id_consulta, id_paciente, fecha_hora_inicio, fecha_hora_fin, motivo_consulta, estado_consulta, cedula_medico_tratante, nombre_medico) VALUES (9065, 99995, '2026-10-01 13:00:00', '2026-10-01 13:30:00', 'Gripe', 'FINALIZADA', '12345678', 'DR. ROBERTO CRUZ');

-- [T4-06] Signos vitales huérfanos: id_consulta 88881 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9066, 88881, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-07] Signos vitales huérfanos: id_consulta 88882 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9067, 88882, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-08] Signos vitales huérfanos: id_consulta 88883 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9068, 88883, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-09] Signos vitales huérfanos: id_consulta 88884 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9069, 88884, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-10] Signos vitales huérfanos: id_consulta 88885 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9070, 88885, 1, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-11] Signos vitales huérfanos: id_paciente 77771 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9071, 1, 77771, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-12] Signos vitales huérfanos: id_paciente 77772 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9072, 1, 77772, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-13] Signos vitales huérfanos: id_paciente 77773 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9073, 1, 77773, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-14] Signos vitales huérfanos: id_paciente 77774 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9074, 1, 77774, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-15] Signos vitales huérfanos: id_paciente 77775 no existe
INSERT INTO fault_corpus.tbl_signos_vitales (id_signo, id_consulta, id_paciente, fecha_registro, presion_sistolica, presion_diastolica, temperatura_celsius, frecuencia_cardiaca, frecuencia_respiratoria) VALUES (9075, 1, 77775, '2026-10-01 09:15:00', 120, 80, 36.5, 75, 18);

-- [T4-16] Diagnóstico huérfano: id_consulta 66661 no existe
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9076, 66661, 1, 'I10', 'Hipertension', '2026-10-01', 'CONFIRMADO');

-- [T4-17] Diagnóstico huérfano: id_consulta 66662 no existe
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9077, 66662, 1, 'E11', 'Diabetes', '2026-10-01', 'CONFIRMADO');

-- [T4-18] Diagnóstico huérfano: id_consulta 66663 no existe
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9078, 66663, 1, 'J00', 'Rinofaringitis', '2026-10-01', 'CONFIRMADO');

-- [T4-19] Diagnóstico huérfano: id_paciente 55551 no existe
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9079, 1, 55551, 'K21', 'Reflujo', '2026-10-01', 'CONFIRMADO');

-- [T4-20] Diagnóstico huérfano: id_paciente 55552 no existe
INSERT INTO fault_corpus.tbl_diagnosticos (id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, fecha_diagnostico, tipo_diagnostico) VALUES (9080, 1, 55552, 'M54.5', 'Lumbago', '2026-10-01', 'CONFIRMADO');

-- [T5-01] CURP de longitud insuficiente (17 caracteres)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9081, 'ROMA900101HCSMNL0', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-02] CURP de longitud excesiva (19 caracteres)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9082, 'ROMA900101HCSMNL099', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-03] CURP con carácter especial '@' al final
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9083, 'ROMA900101HCSMNL0@', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-04] CURP en minúsculas (debe ser mayúsculas)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9084, 'roma900101hcsmnl09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-05] CURP con dígitos en prefijo de 4 letras
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9085, '1234900101HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-06] CURP con dígito en prefijo alfabético
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9086, 'ROM9900101HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-07] CURP con mes 13 en fecha YYMMDD
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9087, 'ROMA901301HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-08] CURP con mes 00 en fecha YYMMDD
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9088, 'ROMA900001HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-09] CURP con día 32 en fecha YYMMDD
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9089, 'ROMA900132HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-10] CURP con día 00 en fecha YYMMDD
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9090, 'ROMA900100HCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-11] CURP con sexo inválido 'Z' (solo H, M, X)
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9091, 'ROMA900101ZCSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-12] CURP con dígito '1' en posición de sexo
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9092, 'ROMA9001011CSMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-13] CURP con entidad federativa inexistente 'XX'
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9093, 'ROMA900101HXXMNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-14] CURP con entidad federativa numérica '99'
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9094, 'ROMA900101H99MNL09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-15] CURP con vocales 'AEI' en lugar de 3 consonantes internas
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9095, 'ROMA900101HCSAEI09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-16] CURP con vocal 'A' en consonante interna 3
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9096, 'ROMA900101HCSMNA09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-17] CURP con letra 'A' en último dígito verificador
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9097, 'ROMA900101HCSMNL0A', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-18] CURP con espacio en blanco intercalado
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9098, 'ROMA 900101HCSMNL0', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-19] CURP con guiones de separación no permitidos
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9099, 'ROMA-900101-HCS-09', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

-- [T5-20] CURP con signos de exclamación al final
INSERT INTO fault_corpus.tbl_pacientes (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto) VALUES (9100, 'ROMA900101HCSMNL!!', 'VALENTINA', NULL, 'MENDEZ', 'CASTILLO', '1990-01-01', 'F', '9618765432');

