# Agent instructions

- This is an assurance *evidence* engine, not a certification authority.
- Never invent official ASVS requirement identifiers or texts. Pin and validate official catalogue imports.
- Do not change public JSON contracts without documenting versioning and migration.
- Never add a rule that automatically passes solely because a scanner ran, a file exists, or an LLM says so.
- Always reject duplicate identifiers, stale commits and malformed inputs. Fail closed on ambiguous cases.
- Avoid logging secrets or raw tokens. External adapters must be read-only and scoped.
- Add tests for every behavioural change; run cargo fmt --check, cargo clippy --all-targets -- -D warnings, cargo test.
- Update README, knowledge/spec.md and ADRs for externally visible design changes.
- Keep examples fictional and clearly labelled; no fabricated real security attestations.
- Prefer deterministic, reproducible assessments. Human decisions must be explicit and auditable.
