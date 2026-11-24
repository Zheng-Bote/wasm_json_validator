---
feature: spin-http-gateway
status: archived
archive-after:
touches:
  - src/**
  - Cargo.toml
  - spin/**
  - spin.toml
release: 0.1.0
---

# PRD — Spin Inbound HTTP Interceptor & Payload Validation Gateway (`spin-http-gateway`)

## Problem

In Microservice- und Cloud-Architekturen erreichen ungeprüfte Datenströme (über HTTP REST Endpunkte) teure Backend-Datenbanken oder ressourcenintensive Machine-Learning-Inferenzdienste. Ungültige Payloads verschwenden Rechenleistung und verursachen Fehler in nachgelagerten Systemen.

## Ziel

Bereitstellung einer serverlosen HTTP-Gateway-Komponente auf Basis von Fermyon Spin und WASI Preview 2. Eingehende HTTP `POST`/`PUT`-Anfragen werden anhand eines per Query-Parameter (`?schema=...`) oder Header (`X-Schema-ID`) spezifizierten JSON-Schemas validiert, bevor sie Downstream-Sinks erreichen.

## Nutzer

- API-Gateway-Betreiber und Cloud-Architekten.
- Data Engineers und ML-Entwickler, die Inferenz- und Trainingsdaten vor dem Ingestion-Prozess filtern wollen.

## Umfang

- HTTP-Endpunkt (`/validate`, `/v1/validate` oder Root-Trigger in Spin).
- Schema-Auswahl über URL-Query-Parameter (`?schema=...`) oder HTTP-Header `X-Schema-ID`.
- Format-Erkennung über `Content-Type` oder `?format=` (JSON, CSV, XLSX).
- HTTP-Antwortcodes:
  - `200 OK` bei erfolgreicher Schemavalidierung.
  - `400 Bad Request` mit strukturiertem JSON-Fehlerbericht bei Validierungs- oder Parserfehlern.
  - `405 Method Not Allowed` bei Nicht-POST/PUT-Methoden.
- Spin-Manifest `spin.toml` und Binär-Einstiegspunkt für Spin.

## Nicht-Ziele

- Dauerhafte Datenbankspeicherung validierter Payloads (zustandsloses Gateway).
- Komplexe Authentifizierungsprovider (diese können vorgelagert oder in einem separaten Gateway betrieben werden).

## Akzeptanzkriterien

- **AC-1** Die Spin-HTTP-Komponente akzeptiert `POST`-Anfragen mit Payloads und Schema-Spezifikation (`?schema=` oder `X-Schema-ID`).
- **AC-2** Gültige Anfragen gegen das Schema beantwortet das Gateway mit HTTP `200 OK` und JSON-Bestätigung.
- **AC-3** Ungültige Anfragen oder Parserfehler werden mit HTTP `400 Bad Request` und detailliertem Diagnose-JSON abgewiesen.
- **AC-4** Das Gateway unterstützt CSV- und XLSX-Payloads basierend auf `Content-Type` oder Format-Parameter.
