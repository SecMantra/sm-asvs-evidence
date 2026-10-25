# ADR-0003: Library-first interfaces and versioning

Status: Accepted (2026-10-25)

Context: Developers need to embed the collector in CI, agents, desktop apps and web services.

Decision: Publish a Rust library API, CLI and JSON report contract with schema_version. Use semantic versioning; breaking contract changes require a major schema version.

Consequences: Consumers can embed the crate without running a service. A future adapter SDK should be independently versioned.
