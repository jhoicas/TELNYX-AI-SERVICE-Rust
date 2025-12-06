# 1. Etapa de Construcción (Builder)
# CAMBIO CLAVE: Usamos 'rust:1' en lugar de '1.75' para soportar edition2024
FROM rust:1-slim-bookworm as builder

WORKDIR /app

# Instalamos las herramientas necesarias para compilar librerías de AWS y Reqwest
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

# Construimos la app en modo release
RUN cargo build --release

# 2. Etapa Final (Runtime)
FROM debian:bookworm-slim

# Instalamos dependencias para CORRER la app (SSL y Curl)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/telnyx_ai_service /usr/local/bin/telnyx_ai_service

WORKDIR /app

# Exponemos el puerto 3000
EXPOSE 3000
ENV PORT=3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

CMD ["telnyx_ai_service"]
