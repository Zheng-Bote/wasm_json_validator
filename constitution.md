---
status: active
---

# Verfassung — wasm_json_validator

Diese Verfassung enthält die systemweiten, dauerhaften Regeln des Projekts. Feature-Inhalte (User Stories, Tasklisten, Planung) gehören in PRD, Tech Spec und Plan. Jede Änderung einer Regel verweist auf ein ADR unter `docs/adr`.

## Architektur/Layering

1. Das System ist in klar getrennte Schichten unterteilt:
   - `src/formats`: Formateingabe und -transformation für JSON, CSV (konfigurierbare Delimiter/Header) und XLSX (Tabellenblatt-Extraktion). Reines Parsing in kanonische JSON-kompatible Datenstrukturen ohne Abhängigkeit zu I/O- oder Transport-Schichten.
   - `src/schema`: JSON-Schema-Kompilierung, Schema-Registry/Cache (Draft 7 / 2020-12) und Ausführung der Validierungsregeln. Darf nicht von `src/cli`, `src/provider` oder `spin` abhängen.
   - `src/report`: Strukturierte Fehler- und Diagnoseberichterstattung (JSON-Pointers, Validierungsdetails, Formatierungsoptionen).
   - `src/cli`, `src/provider`, `spin`: Bereitstellungsschichten (WASI Preview 2 CLI, WIT Component Model Export und optionaler Spin HTTP Reverse-Proxy/Payload-Filter).

## Security

1. **Fail-Closed-Prinzip:** Jeder Payload wird vor Weiterleitung an Datenbank- oder ML-Dienste standardmäßig als unvalidiert eingestuft; bei Schemarestriktionsverletzungen, Parserfehlern oder unbekannten Schemata wird die Anfrage mit strukturierter Diagnose abgewiesen.
2. **Ressourcen- und DoS-Schutz:**
   - Schutz vor Rekursionsbomben / tief verschachtelten JSON-Objekten (begrenzte Rekursionstiefe).
   - Maximalgrößenbegrenzung für Eingabedatenströme.
   - Schutz vor XLSX-/Zip-Bomben (Beschränkung auf maximale Zell-/Zeilenanzahl und Speicherlimit beim Entpacken).
3. **Datenschutz:** Fehlermeldungen dürfen keine sensiblen Payload-Werte (z. B. Passwörter, PII, Geheimnisse) im Klartext in ungeschützten Logs exponieren.

## Strikte Regeln

1. Jedes Akzeptanzkriterium (`AC-n`) im PRD muss durch mindestens einen automatisierten Test (`// <slug> AC-<n>: ...`) abgedeckt und verifiziert sein.
2. Der Code muss sowohl nativ (`cargo test`) als auch für WASI Preview 2 (`wasm32-wasip2`) warnungsfrei kompilieren.
3. Sämtliche SDD-Gates (`sdd lint`, `sdd gate pre-implementation`, `sdd gate pre-merge`) müssen ohne Fehler durchlaufen.
