# 1. Etapa de Construcción (Builder)
FROM rust:1-slim-bookworm as builder

WORKDIR /app
COPY . .

# Construimos la app en modo release
RUN cargo build --release

# 2. Etapa Final (Runtime)
FROM debian:bookworm-slim

# --- CORRECCIÓN AQUÍ ---
# Instalamos 'curl' (para el healthcheck) y 'libssl-dev' (para conexiones HTTPS)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
# -----------------------

COPY --from=builder /app/target/release/telnyx_ai_service /usr/local/bin/telnyx_ai_service

WORKDIR /app

# OJO: No copiamos .env.example porque usaremos las variables de DigitalOcean
# COPY .env.example .env.example 

# Exponemos el puerto 3000 (Importante configurar esto en DigitalOcean también)
EXPOSE 3000
ENV PORT=3000

# Health check (Ahora sí funcionará porque instalamos curl)
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

CMD ["telnyx_ai_service"]
