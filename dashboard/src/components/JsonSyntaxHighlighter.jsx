import React, { useState } from 'react';
import { Copy, Check, Download } from 'lucide-react';

export default function JsonSyntaxHighlighter({ data, title = "Recurso HL7 FHIR R4 (application/fhir+json)" }) {
  const [copied, setCopied] = useState(false);

  const jsonString = typeof data === 'string' ? data : JSON.stringify(data, null, 2);

  const handleCopy = () => {
    navigator.clipboard.writeText(jsonString);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    const blob = new Blob([jsonString], { type: 'application/fhir+json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${data?.resourceType || 'recurso'}-${data?.id || 'export'}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  // Resaltado de sintaxis JSON con regex para estilos seguros
  const formatJsonHtml = (json) => {
    if (!json) return '';
    const escaped = json
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');

    return escaped.replace(
      /("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+-]?\d+)?)/g,
      (match) => {
        let cls = 'text-amber-400'; // número
        if (/^"/.test(match)) {
          if (/:$/.test(match)) {
            // Clave JSON
            if (match.includes('"resourceType"')) {
              cls = 'text-cyan-400 font-semibold';
            } else if (match.includes('"id"') || match.includes('"system"') || match.includes('"code"')) {
              cls = 'text-sky-300 font-medium';
            } else {
              cls = 'text-slate-300';
            }
          } else {
            // Valor String
            if (match.includes('"Patient"') || match.includes('"Encounter"') || match.includes('"Observation"') || match.includes('"Condition"') || match.includes('"OperationOutcome"')) {
              cls = 'text-emerald-400 font-bold';
            } else {
              cls = 'text-emerald-300';
            }
          }
        } else if (/true|false/.test(match)) {
          cls = 'text-purple-400 font-medium';
        } else if (/null/.test(match)) {
          cls = 'text-rose-400 italic';
        }
        return `<span class="${cls}">${match}</span>`;
      }
    );
  };

  return (
    <div className="rounded-xl border border-slate-800 bg-slate-900/90 shadow-2xl overflow-hidden backdrop-blur flex flex-col h-full">
      <div className="flex items-center justify-between border-b border-slate-800 px-4 py-2.5 bg-slate-950/70">
        <div className="flex items-center gap-2">
          <div className="flex gap-1.5">
            <span className="w-2.5 h-2.5 rounded-full bg-rose-500/80"></span>
            <span className="w-2.5 h-2.5 rounded-full bg-amber-500/80"></span>
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-500/80"></span>
          </div>
          <span className="text-xs font-mono text-slate-400 ml-2">{title}</span>
        </div>
        <div className="flex items-center gap-1.5">
          <button
            onClick={handleCopy}
            className="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800/80 hover:bg-slate-700 transition"
            title="Copiar JSON"
          >
            {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
            <span>{copied ? 'Copiado' : 'Copiar'}</span>
          </button>
          <button
            onClick={handleDownload}
            className="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium text-slate-300 hover:text-white bg-slate-800/80 hover:bg-slate-700 transition"
            title="Descargar archivo JSON"
          >
            <Download className="w-3.5 h-3.5 text-sky-400" />
            <span>Descargar</span>
          </button>
        </div>
      </div>
      <div className="p-4 overflow-auto flex-1 font-mono text-xs leading-relaxed max-h-[520px]">
        <pre
          className="whitespace-pre-wrap select-text"
          dangerouslySetInnerHTML={{ __html: formatJsonHtml(jsonString) }}
        />
      </div>
    </div>
  );
}
