use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Estructura que representa una fila de `tbl_pacientes` del esquema legado simulado (NOM-004-SSA3-2012).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyPaciente {
    pub id_paciente: i32,
    pub curp: String,
    pub primer_nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido_paterno: String,
    pub apellido_materno: Option<String>,
    pub fecha_nacimiento: NaiveDate,
    pub sexo_biologico: Option<String>,
    pub telefono_contacto: Option<String>,
    pub fecha_registro: Option<NaiveDateTime>,
}

/// Estructura que representa una fila de `tbl_consultas` del esquema legado simulado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyConsulta {
    pub id_consulta: i32,
    pub id_paciente: i32,
    pub cedula_medico_tratante: String,
    pub nombre_medico: String,
    pub estado_consulta: String,
    pub motivo_consulta: String,
    pub fecha_hora_inicio: NaiveDateTime,
    pub fecha_hora_fin: NaiveDateTime,
    pub unidad_medica: Option<String>,
}

/// Estructura que representa una fila de `tbl_signos_vitales` del esquema legado simulado.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegacySignoVital {
    pub id_signo: i32,
    pub id_consulta: i32,
    pub id_paciente: i32,
    pub presion_sistolica: i32,
    pub presion_diastolica: i32,
    pub frecuencia_cardiaca: i32,
    pub frecuencia_respiratoria: Option<i32>,
    pub temperatura_celsius: Decimal,
    pub peso_kg: Option<Decimal>,
    pub talla_cm: Option<i32>,
    pub fecha_registro: NaiveDateTime,
}

/// Estructura que representa una fila de `tbl_diagnosticos` del esquema legado simulado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyDiagnostico {
    pub id_diagnostico: i32,
    pub id_consulta: i32,
    pub id_paciente: i32,
    pub codigo_cie10: String,
    pub descripcion_diagnostico: String,
    pub tipo_diagnostico: String,
    pub fecha_diagnostico: NaiveDate,
}
