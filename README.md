# wasm_json_validator

Fast, low-latency JSON Schema payload validator designed for **WASM/WASI** and optionally **Spin**, intercepting and validating data payloads before they hit downstream databases or Machine Learning services.

## Highlights

- **Multi-Format Ingestion**: Validate raw `JSON`, tabular `CSV`, or Excel `XLSX` against a unified JSON Schema.
- **WASM / WASI Preview 2**: Compiles to `wasm32-wasip2` and native targets with near-zero cold-start latency.
- **Fail-Closed & DoS-Resilient**: Guards against payload bombs, deep recursion, and malformed files before reaching expensive sinks.
- **Spec-Driven Architecture**: Governed by `sdd` with strict layer separation and automated gates.
- **Optional Spin Gateway**: Intercept HTTP payloads at the gateway level with Fermyon Spin.

## Quick Start

### Build Native & WASI

```bash
# Build native binary
cargo build --release

# Build for WASI Preview 2
cargo build --release --target wasm32-wasip2
```

### Usage (CLI)

```bash
# Validate JSON file against schema
wasm_json_validator --schema schema.json --input data.json

# Validate CSV file
wasm_json_validator --schema schema.json --input data.csv --format csv

# Validate Excel XLSX file
wasm_json_validator --schema schema.json --input records.xlsx --format xlsx

# JSON report output
wasm_json_validator --schema schema.json --input data.json --output json
```

### Usage (Spin HTTP Gateway)

Run the serverless HTTP payload validation gateway with Fermyon Spin:

```bash
# Build the Spin component
spin build

# Start the local Spin server
spin up

# Validate a JSON payload via HTTP POST
curl -i -X POST "http://localhost:3000/validate?schema=examples/user_schema.json" \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "age": 30}'

# Test invalid payload (returns HTTP 400 with diagnostics)
curl -i -X POST "http://localhost:3000/validate?schema=examples/user_schema.json" \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob", "age": -5}'
```

## License

Licensed under the Apache License, Version 2.0.
