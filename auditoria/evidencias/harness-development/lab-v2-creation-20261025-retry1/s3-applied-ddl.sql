CREATE TABLE IF NOT EXISTS tbl_pacientes_s3 (
    pac_id SERIAL PRIMARY KEY,
    pac_curp VARCHAR(18) NOT NULL,
    pac_nom1 VARCHAR(50) NOT NULL,
    pac_nom2 VARCHAR(50),
    pac_ap_pat VARCHAR(50) NOT NULL,
    pac_ap_mat VARCHAR(50),
    fech_nac_txt VARCHAR(20) NOT NULL,
    sexo CHAR(1),
    tel_contacto VARCHAR(15),
    fech_reg_txt VARCHAR(25)
);

-- 2. Consultas sin restricción FOREIGN KEY declarada en el DDL (deuda técnica T4)
CREATE TABLE IF NOT EXISTS tbl_consultas_s3 (
    con_id SERIAL PRIMARY KEY,
    pac_id INT NOT NULL,  -- Deuda técnica: sin FK declarada en DDL
    ced_medico VARCHAR(20) NOT NULL,
    nom_medico VARCHAR(100) NOT NULL,
    est_consulta VARCHAR(20) DEFAULT 'FINALIZADA',
    motivo TEXT NOT NULL,
    fech_ini_txt VARCHAR(25) NOT NULL,
    fech_fin_txt VARCHAR(25) NOT NULL,
    unidad_medica VARCHAR(100) DEFAULT 'CESSA Tuxtla Poniente'
);

-- 3. Signos vitales con presión arterial concatenada "120/80" y peso en libras (deuda técnica T2)
CREATE TABLE IF NOT EXISTS tbl_signos_s3 (
    sig_id SERIAL PRIMARY KEY,
    con_id INT NOT NULL,  -- Deuda técnica: sin FK declarada en DDL
    pac_id INT NOT NULL,  -- Deuda técnica: sin FK declarada en DDL
    presion_arterial_txt VARCHAR(15) NOT NULL,
    fc INT NOT NULL,
    fr INT,
    temp_celsius NUMERIC(4, 1) NOT NULL,
    peso_libras NUMERIC(6, 2),
    talla_cm INT,
    fech_reg_txt VARCHAR(25) NOT NULL
);

-- 4. Diagnósticos con columnas abreviadas y sin restricción FOREIGN KEY DDL
CREATE TABLE IF NOT EXISTS tbl_diagnosticos_s3 (
    diag_id SERIAL PRIMARY KEY,
    con_id INT NOT NULL,  -- Deuda técnica: sin FK declarada en DDL
    pac_id INT NOT NULL,  -- Deuda técnica: sin FK declarada en DDL
    cie10 VARCHAR(10) NOT NULL,
    desc_diag VARCHAR(255) NOT NULL,
    tipo_diag VARCHAR(20),
    fech_diag_txt VARCHAR(25) NOT NULL
);

-- ==============================================================================
-- DATOS SINTÉTICOS DE LABORATORIO (Semilla inicial)
-- ==============================================================================

INSERT INTO tbl_pacientes_s3 (pac_id, pac_curp, pac_nom1, pac_nom2, pac_ap_pat, pac_ap_mat, fech_nac_txt, sexo, tel_contacto, fech_reg_txt)
VALUES 
(1, 'ROMA900101HCSMNL09', 'Alberto', 'Manuel', 'Ramos', 'Gómez', '01/01/1990', 'M', '9611234567', '18/09/2026 09:00:00'),
(2, 'LOPE950512MCSPRN07', 'Mariana', NULL, 'López', 'Hernández', '12/05/1995', 'F', '9619876543', '18/09/2026 09:00:00')
ON CONFLICT (pac_id) DO NOTHING;

INSERT INTO tbl_consultas_s3 (con_id, pac_id, ced_medico, nom_medico, est_consulta, motivo, fech_ini_txt, fech_fin_txt)
VALUES 
(1, 1, '8472910', 'Dra. María Elena Cruz Martínez', 'FINALIZADA', 'Control trimestral de hipertensión arterial', '18/09/2026 09:15:00', '18/09/2026 09:40:00'),
(2, 2, '9182374', 'Dr. Roberto Mendoza Solís', 'FINALIZADA', 'Cefalea persistente y mareos', '18/09/2026 10:00:00', '18/09/2026 10:25:00')
ON CONFLICT (con_id) DO NOTHING;

-- Alberto: 78.5 kg = 173.06 lbs. Mariana: 62.0 kg = 136.69 lbs
INSERT INTO tbl_signos_s3 (sig_id, con_id, pac_id, presion_arterial_txt, fc, fr, temp_celsius, peso_libras, talla_cm, fech_reg_txt)
VALUES 
(1, 1, 1, '130/85', 76, 18, 36.6, 173.06, 172, '18/09/2026 09:18:00'),
(2, 2, 2, '110/70', 80, NULL, 37.1, 136.69, 160, '18/09/2026 10:05:00')
ON CONFLICT (sig_id) DO NOTHING;

INSERT INTO tbl_diagnosticos_s3 (diag_id, con_id, pac_id, cie10, desc_diag, tipo_diag, fech_diag_txt)
VALUES 
(1, 1, 1, 'I10', 'Hipertensión esencial (primaria)', 'CONFIRMADO', '18/09/2026'),
(2, 2, 2, 'G43.9', 'Migraña, no especificada', 'CONFIRMADO', '18/09/2026')
ON CONFLICT (diag_id) DO NOTHING;

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
