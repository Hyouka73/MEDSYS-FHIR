// ============================================================================
// MedSys-FHIR: Prueba de Resiliencia y OperationOutcome (Stress Errors) — k6
// Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
// ============================================================================
// Inyecta fallos y peticiones anómalas concurrentes para certificar que el 100%
// de las excepciones del middleware se traduzcan invariablemente a OperationOutcome
// bajo Content-Type: application/fhir+json; charset=utf-8.
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
  group('Inyección de Fallos Controlados', function () {
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
  });

  sleep(0.05);
}
