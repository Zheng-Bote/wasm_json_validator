# wasm_json_validator (Deutsch)

Schneller, ressourcenschonender JSON-Schema-Payload-Validator für **WASM/WASI** und optional **Spin**. Validiert Payloads gegen hinterlegte Schemas, bevor Requests teure Datenbanken oder Machine-Learning-Dienste erreichen.

## Kernmerkmale

- **Multi-Format-Ingestion:** Direkte Validierung von `JSON`, tabellarischem `CSV` und Excel-Dateien (`XLSX`) gegen ein zentrales JSON-Schema.
- **WASM / WASI Preview 2:** Kompiliert für `wasm32-wasip2` und native Systeme mit minimalen Kaltstartzeiten.
- **Fail-Closed & DoS-geschützt:** Weist ungültige oder schädliche Payloads (z. B. Decompressions- oder Rekursionsbomben) frühzeitig ab.
- **Spec-Driven Architecture:** Verwaltet über `sdd` mit Schichtenarchitektur (`src/formats`, `src/schema`, `src/report`, `src/cli`, `spin`).
- **Optionale Spin-HTTP-Integration:** Prüfung von Inbound-HTTP-Requests direkt am Gateway.

### CLI-Nutzung

```bash
# JSON gegen Schema validieren
wasm_json_validator --schema schema.json --input data.json

# CSV-Datei validieren
wasm_json_validator --schema schema.json --input data.csv --format csv

# Excel XLSX-Datei validieren
wasm_json_validator --schema schema.json --input records.xlsx --format xlsx

# JSON-Fehlerdiagnose ausgeben
wasm_json_validator --schema schema.json --input data.json --output json
```

### Spin HTTP-Gateway Nutzung

Starten des serverlosen HTTP-Payload-Gateways mit Fermyon Spin:

```bash
# Spin-Komponente bauen
spin build

# Lokalen Spin-Server starten
spin up

# JSON-Payload per HTTP POST validieren (liefert 200 OK)
curl -i -X POST "http://localhost:3000/validate?schema=examples/user_schema.json" \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "age": 30}'

# Ungültigen Payload prüfen (liefert 400 Bad Request mit Diagnose)
curl -i -X POST "http://localhost:3000/validate?schema=examples/user_schema.json" \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob", "age": -5}'
```

## Lizenz

Lizenziert unter der Apache License, Version 2.0.
