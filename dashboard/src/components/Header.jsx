import React from 'react';
import { Activity, Database, RefreshCw, Cpu, Flame, Layers, Terminal } from 'lucide-react';

export default function Header({ activeTab, setActiveTab, health, onRefresh, loading }) {
  const isHealthy = health?.status === 'ok';

  return (
    <header className="border-b border-slate-800 bg-slate-950/80 backdrop-blur sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo y título */}
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-tr from-cyan-600 via-sky-500 to-indigo-500 p-0.5 shadow-lg shadow-cyan-500/20 flex items-center justify-center">
              <div className="w-full h-full bg-slate-950 rounded-[10px] flex items-center justify-center">
                <Activity className="w-5 h-5 text-cyan-400" />
              </div>
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h1 className="text-lg font-bold tracking-tight text-white flex items-center gap-1.5">
                  MedSys<span className="text-cyan-400">-FHIR</span>
                </h1>
                <span className="text-[10px] font-mono uppercase tracking-wider px-2 py-0.5 rounded-full bg-cyan-950/80 border border-cyan-800/60 text-cyan-300 font-semibold">
                  v0.1.0
                </span>
              </div>
              <p className="text-xs text-slate-400 hidden sm:block">
                Middleware de Interoperabilidad Clínica • NOM-004-SSA3-2012 ⇄ HL7 FHIR R4
              </p>
            </div>
          </div>

          {/* Badges de tecnología y estado en vivo */}
          <div className="hidden lg:flex items-center gap-3">
            <div className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-xs font-medium text-slate-300">
              <Cpu className="w-3.5 h-3.5 text-amber-400" />
              <span>Rust Axum 0.8</span>
            </div>
            <div className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-xs font-medium text-slate-300">
              <Flame className="w-3.5 h-3.5 text-rose-400" />
              <span>FHIR R4 (helios-fhir)</span>
            </div>
            <div className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-slate-900 border border-slate-800 text-xs font-medium text-slate-300">
              <Database className="w-3.5 h-3.5 text-sky-400" />
              <span>PostgreSQL 16</span>
            </div>

            {/* Indicador de estado */}
            <div className={`flex items-center gap-2 px-3 py-1 rounded-full border text-xs font-medium ${
              isHealthy
                ? 'bg-emerald-950/60 border-emerald-800/80 text-emerald-300'
                : 'bg-amber-950/60 border-amber-800/80 text-amber-300'
            }`}>
              <span className={`w-2 h-2 rounded-full ${isHealthy ? 'bg-emerald-400 animate-ping' : 'bg-amber-400'}`}></span>
              <span>{isHealthy ? 'Axum Online (3000)' : 'Modo Demostración / Offline'}</span>
            </div>
          </div>

          {/* Botón de refresco */}
          <div className="flex items-center gap-2">
            <button
              onClick={onRefresh}
              disabled={loading}
              className="p-2 rounded-lg bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-300 hover:text-white transition flex items-center gap-1.5 text-xs font-medium"
              title="Actualizar estado y métricas"
            >
              <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin text-cyan-400' : ''}`} />
              <span className="hidden md:inline">Actualizar</span>
            </button>
          </div>
        </div>

        {/* Barra de pestañas */}
        <div className="flex gap-2 -mb-px overflow-x-auto pb-1 sm:pb-0">
          <button
            onClick={() => setActiveTab('viewer')}
            className={`flex items-center gap-2 px-4 py-2.5 text-xs font-semibold border-b-2 transition whitespace-nowrap ${
              activeTab === 'viewer'
                ? 'border-cyan-400 text-cyan-300 bg-slate-900/40'
                : 'border-transparent text-slate-400 hover:text-slate-200 hover:border-slate-700'
            }`}
          >
            <Layers className="w-4 h-4" />
            <span>Visor Interactivo FHIR R4</span>
          </button>

          <button
            onClick={() => setActiveTab('comparator')}
            className={`flex items-center gap-2 px-4 py-2.5 text-xs font-semibold border-b-2 transition whitespace-nowrap ${
              activeTab === 'comparator'
                ? 'border-cyan-400 text-cyan-300 bg-slate-900/40'
                : 'border-transparent text-slate-400 hover:text-slate-200 hover:border-slate-700'
            }`}
          >
            <Database className="w-4 h-4" />
            <span>Comparador NOM-004 ⇄ FHIR R4</span>
          </button>

          <button
            onClick={() => setActiveTab('console')}
            className={`flex items-center gap-2 px-4 py-2.5 text-xs font-semibold border-b-2 transition whitespace-nowrap ${
              activeTab === 'console'
                ? 'border-cyan-400 text-cyan-300 bg-slate-900/40'
                : 'border-transparent text-slate-400 hover:text-slate-200 hover:border-slate-700'
            }`}
          >
            <Terminal className="w-4 h-4" />
            <span>Consola de Eventos & OperationOutcome</span>
          </button>
        </div>
      </div>
    </header>
  );
}
