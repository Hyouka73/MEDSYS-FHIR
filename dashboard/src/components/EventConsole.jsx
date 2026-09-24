import React, { useState, useEffect } from 'react';
import { Terminal, Play, AlertCircle, Clock, Trash2, ShieldAlert } from 'lucide-react';
import { api, subscribeToEvents } from '../services/api';
import JsonSyntaxHighlighter from './JsonSyntaxHighlighter';

export default function EventConsole() {
  const [events, setEvents] = useState([]);
  const [selectedEvent, setSelectedEvent] = useState(null);
  const [testing, setTesting] = useState(false);

  useEffect(() => {
    const unsubscribe = subscribeToEvents((newEvent) => {
      setEvents((prev) => [newEvent, ...prev.slice(0, 49)]); // Guardar los últimos 50 eventos
      setSelectedEvent(newEvent);
    });
    return unsubscribe;
  }, []);

  const handleSimulate = async (action) => {
    setTesting(true);
    try {
      if (action === '404') {
        await api.simulate404NotFound();
      } else if (action === 'fallback') {
        await api.simulateInvalidRoute();
      } else if (action === '400') {
        await api.simulate400BadRequest();
      } else if (action === '200') {
        await api.getFhirResource('Patient', '1');
      }
    } finally {
      setTesting(false);
    }
  };

  const getStatusBadge = (status) => {
    if (status === 200) return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-emerald-950 text-emerald-300 border border-emerald-800">200 OK</span>;
    if (status === 404) return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-amber-950 text-amber-300 border border-amber-800">404 Not Found</span>;
    if (status === 400) return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-orange-950 text-orange-300 border border-orange-800">400 Bad Request</span>;
    if (status === 422) return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-purple-950 text-purple-300 border border-purple-800">422 Unprocessable</span>;
    if (status >= 500) return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-rose-950 text-rose-300 border border-rose-800">500 Server Error</span>;
    return <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-slate-800 text-slate-300 border border-slate-700">{status || 'OFFLINE'}</span>;
  };

  return (
    <div className="space-y-6">
      {/* Barra de Simulación Interactiva de Casos de Prueba */}
      <div className="rounded-xl border border-slate-800 bg-slate-900/80 p-4 backdrop-blur">
        <div className="flex items-center justify-between flex-wrap gap-3 mb-3">
          <div className="flex items-center gap-2">
            <ShieldAlert className="w-4 h-4 text-cyan-400" />
            <h3 className="text-xs font-bold uppercase tracking-wider text-slate-200">
              Generador de Pruebas Clínicas & Excepciones FHIR R4
            </h3>
          </div>
          <span className="text-[11px] text-slate-400">
            Valida la regla: Toda excepción emite invariablemente un <span className="font-mono text-cyan-300">OperationOutcome</span>
          </span>
        </div>

        <div className="flex items-center gap-2 flex-wrap">
          <button
            onClick={() => handleSimulate('200')}
            disabled={testing}
            className="px-3 py-1.5 rounded-lg bg-emerald-900/40 hover:bg-emerald-800/60 border border-emerald-700/60 text-emerald-200 text-xs font-medium transition flex items-center gap-1.5"
          >
            <Play className="w-3 h-3 text-emerald-400" />
            <span>Petición Exitosa (200 OK)</span>
          </button>

          <button
            onClick={() => handleSimulate('404')}
            disabled={testing}
            className="px-3 py-1.5 rounded-lg bg-amber-900/40 hover:bg-amber-800/60 border border-amber-700/60 text-amber-200 text-xs font-medium transition flex items-center gap-1.5"
          >
            <Play className="w-3 h-3 text-amber-400" />
            <span>Simular Recurso No Encontrado (404 not-found)</span>
          </button>

          <button
            onClick={() => handleSimulate('fallback')}
            disabled={testing}
            className="px-3 py-1.5 rounded-lg bg-indigo-900/40 hover:bg-indigo-800/60 border border-indigo-700/60 text-indigo-200 text-xs font-medium transition flex items-center gap-1.5"
          >
            <Play className="w-3 h-3 text-indigo-400" />
            <span>Simular Ruta Inexistente (404 Fallback Axum)</span>
          </button>

          <button
            onClick={() => handleSimulate('400')}
            disabled={testing}
            className="px-3 py-1.5 rounded-lg bg-orange-900/40 hover:bg-orange-800/60 border border-orange-700/60 text-orange-200 text-xs font-medium transition flex items-center gap-1.5"
          >
            <Play className="w-3 h-3 text-orange-400" />
            <span>Simular Parámetro Inválido (400 value)</span>
          </button>

          <button
            onClick={() => { setEvents([]); setSelectedEvent(null); }}
            className="ml-auto px-2.5 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white text-xs transition flex items-center gap-1"
            title="Limpiar consola"
          >
            <Trash2 className="w-3.5 h-3.5" />
            <span>Limpiar</span>
          </button>
        </div>
      </div>

      {/* Grid de 2 columnas: Historial de Peticiones y Visor de OperationOutcome */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        {/* Columna Izquierda: Registro de Peticiones */}
        <div className="lg:col-span-6 rounded-xl border border-slate-800 bg-slate-900/80 p-4 shadow-xl flex flex-col h-[520px]">
          <div className="flex items-center justify-between border-b border-slate-800 pb-3 mb-3">
            <div className="flex items-center gap-2">
              <Terminal className="w-4 h-4 text-cyan-400" />
              <h4 className="text-xs font-bold uppercase tracking-wider text-slate-300">
                Historial de Peticiones HTTP ({events.length})
              </h4>
            </div>
            <span className="text-[10px] font-mono text-slate-400">Stream en tiempo real</span>
          </div>

          <div className="flex-1 overflow-y-auto space-y-2 pr-1">
            {events.length === 0 ? (
              <div className="text-center py-16 text-slate-500 text-xs">
                <Clock className="w-6 h-6 mx-auto mb-2 text-slate-600" />
                <p>No se han registrado peticiones aún.</p>
                <p className="text-[11px] text-slate-600 mt-1">Usa los botones superiores para disparar eventos o navega en el visor.</p>
              </div>
            ) : (
              events.map((evt) => {
                const isSelected = selectedEvent?.id === evt.id;
                const isOutcome = evt.response?.resourceType === 'OperationOutcome';
                return (
                  <button
                    key={evt.id}
                    onClick={() => setSelectedEvent(evt)}
                    className={`w-full text-left p-3 rounded-lg border transition flex items-center justify-between gap-3 ${
                      isSelected
                        ? 'border-cyan-500/80 bg-cyan-950/40 shadow'
                        : 'border-slate-800/80 bg-slate-950/60 hover:border-slate-700 hover:bg-slate-900/60'
                    }`}
                  >
                    <div className="flex items-center gap-2.5 min-w-0">
                      <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-slate-800 text-slate-300 font-bold shrink-0">
                        {evt.method}
                      </span>
                      <span className="text-xs font-mono text-slate-200 truncate" title={evt.url}>
                        {evt.url}
                      </span>
                    </div>

                    <div className="flex items-center gap-2 shrink-0">
                      {isOutcome && (
                        <span className="text-[9px] font-mono px-1.5 py-0.5 rounded bg-rose-950 border border-rose-800 text-rose-300">
                          OperationOutcome
                        </span>
                      )}
                      <span className="text-[10px] font-mono text-slate-400">{evt.latency}ms</span>
                      {getStatusBadge(evt.status)}
                    </div>
                  </button>
                );
              })
            )}
          </div>
        </div>

        {/* Columna Derecha: Inspector de Detalle / OperationOutcome */}
        <div className="lg:col-span-6 h-[520px]">
          {selectedEvent ? (
            <div className="flex flex-col h-full space-y-3">
              {/* Tarjeta de diagnóstico si es un OperationOutcome */}
              {selectedEvent.response?.resourceType === 'OperationOutcome' && (
                <div className="p-3.5 rounded-xl border border-rose-800/60 bg-rose-950/20 text-xs">
                  <div className="flex items-center gap-2 font-bold text-rose-300 mb-1.5">
                    <AlertCircle className="w-4 h-4 text-rose-400" />
                    <span>Diagnóstico Clínico Estándar FHIR R4:</span>
                  </div>
                  <p className="text-slate-200 font-mono text-[11px] leading-relaxed">
                    {selectedEvent.response.issue?.[0]?.diagnostics || "Excepción clínica"}
                  </p>
                  <div className="mt-2 flex items-center gap-2 text-[10px] font-mono text-slate-400">
                    <span>Severity: <strong className="text-rose-400">{selectedEvent.response.issue?.[0]?.severity}</strong></span>
                    <span>•</span>
                    <span>Code: <strong className="text-amber-400">{selectedEvent.response.issue?.[0]?.code}</strong></span>
                    <span>•</span>
                    <span>Content-Type: <strong className="text-emerald-400">application/fhir+json</strong></span>
                  </div>
                </div>
              )}

              <div className="flex-1 min-h-0">
                <JsonSyntaxHighlighter
                  data={selectedEvent.response}
                  title={`Respuesta HTTP ${selectedEvent.status} • ${selectedEvent.url}`}
                />
              </div>
            </div>
          ) : (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-12 text-center flex flex-col items-center justify-center h-full text-slate-500 text-xs">
              <Terminal className="w-8 h-8 mx-auto mb-2 text-slate-600" />
              <span>Selecciona una petición del historial para inspeccionar su respuesta JSON y diagnóstico.</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
