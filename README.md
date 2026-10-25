# SecMantra · ASVS Evidence Collector

[![Rust CI](https://github.com/SecMantra/sm-asvs-evidence/actions/workflows/ci.yml/badge.svg)](https://github.com/SecMantra/sm-asvs-evidence/actions/workflows/ci.yml)
![Status](https://img.shields.io/badge/status-experimental-orange)
![Rust](https://img.shields.io/badge/Rust-2021-informational)
![License](https://img.shields.io/badge/license-MIT-blue)

**Embeddable Rust library and CLI for deterministic, evidence-backed security-control assessments.** Designed for use in developer tools, CI pipelines and future security-assurance integrations.

> **Experimental v0.2.0 — not a compliance certification.** The included `DEMO-*` controls are fictional, not OWASP ASVS 5.0 requirements. Evidence is not cryptographically authenticated. A PASS means only that a user-supplied policy matched user-supplied evidence.

## Quick start

Requires Rust stable:

```sh
git clone https://github.com/SecMantra/sm-asvs-evidence.git
cd sm-asvs-evidence
cargo test --all-targets
cargo run --example consume
cargo run -- catalogue --input fixtures/catalogue.json
cargo run -- assess --catalogue fixtures/catalogue.json --evidence fixtures/evidence.json --policy fixtures/policy.json --output report.json
```

Expected demo outcomes: `PASS`, `FAIL`, `NEEDS REVIEW`, `NO EVIDENCE`. Evidence must match the assessed commit.

## Embed in another Rust application

Package naming convention: all SecMantra packages use the `sm-` prefix. Once published on crates.io, install with `cargo add sm-asvs-evidence`. The crate is **not yet published**.

Add as a Git dependency (pin to a reviewed commit in production):

```toml
[dependencies]
sm-asvs-evidence = { git = "https://github.com/SecMantra/sm-asvs-evidence", rev = "<reviewed-commit-sha>" }
serde_json = "1"
```

```rust
use sm_asvs_evidence::{assess, Catalogue, EvidenceFile, PolicyFile};

let catalogue: Catalogue = serde_json::from_str(catalogue_json)?;
let evidence: EvidenceFile = serde_json::from_str(evidence_json)?;
let policy: PolicyFile = serde_json::from_str(policy_json)?;
let report = assess(&catalogue, &evidence, &policy)?;
```

See [compilable example](examples/consume.rs) and [integration contract](knowledge/integration.md).

## Developer experience

| Surface | Available |
| --- | --- |
| Rust crate API | Yes (source dependency) |
| CLI | Yes |
| JSON inputs and reports | Yes |
| Automated CI tests | Configured; verify status badge |
| Official ASVS 5.0 catalogue | Not yet |
| SARIF / JUnit / CycloneDX adapters | Not yet |
| Signed evidence / human review workflow | Not yet |
| crates.io release | Not published |

## Documentation

- [Product specification](knowledge/spec.md)
- [Integration guide](knowledge/integration.md)
- [Architecture](knowledge/architecture.md)
- [Testing](knowledge/testing.md)
- [Threat model](knowledge/threat-model.md)
- [Roadmap](knowledge/roadmap.md)
- [Architecture decisions](knowledge/adrs/)
- [Agent guidance](AGENTS.md)
- [Security policy](SECURITY.md)
- [Contributing](CONTRIBUTING.md)

## Security principle

**No evidence ≠ PASS. Scanner success ≠ compliance.** Never use this PoC as an authoritative security certification. Real-world integrations must authenticate evidence provenance, scope to environment/deployment and obtain independent human review where required.

MIT license.
