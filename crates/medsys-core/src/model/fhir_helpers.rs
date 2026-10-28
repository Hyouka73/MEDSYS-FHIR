use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeZone};
use helios_fhir::r4::{
    Code, CodeableConcept, Coding, Date, DateTime, Decimal, Extension, Id, Identifier, Reference,
    String as FhirString, Uri,
};
use helios_fhir::{DecimalElement, Element, PrecisionDate, PrecisionDateTime};
use rust_decimal::Decimal as RustDecimal;

use crate::error::{MedSysError, Result};

/// Crea un elemento primitivo `String` de FHIR.
pub fn fhir_string(val: impl Into<String>) -> FhirString {
    Element {
        id: None,
        extension: None,
        value: Some(val.into()),
    }
}

/// Crea un elemento primitivo `Id` de FHIR.
pub fn fhir_id(val: impl Into<String>) -> Id {
    Element {
        id: None,
        extension: None,
        value: Some(val.into()),
    }
}

/// Crea un elemento primitivo `Code` de FHIR.
pub fn fhir_code(val: impl Into<String>) -> Code {
    Element {
        id: None,
        extension: None,
        value: Some(val.into()),
    }
}

/// Crea un elemento primitivo `Uri` de FHIR.
pub fn fhir_uri(val: impl Into<String>) -> Uri {
    Element {
        id: None,
        extension: None,
        value: Some(val.into()),
    }
}

/// Crea un elemento primitivo `Date` de FHIR a partir de un `NaiveDate` de Chrono.
pub fn fhir_date(date: NaiveDate) -> Result<Date> {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let precision_date = PrecisionDate::from_ymd(year, month, day);

    Ok(Element {
        id: None,
        extension: None,
        value: Some(precision_date),
    })
}

/// Crea un elemento primitivo `DateTime` de FHIR a partir de un `NaiveDateTime` de Chrono.
pub fn fhir_datetime(dt: NaiveDateTime) -> Result<DateTime> {
    fhir_datetime_in_timezone(dt, "America/Mexico_City")
}

/// Interpreta el reloj de origen en la zona declarada; no adivina en cambios DST.
pub fn fhir_datetime_in_timezone(dt: NaiveDateTime, timezone: &str) -> Result<DateTime> {
    let tz: chrono_tz::Tz = timezone
        .parse()
        .map_err(|_| MedSysError::InvalidRule(format!("Zona horaria inválida: {timezone}")))?;
    let instant = tz.from_local_datetime(&dt).single().ok_or_else(|| {
        MedSysError::ProcessingError(format!(
            "T2 Fecha/hora local ambigua o inexistente en {timezone}: {dt}"
        ))
    })?;
    let iso_str = instant.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, false);
    let precision_dt = PrecisionDateTime::parse(&iso_str).ok_or_else(|| {
        MedSysError::TransformationError(format!("Fecha/hora FHIR inválida: {iso_str}"))
    })?;
    Ok(Element {
        id: None,
        extension: None,
        value: Some(precision_dt),
    })
}

/// Una fecha observada sin hora conserva su precisión en un campo FHIR dateTime.
pub fn fhir_date_as_datetime(date: NaiveDate) -> Result<DateTime> {
    let text = date.format("%Y-%m-%d").to_string();
    let value = PrecisionDateTime::parse(&text)
        .ok_or_else(|| MedSysError::TransformationError(format!("Fecha FHIR inválida: {text}")))?;
    Ok(Element {
        id: None,
        extension: None,
        value: Some(value),
    })
}

/// Crea un elemento numérico de alta precisión `Decimal` de FHIR.
pub fn fhir_decimal(value: RustDecimal) -> Decimal {
    DecimalElement::<Extension>::new(value)
}

/// Crea una estructura `Coding` de FHIR.
pub fn fhir_coding(system: Option<&str>, code: Option<&str>, display: Option<&str>) -> Coding {
    Coding {
        id: None,
        extension: None,
        system: system.map(fhir_uri),
        version: None,
        code: code.map(fhir_code),
        display: display.map(fhir_string),
        user_selected: None,
    }
}

/// Crea una estructura `CodeableConcept` de FHIR con una o varias codificaciones.
pub fn fhir_concept(
    system: Option<&str>,
    code: Option<&str>,
    display: Option<&str>,
    text: Option<&str>,
) -> CodeableConcept {
    let coding = if system.is_some() || code.is_some() || display.is_some() {
        Some(vec![fhir_coding(system, code, display)])
    } else {
        None
    };

    CodeableConcept {
        id: None,
        extension: None,
        coding,
        text: text.map(fhir_string),
    }
}

/// Crea una referencia relativa o absoluta FHIR (`Reference`).
pub fn fhir_reference(reference_str: &str, display: Option<&str>) -> Reference {
    Reference {
        id: None,
        extension: None,
        reference: Some(fhir_string(reference_str)),
        r#type: None,
        identifier: None,
        display: display.map(fhir_string),
    }
}

/// Crea un `Identifier` de FHIR con sistema, valor y propósito de uso.
pub fn fhir_identifier(system: Option<&str>, value: &str, use_code: Option<&str>) -> Identifier {
    Identifier {
        id: None,
        extension: None,
        r#use: use_code.map(fhir_code),
        r#type: None,
        system: system.map(fhir_uri),
        value: Some(fhir_string(value)),
        period: None,
        assigner: None,
    }
}

/// URL oficial de la extensión canónica FHIR R4 para razones de ausencia de datos.
pub const FHIR_DATA_ABSENT_REASON_URL: &str =
    "http://hl7.org/fhir/StructureDefinition/data-absent-reason";

/// Crea una extensión `data-absent-reason` de FHIR R4 con valueCode: "error".
pub fn fhir_data_absent_reason_extension() -> Extension {
    Extension {
        id: None,
        extension: None,
        url: fhir_uri(FHIR_DATA_ABSENT_REASON_URL),
        value: Some(helios_fhir::r4::ExtensionValue::Code(fhir_code("error"))),
    }
}

/// Crea un elemento primitivo FHIR cuyo valor está ausente por error,
/// documentado formalmente bajo la extensión estándar `data-absent-reason`.
pub fn fhir_absent_element<T>() -> Element<T, Extension> {
    Element {
        id: None,
        extension: Some(vec![fhir_data_absent_reason_extension()]),
        value: None,
    }
}
