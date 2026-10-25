# Test strategy

## Run now
1. cargo test verifies PASS/FAIL/NEEDS REVIEW/NO EVIDENCE and stale-commit behaviour.
2. cargo run -- catalogue --input fixtures/catalogue.json validates demo catalogue.
3. cargo run -- assess --catalogue fixtures/catalogue.json --evidence fixtures/evidence.json --policy fixtures/policy.json prints report.
4. Change a passed record to failed; ensure status becomes FAIL.
5. Change assessed commit; ensure stale records cannot yield PASS.

## Before real-world use
- Golden tests for supported adapters; fuzz malformed/untrusted JSON, XML and SARIF.
- Test duplicate evidence IDs, conflicting results, timestamps, applicability, environment mismatch, revocation and policy changes.
- Verify provenance and CI artifact identity, not just user-supplied outcome strings.
- Security review for path traversal, decompression bombs, secret leakage, SSRF and credential permissions.
- Review workflow tests for RBAC, separation of duties, re-opening decisions and audit integrity.
- Evaluate false PASS rate against independently labelled controls.
