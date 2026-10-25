# Integration guide

Crate package: `sm-asvs-evidence`; Rust import: `sm_asvs_evidence`; CLI executable: `sm-asvs-evidence`.

SecMantra packages should consistently use the `sm-` prefix.

## Consume from Rust
Use a pinned Git revision dependency and call `sm_asvs_evidence::assess(&catalogue, &evidence, &policy)`. The function is pure and performs no network access. See `examples/consume.rs`.

## Consume from any language
Run the CLI as a child process and parse its JSON output. A nonzero process exit indicates invalid input or an IO error.

```sh
cargo run -- assess --catalogue fixtures/catalogue.json --evidence fixtures/evidence.json --policy fixtures/policy.json --output report.json
```

Report fields: `schema_version`, `application`, `commit`, `catalogue_version`, `assessments[]`. Each assessment has `requirement_id`, `status`, `rationale` and `evidence_ids`.

## Inputs
- `catalogue.json`: `version`, `requirements[]` with `id`, `text`, `level` (1–3).
- `evidence.json`: `application`, `commit`, `evidence[]` with `id`, `requirement_id`, `kind`, `outcome` (passed/failed/unknown), `commit`, `source`.
- `policy.json`: `rules[]` with `requirement_id`, nonempty `required_kinds[]`, optional `human_review`.

## Result interpretation
- PASS: configured evidence kinds all passed for the commit, no current failures, and no mandatory human review.
- FAIL: current evidence explicitly failed.
- NEEDS REVIEW: review required or supplied evidence insufficient/ambiguous.
- NO EVIDENCE: no policy or no evidence for the assessed commit.

## Limitations
This is a source-consumable library preview, **not** a plugin package for an external marketplace. No official ASVS mapping, real adapters, review UI, attestations, environment binding, cryptographic provenance, or signed release binaries yet. Do not expose this PoC to untrusted file uploads without sandboxing and resource limits.
