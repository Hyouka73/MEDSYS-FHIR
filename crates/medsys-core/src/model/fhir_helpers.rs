use chrono::{Datelike, NaiveDate, NaiveDateTime};
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
    let iso_str = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    let precision_dt = PrecisionDateTime::parse(&iso_str).ok_or_else(|| {
        MedSysError::TransformationError(format!(
            "No se pudo parsear la fecha/hora en formato FHIR: {iso_str}"
        ))
    })?;

    Ok(Element {
        id: None,
        extension: None,
        value: Some(precision_dt),
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
