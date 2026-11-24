---
feature: spin-http-gateway
status: draft
archive-after:
touches:
  - src/**
  - Cargo.toml
  - spin/**
  - spin.toml
---

# Tech Spec — Spin Inbound HTTP Interceptor & Payload Validation Gateway (`spin-http-gateway`)

Setzt das PRD `docs/prd/spin-http-gateway.md` um. Die Verfassung hat Vorrang; diese Tech Spec darf sie präzisieren, nie lockern.

## Architektur

Die HTTP-Gateway-Schicht erweitert das System um eine Präsentationskomponente:
1. `src/provider/http`: Reine zustandslose Funktion `handle_http_request(method, path, query, headers, body) -> HttpResponse`, entkoppelt von der Laufzeitumgebung, vollständig unit-testbar ohne Netzwerk-Sockets.
2. `src/bin/wasm_json_validator_spin.rs`: Spin/WASI HTTP-Entrypoint, der CGI/Spin-Umgebungsvariablen (`REQUEST_METHOD`, `PATH_INFO`, `QUERY_STRING`, Header) ausliest und den Response nach `stdout` serialisiert.
3. `spin.toml`: Spin Manifest v2 zur Konfiguration der HTTP-Komponente und Trigger-Routen.

## Schnittstellen

### HTTP API

```http
POST /validate?schema=examples/user_schema.json HTTP/1.1
Content-Type: application/json

{"name": "Alice", "age": 30}
```

Antwort (Erfolg):
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "valid": true,
  "status": "Payload valid"
}
```

Antwort (Fehler):
```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "valid": false,
  "error_count": 1,
  "errors": [
    {
      "instance_path": "/age",
      "schema_path": "/properties/age/minimum",
      "message": "-10 is less than the minimum of 0"
    }
  ]
}
```

## Datenmodell

- `HttpResponse`:
  - `status`: u16 (200, 400, 405)
  - `headers`: Vec<(String, String)>
  - `body`: String (JSON)

## Umsetzung der Akzeptanzkriterien

| AC | Umsetzung | Test |
|---|---|---|
| AC-1 | `src/provider/http.rs` Schema- & Header-Parsing | `provider_test::test_http_post_valid_payload` |
| AC-2 | `src/provider/http.rs` 200 OK Response | `provider_test::test_http_post_valid_payload` |
| AC-3 | `src/provider/http.rs` 400 Bad Request mit Diagnose | `provider_test::test_http_post_invalid_payload` |
| AC-4 | `src/provider/http.rs` CSV/XLSX Dispatch | `provider_test::test_http_csv_payload` |

## Sicherheit

- **Fail-Closed:** Unbekannte oder fehlende Schemata führen unmittelbar zu HTTP 400.
- **Ressourcenbegrenzung:** Requests mit unzulässiger HTTP-Methode werden mit HTTP 405 abgelehnt.
