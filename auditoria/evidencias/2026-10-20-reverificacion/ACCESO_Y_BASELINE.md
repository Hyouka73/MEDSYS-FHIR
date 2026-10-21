# Inicio de reverificación — 20 de octubre de 2026 (America/Mexico_City)

- Commit local inicial: `18b434a692f8ab28a400f6ccab64c5d52e6bc183`; `git status --porcelain` vacío; ninguna etiqueta devuelta por `git tag --list`.
- Estado histórico STATE/BACKLOG afirma cierre 40/40; no constituye evidencia de CA-1–CA-5. La solicitud actual reabre su verificación experimental sin borrar antecedentes.
- Google Docs: lectura efectiva de registro, paquete de prompts y capítulos III/IV. Herramienta batch_update_document disponible; escritura todavía no ejecutada ni afirmada. Las fuentes se conservan en archivos *-fuente.txt de esta carpeta.
- Windows 11 Home Single Language 10.0.26200; PowerShell 7.6.6; RAM visible 7,985,160 KiB. Cargo/rustc 1.98.1; Python 3.13.1; Microsoft OpenJDK 21.0.10.
- Docker CLI 27.4.0 instalado, contexto desktop-linux; al inicio motor ausente (pipe dockerDesktopLinuxEngine no encontrado). Se intentó iniciar Docker Desktop sin ventana visible; pendiente diagnóstico.
- Ejecutable target/debug/medsys-server.exe preexistente, no se presume corresponde al commit actual. No es identidad congelada.
- No se crea etiqueta v1.2.0-eval, no se publica ni se hace push.

## Umbrales conservados del capítulo III

- CA-1: cero Fatal/Error en 16,250 recursos reales por esquema.
- CA-2: cero discrepancias en atributos declarados, reporte de las veinte mutaciones separado de la corrección de Rust.
- CA-3: 100% rechazo HTTP 422 OperationOutcome del corpus S0–S3 y 0% falsos rechazos del censo válido.
- CA-4: cero cambios compilados después del congelamiento identificado. Las correcciones actuales preceden cualquier nuevo congelamiento; no acreditan el congelamiento histórico ausente.
- CA-5: cobertura completa S0–S3 en el alcance declarado; body_weight adicional de S3 tiene límite de despacho HTTP.
- CT descriptiva: tres réplicas, 90 s rampa + 300 s meseta a 50 VU + 30 s descenso; pausas 0.1–0.3 s; referencia p95 ≤200 ms, éxito ≥99.5%, CPU p95 ≤50% de 1 vCPU, RAM p95 ≤150 MiB. Series conjuntas requieren muestras alineadas.
- Dictamen: regla original Tabla 3.1; evidencia faltante no es aprobación. Pendiente institucional ético requiere determinación de instancia académica, no se fabrica autorización.
