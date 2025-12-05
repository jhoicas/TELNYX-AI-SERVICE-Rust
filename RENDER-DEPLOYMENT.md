# Deployment Guide - Render.com 🚀

## ⚡ Despliegue automático en Render.com

Este proyecto está configurado para desplegarse automáticamente en Render.com usando el archivo `render.yaml`.

---

## 📋 Requisitos previos

- Cuenta en [Render.com](https://render.com/) (prueba gratuita disponible)
- Repositorio en GitHub con este código
- Variables de entorno configuradas

---

## 🚀 Pasos para desplegar

### 1. Conectar repositorio GitHub a Render

1. Accede a https://dashboard.render.com/
2. Click en **"New +"** → **"Web Service"**
3. Selecciona **"Connect a repository"**
4. Busca tu repositorio `TELNYX-AI-SERVICE-Rust`
5. Click en **"Connect"**

### 2. Configurar el servicio

Completa los campos:

**Nombre del servicio:**
```
telnyx-ai-service
```

**Runtime:**
```
Rust
```

**Build Command:**
```
cargo build --release
```

**Start Command:**
```
./target/release/telnyx_ai_service
```

**Plan:**
```
Starter (free tier)
```

### 3. Agregar variables de entorno

Ve a **"Environment"** y agrega:

```
TELNYX_API_KEY = tu_api_key
TELNYX_CONNECTION_ID = tu_connection_id
TELNYX_PHONE_NUMBER = +1234567890
ANTHROPIC_API_KEY = sk-ant-...
WEBHOOK_BASE_URL = https://tu-servicio.onrender.com
NODE_ENV = production
RUST_LOG = telnyx_ai_service=info
AWS_REGION = us-east-1
AWS_ACCESS_KEY_ID = (opcional)
AWS_SECRET_ACCESS_KEY = (opcional)
S3_BUCKET = (opcional)
```

### 4. Deploy

Click en **"Create Web Service"**

Render comenzará el deploy automáticamente:
1. **Fetch del repositorio** (~30s)
2. **Cargo build** (~3-5 minutos)
3. **Start del servicio** (~10s)
4. **Health check** (automático)

---

## ✅ Verificar el deploy

Una vez completado:

```bash
# Health check
curl https://tu-servicio.onrender.com/api/health

# Verificar que está funcionando
curl https://tu-servicio.onrender.com/
```

Deberías ver:
```json
{
  "service": "Telnyx AI Service (Rust)",
  "version": "1.0.0",
  "status": "running",
  "endpoints": { ... }
}
```

---

## 🔄 Auto-deploy desde GitHub

Configurar deploy automático:

1. En Render Dashboard → Tu servicio
2. **"Settings"** → **"Auto-Deploy"**
3. Selecciona **"Deploy latest commit"**
4. Elige rama: `main`
5. Click **"Save"**

Ahora cada push a `main` desplegará automáticamente.

---

## 📊 Monitoreo

### Logs en tiempo real

```bash
# En Render Dashboard → Tu servicio → "Logs"
# O via CLI:
render logs -n 100
```

### Métricas

Render proporciona:
- CPU usage
- Memory usage
- Network I/O
- Response times
- Error rates

---

## 🆘 Troubleshooting

### Error: "Build failed - cargo not found"

**Solución:** Render soporta Rust automáticamente. Si el error persiste:
1. Verifica que `Cargo.toml` está en la raíz
2. Verifica que `src/main.rs` existe

### Error: "Health check failed"

**Solución:** 
1. Espera 30-60 segundos (el servidor toma tiempo en iniciar)
2. Verifica el endpoint `/api/health` está implementado ✓
3. Revisa logs para ver el error exacto

### Error: "TELNYX_API_KEY must be set"

**Solución:**
1. Ve a **Settings** → **Environment**
2. Verifica que `TELNYX_API_KEY` está definido
3. Click **"Save"** para reiniciar con nuevas variables

### El servicio se reinicia constantemente

**Solución:**
1. Revisa los logs para ver qué error hay
2. Asegúrate que todas las variables de entorno están definidas
3. Prueba localmente con `cargo run` primero

### Lentitud en el build

**Esperado:** El primer build toma 3-5 minutos (es normal en Rust)
- Los builds posteriores son más rápidos (~2 minutos)
- Usa `render logs` para ver el progreso

---

## 📈 Performance en Render

### Free tier (Starter)

- **CPU:** Shared
- **Memory:** 0.5GB RAM
- **Bandwidth:** Unlimited
- **Storage:** No persistente
- **Uptime:** 99.9%
- **Auto-sleep:** Sí (después 15 min inactividad)

Para aplicaciones de producción:
- Actualiza a **Professional** ($12/mes)
- Dedicated CPU y RAM
- Auto-sleep deshabilitado

### Estimación de capacidad (Starter)

```
Telnyx AI Service puede manejar:
- 50-100 llamadas concurrentes
- 100-200 requests/segundo
- Ideal para pequeño/mediano volumen
```

---

## 🔐 Seguridad

### Variables de entorno sensibles

Las variables marcadas con `sync: false` en `render.yaml` no se sincronizan desde git:
- TELNYX_API_KEY
- ANTHROPIC_API_KEY
- AWS_SECRET_ACCESS_KEY
- etc.

Debes ingresarlas manualmente en el dashboard.

### HTTPS automático

Render proporciona:
- ✅ HTTPS automático
- ✅ Certificado SSL/TLS
- ✅ Renovación automática
- ✅ Dominio: `tu-servicio.onrender.com`

---

## 🌐 Dominio personalizado

Para usar tu propio dominio:

1. Render Dashboard → Tu servicio → **Settings** → **Custom Domain**
2. Agrega tu dominio
3. En tu DNS provider, crea CNAME:
   ```
   CNAME -> tu-servicio.onrender.com
   ```
4. Espera 5-10 minutos para propagación

---

## 💾 Persistencia de datos

**NOTA:** Render Free tier no tiene storage persistente.

Para guardar datos:
- Usa AWS S3 (ya soportado en el código)
- Usa una base de datos remota
- O actualiza a plan con almacenamiento

---

## 🔄 Desplegar actualizaciones

### Desde GitHub

```bash
# Hacer cambios localmente
git add .
git commit -m "Fix: mejoras"
git push origin main

# Render desplegará automáticamente
```

### Manualmente desde Render

1. Dashboard → Tu servicio
2. **"Manual Deploy"** → **"Deploy latest"**

---

## 📊 Alternativas a Render

Si Render no funciona para ti:

| Plataforma | Ventajas | Costo |
|-----------|----------|-------|
| **Railway** | Más simple, Rust ready | Free + pagos |
| **Fly.io** | Global, optimizado | Free + pagos |
| **Heroku** | Tradicional | Pago solo |
| **AWS EC2** | Control total | Variable |
| **DigitalOcean** | Simple VPS | $5-12/mes |

---

## 🧪 Validar antes de desplegar

Antes de hacer push a GitHub:

```powershell
# Test local
cargo build --release
./target/release/telnyx_ai_service

# Verificar compilación
cargo check

# Tests
cargo test

# Lint
cargo clippy
```

---

## 📞 Obtener la URL del servicio

Una vez deployado:

```
https://telnyx-ai-service.onrender.com
```

(Reemplaza `telnyx-ai-service` con el nombre que elegiste)

Actualiza `WEBHOOK_BASE_URL` en variables de entorno con esta URL.

---

## 🎯 Configuración recomendada

Para producción en Render:

```yaml
Plan: Professional ($12/mes)
Instances: 1 (auto-scaling disponible)
Region: USA (se selecciona automáticamente)
Auto-deploy: Enabled
Health checks: Enabled
```

---

## 📚 Recursos adicionales

- [Documentación Render](https://render.com/docs)
- [Render Rust Guide](https://render.com/docs/deploy-rust)
- [Guía de YAML de Render](https://render.com/docs/yaml-spec)

---

## ✅ Checklist de deploy

- [ ] Repositorio está en GitHub
- [ ] `render.yaml` está en la raíz
- [ ] `Cargo.toml` está en la raíz
- [ ] Variables de entorno están listas
- [ ] Webhook URL será: `https://tu-servicio.onrender.com/webhook/telnyx`
- [ ] Test local con `cargo run`
- [ ] Push a GitHub
- [ ] Conectar a Render
- [ ] Configurar variables
- [ ] Deploy
- [ ] Verificar salud con `/api/health`

---

**Versión:** 1.0.0  
**Última actualización:** Diciembre 2025  
**Status:** ✅ Ready for Render deployment
