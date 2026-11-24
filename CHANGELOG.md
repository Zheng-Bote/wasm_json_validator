# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-24

### Added
- **Multi-Format Ingestion Layer (`src/formats/`):**
  - High-performance, pure-Rust parsing of raw `JSON` payloads.
  - Tabular `CSV` parsing with automatic header detection and type inference (integers, floats, booleans, strings) into canonical JSON object arrays.
  - Microsoft Excel `XLSX` worksheet parsing via `calamine`, extracting rows and cells into canonical JSON structures without native C dependencies.
  - Automatic format detection via file extensions (`.json`, `.csv`, `.xlsx`).
- **JSON-Schema Validation Engine (`src/schema/`):**
  - Pure-Rust schema compilation and validation (Draft 7 / 2020-12) via `jsonschema`.
  - Configured with `default-features = false` to run fully self-contained in WASM/WASI environments without external network or file resolver dependencies.
- **Structured Error Diagnostics & Reporting (`src/report/`):**
  - Fine-grained error locations with standard JSON Pointer paths (e.g. `/age`, `/items/0/email`).
  - Terminal-friendly summary formatting and machine-readable JSON output (`--output json`).
- **WASI Preview 2 CLI (`src/main.rs`):**
  - Command-line interface with options: `--schema`, `--input` (file or stdin `-`), `--format` (json, csv, xlsx), `--output` (text, json).
  - Strict Fail-Closed error handling (exit code `0` on success, `1` on validation failure, `2` on parsing errors).
  - Clean, warning-free compilation for `wasm32-wasip2` and native targets.
- **Fermyon Spin HTTP Gateway (`spin/`):**
  - Dedicated workspace component with `crate-type = ["cdylib"]` using `spin-sdk 3.1.1`.
  - Exports standard `wasi:http/incoming-handler@0.2.0#handle` for native compatibility with Spin 4.3+.
  - Dynamic schema loading via query parameters (`?schema=...`) or HTTP headers (`X-Schema-ID`).
  - Content-Type based format negotiation (`application/json`, `text/csv`, `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`).
  - HTTP status codes: `200 OK` on valid payload, `400 Bad Request` with diagnostic JSON on schema violations, `405 Method Not Allowed` for unsupported methods.
  - Spin application manifest `spin.toml`.
- **Spec-Driven Development (`sdd_studio`):**
  - Formal constitution in `constitution.md`.
  - SDD specifications: PRDs, Tech Specs, and Plans for `payload-validation-core` and `spin-http-gateway`.
  - GitHub issue template `.github/ISSUE_TEMPLATE/feature_spec.md` with EARS syntax and SpecDD architecture drift control.
- **Documentation & Examples:**
  - Bilingual documentation in `README.md` and `README_de.md`.
  - Complete validation examples (`examples/user_schema.json`, `examples/valid_user.json`, `examples/invalid_user.json`, `examples/user_batch_schema.json`, `examples/users.csv`).
  - Apache-2.0 `NOTICE` and `LICENSE`.

## [0.0.1] - 2026-11-24

### Added
- Initial project repository initialization with Apache-2.0 license.
