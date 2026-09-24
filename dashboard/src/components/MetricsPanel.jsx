import React from 'react';
import { Activity, Clock, Database, ShieldCheck, CheckCircle2, AlertTriangle } from 'lucide-react';

export default function MetricsPanel({ health, latency }) {
  const isHealthy = health?.status === 'ok';
  const dbConnected = health?.database_status === 'connected';

  // Determinación de color según latencia en milisegundos
  const getLatencyBadge = (ms) => {
    if (ms === undefined || ms === null) return { text: 'N/A', color: 'text-slate-400', bg: 'bg-slate-800' };
    if (ms <= 15) return { text: `${ms} ms (Excelente)`, color: 'text-emerald-400', bg: 'bg-emerald-950/60 border-emerald-800/80' };
    if (ms <= 50) return { text: `${ms} ms (Óptima)`, color: 'text-cyan-400', bg: 'bg-cyan-950/60 border-cyan-800/80' };
    return { text: `${ms} ms (Elevada)`, color: 'text-amber-400', bg: 'bg-amber-950/60 border-amber-800/80' };
  };

  const latInfo = getLatencyBadge(latency);

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      {/* Tarjeta 1: Salud del Middleware */}
      <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-4 backdrop-blur shadow-sm hover:border-slate-700 transition">
        <div className="flex items-center justify-between">
          <span className="text-xs font-medium text-slate-400">Salud del Middleware</span>
          <div className="p-2 rounded-lg bg-cyan-950/50 border border-cyan-800/40 text-cyan-400">
            <Activity className="w-4 h-4" />
          </div>
        </div>
        <div className="mt-3 flex items-baseline gap-2">
          <span className="text-2xl font-bold tracking-tight text-white">
            {isHealthy ? 'Saludable' : (health?.status === 'offline' ? 'Offline' : 'Degradado')}
          </span>
          <span className="text-xs font-mono text-slate-400">/health</span>
        </div>
        <div className="mt-2 flex items-center gap-1.5 text-xs">
          {isHealthy ? (
            <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
          ) : (
            <AlertTriangle className="w-3.5 h-3.5 text-amber-400" />
          )}
          <span className="text-slate-300">
            Uptime: <span className="font-mono text-white">{health?.uptime_seconds || 0}s</span>
          </span>
        </div>
      </div>

      {/* Tarjeta 2: Latencia de Transformación */}
      <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-4 backdrop-blur shadow-sm hover:border-slate-700 transition">
        <div className="flex items-center justify-between">
          <span className="text-xs font-medium text-slate-400">Latencia de Respuesta</span>
          <div className="p-2 rounded-lg bg-emerald-950/50 border border-emerald-800/40 text-emerald-400">
            <Clock className="w-4 h-4" />
          </div>
        </div>
        <div className="mt-3 flex items-baseline gap-2">
          <span className="text-2xl font-bold tracking-tight font-mono text-white">
            {latency !== null && latency !== undefined ? `${latency} ms` : '—'}
          </span>
          <span className={`text-[10px] font-semibold px-2 py-0.5 rounded-full border ${latInfo.bg} ${latInfo.color}`}>
            {latInfo.text}
          </span>
        </div>
        <div className="mt-2 flex items-center gap-1.5 text-xs text-slate-400">
          <span>Runtime Tokio asíncrono sin bloqueo de hilos</span>
        </div>
      </div>

      {/* Tarjeta 3: Base de Datos Relacional */}
      <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-4 backdrop-blur shadow-sm hover:border-slate-700 transition">
        <div className="flex items-center justify-between">
          <span className="text-xs font-medium text-slate-400">Persistencia PostgreSQL 16</span>
          <div className="p-2 rounded-lg bg-sky-950/50 border border-sky-800/40 text-sky-400">
            <Database className="w-4 h-4" />
          </div>
        </div>
        <div className="mt-3 flex items-baseline gap-2">
          <span className="text-2xl font-bold tracking-tight text-white">
            {dbConnected ? 'Conectada' : 'Simulada'}
          </span>
          <span className={`w-2.5 h-2.5 rounded-full ${dbConnected ? 'bg-emerald-400 animate-pulse' : 'bg-amber-400'}`}></span>
        </div>
        <div className="mt-2 flex items-center gap-1.5 text-xs text-slate-300">
          <span className="font-mono text-slate-400">Solo lectura ($1, $2)</span>
          <span className="text-slate-600">•</span>
          <span className="text-emerald-400 font-medium">NOM-004</span>
        </div>
      </div>

      {/* Tarjeta 4: Estándar y Recursos */}
      <div className="rounded-xl border border-slate-800/80 bg-slate-900/60 p-4 backdrop-blur shadow-sm hover:border-slate-700 transition">
        <div className="flex items-center justify-between">
          <span className="text-xs font-medium text-slate-400">Conformidad Estándar</span>
          <div className="p-2 rounded-lg bg-indigo-950/50 border border-indigo-800/40 text-indigo-400">
            <ShieldCheck className="w-4 h-4" />
          </div>
        </div>
        <div className="mt-3 flex items-baseline gap-2">
          <span className="text-2xl font-bold tracking-tight text-white">
            HL7 FHIR R4
          </span>
          <span className="text-xs font-mono text-cyan-400">4.0.1</span>
        </div>
        <div className="mt-2 flex items-center gap-1.5 text-xs text-slate-300">
          <span>4 Recursos: Patient, Encounter, Obs, Cond</span>
        </div>
      </div>
    </div>
  );
}
