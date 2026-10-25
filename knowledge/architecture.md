# Architecture decisions

## Purpose
Separate raw evidence ingestion, policy evaluation and human review. Never infer compliance merely from catalogue presence or scanner success.

## Current PoC
- CLI parses three JSON inputs: catalogue, evidence and policy.
- A requirement without a rule returns NO EVIDENCE.
- Evidence must match the assessed commit.
- A failed current record returns FAIL; required human review returns NEEDS REVIEW.
- An explicit required evidence-kind set can produce PASS **only in this demonstration**, without external verification.

## Target architecture
Rust workspace crates: catalogue, evidence-core, adapters, policy-engine, review, storage, cli, api.

Source adapters: SARIF, JUnit, CycloneDX, GitHub Actions, Azure DevOps, SonarQube, Dependency-Track and authorised runtime probes. Normalise to a signed, content-addressed evidence envelope; enforce scoped credentials and record provenance.

Target assessment states: PASS, FAIL, NEEDS REVIEW, NO EVIDENCE, N/A (with approval and rationale). Assessments are versioned by ASVS edition, application, commit, environment and policy version.

## Official catalogue
Import the official machine-readable ASVS 5.0.0 release as a pinned, integrity-checked build input. Preserve requirement identifiers and wording; verify expected requirement count and IDs. The bundled demo does not contain or purport to represent official requirements.
