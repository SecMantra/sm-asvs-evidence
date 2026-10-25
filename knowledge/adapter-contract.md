# Adapter contract — proposed SDK

Adapters are isolated from the assessment core. They should expose a versioned manifest (name, version, supported input formats, required permissions), a `collect` method and structured errors.

## Supported input priorities
P0: JUnit XML, SARIF 2.1, CycloneDX JSON; P1: GitHub Actions, Azure DevOps, SonarQube, Dependency-Track; P2: authorised runtime probes.

## Normalised record
Evidence ID, requirement mapping, source URI, content digest, subject commit and environment, outcome, timestamps, collector identity and verification state.

## Trust policy
Never equate an imported scanner PASS with ASVS compliance. Validate size, schema, file paths, content digests and provenance. Adapters must not invoke shell commands from untrusted evidence. External network access must be opt-in and scoped.

**Status:** Design only. Current PoC accepts manually prepared JSON evidence.
