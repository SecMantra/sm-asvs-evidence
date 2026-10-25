# sm-asvs-evidence

Offline-first Rust proof of concept for evidence-backed application security assessments.

> **PoC only:** Bundled `DEMO-*` requirements are fictional, **not** the official OWASP ASVS 5.0 catalogue. Imported evidence is not cryptographically authenticated. Do not interpret PASS as an audit or compliance certification.

## Quick start

Requires Rust stable and Cargo.

```bash
cargo test
cargo run -- catalogue --input fixtures/catalogue.json
cargo run -- assess \
  --catalogue fixtures/catalogue.json \
  --evidence fixtures/evidence.json \
  --policy fixtures/policy.json \
  --output report.json
cat report.json
```

Expected demo statuses, in order: **PASS**, **FAIL**, **NEEDS REVIEW**, **NO EVIDENCE**.

## How it works

The CLI reads a JSON catalogue, evidence records and explicit per-requirement policies. It validates the catalogue, restricts evidence to the assessed commit, reports failures, requires human review when configured, and only emits demo PASS when every required evidence kind has a passed record.

This is an initial deterministic assessment demonstration, **not** a real scanner or evidence verifier.

## Test cases

- Run `cargo test` to exercise all four statuses and stale-commit rejection.
- Change the first evidence record's outcome from `passed` to `failed` and rerun: DEMO-1 should FAIL.
- Change the top-level assessed `commit` to a different value: all demo controls should report NO EVIDENCE.
- GitHub Actions runs `cargo test --all-targets` and the CLI example on push/PR.

## Knowledge base

- [Architecture](knowledge/architecture.md)
- [Threat model](knowledge/threat-model.md)
- [Testing strategy](knowledge/testing.md)
- [Roadmap](knowledge/roadmap.md)

## Planned

Official version-pinned OWASP ASVS 5.0.0 catalogue; SARIF, JUnit and CycloneDX adapters; provenance and freshness; SQLite; human reviewer workflow and audit log; API and optional desktop UI.

MIT licensed.
