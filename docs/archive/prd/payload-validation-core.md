---
feature: payload-validation-core
status: archived
archive-after:
touches:
  - src/**
  - Cargo.toml
release: 0.1.0
---

# PRD — Multi-Format JSON-Schema Payload Validator for WASI Preview 2 (JSON, CSV, XLSX) (`payload-validation-core`)

## Problem

Eingehende Datenströme und Datei-Uploads (JSON, CSV, XLSX) erreichen häufig nachgelagerte Datenbanken oder ressourcenintensive Machine-Learning-Pipelines ohne vorherige, strikte Schemavalidierung. Fehlerhafte, korrupte oder unvollständige Payloads führen zu Schema-Drift, Runtime-Fehlern in ML-Inferenzdiensten und unnötigem Ressourcenverbrauch.

## Ziel

Bereitstellung eines schnellen, ressourcenschonenden und deterministischen Payload-Validators auf Basis von WASM / WASI Preview 2. Der Validator prüft Eingabedaten (JSON, CSV, XLSX) gegen ein via CLI oder Parameter spezifiziertes JSON-Schema (Draft 7 / 2020-12), bevor Daten weitergeleitet oder gespeichert werden.

## Nutzer

- Entwickler und Architekten von Data-Pipelines, ML-Ingestion-Gateways und Microservices.
- Edge- und Cloud-Gateway-Komponenten, die Daten noch vor dem Eintritt in interne Services filtern.

## Umfang

- Unterstützung von JSON-Eingabedateien oder stdin.
- Unterstützung von CSV-Dateien mit Header-Erkennung und Typeninferenz.
- Unterstützung von XLSX-Dateien mit Tabellenblatt-Auswertung.
- Ausgabe strukturierter Validierungsberichte (Textuell & JSON) mit genauen JSON-Pointer-Pfaden.
- Fehlerfreie Ausführung sowohl nativ als auch unter WASI Preview 2 (`wasm32-wasip2`).

## Nicht-Ziele

- Persistenz von hochgeladenen Dateien (der Validator ist zustandslos / stateless).
- Modifikation oder Reparatur von fehlerhaften Payloads (nur Validierung und Reporting).

## Akzeptanzkriterien

- **AC-1** Das System validiert JSON-Payloads gegen ein übergebenes JSON-Schema via `--schema` und `--input`.
- **AC-2** Das System parst CSV-Dateien und validiert die tabellarischen Datensätze gegen das JSON-Schema.
- **AC-3** Das System liest XLSX-Arbeitsblätter und validiert die extrahierten Zeilen gegen das JSON-Schema.
- **AC-4** Bei Validierungsfehlern erzeugt das System strukturierte Berichte mit JSON-Pointern und liefert Exit-Code 1.
- **AC-5** Das System kompiliert warnungsfrei für `wasm32-wasip2` und native Architekturen.
