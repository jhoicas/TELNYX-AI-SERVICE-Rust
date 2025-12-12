# 🔧 Instalar Build Tools para Compilar

## ❌ Problema Actual

```
error: linker `link.exe` not found
  = note: program not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.
```

**Causa**: Falta Visual Studio 2022 Build Tools con C++ compiler y linker.

---

## ✅ Solución: Instalar Build Tools

### Opción 1: Instalador Automático (Recomendado)

```powershell
# En PowerShell como administrador
winget install --id Microsoft.VisualStudio.2022.BuildTools --source winget --accept-source-agreements
```

**Espera a que termine (~5-10 min descarga + instalación)**

### Opción 2: Descarga Manual

1. Descargar: https://aka.ms/vs/17/release/vs_BuildTools.exe
2. Ejecutar el instalador
3. **IMPORTANTE**: Seleccionar exactamente esto:
   ```
   ✓ Desktop development with C++
   ```
4. Esperar instalación completa (~15-20 min)
5. Reiniciar (opcional pero recomendado)

---

## 🔍 Verificar Instalación

Después de instalar, abre **NUEVA** terminal PowerShell y verifica:

```powershell
# Verificar que link.exe está disponible
where link.exe
# Debe mostrar: C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xxx\bin\Hostx64\x64\link.exe

# Verificar que cargo funciona
cargo --version
# Debe mostrar: cargo 1.xx.x
```

---

## 🚀 Una Vez Instalado

Ejecuta en la nueva terminal:

```powershell
cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust

# Compilación rápida (verificar)
cargo check

# O compilación completa (optimizada)
cargo build --release
```

**Primera compilación tarda ~3-5 minutos.**

---

## ❓ Si Aún Falla

### link.exe no encontrado después de instalar

```powershell
# 1. Cierra TODAS las terminales PowerShell

# 2. Abre "x64 Native Tools Command Prompt for VS 2022"
#    (Busca en Start menu)

# 3. En esa terminal, ve a tu repo y compila:
cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
cargo build --release
```

### Error: "Visual Studio 2022 not found"

```powershell
# Desinstala e reinstala completamente
winget uninstall Microsoft.VisualStudio.2022.BuildTools
# Espera a que termine

# Reinstala
winget install --id Microsoft.VisualStudio.2022.BuildTools --source winget
# En el instalador: marca "Desktop development with C++"
```

### Error: "Visual C++ workload not installed"

```powershell
# El instalador de Visual Studio se abre automáticamente
# MARCA EXACTAMENTE:
# ✓ Desktop development with C++
# 
# Luego "Install" y espera
```

---

## 📋 Checklist de Instalación

- [ ] Descargar e instalar Visual Studio 2022 Build Tools
- [ ] Seleccionar "Desktop development with C++"
- [ ] Esperar instalación (10-20 min)
- [ ] Cerrar todas las terminales PowerShell
- [ ] Abrir NUEVA terminal PowerShell
- [ ] Ejecutar: `where link.exe`
- [ ] Debe mostrar una ruta (no error)
- [ ] Ejecutar: `cargo --version`
- [ ] Debe mostrar versión de cargo
- [ ] Ejecutar: `cargo check` en tu repo
- [ ] Debe comenzar a compilar (no error de linker)

---

## 🎯 Una Vez Compilado Exitosamente

Verás algo como:

```
    Finished `check` profile [unoptimized + debuginfo] target(s) in 45.32s
```

Entonces estás listo para:

```powershell
cargo build --release
cargo run
```

---

## 📞 Soporte Rápido

| Problema | Solución |
|----------|----------|
| link.exe not found | Instalar VS Build Tools |
| Installer no descarga | Descargar manualmente desde aka.ms/vs/17/release/vs_BuildTools.exe |
| Sigue fallando | Usar "x64 Native Tools Command Prompt for VS 2022" |
| Necesito GUI | Instalar "Visual Studio 2022 Community" en lugar de Build Tools |

---

## ⏱️ Tiempo Total

```
Descarga:       5-10 min (depende internet)
Instalación:    10-15 min
Total:          15-25 minutos
```

**Luego**:
- Primera compilación: 3-5 min
- Compilaciones subsecuentes: 30-60 seg

---

## ✅ Siguiente Paso

Una vez que `cargo check` funciona sin errors:

```powershell
cargo build --release  # Compilación optimizada
```

Entonces:

```powershell
cargo run  # Ejecutar servidor
```

---

**Cuando link.exe esté disponible y `cargo check` funcione, el WebSocket migration está listo para testear.**

Adelante! 🚀
