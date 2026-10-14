-- ==============================================================================
-- ARQUETIPO S1: SÁBANA PLANA DESNORMALIZADA (ESTILO SINBA / SIS)
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

-- 1. Tabla plana desnormalizada que concentra Paciente, Consulta y Signos Vitales
CREATE TABLE IF NOT EXISTS tbl_sabana_consulta_externa (
    id_consulta SERIAL PRIMARY KEY,
    id_paciente INT NOT NULL,
    curp VARCHAR(18) NOT NULL,
    primer_nombre VARCHAR(50) NOT NULL,
    segundo_nombre VARCHAR(50),
    apellido_paterno VARCHAR(50) NOT NULL,
    apellido_materno VARCHAR(50),
    fecha_nacimiento DATE NOT NULL,
    sexo_biologico CHAR(1) CHECK (sexo_biologico IN ('M', 'F', 'I')),
    telefono_contacto VARCHAR(15),
    cedula_medico_tratante VARCHAR(20) NOT NULL,
    nombre_medico VARCHAR(100) NOT NULL,
    estado_consulta VARCHAR(20) DEFAULT 'FINALIZADA' CHECK (estado_consulta IN ('FINALIZADA', 'EN_CURSO', 'CANCELADA')),
    motivo_consulta TEXT NOT NULL,
    fecha_hora_inicio TIMESTAMP NOT NULL,
    fecha_hora_fin TIMESTAMP NOT NULL,
    unidad_medica VARCHAR(100) DEFAULT 'CESSA Tuxtla Poniente',
    id_signo INT UNIQUE NOT NULL,
    presion_sistolica INT NOT NULL,
    presion_diastolica INT NOT NULL,
    frecuencia_cardiaca INT NOT NULL,
    frecuencia_respiratoria INT,
    temperatura_celsius NUMERIC(4, 1) NOT NULL,
    peso_kg NUMERIC(5, 2),
    talla_cm INT,
    fecha_registro TIMESTAMP NOT NULL
);

-- 2. Tabla plana de diagnósticos asociados
CREATE TABLE IF NOT EXISTS tbl_sabana_diagnosticos (
    id_diagnostico SERIAL PRIMARY KEY,
    id_consulta INT NOT NULL,
    id_paciente INT NOT NULL,
    codigo_cie10 VARCHAR(10) NOT NULL,
    descripcion_diagnostico VARCHAR(255) NOT NULL,
    tipo_diagnostico VARCHAR(20) CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
    fecha_diagnostico DATE NOT NULL
);

-- Índices recomendados para consultas desnormalizadas con DISTINCT ON
CREATE INDEX IF NOT EXISTS idx_sabana_id_paciente ON tbl_sabana_consulta_externa(id_paciente);
CREATE INDEX IF NOT EXISTS idx_sabana_curp ON tbl_sabana_consulta_externa(curp);
CREATE INDEX IF NOT EXISTS idx_sabana_diag_consulta ON tbl_sabana_diagnosticos(id_consulta);

-- ==============================================================================
-- DATOS SINTÉTICOS DE LABORATORIO (Semilla inicial)
-- ==============================================================================

INSERT INTO tbl_sabana_consulta_externa (
    id_consulta, id_paciente, curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno,
    fecha_nacimiento, sexo_biologico, telefono_contacto, cedula_medico_tratante, nombre_medico,
    estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin, unidad_medica,
    id_signo, presion_sistolica, presion_diastolica, frecuencia_cardiaca, frecuencia_respiratoria,
    temperatura_celsius, peso_kg, talla_cm, fecha_registro
) VALUES
(1, 1, 'ROMA900101HCSMNL09', 'Alberto', 'Manuel', 'Ramos', 'Gómez', '1990-01-01', 'M', '9611234567',
 '8472910', 'Dra. María Elena Cruz Martínez', 'FINALIZADA', 'Control trimestral de hipertensión arterial',
 '2026-09-18 09:15:00', '2026-09-18 09:40:00', 'CESSA Tuxtla Poniente',
 1, 130, 85, 76, 18, 36.6, 78.5, 172, '2026-09-18 09:18:00'),
(2, 2, 'LOPE950512MCSPRN07', 'Mariana', NULL, 'López', 'Hernández', '1995-05-12', 'F', '9619876543',
 '9182374', 'Dr. Roberto Mendoza Solís', 'FINALIZADA', 'Cefalea persistente y mareos',
 '2026-09-18 10:00:00', '2026-09-18 10:25:00', 'CESSA Tuxtla Poniente',
 2, 110, 70, 80, NULL, 37.1, 62.0, 160, '2026-09-18 10:05:00')
ON CONFLICT (id_consulta) DO NOTHING;

INSERT INTO tbl_sabana_diagnosticos (
    id_diagnostico, id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
) VALUES
(1, 1, 1, 'I10', 'Hipertensión esencial (primaria)', 'CONFIRMADO', '2026-09-18'),
(2, 2, 2, 'G43.9', 'Migraña, no especificada', 'CONFIRMADO', '2026-09-18')
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
