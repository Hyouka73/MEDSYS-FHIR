# MANUAL DE DESPLIEGUE Y ESPECIFICACIÓN TÉCNICA — MedSys-FHIR

**Tesis de Licenciatura:** *Diseño e Implementación de un Middleware de Interoperabilidad en Salud para la Transformación de Expedientes Clínicos Electrónicos Relacionales al Estándar HL7 FHIR R4 empleando Rust.*  
**Autores:** Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero  
**Institución:** Universidad Autónoma de Chiapas (UNACH), 2026.

---

## 1. Resumen Ejecutivo y Arquitectura de la Solución

MedSys-FHIR es un middleware de grado clínico desarrollado en el lenguaje de programación de sistemas **Rust (edición 2021)**, diseñado para resolver la interoperabilidad semántica en instituciones de salud sin alterar los esquemas ni arriesgar los datos de los sistemas legados existentes.

El middleware implementa los patrones de diseño **Adapter**, **Facade** y **Data Mapper**, aplicando de forma desacoplada y estricta el principio de **sólo lectura** (`Read-Only Data Access`) para dar cumplimiento irrestricto a la **NOM-004-SSA3-2012** (expediente clínico mexicano) y la **LFPDPPP** (protección de datos personales en posesión de particulares).

```
┌────────────────────────────────────────────────────────────────────────┐
│                   Cliente / Ecosistema Hospitalario                   │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ HTTP / JSON
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                 CAPA FACADE & ENRUTAMIENTO HTTP (Axum)                 │
│  - Negociación: Content-Type: application/fhir+json; charset=utf-8     │
│  - Middleware: CORS permisivo, Telemetría estructurada con tracing     │
│  - Manejador Global de Errores: Conversión universal a OperationOutcome│
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│             MOTOR DE TRANSFORMACIÓN SEMÁNTICA (medsys-core)            │
│  - Especificación declarativa desacoplada: mapping_rules.yaml         │
│  - Modelado estándar canónico HL7 FHIR Release 4 (helios-fhir v0.2)    │
│  - Patient (CURP oficial) | Encounter (AMB + Cédula SEP)              │
│  - Observation (Presión Arterial LOINC 85354-9 / Temp LOINC 8310-5)    │
│  - Condition (Codificación Internacional CIE-10)                      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ DTOs intermedios
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│               ADAPTADOR DE PERSISTENCIA (medsys-db / SQLx)             │
│  - Pool asíncrono no bloqueante con sqlx::PgPool                       │
│  - Consultas 100% parametrizadas con placeholders ($1, $2)             │
│  - Cero mutaciones (SELECT exclusivo sobre tablas legadas)             │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ TCP / PostgreSQL Wire Protocol
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                 BASE DE DATOS RELACIONAL LEGADA NOM-004                │
│             (PostgreSQL 16 en contenedor Docker o Servidor Físico)     │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Estructura del Monorepo Cargo Workspace

```text
MEDSYS-FHIR/
├── Cargo.toml                              # Workspace raíz y perfil release (opt-level 3, LTO)
├── docker-compose.yml                      # Contenedor PostgreSQL 16 con volumen persistente
├── schema_legado_simulado_nom004.sql       # Esquema DDL y datos sintéticos clínicos
├── mapping_rules_specification.yaml        # Reglas declarativas YAML (v1.1.0)
│
├── crates/
│   ├── medsys-core/                        # Núcleo de dominio puro (sin dependencias web/BD)
│   │   ├── src/
│   │   │   ├── engine/                     # Parser YAML, transformaciones FHIR y Bundles
│   │   │   ├── error/                      # Jerarquía tipada MedSysError (thiserror)
│   │   │   └── model/                      # Modelos legados y reglas intermedias
│   │   └── Cargo.toml
│   │
│   ├── medsys-db/                          # Adaptador de persistencia asíncrona SQLx
│   │   ├── src/
│   │   │   ├── config/                     # Configuración de pool y masking de credenciales
│   │   │   ├── entities/                   # Entidades de mapeo relacional de PostgreSQL
│   │   │   ├── pool/                       # Gestor de conexiones DbManager y PgPool
│   │   │   └── repository/                 # Repositorios parametrizados ($1, $2)
│   │   ├── tests/                          # Pruebas de integración de persistencia
│   │   └── Cargo.toml
│   │
│   └── medsys-server/                      # Servidor HTTP REST en Axum 0.8
│       ├── src/
│       │   ├── config/                     # Variables de entorno y configuración
│       │   ├── error/                      # ServerError -> IntoResponse (OperationOutcome)
│       │   ├── handlers/                   # Controladores REST para Patient, Encounter, etc.
│       │   ├── response/                   # Wrapper con cabecera application/fhir+json
│       │   ├── router/                     # Enrutador Axum, middlewares y fallback
│       │   └── state/                      # Estado compartido AppState thread-safe
│       ├── tests/                          # Pruebas de integración E2E
│       └── Cargo.toml
│
├── dashboard/                              # Frontend React 19 + Vite + Tailwind CSS
│   ├── src/
│   │   ├── components/                     # MetricsPanel, FhirViewer, InteroperabilityComparator
│   │   └── services/                       # Cliente API con telemetría de latencia
│   └── vite.config.js                      # Proxy hacia http://localhost:3000
│
└── tests/k6/                               # Suite de pruebas de carga y rendimiento k6
    ├── smoke_test.js                       # Prueba de disponibilidad y conformidad
    ├── load_test.js                        # Prueba de concurrencia clínica
    ├── resilience_and_errors_test.js       # Prueba de inyección de fallos OperationOutcome
    └── run_all_benchmarks.ps1              # Ejecutor automatizado PowerShell
```

---

## 3. Matriz Canónica de Endpoints REST FHIR R4

| Método | Endpoint | Recurso Emitido | Descripción Semántica |
| :--- | :--- | :--- | :--- |
| `GET` | `/health` | `JSON` | Diagnóstico de uptime, versión del servidor y estado de PostgreSQL. |
| `GET` | `/fhir/r4/Patient/{id}` | `Patient` | Paciente con CURP oficial (`urn:oid:2.16.840.1.113883.4.629`). |
| `GET` | `/fhir/r4/Patient` | `Bundle` (searchset) | Colección paginada de todos los pacientes en formato searchset. |
| `GET` | `/fhir/r4/Encounter/{id}` | `Encounter` | Encuentro clínico con clase `AMB` y médico tratante SEP. |
| `GET` | `/fhir/r4/Encounter` | `Bundle` (searchset) | Colección de encuentros con filtro opcional `?patient={id}`. |
| `GET` | `/fhir/r4/Observation/bp-{id}` | `Observation` | Panel de Presión Arterial (LOINC `85354-9`, sistólica `8480-6`, diastólica `8462-4`). |
| `GET` | `/fhir/r4/Observation/temp-{id}` | `Observation` | Temperatura corporal (LOINC `8310-5`) en grados Celsius UCUM (`Cel`). |
| `GET` | `/fhir/r4/Observation` | `Bundle` (searchset) | Colección de observaciones vitales desacopladas. |
| `GET` | `/fhir/r4/Condition/{id}` | `Condition` | Diagnóstico clínico codificado bajo el estándar internacional CIE-10. |
| `GET` | `/fhir/r4/Condition` | `Bundle` (searchset) | Colección de condiciones con filtros por paciente, consulta o código CIE-10. |
| `GET` | `/api/legacy/patients/{id}/full` | `JSON` | Vista cruda del registro relacional para el comparador de interoperabilidad. |
| `*` | *Rutas no registradas* | `OperationOutcome` | Captura universal fallback que devuelve HTTP 404 con diagnóstico clínico. |

---

## 4. Requisitos de Instalación y Despliegue

### 4.1 Prerrequisitos de Software
- **Rust Toolchain:** v1.80 o superior (compatible con MinGW GNU y MSVC).
- **PostgreSQL:** v16.x (local o mediante Docker Compose).
- **Node.js:** v18 o superior con NPM.
- **k6 (Grafana Labs):** v2.x para pruebas de rendimiento.

### 4.2 Variables de Entorno (`.env`)
Configurar en la raíz del proyecto o exportar en el entorno:
```bash
# Conexión a Base de Datos PostgreSQL NOM-004
DATABASE_URL=postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy

# Servidor Web Axum
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Telemetría y Registro
RUST_LOG=info,medsys_server=debug,medsys_db=debug
```

### 4.3 Inicialización de la Base de Datos con Docker
```bash
docker compose up -d
```
El contenedor inicializa automáticamente las tablas `tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales` y `tbl_diagnosticos` con datos sintéticos representativos.

### 4.4 Compilación y Ejecución del Servidor Rust (Modo Release)
```bash
# Compilar binario altamente optimizado (LTO + strip)
cargo build --release

# Ejecutar el servidor de producción
./target/release/medsys-server
```

### 4.5 Ejecución del Dashboard de Monitoreo (React 19)
```bash
cd dashboard
npm install
npm run dev
```
Acceder mediante navegador web en `http://localhost:5173`.

---

## 5. Resultados de Benchmarking de Rendimiento (k6)

Las pruebas de carga ejecutadas con **k6 v2.2.0** contra el binario release en Windows demostraron los siguientes resultados de rendimiento:

| Métrica Evaluada | Requisito de Tesis | Resultado Obtenido | Estado |
| :--- | :---: | :---: | :---: |
| **Latencia de Lectura p95 ($p_{95}$)** | $< 50 \text{ ms}$ | **$1.14 \text{ ms}$** | **SUPERADO** (43x más rápido) |
| **Latencia Promedio** | $< 25 \text{ ms}$ | **$486 \mu\text{s}$ ($0.48 \text{ ms}$)** | **SUPERADO** |
| **Conformidad `OperationOutcome`** | $100\%$ | **$100.00\%$** (5,780/5,780 reqs) | **CUMPLIDO** |
| **Throughput de Errores Clínicos** | $> 100 \text{ req/s}$ | **$384.41 \text{ req/s}$** | **SUPERADO** |
| **Tasa de Falla en Validaciones** | $< 1\%$ | **$0.00\%$** | **CUMPLIDO** |

---

## 6. Aseguramiento de Calidad y Verificación

1. **Pruebas de Software Workspace:**
   - **46 / 46 pruebas automatizadas aprobadas (100% PASS)**:
     - 11 pruebas unitarias en `medsys-core` (parser YAML, transformaciones semánticas, Bundles).
     - 19 pruebas unitarias en `medsys-db` (SQLx pooling, repositorios parametrizados, entidades).
     - 3 pruebas de integración en `medsys-db` (persistencia relacional, mapeo sintético).
     - 5 pruebas de servidor en `medsys-server` (enrutamiento HTTP, OperationOutcome, fallback).
     - 8 pruebas de integración E2E en `medsys-server` (ciclo completo relacional ➔ FHIR R4).
2. **Auditoría de Código Estricta:**
   - `cargo clippy --workspace --all-targets -- -D warnings`: **0 advertencias**.
   - `cargo fmt --check`: **Formateo canónico aprobado**.
   - `oxlint` (Dashboard): **0 errores y 0 advertencias**.
3. **Optimización del Binario:**
   - Binario autosuficiente de **6.55 MB** compilado con LTO completo (`lto = true`), 1 unidad de generación de código (`codegen-units = 1`) y eliminación de símbolos (`strip = true`).

---

## 7. Dictamen de Acreditación Académica y Legal

El sistema **MedSys-FHIR** cumple en su totalidad con:
1. **NOM-004-SSA3-2012**: Respeto absoluto a la integridad del expediente clínico mediante acceso estrictamente no invasivo de sólo lectura (`Read-Only`).
2. **LFPDPPP**: Empleo exclusivo de datos clínicos sintéticos para investigación académica sin exposición de pacientes reales.
3. **HL7 FHIR Release 4**: Emisión canónica de los 4 recursos objeto de la tesis con sus terminologías vinculadas (CURP, AMB, Cédula SEP, LOINC 85354-9/8310-5, CIE-10) y manejo de diagnósticos mediante `OperationOutcome`.
