# ADR-003 — Core Time Randomness and Identity

Project: Interstellar Meridian | 2026-11-26 | Final review — consolidated acceptance pending.
License: CC BY 4.0; official branding and separately identified third-party materials excluded.

Adopt D4 lifecycle, exclusive logical state ownership, validated commits and fail-stop; fixed 100 ms fixture quantum, four-step/400 ms pacing cap, ordered phases; SplitMix64 v1 with exact derivation and bounded sampling; world-local monotonic nonzero u64 IDs. Alternatives: render-driven clock, shared RNG, ECS IDs, UUIDs. Reject for fixture reproducibility or unnecessary complexity. Consequences: explicit pending work, state ownership and versioned golden tests; no final physics frequency. Evidence: D4 v1.0.0, T4.2–4.4 and synthetic handoff T4.6. Consolidated owner approval pending.


## Closure disposition
Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. No production verification is claimed. V2/V2.5 deferred to T4.2–4.3; V3 to T4.7; see final checkpoint.
