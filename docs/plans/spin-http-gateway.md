---
feature: spin-http-gateway
status: draft
archive-after:
---

# Plan — Spin Inbound HTTP Interceptor & Payload Validation Gateway (`spin-http-gateway`)

Reihenfolge der Umsetzung. Jede Task nennt das Akzeptanzkriterium, auf das sie einzahlt.

## Tasks

- [x] AC-1 HTTP Request Dispatcher und Schema-Extraktion (`src/provider/http.rs`)
- [x] AC-2 Erfolgreiche Validierungsantwort mit HTTP 200 OK (`src/provider/http.rs`)
- [x] AC-3 Ablehnung ungültiger Payloads mit HTTP 400 Bad Request & Diagnose (`src/provider/http.rs`)
- [x] AC-4 Unterstützung von Multi-Format Ingestion (CSV/XLSX) über Content-Type (`src/provider/http.rs`)
- [x] AC-1..4 Spin Manifest `spin.toml` und Entrypoint `src/bin/wasm_json_validator_spin.rs`
- [x] AC-1..4 Automatisierte Tests mit SDD-Attributierung und Gate-Prüfungen
