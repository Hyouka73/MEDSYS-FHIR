import React, { useState, useEffect } from 'react';
import Header from './components/Header';
import MetricsPanel from './components/MetricsPanel';
import FhirViewer from './components/FhirViewer';
import InteroperabilityComparator from './components/InteroperabilityComparator';
import EventConsole from './components/EventConsole';
import { api } from './services/api';
import { ShieldCheck, HeartPulse } from 'lucide-react';

export default function App() {
  const [activeTab, setActiveTab] = useState('viewer'); // 'viewer', 'comparator', 'console'
  const [health, setHealth] = useState(null);
  const [latency, setLatency] = useState(null);
  const [loading, setLoading] = useState(false);

  const loadHealth = React.useCallback(async () => {
    setLoading(true);
    try {
      const data = await api.getHealth();
      setHealth(data);
      setLatency(data.latency || 0);
    } catch (err) {
      console.error("Error cargando salud:", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    let isMounted = true;
    api.getHealth().then((data) => {
      if (isMounted) {
        setHealth(data);
        setLatency(data.latency || 0);
      }
    });

    const interval = setInterval(() => {
      loadHealth();
    }, 10000);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, [loadHealth]);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col">
      {/* Encabezado y Navegación Principal */}
      <Header
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        health={health}
        onRefresh={loadHealth}
        loading={loading}
      />

      {/* Contenedor Principal */}
      <main className="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6">
        {/* Panel Superior de Métricas en Vivo */}
        <MetricsPanel health={health} latency={latency} />

        {/* Contenido Dinámico según Pestaña Activa */}
        <div className="transition-all duration-200">
          {activeTab === 'viewer' && <FhirViewer />}
          {activeTab === 'comparator' && <InteroperabilityComparator />}
          {activeTab === 'console' && <EventConsole />}
        </div>
      </main>

      {/* Pie de Página Académico y Normativo */}
      <footer className="border-t border-slate-900 bg-slate-950/90 py-6 mt-auto text-xs text-slate-500">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row items-center justify-between gap-4">
          <div className="flex items-center gap-2">
            <HeartPulse className="w-4 h-4 text-cyan-400" />
            <span className="font-semibold text-slate-300">MedSys-FHIR</span>
            <span>•</span>
            <span>Tesis de Licenciatura UNACH 2026</span>
          </div>

          <div className="flex items-center gap-4 text-[11px] text-slate-400 flex-wrap justify-center">
            <span>Autores: Carlos E. Iglesias & Alexis A. Gálvez</span>
            <span>•</span>
            <span>NOM-004-SSA3-2012</span>
            <span>•</span>
            <span>NOM-024-SSA3-2012</span>
            <span>•</span>
            <span>HL7 FHIR R4 (4.0.1)</span>
          </div>

          <div className="flex items-center gap-1.5 text-[11px] text-emerald-400/90">
            <ShieldCheck className="w-3.5 h-3.5" />
            <span>Datos Sintéticos de Laboratorio (LFPDPPP)</span>
          </div>
        </div>
      </footer>
    </div>
  );
}
