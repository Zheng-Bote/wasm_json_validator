---
feature: payload-validation-core
status: draft
archive-after:
---

# Plan — Multi-Format JSON-Schema Payload Validator for WASI Preview 2 (JSON, CSV, XLSX) (`payload-validation-core`)

Reihenfolge der Umsetzung. Jede Task nennt das Akzeptanzkriterium, auf das sie einzahlt.

## Tasks

- [x] AC-1 JSON Schema Kompilierung und Validierung von JSON Payloads (`src/schema/`, `src/formats/`)
- [x] AC-2 CSV Parser mit Typeninferenz und Tabellen-Konvertierung (`src/formats/mod.rs`)
- [x] AC-3 XLSX Tabellenblatt-Extraktion über Calamine (`src/formats/mod.rs`)
- [x] AC-4 Strukturierte Fehlerdiagnose und JSON-Pointer Berichterstattung (`src/report/mod.rs`)
- [x] AC-5 WASI Preview 2 (`wasm32-wasip2`) und native Kompilierungsprüfung
- [x] AC-1..5 Automatisierte Tests mit SDD-Attributierung und SDD-Lint-Verifikation
