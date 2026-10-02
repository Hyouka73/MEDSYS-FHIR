// ============================================================================
// MedSys-FHIR: Prueba de Resiliencia y OperationOutcome (Stress Errors) — k6
// Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
// ============================================================================
// Inyecta fallos y peticiones anómalas concurrentes para certificar que el 100%
// de las excepciones del middleware se traduzcan invariablemente a OperationOutcome
// bajo Content-Type: application/fhir+json; charset=utf-8.
//
// Refactorización: Se elimina el anti-patrón de inventar datos falsos (fallback_value).
// Ahora ante datos críticos corruptos el middleware emite OperationOutcome HTTP 422,
// y ante datos ausentes o no casteables se valida la presencia de data-absent-reason.
// ============================================================================

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Rate } from 'k6/metrics';

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export const operationOutcomeConformityRate = new Rate('operation_outcome_conformity_rate');

export const options = {
  vus: 10,
  duration: '15s',
  thresholds: {
    // 100% de las respuestas anómalas deben ser conformes a OperationOutcome
    'operation_outcome_conformity_rate': ['rate==1.0'],
    'http_req_duration': ['p(95)<30'], // El middleware debe responder a errores en menos de 30ms
  },
};

const fhirHeaders = {
  headers: {
    'Accept': 'application/fhir+json, application/json',
  },
};

export default function () {
  group('Inyección de Fallos Controlados y Resiliencia FHIR R4', function () {
    // 1. Error 400: Parámetro de ruta con formato inválido
    const resBadParam = http.get(
      `${BASE_URL}/fhir/r4/Condition/id_texto_invalido_alfa`,
      fhirHeaders
    );
    const isBadParamConformant =
      resBadParam.status === 400 &&
      resBadParam.headers['Content-Type'] &&
      resBadParam.headers['Content-Type'].includes('application/fhir+json') &&
      resBadParam.body.includes('"resourceType": "OperationOutcome"') &&
      resBadParam.body.includes('"code": "value"');

    operationOutcomeConformityRate.add(isBadParamConformant);
    check(resBadParam, {
      'BadParam: Status 400': (r) => r.status === 400,
      'BadParam: Content-Type FHIR': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'BadParam: OperationOutcome emitido': () => isBadParamConformant,
    });

    // 2. Error 404: Ruta inexistente fuera de catálogo (Fallback Universal Axum)
    const resFallback = http.get(
      `${BASE_URL}/fhir/r4/ServicioInexistente/test-999`,
      fhirHeaders
    );
    const isFallbackConformant =
      resFallback.status === 404 &&
      resFallback.headers['Content-Type'] &&
      resFallback.headers['Content-Type'].includes('application/fhir+json') &&
      resFallback.body.includes('"resourceType": "OperationOutcome"') &&
      resFallback.body.includes('"code": "not-found"');

    operationOutcomeConformityRate.add(isFallbackConformant);
    check(resFallback, {
      'Fallback: Status 404': (r) => r.status === 404,
      'Fallback: Content-Type FHIR': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'Fallback: OperationOutcome emitido': () => isFallbackConformant,
    });

    // 3. Error 404: Identificador numérico negativo (fuera de dominio relacional)
    const resNegativeId = http.get(
      `${BASE_URL}/fhir/r4/Patient/-1`,
      fhirHeaders
    );
    const isNegativeIdConformant =
      resNegativeId.status === 404 &&
      resNegativeId.headers['Content-Type'] &&
      resNegativeId.headers['Content-Type'].includes('application/fhir+json') &&
      resNegativeId.body.includes('"resourceType": "OperationOutcome"');

    operationOutcomeConformityRate.add(isNegativeIdConformant);
    check(resNegativeId, {
      'NegativeId: Status 404': (r) => r.status === 404,
      'NegativeId: Content-Type FHIR': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'NegativeId: OperationOutcome emitido': () => isNegativeIdConformant,
    });

    // 4. Error 422 o 404: ID inexistente fuera de rango devuelve OperationOutcome
    // El motor elimina el anti-patrón de inventar datos falseados; ante datos obligatorios ausentes
    // debe emitirse estrictamente un OperationOutcome con status HTTP 422 o 404 según corresponda.
    const resCorruptEntity = http.get(
      `${BASE_URL}/fhir/r4/Patient/999999`,
      fhirHeaders
    );
    const isErrorOrOutcome =
      (resCorruptEntity.status === 422 || resCorruptEntity.status === 404) &&
      resCorruptEntity.headers['Content-Type'] &&
      resCorruptEntity.headers['Content-Type'].includes('application/fhir+json') &&
      resCorruptEntity.body.includes('"resourceType": "OperationOutcome"');

    operationOutcomeConformityRate.add(isErrorOrOutcome);
    check(resCorruptEntity, {
      'Corrupt/Missing: Status 422 o 404 controlado': (r) =>
        r.status === 422 || r.status === 404,
      'Corrupt/Missing: Content-Type FHIR': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'Corrupt/Missing: OperationOutcome emitido sin datos falseados': () => isErrorOrOutcome,
    });

    // 5. Validación de la extensión data-absent-reason (HL7 FHIR R4):
    // Se certifica que las respuestas exitosas NO contengan datos clínicos falseados arbitrarios
    // (como '1970-01-01' o valores de rescate inventados). En caso de existir degradación por datos
    // ausentes o no conformes, se valida la presencia de la extensión canónica data-absent-reason.
    const resPatient = http.get(`${BASE_URL}/fhir/r4/Patient/1`, fhirHeaders);
    if (resPatient.status === 200) {
      check(resPatient, {
        'No contiene fechas falseadas 1970-01-01': (r) => !r.body.includes('1970-01-01'),
        'No contiene valores de rescate inventados': (r) =>
          !r.body.includes('VALOR_RESCATE_DEFAULT'),
        'Estructura FHIR R4 conforme (Patient o data-absent-reason)': (r) => {
          try {
            const body = r.body;
            if (body.includes('data-absent-reason')) {
              return (
                body.includes('http://hl7.org/fhir/StructureDefinition/data-absent-reason') &&
                body.includes('"valueCode": "error"')
              );
            }
            return body.includes('"resourceType": "Patient"');
          } catch (_) {
            return false;
          }
        },
      });
    } else {
      const isCriticalOutcome =
        resPatient.status === 422 &&
        resPatient.headers['Content-Type'] &&
        resPatient.headers['Content-Type'].includes('application/fhir+json') &&
        resPatient.body.includes('"resourceType": "OperationOutcome"');
      operationOutcomeConformityRate.add(isCriticalOutcome);
      check(resPatient, {
        'Patient corrupto: Status 422 con OperationOutcome': () => isCriticalOutcome,
      });
    }
  });

  sleep(0.05);
}
