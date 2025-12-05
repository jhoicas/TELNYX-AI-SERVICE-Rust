.PHONY: help build run release clean test fmt lint doc setup deploy watch

# Default target
help:
	@echo "Telnyx AI Service (Rust) - Comandos disponibles:"
	@echo ""
	@echo "Development:"
	@echo "  make setup        - Configuración inicial"
	@echo "  make run          - Ejecutar en desarrollo"
	@echo "  make watch        - Ejecutar con auto-reload (requiere cargo-watch)"
	@echo ""
	@echo "Building:"
	@echo "  make build        - Compilar (debug)"
	@echo "  make release      - Compilar optimizado para producción"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt          - Formatear código"
	@echo "  make lint         - Ejecutar clippy (linter)"
	@echo "  make test         - Correr tests"
	@echo ""
	@echo "Documentation:"
	@echo "  make doc          - Generar documentación"
	@echo "  make doc-open     - Generar y abrir documentación"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean        - Limpiar build artifacts"
	@echo "  make check        - Verificar compilación (rápido)"
	@echo "  make verify       - Verificar estructura del proyecto"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Compilar imagen Docker"
	@echo "  make docker-run   - Ejecutar en Docker"
	@echo "  make docker-logs  - Ver logs del container"
	@echo "  make docker-stop  - Detener container"
	@echo ""

setup:
	@echo "📦 Instalando dependencias..."
	@cargo fetch
	@echo "✅ Setup completado"
	@echo ""
	@echo "⚠️  Próximos pasos:"
	@echo "  1. Editar .env con tus credenciales"
	@echo "  2. Ejecutar: make run"

build:
	@echo "🔨 Compilando (debug)..."
	@cargo build
	@echo "✅ Build completado"

release:
	@echo "🚀 Compilando (release - optimizado)..."
	@cargo build --release
	@echo "✅ Release completado"
	@echo ""
	@echo "📦 Binario en: target/release/telnyx_ai_service"

run:
	@echo "🏃 Ejecutando servidor..."
	@cargo run

watch:
	@echo "👀 Ejecutando con auto-reload..."
	@command -v cargo-watch >/dev/null 2>&1 || { echo "Instalando cargo-watch..."; cargo install cargo-watch; }
	@cargo watch -x run

test:
	@echo "🧪 Ejecutando tests..."
	@cargo test

clean:
	@echo "🧹 Limpiando..."
	@cargo clean
	@echo "✅ Limpieza completada"

check:
	@echo "🔍 Verificando compilación..."
	@cargo check
	@echo "✅ Check completado"

fmt:
	@echo "✨ Formateando código..."
	@cargo fmt
	@echo "✅ Formato aplicado"

lint:
	@echo "🎯 Ejecutando clippy..."
	@cargo clippy -- -D warnings
	@echo "✅ Lint completado"

doc:
	@echo "📖 Generando documentación..."
	@cargo doc --no-deps

doc-open:
	@echo "📖 Generando y abriendo documentación..."
	@cargo doc --no-deps --open

verify:
	@echo "🔍 Verificando estructura del proyecto..."
	@python3 verify_structure.py || echo "verify_structure.py no encontrado"

docker-build:
	@echo "🐳 Compilando imagen Docker..."
	@docker build -t telnyx-ai-service:latest .
	@echo "✅ Imagen compilada"

docker-run:
	@echo "🚀 Ejecutando en Docker..."
	@docker-compose up -d
	@echo "✅ Servidor ejecutándose en http://localhost:3000"
	@echo "   Ver logs: docker-compose logs -f"

docker-logs:
	@docker-compose logs -f

docker-stop:
	@echo "🛑 Deteniendo container..."
	@docker-compose down
	@echo "✅ Container detenido"

# Extras - para desarrollo local
fmt-check:
	@cargo fmt -- --check

audit:
	@echo "🔐 Verificando vulnerabilidades..."
	@cargo audit

deps-tree:
	@echo "📦 Árbol de dependencias:"
	@command -v cargo-tree >/dev/null 2>&1 || { cargo install cargo-tree; }
	@cargo tree

outdated:
	@echo "🔄 Verificando dependencias outdated..."
	@command -v cargo-outdated >/dev/null 2>&1 || { cargo install cargo-outdated; }
	@cargo outdated

# All in one for CI/CD
ci: check fmt-check lint test
	@echo "✅ CI checks passed!"

all: clean build test lint doc
	@echo "✅ Proceso completo finalizado!"

# Para desarrollo rápido
dev: build run
	@true

# Para producción
prod: release docker-build
	@echo "✅ Listo para producción!"
	@echo "   1. Ejecutar: docker-compose up -d"
	@echo "   2. Verificar: curl http://localhost:3000/api/health"
