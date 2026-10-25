# ADR-0001: Offline deterministic assessment core

Status: Accepted (2026-10-25)

Context: Evidence may originate from multiple untrusted tools; external LLM and SaaS dependencies would undermine reproducibility.

Decision: Keep the core as a pure Rust library without network calls. CLI and future adapters are separate integration surfaces.

Consequences: Tests are fast and reproducible. Integrators must explicitly supply evidence and policy; provenance validation remains separate.
