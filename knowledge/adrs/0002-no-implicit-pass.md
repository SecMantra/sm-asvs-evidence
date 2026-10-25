# ADR-0002: No implicit PASS

Status: Accepted (2026-10-25)

Context: Scanner success, absent findings or catalogue presence are not proof of security controls.

Decision: Require explicit nonempty evidence kinds in policy. Unmapped controls return NO EVIDENCE; human-review controls remain NEEDS REVIEW. Current failures take precedence.

Consequences: Conservative assessments reduce false assurance. Evidence authentication and environment scope are necessary before production use.
