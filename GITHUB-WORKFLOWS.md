# GitHub Workflows Setup 🚀

## Configuración de CI/CD automático

Este proyecto incluye workflows de GitHub Actions para:
- Validación automática de código
- Tests automáticos
- Deploy automático a Render

---

## 📋 Workflows incluidos

### 1. `test.yml` - Pruebas automáticas

Se ejecuta en cada **push** y **pull request**.

**Incluye:**
- ✅ Validación de formato (cargo fmt)
- ✅ Linter (clippy)
- ✅ Build
- ✅ Tests
- ✅ Auditoría de vulnerabilidades
- ✅ Chequeo de dependencias desactualizadas

**Duración:** ~5-7 minutos

### 2. `deploy.yml` - Deployment automático

Se ejecuta después de merge a `main`.

**Incluye:**
- ✅ Formato + Linter
- ✅ Tests
- ✅ Build release
- ✅ Notificación a Render

---

## 🔧 Configuración inicial

### 1. Generar Deploy Hook en Render

En Render Dashboard:
1. Tu servicio → **Settings**
2. **Deploy Hook**
3. Copiar la URL

### 2. Agregar Secret en GitHub

1. Ir a tu repositorio GitHub
2. **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Nombre: `RENDER_DEPLOY_HOOK`
5. Valor: (pega la URL de Render)
6. Click **Add secret**

---

## ✅ Verificación automática

### Cada PR verifica:

```
✓ Código formateado correctamente
✓ Clippy warnings (no hay)
✓ Tests pasan
✓ Build exitoso
✓ Sin vulnerabilidades conocidas
```

Si algo falla, el PR mostrará rojo 🔴 y no puedes hacer merge.

---

## 🔄 Flujo de trabajo recomendado

```
1. Crear rama feature
   git checkout -b feature/nueva-feature

2. Hacer cambios
   vim src/main.rs

3. Commit y push
   git add .
   git commit -m "Add: nueva feature"
   git push origin feature/nueva-feature

4. Abrir PR
   GitHub detecta cambios → Ejecuta tests automáticamente

5. Resolver si hay problemas
   Hacer cambios → Push → Tests ejecutan de nuevo

6. Merge a main
   GitHub Actions ejecuta deploy.yml → Deploy a Render

7. Render recibe notificación
   Descarga código → Build → Deploy
```

---

## 📊 Dashboard de Actions

Ver estado en GitHub:

1. Tu repositorio → **Actions**
2. Ves historial de ejecutiones
3. Click en uno para ver detalles
4. Ver logs de cada paso

---

## 🚨 Troubleshooting

### Tests fallan pero funciona localmente

1. Ejecuta localmente con `cargo test`
2. A veces diferencias entre OS (Windows vs Linux)
3. Los workflows corren en Linux, asegúrate compatibilidad

### Deploy no se ejecuta después de merge

1. Verifica que el archivo es `.github/workflows/deploy.yml` (no `.github/workflow/`)
2. Verifica que la rama es `main` o `master`
3. Verifica que el Secret `RENDER_DEPLOY_HOOK` está configurado

### Auditoría tarda mucho

La auditoría (`cargo audit`) puede ser lenta. Es opcional, así que puede removerse si molesta.

---

## ⚡ Optimizaciones

Para acelerar CI/CD:

### 1. Caché mejorado (ya incluido)
- Cachea cargo registry
- Cachea compilaciones
- Ahorra 2-3 minutos

### 2. Compilación paralela
```yaml
- name: Build
  run: cargo build -j 2  # Limita a 2 jobs
```

### 3. Tests selectivos
```yaml
- name: Run tests
  run: cargo test --lib  # Solo unit tests, no integration
```

---

## 📝 Personalizaciones comunes

### Ejecutar solo en cambios específicos

```yaml
on:
  push:
    paths:
      - 'src/**'
      - 'Cargo.toml'
```

### Agregar más branches

```yaml
branches:
  - main
  - develop
  - staging
```

### Notificaciones de Slack

```yaml
- name: Notify Slack
  uses: slackapi/slack-github-action@v1
  with:
    webhook-url: ${{ secrets.SLACK_WEBHOOK }}
```

---

## 🔍 Ver logs detallados

En GitHub Actions:

1. **Actions** → Tu workflow
2. Selecciona el run
3. Expande cada step para ver logs
4. Busca sección que falló
5. Lee error mensaje

---

## 📈 Monitoreo

### Badges para README

Agrega a tu README.md:

```markdown
![Tests](https://github.com/tu-usuario/tu-repo/actions/workflows/test.yml/badge.svg)
```

---

## 🎯 Próximos pasos

1. ✅ Crear Secret `RENDER_DEPLOY_HOOK` en GitHub
2. ✅ Hacer un test push a una rama
3. ✅ Ver que los tests ejecutan en **Actions**
4. ✅ Hacer merge a main
5. ✅ Ver que deploy se ejecuta automáticamente
6. ✅ Verificar en Render que está actualizado

---

## 📚 Recursos

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://github.com/dtolnay/rust-toolchain)
- [Caching in Actions](https://docs.github.com/en/actions/using-workflows/caching-dependencies-to-speed-up-workflows)

---

**Última actualización:** Diciembre 2025  
**Status:** ✅ Ready to use
