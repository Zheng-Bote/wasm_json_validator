# wasm_json_validator (Deutsch)

Schneller, ressourcenschonender JSON-Schema-Payload-Validator für **WASM/WASI** und optional **Spin**. Validiert Payloads gegen hinterlegte Schemas, bevor Requests teure Datenbanken oder Machine-Learning-Dienste erreichen.

## Kernmerkmale

- **Multi-Format-Ingestion:** Direkte Validierung von `JSON`, tabellarischem `CSV` und Excel-Dateien (`XLSX`) gegen ein zentrales JSON-Schema.
- **WASM / WASI Preview 2:** Kompiliert für `wasm32-wasip2` und native Systeme mit minimalen Kaltstartzeiten.
- **Fail-Closed & DoS-geschützt:** Weist ungültige oder schädliche Payloads (z. B. Decompressions- oder Rekursionsbomben) frühzeitig ab.
- **Spec-Driven Architecture:** Verwaltet über `sdd` mit Schichtenarchitektur (`src/formats`, `src/schema`, `src/report`, `src/cli`, `spin`).
- **Optionale Spin-HTTP-Integration:** Prüfung von Inbound-HTTP-Requests direkt am Gateway.

## Schnelleinstieg

```bash
# Nativ bauen
cargo build --release

# Für WASI Preview 2 bauen
cargo build --release --target wasm32-wasip2
```

## Lizenz

Lizenziert unter der Apache License, Version 2.0.
