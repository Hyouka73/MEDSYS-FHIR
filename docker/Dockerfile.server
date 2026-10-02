# ==============================================================================
# Multi-stage Dockerfile para MedSys-FHIR Middleware (Axum / HL7 FHIR R4)
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
# ==============================================================================

# Etapa 1: Compilación de la aplicación en Rust
FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

# Instalar certificados y dependencias para compilación
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar manifiestos y código fuente del workspace
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Compilar el binario del middleware en modo release
RUN cargo build --release --bin medsys-server

# ==============================================================================
# Etapa 2: Entorno de ejecución ligero (debian:bookworm-slim)
# ==============================================================================
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Instalar certificados CA y utilería curl para healthcheck
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado y la especificación de reglas de mapeo YAML
COPY --from=builder /usr/src/app/target/release/medsys-server /app/medsys-server
COPY mapping_rules.yaml /app/mapping_rules.yaml

# Configuración de variables de entorno canónicas
ENV SERVER_HOST=0.0.0.0 \
    SERVER_PORT=3000 \
    MAPPING_RULES_PATH=/app/mapping_rules.yaml \
    RUST_LOG=info,medsys_server=debug,medsys_db=debug

# Exponer el puerto HTTP del middleware
EXPOSE 3000

# Healthcheck interno del contenedor
HEALTHCHECK --interval=10s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Comando de arranque del servidor
ENTRYPOINT ["/app/medsys-server"]
