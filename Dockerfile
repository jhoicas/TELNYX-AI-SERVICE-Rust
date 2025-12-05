# Use official Rust runtime as base image
FROM rust:1.75-slim-bookworm as builder

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Final stage - minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/telnyx_ai_service /usr/local/bin/telnyx_ai_service

# Set working directory
WORKDIR /app

# Copy .env.example (user will override with .env in docker-compose)
COPY .env.example .env.example

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:3000/api/health || exit 1

# Run the application
CMD ["telnyx_ai_service"]
