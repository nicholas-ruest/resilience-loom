# Dependency and security posture

`cargo deny check` passes advisories, bans, licenses and sources. The advisory policy records one explicit exception: RUSTSEC-2025-0141 marks bincode 2.0.1 unmaintained; it reports no exploit, has no safe upgrade, and is transitively required by pinned RuVector Core 2.4.0. Removal requires an upstream RuVector change, so the exception is narrow, visible and rechecked on every CI run. Runtime Node dependencies pass `npm audit --omit=dev`; the complete locked Node tree passes `npm audit`. No credentials are committed.
