// ============================================================================
// MedSys-FHIR: Prueba de Carga y Concurrencia (Load Test) — k6
// Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
// ============================================================================
// Evalúa la latencia y rendimiento del middleware de interoperabilidad en Rust
// bajo concurrencia simétrica de usuarios clínicos concurrentes (10 - 25 VUs).
// ============================================================================

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Counter, Rate, Trend } from 'k6/metrics';

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

// Métricas personalizadas
export const fhirLatencyTrend = new Trend('fhir_read_duration_ms');
export const bundleLatencyTrend = new Trend('fhir_bundle_duration_ms');
export const successRate = new Rate('fhir_success_rate');
export const transactionCounter = new Counter('fhir_total_transactions');

export const options = {
  stages: [
    { duration: '5s', target: 5 },   // Ramp-up inicial a 5 VUs
    { duration: '15s', target: 20 }, // Carga sostenida a 20 VUs
    { duration: '5s', target: 0 },   // Ramp-down a 0
  ],
  thresholds: {
    // Requisitos de tesis: Latencia p95 < 50ms para lecturas del middleware
    'http_req_duration': ['p(95)<50', 'p(99)<100'],
    'fhir_read_duration_ms': ['p(95)<50'],
    'fhir_bundle_duration_ms': ['p(95)<150'],
    'fhir_success_rate': ['rate>0.95'],
  },
};

const fhirHeaders = {
  headers: {
    'Accept': 'application/fhir+json, application/json',
    'User-Agent': 'MedSys-FHIR-k6-LoadTester/1.0',
  },
};

export default function () {
  // Simular IDs de pacientes sintéticos (1 al 10)
  const patientId = Math.floor(Math.random() * 5) + 1;

  group('Flujo Clínico Concurrente', function () {
    // 1. Consulta de Paciente
    const t0 = new Date();
    const resPatient = http.get(`${BASE_URL}/fhir/r4/Patient/${patientId}`, fhirHeaders);
    const durPatient = new Date() - t0;
    fhirLatencyTrend.add(durPatient);
    transactionCounter.add(1);

    const isOkPatient = resPatient.status === 200 || resPatient.status === 404;
    successRate.add(isOkPatient);

    check(resPatient, {
      'Patient: Status 200/404': (r) => r.status === 200 || r.status === 404,
      'Patient: Content-Type FHIR': (r) =>
        r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
    });

    // 2. Consulta de Encuentros del Paciente
    const resEnc = http.get(`${BASE_URL}/fhir/r4/Encounter/${patientId}`, fhirHeaders);
    transactionCounter.add(1);
    check(resEnc, {
      'Encounter: Status 200/404': (r) => r.status === 200 || r.status === 404,
    });

    // 3. Consulta de Signos Vitales (Panel Presión Arterial)
    const resObs = http.get(`${BASE_URL}/fhir/r4/Observation/bp-${patientId}`, fhirHeaders);
    transactionCounter.add(1);
    check(resObs, {
      'Observation BP: Status 200/404': (r) => r.status === 200 || r.status === 404,
    });

    // 4. Búsqueda masiva en Bundle (Cada ~5 iteraciones)
    if (Math.random() < 0.25) {
      const tb0 = new Date();
      const resBundle = http.get(`${BASE_URL}/fhir/r4/Patient`, fhirHeaders);
      const durBundle = new Date() - tb0;
      bundleLatencyTrend.add(durBundle);
      transactionCounter.add(1);

      check(resBundle, {
        'Bundle: Content-Type FHIR': (r) =>
          r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/fhir+json'),
      });
    }
  });

  // Pausa de cortesía entre transacciones clínicas (100ms - 300ms)
  sleep(0.1 + Math.random() * 0.2);
}
