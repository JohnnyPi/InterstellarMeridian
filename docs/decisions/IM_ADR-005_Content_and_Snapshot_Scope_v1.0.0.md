# ADR-005 — Content and Snapshot Scope

Project: Interstellar Meridian | 2026-11-26 | Final review — consolidated acceptance pending.
License: CC BY 4.0; official branding and separately identified third-party materials excluded.

Propose strict bounded JSON external definitions, typed in-memory snapshot foundation and exact catalog matching. Alternatives: TOML/RON, custom parser, disk snapshot now. JSON supports common authoring tools; typed snapshots isolate continuation correctness. Candidate pins serde =1.0.228 derive and serde_json =1.0.154 only at T4.5 after verification and explicit amendment of ADR-002; no bootstrap dependency change. D9-P22/P23 deferred T5.7, not waived. V1 native decoder and dual-profile dependency evidence passed; consolidated owner approval pending. Production graph review at T4.5 remains required.


## Closure disposition
Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. No production verification is claimed. V2/V2.5 deferred to T4.2–4.3; V3 to T4.7; see final checkpoint.
