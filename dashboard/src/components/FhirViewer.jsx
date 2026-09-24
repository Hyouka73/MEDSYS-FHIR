import React, { useState, useEffect } from 'react';
import { User, Calendar, Activity, Stethoscope, Search, Filter } from 'lucide-react';
import { api } from '../services/api';
import JsonSyntaxHighlighter from './JsonSyntaxHighlighter';

const RESOURCES = [
  { type: 'Patient', name: 'Patient (Paciente)', icon: User, defaultId: '1', desc: 'Identificador oficial CURP, nombres desglosados y contacto' },
  { type: 'Encounter', name: 'Encounter (Consulta)', icon: Calendar, defaultId: '1', desc: 'Clase ambulatoria AMB, médico tratante con Cédula SEP' },
  { type: 'Observation', name: 'Observation (Signos Vitales)', icon: Activity, defaultId: 'bp-1', desc: 'Desacoplamiento: Presión (LOINC 85354-9) y Temp (LOINC 8310-5)' },
  { type: 'Condition', name: 'Condition (Diagnóstico)', icon: Stethoscope, defaultId: 'cond-1', desc: 'Afección clínica codificada bajo catálogo internacional CIE-10' },
];

export default function FhirViewer() {
  const [resourceType, setResourceType] = useState('Patient');
  const [selectedId, setSelectedId] = useState('1');
  const [viewMode, setViewMode] = useState('single'); // 'single' o 'bundle'
  const [resourceData, setResourceData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [customId, setCustomId] = useState('');

  const handleSelectResourceType = (type) => {
    setResourceType(type);
    const defId = RESOURCES.find((r) => r.type === type)?.defaultId || '1';
    setSelectedId(defId);
    setViewMode('single');
  };

  useEffect(() => {
    let active = true;
    
    // Iniciar carga asíncrona para sincronización de datos
    Promise.resolve().then(() => {
      if (active) setLoading(true);
    });

    api.getFhirResource(resourceType, viewMode === 'single' ? selectedId : null)
      .then((res) => {
        if (active) {
          setResourceData(res.data);
          setLoading(false);
        }
      })
      .catch((err) => {
        if (active) {
          console.error("Error al obtener recurso FHIR:", err);
          setLoading(false);
        }
      });

    return () => {
      active = false;
    };
  }, [resourceType, selectedId, viewMode]);

  const handleSelectId = (id) => {
    setSelectedId(id);
    setViewMode('single');
  };

  const handleCustomSearch = (e) => {
    e.preventDefault();
    if (customId.trim()) {
      handleSelectId(customId.trim());
    }
  };

  // IDs preestablecidos de prueba por tipo de recurso
  const getPresetIds = () => {
    switch (resourceType) {
      case 'Patient':
        return ['1', '2'];
      case 'Encounter':
        return ['1', '2'];
      case 'Observation':
        return ['bp-1', 'temp-1', 'bp-2', 'temp-2'];
      case 'Condition':
        return ['cond-1', 'cond-2'];
      default:
        return ['1'];
    }
  };

  return (
    <div className="space-y-6">
      {/* Selector superior de Recursos FHIR */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        {RESOURCES.map((res) => {
          const Icon = res.icon;
          const isSelected = resourceType === res.type;
          return (
            <button
              key={res.type}
              onClick={() => handleSelectResourceType(res.type)}
              className={`p-3.5 rounded-xl border text-left transition flex flex-col justify-between ${
                isSelected
                  ? 'border-cyan-500/80 bg-cyan-950/30 shadow-lg shadow-cyan-950/50'
                  : 'border-slate-800 bg-slate-900/60 hover:border-slate-700 hover:bg-slate-900'
              }`}
            >
              <div className="flex items-center justify-between mb-2">
                <span className={`text-xs font-mono font-bold ${isSelected ? 'text-cyan-400' : 'text-slate-300'}`}>
                  /fhir/r4/{res.type}
                </span>
                <Icon className={`w-4 h-4 ${isSelected ? 'text-cyan-400' : 'text-slate-400'}`} />
              </div>
              <p className="text-[11px] text-slate-400 line-clamp-2">{res.desc}</p>
            </button>
          );
        })}
      </div>

      {/* Barra de control: Selección de instancia / Bundle / Búsqueda */}
      <div className="rounded-xl border border-slate-800 bg-slate-900/80 p-4 backdrop-blur flex flex-wrap items-center justify-between gap-4">
        {/* Selector de modo y botones de instancia */}
        <div className="flex items-center gap-2 flex-wrap">
          <div className="flex rounded-lg border border-slate-700/80 p-0.5 bg-slate-950">
            <button
              onClick={() => {
                setViewMode('single');
                fetchResource(resourceType, selectedId, 'single');
              }}
              className={`px-3 py-1 rounded-md text-xs font-medium transition ${
                viewMode === 'single'
                  ? 'bg-cyan-500 text-slate-950 font-semibold shadow'
                  : 'text-slate-400 hover:text-white'
              }`}
            >
              Instancia Única
            </button>
            <button
              onClick={() => {
                setViewMode('bundle');
                fetchResource(resourceType, null, 'bundle');
              }}
              className={`px-3 py-1 rounded-md text-xs font-medium transition ${
                viewMode === 'bundle'
                  ? 'bg-cyan-500 text-slate-950 font-semibold shadow'
                  : 'text-slate-400 hover:text-white'
              }`}
            >
              Colección (Bundle searchset)
            </button>
          </div>

          {viewMode === 'single' && (
            <div className="flex items-center gap-1.5 ml-2">
              <span className="text-xs text-slate-400 font-mono">Ejemplos:</span>
              {getPresetIds().map((id) => (
                <button
                  key={id}
                  onClick={() => handleSelectId(id)}
                  className={`px-2.5 py-1 rounded text-xs font-mono transition ${
                    selectedId === id
                      ? 'bg-cyan-950 border border-cyan-700 text-cyan-300 font-bold'
                      : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
                  }`}
                >
                  {id}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Buscador por ID personalizado */}
        <form onSubmit={handleCustomSearch} className="flex items-center gap-2 w-full sm:w-auto">
          <div className="relative flex-1 sm:w-48">
            <input
              type="text"
              value={customId}
              onChange={(e) => setCustomId(e.target.value)}
              placeholder="Buscar por ID..."
              className="w-full rounded-lg bg-slate-950 border border-slate-700 px-3 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500 font-mono"
            />
            <Search className="w-3.5 h-3.5 text-slate-400 absolute right-2.5 top-2.5" />
          </div>
          <button
            type="submit"
            className="px-3 py-1.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-slate-950 font-semibold text-xs transition"
          >
            Consultar
          </button>
        </form>
      </div>

      {/* Contenedor del visor sintáctico y ficha clínica */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Panel lateral: Metadatos Clínicos del Recurso */}
        <div className="lg:col-span-1 space-y-4">
          <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-4">
            <h3 className="text-xs font-bold uppercase tracking-wider text-slate-400 mb-3 flex items-center gap-1.5">
              <Filter className="w-3.5 h-3.5 text-cyan-400" />
              <span>Contrato de Interoperabilidad</span>
            </h3>

            <div className="space-y-3 text-xs">
              <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                <span className="text-[11px] text-slate-400 block mb-0.5">Recurso Oficial:</span>
                <span className="font-mono text-cyan-300 font-semibold">{resourceType} (HL7 FHIR R4)</span>
              </div>

              <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                <span className="text-[11px] text-slate-400 block mb-0.5">URI Canónica:</span>
                <span className="font-mono text-slate-200 break-all">
                  {viewMode === 'single' ? `/fhir/r4/${resourceType}/${selectedId}` : `/fhir/r4/${resourceType}`}
                </span>
              </div>

              <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                <span className="text-[11px] text-slate-400 block mb-0.5">Cabecera Content-Type:</span>
                <span className="font-mono text-emerald-400 font-semibold">application/fhir+json; charset=utf-8</span>
              </div>

              {resourceType === 'Patient' && (
                <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                  <span className="text-[11px] text-slate-400 block mb-0.5">Identificador Nacional (México):</span>
                  <span className="font-mono text-amber-300">urn:oid:2.16.840.1.113883.4.629 (CURP)</span>
                </div>
              )}

              {resourceType === 'Encounter' && (
                <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                  <span className="text-[11px] text-slate-400 block mb-0.5">Clase y Cédula SEP:</span>
                  <span className="font-mono text-amber-300">AMB • http://cedulaprofesional.sep.gob.mx</span>
                </div>
              )}

              {resourceType === 'Observation' && (
                <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                  <span className="text-[11px] text-slate-400 block mb-0.5">Codificación LOINC Oficial:</span>
                  <span className="font-mono text-amber-300">PA: 85354-9 (8480-6 / 8462-4) • Temp: 8310-5 (Cel)</span>
                </div>
              )}

              {resourceType === 'Condition' && (
                <div className="p-2.5 rounded-lg bg-slate-950/80 border border-slate-800">
                  <span className="text-[11px] text-slate-400 block mb-0.5">Catálogo Internacional:</span>
                  <span className="font-mono text-amber-300">CIE-10 (http://hl7.org/fhir/sid/icd-10)</span>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Panel central/derecho: Resaltador Sintáctico JSON */}
        <div className="lg:col-span-2">
          {loading ? (
            <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-12 text-center flex flex-col items-center justify-center min-h-[400px]">
              <div className="w-8 h-8 rounded-full border-2 border-cyan-400 border-t-transparent animate-spin mb-3"></div>
              <span className="text-xs text-slate-400 font-mono">Transformando y serializando FHIR R4...</span>
            </div>
          ) : (
            <JsonSyntaxHighlighter
              data={resourceData}
              title={`GET /fhir/r4/${resourceType}${viewMode === 'single' ? `/${selectedId}` : ''} • application/fhir+json`}
            />
          )}
        </div>
      </div>
    </div>
  );
}
