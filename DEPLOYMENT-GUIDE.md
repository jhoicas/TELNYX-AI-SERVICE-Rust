# 🚀 GUÍA COMPLETA DE DEPLOYMENT

Guía paso a paso para desplegar tu servicio Telnyx AI en Render.com con CI/CD automático.

---

## 📋 Índice rápido

1. [Opción A: Deploy manual en Render (5 minutos)](#opción-a-deploy-manual)
2. [Opción B: Deploy automático con GitHub (15 minutos)](#opción-b-deploy-automático)
3. [Verificación y Monitoreo](#verificación-y-monitoreo)
4. [Troubleshooting](#troubleshooting)

---

## Opción A: Deploy manual

### ⏱️ Tiempo: 5 minutos

**Para pruebas rápidas o si prefieres no usar CI/CD.**

### Pasos

1. **Crear cuenta en Render** (si no la tienes)
   ```
   https://render.com/ → Sign up
   ```

2. **Conectar repositorio GitHub**
   - Click **"New +"** → **"Web Service"**
   - **"Connect a repository"**
   - Selecciona `TELNYX-AI-SERVICE-Rust`

3. **Configurar servicio**
   ```
   Name: telnyx-ai-service
   Runtime: Rust
   Build command: cargo build --release
   Start command: ./target/release/telnyx_ai_service
   Plan: Starter (free)
   ```

4. **Agregar variables de entorno**
   ```
   TELNYX_API_KEY = tu_key
   TELNYX_CONNECTION_ID = tu_id
   TELNYX_PHONE_NUMBER = +1234567890
   ANTHROPIC_API_KEY = sk-ant-...
   WEBHOOK_BASE_URL = (lo verás después de crear)
   NODE_ENV = production
   RUST_LOG = telnyx_ai_service=info
   ```

5. **Crear servicio**
   - Click **"Create Web Service"**
   - Espera 5-10 minutos
   - Verifica en `/api/health`

6. **Actualizar WEBHOOK_BASE_URL**
   - Una vez que tengas la URL (ej: `https://telnyx-ai-service.onrender.com`)
   - Settings → Environment → Edita `WEBHOOK_BASE_URL`
   - Agrega: `https://telnyx-ai-service.onrender.com`

---

## Opción B: Deploy automático

### ⏱️ Tiempo: 15 minutos

**Setup una sola vez, luego todos los deploys son automáticos.**

### Ventajas
✅ Cada push a `main` deploya automáticamente  
✅ Tests ejecutan antes de desplegar  
✅ Automatic rollback si hay problemas  
✅ Historial completo en GitHub Actions  

### Pasos

#### Paso 1: Crear servicio en Render
```
1. https://dashboard.render.com/
2. New → Web Service
3. Busca tu repositorio TELNYX-AI-SERVICE-Rust
4. Configurar igual que Opción A
```

#### Paso 2: Obtener Deploy Hook
```
1. Dashboard → Tu servicio
2. Settings → Deploy Hook
3. Copiar URL (ej: https://api.render.com/deploy/srv-xxx?key=yyy)
```

#### Paso 3: Agregar Secret a GitHub
```
1. GitHub → Tu repositorio
2. Settings → Secrets and variables → Actions
3. New repository secret
4. Nombre: RENDER_DEPLOY_HOOK
5. Valor: (pega el URL del paso anterior)
6. Add secret
```

#### Paso 4: Commit y Push
```powershell
# Los archivos ya existen (render.yaml, .github/workflows/*)
git add .
git commit -m "Enable: CI/CD deployment"
git push origin main
```

#### Paso 5: Verificar
```
1. GitHub → Actions
   → Ves que tests ejecutan automáticamente
   
2. Render → Deploy Logs
   → Ves que automáticamente descarga y deploya
```

---

## Verificación y Monitoreo

### Health Check
```bash
curl https://tu-servicio.onrender.com/api/health

# Respuesta esperada:
# {
#   "status": "healthy",
#   "timestamp": "2025-12-05T..."
# }
```

### Ver Logs en Render
```
Dashboard → Tu servicio → Logs
(puedes ver logs en tiempo real)
```

### Ver Logs en GitHub
```
Actions → Tu workflow → Paso
(puedes ver detalles de cada paso del build)
```

---

## Flujo de trabajo típico

```
Desarrollo local:
  git checkout -b feature/nueva-feature
  # Hacer cambios
  cargo test  # Verificar localmente
  
Push a GitHub:
  git push origin feature/nueva-feature
  # GitHub Actions automáticamente ejecuta tests
  
Si tests pasan:
  Hacer Pull Request
  # Revisar cambios
  
Merge a main:
  git merge feature/nueva-feature
  # GitHub Actions notifica a Render
  
Render descarga y deploya:
  Dashboard muestra deploy en progreso
  En 5-10 minutos está en producción
```

---

## 🔍 Troubleshooting

### El servicio se reinicia constantemente

**Síntomas:** Logs muestran error y reinicia cada 10s

**Causas comunes:**
1. Variable de entorno faltante
2. Error en el código

**Solución:**
```
1. Dashboard → Logs
2. Lee el error exacto
3. Settings → Environment → Verifica todas las variables
4. Si es código: 
   - cargo test localmente
   - Fix el error
   - git push (auto-deploya)
```

### Build tarda mucho (timeout)

**Normal:** Primer build toma 5-10 minutos (Rust es lento)

**Si pasa mucho tiempo:**
```
1. Ver logs: Dashboard → Build Logs
2. Si está compilando: Espera, es normal
3. Si está atorado: Cancela y reinicia
   → Dashboard → Deployments → Cancel
   → Manual Deploy → Deploy latest
```

### Health check falla

**Síntomas:** Servicio se reinicia, health check rojo

**Solución:**
```
1. Espera 60s (startup toma tiempo)
2. Verifica que /api/health está en código
3. Revisa logs para ver error exacto
4. Si está correcto: Render a veces tarda
5. Si persiste: Verifica variables de entorno
```

### No puedo conectar a la API

**Síntomas:** `curl` da error de conexión

**Causas:**
1. Servicio aún está deployando
2. Dominio no está propagado
3. Firewall/CORS issue

**Solución:**
```
1. Espera 5-10 minutos desde crear
2. Verifica URL en Dashboard (no inventes)
3. Prueba con curl:
   curl -v https://tu-servicio.onrender.com/
4. Si da error CORS: Normal (proviene de otra URL)
```

---

## 📊 Monitoreo avanzado

### Ver logs en tiempo real
```powershell
# Opción 1: Dashboard (más fácil)
https://dashboard.render.com/
# Selecciona servicio → Logs

# Opción 2: Render CLI (si lo tienes)
render logs -n 100 --service telnyx-ai-service
```

### Ver métricas
```
Dashboard → Tu servicio → Metrics
- CPU usage
- Memory usage
- Network I/O
- Response times
```

### Alertas
En plan Professional:
- Puedes configurar Slack alerts
- Email notifications
- Custom webhooks

---

## 🔐 Actualizar variables de entorno

Si necesitas cambiar una variable:

```
1. Dashboard → Settings → Environment
2. Edita la variable
3. Click Save
4. Render automáticamente reinicia con nueva config
```

**NOTA:** Algunos cambios requieren rebuild:
```
1. Dashboard → Manual Deploy → Deploy latest
2. O: git push con cambios dummy
```

---

## 🆘 Emergencias

### Rollback a versión anterior

Si algo se rompió:

```
1. Dashboard → Deployments
2. Haz click en el deploy anterior
3. Click Redeploy
```

Vuelve a la versión anterior en ~30 segundos.

### Deshabilitar auto-deploy

Si quieres cambios sin desplegar:

```
1. Dashboard → Settings
2. Auto-Deploy → Disable
3. Haz cambios, push
4. Cuando estés listo:
   Dashboard → Manual Deploy → Deploy latest
```

---

## 📈 Optimizaciones recomendadas

### Para producción

Actualiza plan a **Professional** ($12/mes):
- ✅ CPU dedicada
- ✅ RAM dedicada
- ✅ Sin auto-sleep
- ✅ SLA de 99.9% uptime
- ✅ Priority support

### Mejorar performance
```yaml
# En render.yaml
numInstances: 2  # Más de 1 instancia
plan: professional  # Mejor hardware
```

---

## 🌐 Dominio personalizado

Para usar `llamadas.tuempresa.com` en lugar de `tu-servicio.onrender.com`:

```
1. Dashboard → Settings → Custom Domains
2. Add custom domain
3. Sigue instrucciones para CNAME
4. Espera 5-10 min para DNS propagación
```

---

## 📚 Documentación adicional

- [RENDER-DEPLOYMENT.md](RENDER-DEPLOYMENT.md) - Guía detallada de Render
- [GITHUB-WORKFLOWS.md](GITHUB-WORKFLOWS.md) - Guía de CI/CD
- [README.md](README.md) - Documentación general

---

## ✅ Checklist antes de desplegar

- [ ] Variables de entorno están listas
- [ ] `cargo test` pasa localmente
- [ ] Repositorio está en GitHub
- [ ] `render.yaml` está en raíz
- [ ] `.github/workflows/` existen
- [ ] .env.example tiene todas las variables
- [ ] WEBHOOK_BASE_URL será la URL de Render

---

## 🎯 Resumen rápido

### Opción A (Manual - 5 min)
1. Render Dashboard → New Web Service
2. Seleccionar repositorio
3. Configurar variables
4. Create

### Opción B (Automático - 15 min)
1. Igual a Opción A
2. Obtener Deploy Hook
3. Agregar Secret a GitHub
4. git push → Render deploya automáticamente

---

## 📞 Soporte

- **Render:** https://render.com/docs
- **GitHub Actions:** https://docs.github.com/en/actions
- **Este proyecto:** Ver README.md o QUICKSTART.md

---

**Versión:** 1.0.0  
**Última actualización:** Diciembre 2025  
**Status:** ✅ Ready for deployment
