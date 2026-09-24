# Infraestructura de Base de Datos PostgreSQL 16 — MedSys-FHIR

Este directorio contiene la configuración contenerizada de persistencia relacional para simular el sistema hospitalario legado normativo (**NOM-004-SSA3-2012** y **NOM-024-SSA3-2012**).

---

## 1. Parámetros de Conexión

| Variable | Valor por Defecto | Descripción |
| :--- | :--- | :--- |
| `POSTGRES_HOST` | `localhost` | Host local de acceso |
| `POSTGRES_PORT` | `5432` | Puerto estándar de PostgreSQL |
| `POSTGRES_DB` | `medsys_legacy` | Nombre de la base de datos |
| `POSTGRES_USER` | `medsys_user` | Usuario de conexión |
| `POSTGRES_PASSWORD` | `medsys_secure_pass_2026` | Contraseña segura |
| `DATABASE_URL` | `postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy` | Cadena de conexión canónica para SQLx |

---

## 2. Esquema Relacional Cargado Automáticamente

Al levantar el contenedor por primera vez (o tras un reset), Docker inicializa automáticamente el archivo:
`schema_legado_simulado_nom004.sql` a través del punto de montaje `/docker-entrypoint-initdb.d/`.

Tablas creadas y datos sintéticos incluidos:
- **`tbl_pacientes`**: Identificador nacional CURP, nombres, apellidos paterno/materno, sexo biológico, fecha de nacimiento y contacto.
- **`tbl_consultas`**: Encuentros médicos, cédula profesional SEP del médico tratante, motivo de atención y periodo (inicio/fin).
- **`tbl_signos_vitales`**: Panel de presión arterial (sistólica/diastólica), frecuencia cardíaca/respiratoria y temperatura corporal.
- **`tbl_diagnosticos`**: Códigos CIE-10 (e.g. `I10`, `G43.9`) y estado de confirmación.

---

## 3. Comandos de Uso

### En Windows (PowerShell):
```powershell
# Iniciar base de datos con verificación de salud
.\docker\start-db.ps1

# Detener el contenedor
.\docker\stop-db.ps1

# Reiniciar desde cero (borra volumen y re-ejecuta el script SQL)
.\docker\reset-db.ps1
```

### En Linux / macOS / WSL (Bash):
```bash
# Iniciar base de datos
./docker/start-db.sh

# Detener el contenedor
./docker/stop-db.sh

# Reiniciar desde cero
./docker/reset-db.sh
```

### Directo con Docker Compose:
```bash
# Levantar en segundo plano
docker compose -f docker/docker-compose.yml up -d

# Ver logs
docker compose -f docker/docker-compose.yml logs -f postgres

# Detener
docker compose -f docker/docker-compose.yml down
```

---

## 4. Verificación de la Base de Datos

Para comprobar interactivamente que las tablas y registros están disponibles:
```bash
docker exec -it medsys_fhir_postgres psql -U medsys_user -d medsys_legacy -c "SELECT curp, primer_nombre, apellido_paterno FROM tbl_pacientes;"
```
