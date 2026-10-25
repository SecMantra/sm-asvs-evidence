# Initial threat model

**Assets:** CI credentials, private source and configuration, evidence, reviewer identities, assessment decisions.

**Trust boundaries:** external adapters -> normalizer -> evidence store -> policy engine -> reviewer -> export.

**Threats:** fabricated PASS evidence, stale commit replay, malicious files, leaked tokens, privilege escalation, evidence tampering, policy injection, SSRF and reviewer conflicts of interest.

**Controls to implement:** least-privilege read-only tokens, content hashes and signed provenance, schema/size validation, safe parsers, restricted network access, immutable audit events, role separation, credential redaction, re-evaluation on changes.

**Current PoC limitation:** inputs are untrusted local JSON and provenance is NOT cryptographically validated. Do not use output as audit or compliance proof.
