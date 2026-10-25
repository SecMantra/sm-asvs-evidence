# Human review workflow — planned design, not implemented

## Roles
Evidence contributor, reviewer, approver, auditor and administrator. A contributor cannot approve their own submission for high-assurance applications.

## State machine
`OPEN -> ASSIGNED -> IN_REVIEW -> APPROVED | REJECTED | MORE_EVIDENCE`. Approval does not alter raw evidence. Each decision stores actor, timestamp, requirement, evidence hashes, justification, application commit, environment and policy version.

## Reopen conditions
New contradictory evidence, new deployment, expired evidence, catalogue upgrade, or a revoked attestation must reopen review.

## N/A
Non-applicability requires explicit scope, reason, reviewer and expiry/reassessment trigger.

## Security
Identity verification, role enforcement, immutable event log and two-person approval must be implemented before claiming production human-review support.
