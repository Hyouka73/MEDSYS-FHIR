import React, { useState, useEffect } from 'react';
import { ArrowRight, Database, FileCode, Shield, User, Calendar, Activity, Stethoscope } from 'lucide-react';
import { api } from '../services/api';
import JsonSyntaxHighlighter from './JsonSyntaxHighlighter';

export default function InteroperabilityComparator() {
  const [selectedPatientId, setSelectedPatientId] = useState(1);
  const [activeEntity, setActiveEntity] = useState('patient'); // 'patient', 'encounter', 'observation', 'condition'
  const [comparisonData, setComparisonData] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function loadData() {
      setLoading(true);
      try {
        const res = await api.getPatientFullComparison(selectedPatientId);
        setComparisonData(res.data);
      } catch (err) {
        console.error("Error al cargar comparador:", err);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, [selectedPatientId]);

  if (loading || !comparisonData) {
    return (
      <div className="rounded-xl border border-slate-800 bg-slate-900/60 p-12 text-center flex flex-col items-center justify-center min-h-[400px]">
        <div className="w-8 h-8 rounded-full border-2 border-cyan-400 border-t-transparent animate-spin mb-3"></div>
        <span className="text-xs text-slate-400 font-mono">Consultando datos relacionales y transformando a FHIR...</span>
      </div>
    );
  }

  const { paciente_legado, consultas_legadas, signos_vitales_legados, diagnosticos_legados, fhir_patient, fhir_encounters, fhir_observations, fhir_conditions } = comparisonData;

  // Selección de datos según entidad activa
  let legacyTableContent = null;
  let ruleExplanation = null;
  let fhirResourceContent = null;

  if (activeEntity === 'patient') {
    legacyTableContent = (
      <div className="space-y-2">
        <div className="text-xs font-mono font-semibold text-sky-400 mb-2">Tabla: tbl_pacientes</div>
        <table className="w-full text-left text-xs font-mono border-collapse">
          <tbody>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">id_paciente:</td><td className="py-1 text-white font-bold">{paciente_legado.id_paciente}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">curp:</td><td className="py-1 text-amber-300 font-bold">{paciente_legado.curp}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">primer_nombre:</td><td className="py-1 text-slate-200">{paciente_legado.primer_nombre}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">segundo_nombre:</td><td className="py-1 text-slate-200">{paciente_legado.segundo_nombre || 'NULL'}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">apellido_paterno:</td><td className="py-1 text-slate-200">{paciente_legado.apellido_paterno}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">apellido_materno:</td><td className="py-1 text-slate-200">{paciente_legado.apellido_materno || 'NULL'}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">fecha_nacimiento:</td><td className="py-1 text-slate-200">{paciente_legado.fecha_nacimiento}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">sexo_biologico:</td><td className="py-1 text-purple-300">{paciente_legado.sexo_biologico}</td></tr>
            <tr><td className="py-1 text-slate-400">telefono_contacto:</td><td className="py-1 text-slate-200">{paciente_legado.telefono_contacto}</td></tr>
          </tbody>
        </table>
      </div>
    );

    ruleExplanation = [
      { from: 'curp', to: 'identifier[0].value', detail: 'System OID oficial urn:oid:2.16.840.1.113883.4.629 con use="official"' },
      { from: 'primer_nombre + segundo_nombre', to: 'name[0].given', detail: 'Desglose en array de nombres de pila' },
      { from: 'apellido_paterno + apellido_materno', to: 'name[0].family', detail: 'Concatenación declarativa combine_with: apellido_materno' },
      { from: 'sexo_biologico ("M"/"F")', to: 'gender ("male"/"female")', detail: 'Mapeo terminológico declarativo vía diccionario YAML' },
    ];

    fhirResourceContent = fhir_patient;
  } else if (activeEntity === 'encounter') {
    const consulta = consultas_legadas[0] || {};
    legacyTableContent = (
      <div className="space-y-2">
        <div className="text-xs font-mono font-semibold text-sky-400 mb-2">Tabla: tbl_consultas</div>
        <table className="w-full text-left text-xs font-mono border-collapse">
          <tbody>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">id_consulta:</td><td className="py-1 text-white font-bold">{consulta.id_consulta}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">cedula_medico:</td><td className="py-1 text-amber-300 font-bold">{consulta.cedula_medico_tratante}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">nombre_medico:</td><td className="py-1 text-slate-200">{consulta.nombre_medico}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">estado_consulta:</td><td className="py-1 text-emerald-300">{consulta.estado_consulta}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">fecha_hora_inicio:</td><td className="py-1 text-slate-200">{consulta.fecha_hora_inicio}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">fecha_hora_fin:</td><td className="py-1 text-slate-200">{consulta.fecha_hora_fin}</td></tr>
            <tr><td className="py-1 text-slate-400">unidad_medica:</td><td className="py-1 text-slate-200">{consulta.unidad_medica}</td></tr>
          </tbody>
        </table>
      </div>
    );

    ruleExplanation = [
      { from: 'Constante "AMB"', to: 'class.code', detail: 'Clase ambulatoria obligatoria (http://terminology.hl7.org/CodeSystem/v3-ActCode)' },
      { from: 'cedula_medico_tratante', to: 'participant[0].individual.identifier', detail: 'System: http://cedulaprofesional.sep.gob.mx' },
      { from: 'estado_consulta ("FINALIZADA")', to: 'status ("finished")', detail: 'Diccionario normativo de estados FHIR' },
      { from: 'fecha_hora_inicio / fin', to: 'period.start / end', detail: 'Periodo estructurado en formato ISO 8601' },
    ];

    fhirResourceContent = fhir_encounters[0] || {};
  } else if (activeEntity === 'observation') {
    const signo = signos_vitales_legados[0] || {};
    legacyTableContent = (
      <div className="space-y-2">
        <div className="text-xs font-mono font-semibold text-sky-400 mb-2">Tabla: tbl_signos_vitales</div>
        <table className="w-full text-left text-xs font-mono border-collapse">
          <tbody>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">id_signo:</td><td className="py-1 text-white font-bold">{signo.id_signo}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">presion_sistolica:</td><td className="py-1 text-rose-300 font-bold">{signo.presion_sistolica} mmHg</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">presion_diastolica:</td><td className="py-1 text-rose-300 font-bold">{signo.presion_diastolica} mmHg</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">temperatura_celsius:</td><td className="py-1 text-amber-300 font-bold">{signo.temperatura_celsius} °C</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">frecuencia_cardiaca:</td><td className="py-1 text-slate-200">{signo.frecuencia_cardiaca} lpm</td></tr>
            <tr><td className="py-1 text-slate-400">fecha_registro:</td><td className="py-1 text-slate-200">{signo.fecha_registro}</td></tr>
          </tbody>
        </table>
      </div>
    );

    ruleExplanation = [
      { from: 'presion_sistolica + diastolica', to: 'Observation (bp-1)', detail: 'Panel LOINC 85354-9 con componentes sistólica (8480-6) y diastólica (8462-4) en mmHg' },
      { from: 'temperatura_celsius', to: 'Observation (temp-1)', detail: 'Desacoplamiento canónico: LOINC 8310-5 en unidad UCUM Cel' },
      { from: 'Constante "vital-signs"', to: 'category[0].coding[0].code', detail: 'Categoría oficial HL7 observation-category' },
    ];

    fhirResourceContent = fhir_observations[0] || {};
  } else if (activeEntity === 'condition') {
    const diag = diagnosticos_legados[0] || {};
    legacyTableContent = (
      <div className="space-y-2">
        <div className="text-xs font-mono font-semibold text-sky-400 mb-2">Tabla: tbl_diagnosticos</div>
        <table className="w-full text-left text-xs font-mono border-collapse">
          <tbody>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">id_diagnostico:</td><td className="py-1 text-white font-bold">{diag.id_diagnostico}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">codigo_cie10:</td><td className="py-1 text-amber-300 font-bold">{diag.codigo_cie10}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">descripcion:</td><td className="py-1 text-slate-200">{diag.descripcion_diagnostico}</td></tr>
            <tr className="border-b border-slate-800"><td className="py-1 text-slate-400">tipo_diagnostico:</td><td className="py-1 text-emerald-300">{diag.tipo_diagnostico}</td></tr>
            <tr><td className="py-1 text-slate-400">fecha_diagnostico:</td><td className="py-1 text-slate-200">{diag.fecha_diagnostico}</td></tr>
          </tbody>
        </table>
      </div>
    );

    ruleExplanation = [
      { from: 'codigo_cie10', to: 'code.coding[0].code', detail: 'Catálogo CIE-10 (http://hl7.org/fhir/sid/icd-10)' },
      { from: 'tipo_diagnostico ("CONFIRMADO")', to: 'verificationStatus ("confirmed")', detail: 'Mapeo clínico a estados de verificación FHIR' },
      { from: 'Constante "active"', to: 'clinicalStatus.coding[0].code', detail: 'Estado clínico activo obligatorio' },
    ];

    fhirResourceContent = fhir_conditions[0] || {};
  }

  return (
    <div className="space-y-6">
      {/* Banner de Principio Arquitectónico */}
      <div className="rounded-xl border border-emerald-800/60 bg-emerald-950/20 p-4 flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <div className="p-2 rounded-lg bg-emerald-900/60 text-emerald-400">
            <Shield className="w-5 h-5" />
          </div>
          <div>
            <h3 className="text-sm font-bold text-emerald-300">
              Invariante de Persistencia de Solo Lectura (NOM-004-SSA3-2012)
            </h3>
            <p className="text-xs text-slate-300">
              El middleware traduce y adapta estructuras relacionales hacia HL7 FHIR R4 en tiempo de ejecución. 
              <span className="font-semibold text-emerald-400"> Cero sentencias INSERT, UPDATE o DELETE</span> sobre la base de datos de origen.
            </p>
          </div>
        </div>

        {/* Selector de Paciente de Prueba */}
        <div className="flex items-center gap-2">
          <span className="text-xs text-slate-400 font-mono hidden sm:inline">Paciente:</span>
          <button
            onClick={() => setSelectedPatientId(1)}
            className={`px-3 py-1.5 rounded-lg text-xs font-mono font-medium transition ${
              selectedPatientId === 1
                ? 'bg-cyan-950 border border-cyan-700 text-cyan-300 font-bold'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
            }`}
          >
            #1: Alberto Ramos
          </button>
          <button
            onClick={() => setSelectedPatientId(2)}
            className={`px-3 py-1.5 rounded-lg text-xs font-mono font-medium transition ${
              selectedPatientId === 2
                ? 'bg-cyan-950 border border-cyan-700 text-cyan-300 font-bold'
                : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
            }`}
          >
            #2: Sofía López
          </button>
        </div>
      </div>

      {/* Selector de Entidad Clínica */}
      <div className="flex gap-2 border-b border-slate-800 pb-2">
        <button
          onClick={() => setActiveEntity('patient')}
          className={`px-4 py-2 rounded-lg text-xs font-semibold flex items-center gap-2 transition ${
            activeEntity === 'patient'
              ? 'bg-cyan-600 text-slate-950 shadow-md shadow-cyan-950'
              : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'
          }`}
        >
          <User className="w-3.5 h-3.5" />
          <span>Paciente (tbl_pacientes ⇄ Patient)</span>
        </button>

        <button
          onClick={() => setActiveEntity('encounter')}
          className={`px-4 py-2 rounded-lg text-xs font-semibold flex items-center gap-2 transition ${
            activeEntity === 'encounter'
              ? 'bg-cyan-600 text-slate-950 shadow-md shadow-cyan-950'
              : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'
          }`}
        >
          <Calendar className="w-3.5 h-3.5" />
          <span>Consulta (tbl_consultas ⇄ Encounter)</span>
        </button>

        <button
          onClick={() => setActiveEntity('observation')}
          className={`px-4 py-2 rounded-lg text-xs font-semibold flex items-center gap-2 transition ${
            activeEntity === 'observation'
              ? 'bg-cyan-600 text-slate-950 shadow-md shadow-cyan-950'
              : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'
          }`}
        >
          <Activity className="w-3.5 h-3.5" />
          <span>Signos Vitales (tbl_signos_vitales ⇄ Observation)</span>
        </button>

        <button
          onClick={() => setActiveEntity('condition')}
          className={`px-4 py-2 rounded-lg text-xs font-semibold flex items-center gap-2 transition ${
            activeEntity === 'condition'
              ? 'bg-cyan-600 text-slate-950 shadow-md shadow-cyan-950'
              : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'
          }`}
        >
          <Stethoscope className="w-3.5 h-3.5" />
          <span>Diagnóstico (tbl_diagnosticos ⇄ Condition)</span>
        </button>
      </div>

      {/* Comparador Visual Lado a Lado */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        {/* Columna Izquierda: Esquema Relacional Legado */}
        <div className="lg:col-span-4 rounded-xl border border-slate-800 bg-slate-900/80 p-4 shadow-xl">
          <div className="flex items-center justify-between border-b border-slate-800 pb-3 mb-3">
            <div className="flex items-center gap-2">
              <Database className="w-4 h-4 text-sky-400" />
              <h4 className="text-xs font-bold uppercase tracking-wider text-slate-300">
                1. Esquema Legado (SQL)
              </h4>
            </div>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-sky-950/80 border border-sky-800 text-sky-300">
              NOM-004-SSA3
            </span>
          </div>

          <div className="p-3 rounded-lg bg-slate-950/90 border border-slate-800/80">
            {legacyTableContent}
          </div>
        </div>

        {/* Columna Central: Reglas y Mapeos Declarativos */}
        <div className="lg:col-span-3 rounded-xl border border-slate-800 bg-slate-900/80 p-4 shadow-xl">
          <div className="flex items-center justify-between border-b border-slate-800 pb-3 mb-3">
            <div className="flex items-center gap-2">
              <FileCode className="w-4 h-4 text-amber-400" />
              <h4 className="text-xs font-bold uppercase tracking-wider text-slate-300">
                2. Motor de Mapeo
              </h4>
            </div>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-amber-950/80 border border-amber-800 text-amber-300">
              YAML v1.1.0
            </span>
          </div>

          <div className="space-y-3">
            {ruleExplanation?.map((rule, idx) => (
              <div key={idx} className="p-2.5 rounded-lg bg-slate-950 border border-slate-800 text-xs">
                <div className="flex items-center gap-1.5 font-mono text-cyan-300 font-semibold mb-1">
                  <span>{rule.from}</span>
                  <ArrowRight className="w-3 h-3 text-amber-400 shrink-0" />
                  <span className="text-emerald-400">{rule.to}</span>
                </div>
                <p className="text-[11px] text-slate-400">{rule.detail}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Columna Derecha: Recurso Canónico HL7 FHIR R4 */}
        <div className="lg:col-span-5 h-[520px]">
          <JsonSyntaxHighlighter
            data={fhirResourceContent}
            title="3. Recurso HL7 FHIR R4 Emitido (application/fhir+json)"
          />
        </div>
      </div>
    </div>
  );
}
