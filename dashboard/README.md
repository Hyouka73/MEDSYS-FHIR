# MedSys-FHIR Dashboard

Dashboard analítico e interactivo desarrollado con **React 19**, **Vite**, **Tailwind CSS** y **Lucide Icons** para la supervisión y validación en tiempo real del middleware de interoperabilidad clínica **MedSys-FHIR** (UNACH 2026).

---

## 🌟 Características Principales (Sección 3.5 de la Tesis)

1. **Panel Superior de Métricas en Tiempo Real**:
   - Monitoreo continuo del endpoint `/health` de Axum.
   - Latencia de red y procesamiento en milisegundos (`ms`).
   - Estado de salud de la base de datos relacional PostgreSQL NOM-004.
   - Total de eventos y transacciones capturadas en la sesión.

2. **Visor Interactivo de Recursos FHIR R4**:
   - Explorador de recursos: `Patient`, `Encounter`, `Observation`, `Condition`.
   - Modos de visualización: Instancia única (`/fhir/r4/{Resource}/{id}`) y Bundle de búsqueda canónico (`/fhir/r4/{Resource}`).
   - Resaltado sintáctico JSON optimizado con botones de copiado al portapapeles y descarga de archivo `.json`.
   - Metadatos clínicos e identificadores semánticos (CURP, NSS, CIE-10, LOINC).

3. **Comparador Visual de Interoperabilidad Lado a Lado**:
   - Demostración de traducción sin alteración de la base original (read-only NOM-004).
   - Vista trifásica:
     1. **Registro Relacional Legado (SQL NOM-004)**: Esquema `tbl_pacientes`, `tbl_consultas`, etc.
     2. **Matriz de Reglas de Mapeo Declarativo**: Reglas YAML de transformación semántica.
     3. **Recurso FHIR R4 Canónico**: Salida normalizada HL7 FHIR R4 JSON.

4. **Consola de Eventos y Diagnósticos `OperationOutcome`**:
   - Registro cronológico de peticiones HTTP, códigos de estado (200, 400, 404, 500) y tiempos de respuesta.
   - Banco de pruebas integrado para disparar fallos controlados (404 Not Found, 400 Bad Request, 404 Fallback).
   - Inspector de recursos `OperationOutcome` con diagnóstico clínico detallado en español técnico.

---

## 🚀 Inicio Rápido

### Prerrequisitos
- **Node.js** >= 18 (Probado en Node.js v24.15.0)
- **NPM** >= 9 (Probado en npm 11.12.1)
- Servidor Backend `medsys-server` ejecutándose en `http://localhost:3000` (opcional; el frontend incluye fallback automático a datos sintéticos clínicos si el servidor no está en ejecución).

### Instalación de Dependencias
```bash
cd dashboard
npm install
```

### Ejecución en Modo Desarrollo
```bash
npm run dev
```
El servidor de desarrollo Vite estará disponible en `http://localhost:5173`. Las peticiones hacia `/fhir`, `/health`, y `/api` se reenvían automáticamente al backend en `http://localhost:3000` vía proxy Vite.

### Compilación para Producción
```bash
npm run build
```
Genera el paquete estático ultra-optimizado en la carpeta `dist/`.

### Análisis Estático (Linting)
```bash
npm run lint
```
Utiliza **Oxlint** para verificación ultra-rápida y garantía de cero advertencias.

---

## 🏗️ Arquitectura de Carpetas

```text
dashboard/
├── dist/                      # Salida de compilación estática
├── public/                    # Recursos públicos y favicon
├── src/
│   ├── components/
│   │   ├── EventConsole.jsx               # Consola HTTP y OperationOutcome
│   │   ├── FhirViewer.jsx                 # Visor de recursos e instancias FHIR
│   │   ├── Header.jsx                     # Encabezado con estado y navegación
│   │   ├── InteroperabilityComparator.jsx # Vista lado a lado SQL ⇄ FHIR
│   │   ├── JsonSyntaxHighlighter.jsx      # Resaltador de código JSON
│   │   └── MetricsPanel.jsx               # Tarjetas KPI de latencia y estado
│   ├── services/
│   │   ├── api.js                         # Cliente HTTP con métricas y fallback
│   │   └── mockData.js                    # Datos sintéticos clínicos NOM-004
│   ├── App.jsx                            # Orquestador principal de la aplicación
│   ├── index.css                          # Estilos globales con Tailwind CSS
│   └── main.jsx                           # Punto de entrada React 19
├── index.html                             # Plantilla HTML con metadata SEO
├── package.json                           # Dependencias y scripts
└── vite.config.js                         # Configuración Vite + Proxy Backend
```

---

## 🔒 Cumplimiento Normativo y Privacidad
Este frontend y sus datos de respaldo se adhieren rigurosamente a:
- **NOM-004-SSA3-2012**: Estructura del expediente clínico mexicano.
- **LFPDPPP**: Datos 100% sintéticos generados para fines de validación académica sin exposición de información personal real identificable (PII).
- **HL7 FHIR R4**: Estándar internacional de interoperabilidad en salud.
