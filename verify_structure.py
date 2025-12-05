#!/usr/bin/env python3
"""
Script para verificar la estructura del proyecto TELNYX-AI-SERVICE-Rust
Ejecutar: python3 verify_structure.py
"""

import os
from pathlib import Path

EXPECTED_FILES = {
    "root": [
        "Cargo.toml",
        ".env.example",
        ".gitignore",
        "Dockerfile",
        "docker-compose.yml",
        "README.md",
        "QUICKSTART.md",
        "MIGRACION.md",
        "BENCHMARKS.md",
        "RESUMEN.md",
        "setup.sh",
        "setup.bat",
        "Cargo-optimization.toml"
    ],
    "src": [
        "main.rs",
        "models.rs"
    ],
    "src/services": [
        "mod.rs",
        "telnyx.rs",
        "claude.rs",
        "session.rs",
        "s3.rs",
        "app_state.rs"
    ],
    "src/handlers": [
        "mod.rs",
        "call.rs",
        "webhook.rs"
    ],
    "src/utils": [
        "mod.rs",
        "logger.rs"
    ],
    "src/middleware": [
        "mod.rs"
    ],
    "tests": [
        "integration_tests.rs"
    ]
}

def check_structure(root_path="."):
    """Verificar estructura del proyecto"""
    print("🔍 Verificando estructura de TELNYX-AI-SERVICE-Rust\n")
    
    all_good = True
    
    for directory, files in EXPECTED_FILES.items():
        dir_path = Path(root_path) / directory if directory != "root" else Path(root_path)
        
        print(f"📁 {directory or 'root'}/ ", end="")
        
        if not dir_path.exists():
            print(f"❌ NO EXISTE")
            all_good = False
            continue
        
        print(f"✅")
        
        for file in files:
            file_path = dir_path / file
            status = "✅" if file_path.exists() else "❌"
            print(f"  {status} {file}")
            if not file_path.exists():
                all_good = False
    
    print("\n" + "="*50)
    if all_good:
        print("✅ ESTRUCTURA CORRECTA - Todo listo para comenzar!")
        print("\nPróximos pasos:")
        print("  1. Editar .env con credenciales")
        print("  2. cargo build")
        print("  3. cargo run")
    else:
        print("❌ FALTAN ARCHIVOS - Verifica la estructura")
    
    return all_good

def show_statistics(root_path="."):
    """Mostrar estadísticas del proyecto"""
    print("\n📊 Estadísticas del proyecto:\n")
    
    total_files = 0
    total_lines = 0
    rust_lines = 0
    
    for root, dirs, files in os.walk(root_path):
        # Ignorar directorios de build y .git
        dirs[:] = [d for d in dirs if d not in ['target', '.git', 'node_modules']]
        
        for file in files:
            if file.endswith(('.rs', '.md', '.toml', '.yml', '.yaml', '.json')):
                total_files += 1
                file_path = os.path.join(root, file)
                
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        lines = len(f.readlines())
                        total_lines += lines
                        if file.endswith('.rs'):
                            rust_lines += lines
                except:
                    pass
    
    print(f"  Total de archivos: {total_files}")
    print(f"  Líneas de código Rust: {rust_lines}")
    print(f"  Líneas totales (incluye docs): {total_lines}")

if __name__ == "__main__":
    import sys
    root = sys.argv[1] if len(sys.argv) > 1 else "."
    
    check_structure(root)
    show_statistics(root)
