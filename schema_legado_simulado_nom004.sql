-- ==============================================================================
-- ESQUEMA RELACIONAL LEGADO SIMULADO — MedSys-FHIR
-- Basado deductivamente en campos normativos de la NOM-004-SSA3-2012 y NOM-024-SSA3-2012
-- Motor: PostgreSQL 16
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
END $$;

CREATE TABLE IF NOT EXISTS tbl_pacientes (
    id_paciente SERIAL PRIMARY KEY,
    curp VARCHAR(18) UNIQUE NOT NULL,
    primer_nombre VARCHAR(50) NOT NULL,
    segundo_nombre VARCHAR(50),
    apellido_paterno VARCHAR(50) NOT NULL,
    apellido_materno VARCHAR(50),
    fecha_nacimiento DATE NOT NULL,
    sexo_biologico CHAR(1) CHECK (sexo_biologico IN ('M', 'F', 'I')),
    telefono_contacto VARCHAR(15),
    fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tbl_consultas (
    id_consulta SERIAL PRIMARY KEY,
    id_paciente INT REFERENCES tbl_pacientes(id_paciente) ON DELETE RESTRICT,
    cedula_medico_tratante VARCHAR(20) NOT NULL,
    nombre_medico VARCHAR(100) NOT NULL,
    estado_consulta VARCHAR(20) DEFAULT 'FINALIZADA' CHECK (estado_consulta IN ('FINALIZADA', 'EN_CURSO', 'CANCELADA')),
    motivo_consulta TEXT NOT NULL,
    fecha_hora_inicio TIMESTAMP NOT NULL,
    fecha_hora_fin TIMESTAMP NOT NULL,
    unidad_medica VARCHAR(100) DEFAULT 'CESSA Tuxtla Poniente'
);

CREATE TABLE IF NOT EXISTS tbl_signos_vitales (
    id_signo SERIAL PRIMARY KEY,
    id_consulta INT REFERENCES tbl_consultas(id_consulta) ON DELETE CASCADE,
    id_paciente INT REFERENCES tbl_pacientes(id_paciente) ON DELETE RESTRICT,
    presion_sistolica INT NOT NULL,
    presion_diastolica INT NOT NULL,
    frecuencia_cardiaca INT NOT NULL,
    frecuencia_respiratoria INT,
    temperatura_celsius NUMERIC(4, 1) NOT NULL,
    peso_kg NUMERIC(5, 2),
    talla_cm INT,
    fecha_registro TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS tbl_diagnosticos (
    id_diagnostico SERIAL PRIMARY KEY,
    id_consulta INT REFERENCES tbl_consultas(id_consulta) ON DELETE CASCADE,
    id_paciente INT REFERENCES tbl_pacientes(id_paciente) ON DELETE RESTRICT,
    codigo_cie10 VARCHAR(10) NOT NULL,
    descripcion_diagnostico VARCHAR(255) NOT NULL,
    tipo_diagnostico VARCHAR(20) DEFAULT 'CONFIRMADO' CHECK (tipo_diagnostico IN ('PRESUNTIVO', 'CONFIRMADO')),
    fecha_diagnostico DATE NOT NULL
);

-- ==============================================================================
-- DATOS SINTÉTICOS DE PRUEBA (LABORATORIO)
-- ==============================================================================

INSERT INTO tbl_pacientes (curp, primer_nombre, segundo_nombre, apellido_paterno, apellido_materno, fecha_nacimiento, sexo_biologico, telefono_contacto)
VALUES 
('ROMA900101HCSNN01', 'Alberto', 'Manuel', 'Ramos', 'Gómez', '1990-01-01', 'M', '9611234567'),
('LOPE950512MCSNN02', 'Mariana', NULL, 'López', 'Hernández', '1995-05-12', 'F', '9619876543');

INSERT INTO tbl_consultas (id_paciente, cedula_medico_tratante, nombre_medico, estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin)
VALUES 
(1, '8472910', 'Dra. María Elena Cruz Martínez', 'FINALIZADA', 'Control trimestral de hipertensión arterial', '2026-09-18 09:15:00', '2026-09-18 09:40:00'),
(2, '9182374', 'Dr. Roberto Mendoza Solís', 'FINALIZADA', 'Cefalea persistente y mareos', '2026-09-18 10:00:00', '2026-09-18 10:25:00');

INSERT INTO tbl_signos_vitales (id_consulta, id_paciente, presion_sistolica, presion_diastolica, frecuencia_cardiaca, temperatura_celsius, peso_kg, talla_cm, fecha_registro)
VALUES 
(1, 1, 130, 85, 76, 36.6, 78.5, 172, '2026-09-18 09:18:00'),
(2, 2, 110, 70, 80, 37.1, 62.0, 160, '2026-09-18 10:05:00');

INSERT INTO tbl_diagnosticos (id_consulta, id_paciente, codigo_cie10, descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico)
VALUES 
(1, 1, 'I10', 'Hipertensión esencial (primaria)', 'CONFIRMADO', '2026-09-18'),
(2, 2, 'G43.9', 'Migraña, no especificada', 'CONFIRMADO', '2026-09-18');
