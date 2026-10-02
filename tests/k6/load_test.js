// ============================================================================
// MedSys-FHIR: Prueba de Carga Nominal y Concurrencia (Load Test) — k6
// Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
// ============================================================================
// Evalúa la latencia y rendimiento del middleware de interoperabilidad en Rust
// bajo una meseta sostenida de 50 usuarios virtuales concurrentes (VUs).
//
// Separación estricta de escenarios (AUD-003 / AUD-008 / CON-003):
// - Consulta EXCLUSIVAMENTE identificadores existentes precargados en la base de datos.
// - Rangos de datos sintéticos reproducibles (scripts/generate_data.py):
//     * Patient:    IDs 1 al 1,000
//     * Encounter:  IDs 1 al 2,500
//     * Observation: IDs 1 al 2,500 (desacoplado en -bp, -temp, -hr)
//     * Condition:  IDs 1 al 3,000
// - Aserciones obligatorias: HTTP 200 OK y Content-Type con application/fhir+json.
// - Aislamiento metodológico: Se segregan métricas de la meseta sostenida de 5 minutos
//   respecto al warm-up y cool-down para certificar p95 <= 200ms y éxito >= 99.5%.
// ============================================================================

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Counter, Rate, Trend } from 'k6/metrics';
import { scenario } from 'k6/execution';

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

// ============================================================================
// Métricas Personalizadas Globales y Aisladas para la Meseta (Plateau)
// ============================================================================
export const fhirLatencyTrend = new Trend('fhir_read_duration_ms');
export const successRate = new Rate('fhir_success_rate');
export const transactionCounter = new Counter('fhir_total_transactions');

// Métricas aisladas para la fase de meseta sostenida de 5 minutos
export const plateauLatencyTrend = new Trend('plateau_fhir_latency_ms');
export const plateauSuccessRate = new Rate('plateau_fhir_success_rate');

// ============================================================================
// Configuración de Escenarios y Umbrales de la Hipótesis
// ============================================================================
export const options = {
  scenarios: {
    // Fase 1: Rampa de calentamiento (Warm-up) -> 1m 30s
    warmup: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '30s', target: 5 },   // 30s de calentamiento inicial a 5 VUs
        { duration: '1m', target: 50 },   // 1m de rampa progresiva hasta 50 VUs
      ],
      gracefulRampDown: '0s',
      tags: { phase: 'warmup' },
    },

    // Fase 2: Meseta sostenida bajo evaluación empírica -> 5m a 50 VUs continuos
    plateau: {
      executor: 'constant-vus',
      vus: 50,
      duration: '5m',
      startTime: '1m30s',
      tags: { phase: 'plateau' },
    },

    // Fase 3: Rampa de enfriamiento (Cool-down) -> 30s
    cooldown: {
      executor: 'ramping-vus',
      startVUs: 50,
      stages: [
        { duration: '30s', target: 0 },   // 30s de rampa de descenso hacia 0 VUs
      ],
      startTime: '6m30s',
      tags: { phase: 'cooldown' },
    },
  },

  thresholds: {
    // Requisitos de la hipótesis de investigación (UNACH 2026):
    // 1. Tasa de éxito HTTP >= 99.5% (tasa de fallo http_req_failed < 0.5%)
    'http_req_failed': ['rate<0.005'],
    'fhir_success_rate': ['rate>=0.995'],
    'checks': ['rate>=0.995'],

    // 2. Latencia p95 <= 200ms para lecturas del middleware en Rust
    'http_req_duration': ['p(95)<=200'],
    'fhir_read_duration_ms': ['p(95)<=200'],

    // 3. Aislamiento estricto de métricas en la meseta sostenida (5 minutos @ 50 VUs):
    'http_req_failed{phase:plateau}': ['rate<0.005'],
    'http_req_duration{phase:plateau}': ['p(95)<=200'],
    'plateau_fhir_latency_ms': ['p(95)<=200'],
    'plateau_fhir_success_rate': ['rate>=0.995'],
  },
};

const fhirHeaders = {
  headers: {
    'Accept': 'application/fhir+json, application/json',
    'User-Agent': 'MedSys-FHIR-k6-LoadTester/2.0',
  },
};

/**
 * Genera un número entero aleatorio uniforme dentro de un rango inclusivo [min, max].
 * Garantiza que cada petición consulte un registro estrictamente precargado.
 */
function randomId(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

const OBSERVATION_SUBTYPES = ['bp', 'temp', 'hr'];

export default function () {
  const isPlateau = scenario.name === 'plateau';

  // Selección de IDs válidos dentro de los límites reales de la BD precargada
  const patientId = randomId(1, 1000);
  const encounterId = randomId(1, 2500);
  const obsId = randomId(1, 2500);
  const obsType = OBSERVATION_SUBTYPES[Math.floor(Math.random() * OBSERVATION_SUBTYPES.length)];
  const conditionId = randomId(1, 3000);

  group('Flujo Clínico Nominal Concurrente', function () {
    // ------------------------------------------------------------------------
    // 1. Consulta de Recurso Patient (1 a 1,000)
    // ------------------------------------------------------------------------
    const t0 = new Date();
    const resPatient = http.get(`${BASE_URL}/fhir/r4/Patient/${patientId}`, fhirHeaders);
    const durPatient = new Date() - t0;

    const isPatientOk = resPatient.status === 200;
    fhirLatencyTrend.add(durPatient);
    successRate.add(isPatientOk);
    transactionCounter.add(1);

    if (isPlateau) {
      plateauLatencyTrend.add(durPatient);
      plateauSuccessRate.add(isPatientOk);
    }

    check(resPatient, {
      'Patient: Status 200 OK': (r) => r.status === 200,
      'Patient: Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] !== undefined &&
        r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // ------------------------------------------------------------------------
    // 2. Consulta de Recurso Encounter (1 a 2,500)
    // ------------------------------------------------------------------------
    const t1 = new Date();
    const resEnc = http.get(`${BASE_URL}/fhir/r4/Encounter/${encounterId}`, fhirHeaders);
    const durEnc = new Date() - t1;

    const isEncOk = resEnc.status === 200;
    fhirLatencyTrend.add(durEnc);
    successRate.add(isEncOk);
    transactionCounter.add(1);

    if (isPlateau) {
      plateauLatencyTrend.add(durEnc);
      plateauSuccessRate.add(isEncOk);
    }

    check(resEnc, {
      'Encounter: Status 200 OK': (r) => r.status === 200,
      'Encounter: Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] !== undefined &&
        r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // ------------------------------------------------------------------------
    // 3. Consulta de Recurso Observation desacoplado: bp, temp, hr (1 a 2,500)
    // ------------------------------------------------------------------------
    const t2 = new Date();
    const resObs = http.get(`${BASE_URL}/fhir/r4/Observation/${obsType}-${obsId}`, fhirHeaders);
    const durObs = new Date() - t2;

    const isObsOk = resObs.status === 200;
    fhirLatencyTrend.add(durObs);
    successRate.add(isObsOk);
    transactionCounter.add(1);

    if (isPlateau) {
      plateauLatencyTrend.add(durObs);
      plateauSuccessRate.add(isObsOk);
    }

    check(resObs, {
      'Observation: Status 200 OK': (r) => r.status === 200,
      'Observation: Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] !== undefined &&
        r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // ------------------------------------------------------------------------
    // 4. Consulta de Recurso Condition (1 a 3,000)
    // ------------------------------------------------------------------------
    const t3 = new Date();
    const resCond = http.get(`${BASE_URL}/fhir/r4/Condition/${conditionId}`, fhirHeaders);
    const durCond = new Date() - t3;

    const isCondOk = resCond.status === 200;
    fhirLatencyTrend.add(durCond);
    successRate.add(isCondOk);
    transactionCounter.add(1);

    if (isPlateau) {
      plateauLatencyTrend.add(durCond);
      plateauSuccessRate.add(isCondOk);
    }

    check(resCond, {
      'Condition: Status 200 OK': (r) => r.status === 200,
      'Condition: Content-Type application/fhir+json': (r) =>
        r.headers['Content-Type'] !== undefined &&
        r.headers['Content-Type'].includes('application/fhir+json'),
    });
  });

  // Pausa clínica representativa entre interacciones (100ms - 300ms)
  sleep(0.1 + Math.random() * 0.2);
}
