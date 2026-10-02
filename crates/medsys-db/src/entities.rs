//! Entidades relacionales intermedias mapeadas desde PostgreSQL con SQLx.
//!
//! Estas entidades representan las filas de las tablas del esquema legado sintético
//! basado en la NOM-004-SSA3-2012 y se convierten sin pérdidas hacia los modelos de
//! dominio clínico `LegacyPaciente`, `LegacyConsulta`, `LegacySignoVital` y `LegacyDiagnostico`.

use chrono::{NaiveDate, NaiveDateTime};
use medsys_core::model::legacy::{
    LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
};
use rust_decimal::Decimal;
use sqlx::FromRow;

/// Entidad relacional correspondiente a la tabla `tbl_pacientes`.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct PacienteEntity {
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

impl From<PacienteEntity> for LegacyPaciente {
    fn from(e: PacienteEntity) -> Self {
        Self {
            id_paciente: e.id_paciente,
            curp: e.curp,
            primer_nombre: e.primer_nombre,
            segundo_nombre: e.segundo_nombre,
            apellido_paterno: e.apellido_paterno,
            apellido_materno: e.apellido_materno,
            fecha_nacimiento: e.fecha_nacimiento,
            sexo_biologico: e.sexo_biologico,
            telefono_contacto: e.telefono_contacto,
            fecha_registro: e.fecha_registro,
        }
    }
}

impl From<LegacyPaciente> for PacienteEntity {
    fn from(m: LegacyPaciente) -> Self {
        Self {
            id_paciente: m.id_paciente,
            curp: m.curp,
            primer_nombre: m.primer_nombre,
            segundo_nombre: m.segundo_nombre,
            apellido_paterno: m.apellido_paterno,
            apellido_materno: m.apellido_materno,
            fecha_nacimiento: m.fecha_nacimiento,
            sexo_biologico: m.sexo_biologico,
            telefono_contacto: m.telefono_contacto,
            fecha_registro: m.fecha_registro,
        }
    }
}

/// Entidad relacional correspondiente a la tabla `tbl_consultas`.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct ConsultaEntity {
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

impl From<ConsultaEntity> for LegacyConsulta {
    fn from(e: ConsultaEntity) -> Self {
        Self {
            id_consulta: e.id_consulta,
            id_paciente: e.id_paciente,
            cedula_medico_tratante: e.cedula_medico_tratante,
            nombre_medico: e.nombre_medico,
            estado_consulta: e.estado_consulta,
            motivo_consulta: e.motivo_consulta,
            fecha_hora_inicio: e.fecha_hora_inicio,
            fecha_hora_fin: e.fecha_hora_fin,
            unidad_medica: e.unidad_medica,
        }
    }
}

impl From<LegacyConsulta> for ConsultaEntity {
    fn from(m: LegacyConsulta) -> Self {
        Self {
            id_consulta: m.id_consulta,
            id_paciente: m.id_paciente,
            cedula_medico_tratante: m.cedula_medico_tratante,
            nombre_medico: m.nombre_medico,
            estado_consulta: m.estado_consulta,
            motivo_consulta: m.motivo_consulta,
            fecha_hora_inicio: m.fecha_hora_inicio,
            fecha_hora_fin: m.fecha_hora_fin,
            unidad_medica: m.unidad_medica,
        }
    }
}

/// Entidad relacional correspondiente a la tabla `tbl_signos_vitales`.
#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct SignoVitalEntity {
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

impl From<SignoVitalEntity> for LegacySignoVital {
    fn from(e: SignoVitalEntity) -> Self {
        Self {
            id_signo: e.id_signo,
            id_consulta: e.id_consulta,
            id_paciente: e.id_paciente,
            presion_sistolica: e.presion_sistolica,
            presion_diastolica: e.presion_diastolica,
            frecuencia_cardiaca: e.frecuencia_cardiaca,
            frecuencia_respiratoria: e.frecuencia_respiratoria,
            temperatura_celsius: e.temperatura_celsius,
            peso_kg: e.peso_kg,
            talla_cm: e.talla_cm,
            fecha_registro: e.fecha_registro,
        }
    }
}

impl From<LegacySignoVital> for SignoVitalEntity {
    fn from(m: LegacySignoVital) -> Self {
        Self {
            id_signo: m.id_signo,
            id_consulta: m.id_consulta,
            id_paciente: m.id_paciente,
            presion_sistolica: m.presion_sistolica,
            presion_diastolica: m.presion_diastolica,
            frecuencia_cardiaca: m.frecuencia_cardiaca,
            frecuencia_respiratoria: m.frecuencia_respiratoria,
            temperatura_celsius: m.temperatura_celsius,
            peso_kg: m.peso_kg,
            talla_cm: m.talla_cm,
            fecha_registro: m.fecha_registro,
        }
    }
}

/// Entidad relacional correspondiente a la tabla `tbl_diagnosticos`.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct DiagnosticoEntity {
    pub id_diagnostico: i32,
    pub id_consulta: i32,
    pub id_paciente: i32,
    pub codigo_cie10: String,
    pub descripcion_diagnostico: String,
    pub tipo_diagnostico: Option<String>,
    pub fecha_diagnostico: NaiveDate,
}

impl From<DiagnosticoEntity> for LegacyDiagnostico {
    fn from(e: DiagnosticoEntity) -> Self {
        Self {
            id_diagnostico: e.id_diagnostico,
            id_consulta: e.id_consulta,
            id_paciente: e.id_paciente,
            codigo_cie10: e.codigo_cie10,
            descripcion_diagnostico: e.descripcion_diagnostico,
            tipo_diagnostico: e.tipo_diagnostico,
            fecha_diagnostico: e.fecha_diagnostico,
        }
    }
}

impl From<LegacyDiagnostico> for DiagnosticoEntity {
    fn from(m: LegacyDiagnostico) -> Self {
        Self {
            id_diagnostico: m.id_diagnostico,
            id_consulta: m.id_consulta,
            id_paciente: m.id_paciente,
            codigo_cie10: m.codigo_cie10,
            descripcion_diagnostico: m.descripcion_diagnostico,
            tipo_diagnostico: m.tipo_diagnostico,
            fecha_diagnostico: m.fecha_diagnostico,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paciente_entity_conversion_roundtrip() {
        let original = LegacyPaciente {
            id_paciente: 1,
            curp: "ROMA900101HCSNN01".to_string(),
            primer_nombre: "Alberto".to_string(),
            segundo_nombre: Some("Manuel".to_string()),
            apellido_paterno: "Ramos".to_string(),
            apellido_materno: Some("Gómez".to_string()),
            fecha_nacimiento: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            sexo_biologico: Some("M".to_string()),
            telefono_contacto: Some("9611234567".to_string()),
            fecha_registro: None,
        };

        let entity: PacienteEntity = original.clone().into();
        let back: LegacyPaciente = entity.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_consulta_entity_conversion_roundtrip() {
        let dt_inicio = NaiveDate::from_ymd_opt(2026, 9, 18)
            .unwrap()
            .and_hms_opt(9, 15, 0)
            .unwrap();
        let dt_fin = NaiveDate::from_ymd_opt(2026, 9, 18)
            .unwrap()
            .and_hms_opt(9, 40, 0)
            .unwrap();

        let original = LegacyConsulta {
            id_consulta: 1,
            id_paciente: 1,
            cedula_medico_tratante: "8472910".to_string(),
            nombre_medico: "Dra. María Elena Cruz Martínez".to_string(),
            estado_consulta: "FINALIZADA".to_string(),
            motivo_consulta: "Control trimestral de hipertensión arterial".to_string(),
            fecha_hora_inicio: dt_inicio,
            fecha_hora_fin: dt_fin,
            unidad_medica: Some("CESSA Tuxtla Poniente".to_string()),
        };

        let entity: ConsultaEntity = original.clone().into();
        let back: LegacyConsulta = entity.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_signo_vital_entity_conversion_roundtrip() {
        let dt = NaiveDate::from_ymd_opt(2026, 9, 18)
            .unwrap()
            .and_hms_opt(9, 18, 0)
            .unwrap();

        let original = LegacySignoVital {
            id_signo: 1,
            id_consulta: 1,
            id_paciente: 1,
            presion_sistolica: 130,
            presion_diastolica: 85,
            frecuencia_cardiaca: 76,
            frecuencia_respiratoria: None,
            temperatura_celsius: Decimal::new(366, 1),
            peso_kg: Some(Decimal::new(7850, 2)),
            talla_cm: Some(172),
            fecha_registro: dt,
        };

        let entity: SignoVitalEntity = original.clone().into();
        let back: LegacySignoVital = entity.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_diagnostico_entity_conversion_roundtrip() {
        let original = LegacyDiagnostico {
            id_diagnostico: 1,
            id_consulta: 1,
            id_paciente: 1,
            codigo_cie10: "I10".to_string(),
            descripcion_diagnostico: "Hipertensión esencial (primaria)".to_string(),
            tipo_diagnostico: Some("CONFIRMADO".to_string()),
            fecha_diagnostico: NaiveDate::from_ymd_opt(2026, 9, 18).unwrap(),
        };

        let entity: DiagnosticoEntity = original.clone().into();
        let back: LegacyDiagnostico = entity.into();
        assert_eq!(original, back);
    }
}
