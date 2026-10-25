# Product specification — v0.2 library preview

## Purpose
Provide a reusable Rust crate and CLI to evaluate supplied evidence against explicit security-control policies. It is not a scanner, verifier of authenticity, or ASVS certification product.

## Users
AppSec engineers, CI maintainers, security architects, platform teams, and developers building application assurance workflows.

## Functional contracts
1. Consume catalogue, evidence and policy JSON with explicit schemas.
2. Validate unique requirement, policy and evidence identifiers.
3. Match evidence to assessed application commit; stale evidence never produces PASS.
4. Resolve outcomes deterministically: FAIL on current failure; NEEDS REVIEW on human-review rule; PASS only when all required evidence kinds pass; NO EVIDENCE otherwise.
5. Expose embeddable `asvs_evidence::assess` and a CLI.
6. Serialize stable report JSON with schema_version.
7. Run offline; no external calls in the assessment core.

## Non-goals (current)
- No official ASVS 5.0 catalogue ingestion yet.
- No verified evidence provenance, real adapters, reviewer authentication or signed attestation.
- No automatic compliance assertions.
- No hosted plugin marketplace package, desktop UI or REST server.

## Quality gates
Cargo tests, clippy, formatting, dependency audit, CLI smoke test, API consumer example, documentation and review.

## Future adapter interface
Adapters ingest external records and produce validated `EvidenceFile` values. Adapter output is untrusted until provenance checks, signatures and artifact binding are implemented.

## Security invariant
A PASS is a *policy evaluation result*, not a statement that the application is secure.
