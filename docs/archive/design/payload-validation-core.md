---
feature: payload-validation-core
status: archived
archive-after:
touches:
  - src/**
  - Cargo.toml
release: 0.1.0
---

# Tech Spec — Multi-Format JSON-Schema Payload Validator for WASI Preview 2 (JSON, CSV, XLSX) (`payload-validation-core`)

Setzt das PRD `docs/prd/payload-validation-core.md` um. Die Verfassung hat Vorrang; diese Tech Spec darf sie präzisieren, nie lockern.

## Architektur

Das System folgt der strikten Schichtentrennung:
1. `src/formats`: Ingestion-Parser für `JSON`, `CSV` (über Crate `csv`) und `XLSX` (über Crate `calamine`). Wandelt Eingabedatenströme in standardisierte `serde_json::Value`-Strukturen um.
2. `src/schema`: Kompiliert Schemas via `jsonschema` (ohne HTTP-/File-Resolver in WASM) und führt Validierungen durch.
3. `src/report`: Kapselt Fehler und formatiert diese wahlweise als Terminal-Zusammenfassung oder standardisiertes JSON-Objekt (`ValidationReport`).
4. `src/cli`: CLI-Schnittstelle mittels `clap` für Ein-/Ausgabe, Format-Erkennung und Exit-Code-Steuerung.

## Schnittstellen

### CLI

```text
wasm_json_validator [OPTIONS] --schema <SCHEMA>

Optionen:
  -s, --schema <SCHEMA>   Pfad oder Inline-JSON-Schema
  -i, --input <INPUT>     Dateipfad oder '-' für stdin [Standard: -]
  -f, --format <FORMAT>   Eingabeformat: json, csv, xlsx [Standard: automatisch]
      --output <OUTPUT>   Ausgabeformat: text, json [Standard: text]
```

## Datenmodell

- `InputFormat`: Enum `Json | Csv | Xlsx`.
- `ValidationErrorItem`: Enthält `instance_path` (JSON Pointer), `schema_path` und Fehlermeldung.
- `ValidationReport`: Enthält `valid` (bool), `error_count` (usize) und `errors: Vec<ValidationErrorItem>`.

## Umsetzung der Akzeptanzkriterien

| AC | Umsetzung | Test |
|---|---|---|
| AC-1 | `src/schema/mod.rs` & `src/formats/mod.rs` (JSON) | `schema_test::test_schema_validation_success` |
| AC-2 | `src/formats/mod.rs` (CSV Parsing) | `formats_test::test_parse_csv` |
| AC-3 | `src/formats/mod.rs` (XLSX Extraction) | `formats_test::test_parse_xlsx` |
| AC-4 | `src/report/mod.rs` & `src/main.rs` | `schema_test::test_schema_validation_failure` |
| AC-5 | `Cargo.toml` (`wasm32-wasip2`) | `cargo check --target wasm32-wasip2` |

## Sicherheit

- **Fail-Closed:** Ungültige oder nicht parsbare Payloads terminieren mit Exit-Code 1 bzw. 2.
- **Ressourcenschutz:** Keine unbegrenzte Rekursion; Parsing nutzt streaming-fähige bzw. speicherbegrenzte Reader.

## Offene Punkte

- Keine offenen Punkte für den Kern; Spin-HTTP-Gateway wird in Issue #2 umgesetzt.
