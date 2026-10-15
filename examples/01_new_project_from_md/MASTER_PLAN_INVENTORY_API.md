# Plan Maestro: Microservicio de Inventario Farmacéutico (`PharmaStock-API`)

## 1. Objetivo y Alcance
Desarrollar un servicio REST ligero en Python (FastAPI + SQLite) para controlar entradas, salidas y alertas de caducidad de medicamentos en una clínica local.

## 2. Requisitos Funcionales
- **RF-01:** Endpoint `POST /items` para registrar lotes de medicamentos con código, nombre, cantidad y fecha de caducidad.
- **RF-02:** Endpoint `GET /items/expiring?days=30` que devuelva los lotes próximos a caducar.
- **RF-03:** Validación estricta que rechace cantidades negativas con HTTP 422.

## 3. Fases y Criterios de Aceptación
- **Fase 1:** Modelado de datos SQLite y esquemas Pydantic.
- **Fase 2:** Endpoints FastAPI y manejo de errores HTTP 422.
- **Fase 3:** Suite de pruebas automatizadas con `pytest` (100 % PASS).
