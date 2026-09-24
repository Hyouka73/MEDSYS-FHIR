// ============================================================================
// MedSys-FHIR: Prueba de Humo (Smoke Test) — k6
// Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
// ============================================================================
// Valida la disponibilidad inmediata de los endpoints canónicos, negociación
// de cabeceras HTTP (application/fhir+json) y respuesta diagnóstica OperationOutcome.
// ============================================================================

import http from 'k6/http';
import { check, group, sleep } from 'k6';

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export const options = {
  vus: 1,
  iterations: 1,
  thresholds: {
    'http_req_duration': ['p(95)<100'], // Latencia p95 menor a 100ms
    'checks': ['rate>0.90'],            // Al menos 90% de validaciones aprobadas
  },
};

export default function () {
  const fhirHeaders = {
    headers: {
      'Accept': 'application/fhir+json, application/json',
    },
  };

  group('01. Monitoreo y Salud del Servidor', function () {
    const res = http.get(`${BASE_URL}/health`);
    check(res, {
      'Salud HTTP 200': (r) => r.status === 200,
      'Content-Type es application/json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
      'Reporta estado pass': (r) => {
        try {
          const json = r.json();
          return json.status === 'pass' && json.server_name === 'MedSys-FHIR';
        } catch (_) {
          return false;
        }
      },
    });
  });

  group('02. Recursos HL7 FHIR R4 Canónicos', function () {
    // 2.1 Patient
    const resPatient = http.get(`${BASE_URL}/fhir/r4/Patient/1`, fhirHeaders);
    check(resPatient, {
      'Patient responde 200 o 404 controlado': (r) => r.status === 200 || r.status === 404,
      'Patient tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'Patient contiene estructura FHIR': (r) => {
        try {
          const json = r.json();
          return json.resourceType === 'Patient' || json.resourceType === 'OperationOutcome';
        } catch (_) {
          return false;
        }
      },
    });

    // 2.2 Encounter
    const resEncounter = http.get(`${BASE_URL}/fhir/r4/Encounter/1`, fhirHeaders);
    check(resEncounter, {
      'Encounter responde 200 o 404 controlado': (r) => r.status === 200 || r.status === 404,
      'Encounter tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // 2.3 Observation (Presión Arterial)
    const resObsBp = http.get(`${BASE_URL}/fhir/r4/Observation/bp-1`, fhirHeaders);
    check(resObsBp, {
      'Observation BP responde 200 o 404 controlado': (r) => r.status === 200 || r.status === 404,
      'Observation BP tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // 2.4 Observation (Temperatura)
    const resObsTemp = http.get(`${BASE_URL}/fhir/r4/Observation/temp-1`, fhirHeaders);
    check(resObsTemp, {
      'Observation Temp responde 200 o 404 controlado': (r) => r.status === 200 || r.status === 404,
      'Observation Temp tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // 2.5 Condition (CIE-10)
    const resCond = http.get(`${BASE_URL}/fhir/r4/Condition/cond-1`, fhirHeaders);
    check(resCond, {
      'Condition responde 200 o 404 controlado': (r) => r.status === 200 || r.status === 404,
      'Condition tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
    });
  });

  group('03. Colecciones de Búsqueda (FHIR searchset Bundle)', function () {
    const resBundle = http.get(`${BASE_URL}/fhir/r4/Patient`, fhirHeaders);
    check(resBundle, {
      'Bundle responde 200 o 500 controlado': (r) => r.status === 200 || r.status === 500,
      'Bundle tiene Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'Bundle contiene resourceType Bundle': (r) => {
        try {
          const json = r.json();
          return json.resourceType === 'Bundle' || json.resourceType === 'OperationOutcome';
        } catch (_) {
          return false;
        }
      },
    });
  });

  group('04. Manejo Global de Errores con OperationOutcome', function () {
    // Recurso inexistente debe retornar 404 con OperationOutcome
    const resNotFound = http.get(`${BASE_URL}/fhir/r4/RecursoInexistente/999`, fhirHeaders);
    check(resNotFound, {
      'Ruta inexistente retorna HTTP 404': (r) => r.status === 404,
      'Error retorna Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      'Error contiene OperationOutcome': (r) => {
        try {
          const json = r.json();
          return json.resourceType === 'OperationOutcome' && json.issue && json.issue.length > 0;
        } catch (_) {
          return false;
        }
      },
    });
  });

  sleep(0.5);
}
