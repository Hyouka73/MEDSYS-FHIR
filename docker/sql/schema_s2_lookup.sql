-- ==============================================================================
-- ARQUETIPO S2: ESQUEMA NORMALIZADO CON CATÁLOGOS EXTERNOS (LOOKUP / JOIN)
-- Motor: PostgreSQL 16
-- Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
-- ==============================================================================

DO $$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'medsys_user') THEN
    CREATE ROLE medsys_user WITH SUPERUSER LOGIN PASSWORD 'medsys_secure_pass_2026';
  ELSE
    ALTER ROLE medsys_user WITH PASSWORD 'medsys_secure_pass_2026';
  END IF;
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'postgres') THEN
    CREATE ROLE postgres WITH SUPERUSER LOGIN PASSWORD 'medsys_secure_pass_2026';
  ELSE
    ALTER ROLE postgres WITH PASSWORD 'medsys_secure_pass_2026';
  END IF;
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'medsys_ro') THEN
    CREATE ROLE medsys_ro WITH LOGIN PASSWORD 'medsys_ro_pass_2026';
  ELSE
    ALTER ROLE medsys_ro WITH LOGIN PASSWORD 'medsys_ro_pass_2026';
  END IF;
END $$;

-- 1. Tablas de Catálogo Externo
CREATE TABLE IF NOT EXISTS cat_sexo_biologico (
    id_sexo SMALLINT PRIMARY KEY,
    clave VARCHAR(10) NOT NULL,
    descripcion VARCHAR(50) NOT NULL,
    codigo_fhir VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS cat_estado_consulta (
    id_estado_consulta SMALLINT PRIMARY KEY,
    clave VARCHAR(20) NOT NULL,
    descripcion VARCHAR(50) NOT NULL,
    codigo_fhir VARCHAR(20)
);

CREATE TABLE IF NOT EXISTS cat_cie10 (
    id_cie10 SERIAL PRIMARY KEY,
    clave VARCHAR(10) NOT NULL,
    descripcion VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS cat_tipo_diagnostico (
    id_tipo_diag SMALLINT PRIMARY KEY,
    clave VARCHAR(20) NOT NULL,
    descripcion VARCHAR(50) NOT NULL
);

-- 2. Tablas Principales Vinculadas por Llaves Foráneas
CREATE TABLE IF NOT EXISTS tbl_pacientes_s2 (
    id_paciente SERIAL PRIMARY KEY,
    curp VARCHAR(18) UNIQUE NOT NULL,
    primer_nombre VARCHAR(50) NOT NULL,
    segundo_nombre VARCHAR(50),
    apellido_paterno VARCHAR(50) NOT NULL,
    apellido_materno VARCHAR(50),
    fecha_nacimiento DATE NOT NULL,
    id_sexo SMALLINT REFERENCES cat_sexo_biologico(id_sexo),
    telefono_contacto VARCHAR(15),
    fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tbl_consultas_s2 (
    id_consulta SERIAL PRIMARY KEY,
    id_paciente INT REFERENCES tbl_pacientes_s2(id_paciente) ON DELETE RESTRICT,
    cedula_medico_tratante VARCHAR(20) NOT NULL,
    nombre_medico VARCHAR(100) NOT NULL,
    id_estado_consulta SMALLINT REFERENCES cat_estado_consulta(id_estado_consulta),
    motivo_consulta TEXT NOT NULL,
    fecha_hora_inicio TIMESTAMP NOT NULL,
    fecha_hora_fin TIMESTAMP NOT NULL,
    unidad_medica VARCHAR(100) DEFAULT 'CESSA Tuxtla Poniente'
);

CREATE TABLE IF NOT EXISTS tbl_signos_s2 (
    id_signo SERIAL PRIMARY KEY,
    id_consulta INT REFERENCES tbl_consultas_s2(id_consulta) ON DELETE CASCADE,
    id_paciente INT REFERENCES tbl_pacientes_s2(id_paciente) ON DELETE RESTRICT,
    presion_sistolica INT NOT NULL,
    presion_diastolica INT NOT NULL,
    frecuencia_cardiaca INT NOT NULL,
    frecuencia_respiratoria INT,
    temperatura_celsius NUMERIC(4, 1) NOT NULL,
    peso_kg NUMERIC(5, 2),
    talla_cm INT,
    fecha_registro TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS tbl_diagnosticos_s2 (
    id_diagnostico SERIAL PRIMARY KEY,
    id_consulta INT REFERENCES tbl_consultas_s2(id_consulta) ON DELETE CASCADE,
    id_paciente INT REFERENCES tbl_pacientes_s2(id_paciente) ON DELETE RESTRICT,
    id_cie10 INT REFERENCES cat_cie10(id_cie10),
    descripcion_diagnostico VARCHAR(255) NOT NULL,
    id_tipo_diag SMALLINT REFERENCES cat_tipo_diagnostico(id_tipo_diag),
    fecha_diagnostico DATE NOT NULL
);

-- ==============================================================================
-- POBLACIÓN DE CATÁLOGOS BASE Y DATOS SINTÉTICOS INICIALES
-- ==============================================================================

INSERT INTO cat_sexo_biologico (id_sexo, clave, descripcion, codigo_fhir) VALUES
(1, 'M', 'Masculino', 'male'),
(2, 'F', 'Femenino', 'female'),
(3, 'I', 'Indeterminado', 'other')
ON CONFLICT (id_sexo) DO NOTHING;

INSERT INTO cat_estado_consulta (id_estado_consulta, clave, descripcion, codigo_fhir) VALUES
(1, 'FINALIZADA', 'Consulta Finalizada', 'finished'),
(2, 'EN_CURSO', 'Consulta en Curso', 'in-progress'),
(3, 'CANCELADA', 'Consulta Cancelada', 'cancelled')
ON CONFLICT (id_estado_consulta) DO NOTHING;

INSERT INTO cat_tipo_diagnostico (id_tipo_diag, clave, descripcion) VALUES
(1, 'CONFIRMADO', 'Diagnóstico Confirmado'),
(2, 'PRESUNTIVO', 'Diagnóstico Presuntivo')
ON CONFLICT (id_tipo_diag) DO NOTHING;

INSERT INTO cat_cie10 (id_cie10, clave, descripcion) VALUES
(1, 'I10', 'Hipertensión esencial (primaria)'),
(2, 'G43.9', 'Migraña, no especificada'),
(3, 'E11.9', 'Diabetes mellitus tipo 2 sin mención de complicación')
ON CONFLICT (id_cie10) DO NOTHING;

-- Datos sintéticos iniciales (IDs 1 y 2)
INSERT INTO tbl_pacientes_s2 (id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, id_sexo, telefono_contacto)
VALUES 
(1, 'ROMA900101HCSMNL09', 'Alberto', 'Manuel', 'Ramos', 'Gómez', '1990-01-01', 1, '9611234567'),
(2, 'LOPE950512MCSPRN07', 'Mariana', NULL, 'López', 'Hernández', '1995-05-12', 2, '9619876543')
ON CONFLICT (id_paciente) DO NOTHING;

INSERT INTO tbl_consultas_s2 (id_consulta, id_paciente, cedula_medico_tratante, nombre_medico, id_estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin)
VALUES 
(1, 1, '8472910', 'Dra. María Elena Cruz Martínez', 1, 'Control trimestral de hipertensión arterial', '2026-09-18 09:15:00', '2026-09-18 09:40:00'),
(2, 2, '9182374', 'Dr. Roberto Mendoza Solís', 1, 'Cefalea persistente y mareos', '2026-09-18 10:00:00', '2026-09-18 10:25:00')
ON CONFLICT (id_consulta) DO NOTHING;

INSERT INTO tbl_signos_s2 (id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius, peso_kg, talla_cm, fecha_registro)
VALUES 
(1, 1, 1, 130, 85, 76, 18, 36.6, 78.5, 172, '2026-09-18 09:18:00'),
(2, 2, 2, 110, 70, 80, NULL, 37.1, 62.0, 160, '2026-09-18 10:05:00')
ON CONFLICT (id_signo) DO NOTHING;

INSERT INTO tbl_diagnosticos_s2 (id_diagnostico, id_consulta, id_paciente, id_cie10, descripcion_diagnostico, id_tipo_diag, fecha_diagnostico)
VALUES 
(1, 1, 1, 1, 'Hipertensión esencial (primaria)', 1, '2026-09-18'),
(2, 2, 2, 2, 'Migraña, no especificada', 1, '2026-09-18')
ON CONFLICT (id_diagnostico) DO NOTHING;

-- Concesión de permisos de solo lectura para el rol medsys_ro
DO $$
BEGIN
  EXECUTE format('GRANT CONNECT ON DATABASE %I TO medsys_ro', current_database());
EXCEPTION
  WHEN OTHERS THEN
    NULL;
END $$;

GRANT USAGE ON SCHEMA public TO medsys_ro;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO medsys_ro;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO medsys_ro;
