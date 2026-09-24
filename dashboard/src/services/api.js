// Cliente de API HTTP con medición de latencia y registro de eventos para MedSys-FHIR
import { MOCK_FULL_COMPARISONS, MOCK_PATIENTS_LEGACY } from './mockData';

// Escuchadores de eventos para la Consola en tiempo real
const eventListeners = new Set();

export function subscribeToEvents(callback) {
  eventListeners.add(callback);
  return () => eventListeners.delete(callback);
}

function notifyEvent(event) {
  eventListeners.forEach(cb => cb(event));
}

// Envuelve fetch midiendo latencia en milisegundos y registrando en la consola
async function fetchWithTelemetry(url, options = {}) {
  const startTime = performance.now();
  const timestamp = new Date().toLocaleTimeString();

  try {
    const res = await fetch(url, {
      ...options,
      headers: {
        'Accept': 'application/fhir+json, application/json',
        ...(options.headers || {})
      }
    });
    const latency = Math.round(performance.now() - startTime);
    const contentType = res.headers.get('content-type') || '';
    
    let data;
    if (contentType.includes('json')) {
      data = await res.json();
    } else {
      data = await res.text();
    }

    notifyEvent({
      id: Math.random().toString(36).substring(7),
      timestamp,
      method: options.method || 'GET',
      url,
      status: res.status,
      latency,
      ok: res.ok,
      contentType,
      response: data
    });

    return { ok: res.ok, status: res.status, latency, data };
  } catch (err) {
    const latency = Math.round(performance.now() - startTime);
    
    notifyEvent({
      id: Math.random().toString(36).substring(7),
      timestamp,
      method: options.method || 'GET',
      url,
      status: 0,
      latency,
      ok: false,
      error: err.message,
      response: {
        resourceType: "OperationOutcome",
        issue: [
          {
            severity: "error",
            code: "transient",
            diagnostics: `Fallo de comunicación HTTP con el backend Axum: ${err.message}`
          }
        ]
      }
    });

    return {
      ok: false,
      status: 0,
      latency,
      error: err.message,
      data: null
    };
  }
}

export const api = {
  // 1. Healthcheck (/health o /api/health)
  async getHealth() {
    const result = await fetchWithTelemetry('/health');
    if (result.ok && result.data) {
      return { ...result.data, latency: result.latency };
    }
    // Fallback simulado si el servidor no está corriendo
    return {
      status: "offline",
      server_version: "0.1.0",
      uptime_seconds: 0,
      database_status: "disconnected",
      fhir_version: "R4 (4.0.1)",
      latency: result.latency,
      isFallback: true
    };
  },

  // 2. Recursos FHIR R4
  async getFhirResource(resourceType, id = null) {
    const endpoint = id ? `/fhir/r4/${resourceType}/${id}` : `/fhir/r4/${resourceType}`;
    const result = await fetchWithTelemetry(endpoint);
    
    if (result.ok && result.data) {
      return { success: true, data: result.data, latency: result.latency, status: result.status };
    }

    // Si falló pero retornó OperationOutcome (ej. 404, 422)
    if (result.data && result.data.resourceType === "OperationOutcome") {
      return { success: false, data: result.data, latency: result.latency, status: result.status };
    }

    // Fallback a datos sintéticos de laboratorio si backend no responde
    if (id === "1" || id === "temp-1" || id === "bp-1" || id === "cond-1" || !id) {
      const comp = MOCK_FULL_COMPARISONS[1];
      let mockRes;
      if (resourceType === 'Patient') mockRes = comp.fhir_patient;
      else if (resourceType === 'Encounter') mockRes = comp.fhir_encounters[0];
      else if (resourceType === 'Observation') mockRes = id === 'temp-1' ? comp.fhir_observations[1] : comp.fhir_observations[0];
      else if (resourceType === 'Condition') mockRes = comp.fhir_conditions[0];

      return {
        success: true,
        data: id ? mockRes : {
          resourceType: "Bundle",
          type: "searchset",
          total: 1,
          entry: [{ resource: mockRes }]
        },
        latency: 2,
        status: 200,
        isFallback: true
      };
    }

    return { success: false, data: result.data, latency: result.latency, status: result.status };
  },

  // 3. Comparador Relacional vs FHIR (/api/legacy/patients/{id}/full)
  async getPatientFullComparison(id = 1) {
    const result = await fetchWithTelemetry(`/api/legacy/patients/${id}/full`);
    if (result.ok && result.data) {
      return { success: true, data: result.data, latency: result.latency };
    }
    // Fallback a mock si el servidor está en arranque
    return {
      success: true,
      data: MOCK_FULL_COMPARISONS[id] || MOCK_FULL_COMPARISONS[1],
      latency: 3,
      isFallback: true
    };
  },

  // 4. Lista de pacientes legados
  async getLegacyPatients() {
    const result = await fetchWithTelemetry('/api/legacy/patients');
    if (result.ok && result.data) {
      return { success: true, data: result.data };
    }
    return { success: true, data: MOCK_PATIENTS_LEGACY, isFallback: true };
  },

  // 5. Generadores de pruebas para OperationOutcome
  async simulate404NotFound() {
    return await fetchWithTelemetry('/fhir/r4/Patient/999');
  },

  async simulateInvalidRoute() {
    return await fetchWithTelemetry('/fhir/r4/RutaInexistenteParaValidarOutcome');
  },

  async simulate400BadRequest() {
    return await fetchWithTelemetry('/fhir/r4/Condition/cond-identificador-invalido');
  }
};
