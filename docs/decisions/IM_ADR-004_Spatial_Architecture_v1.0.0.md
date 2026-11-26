# ADR-004 — Spatial Architecture

Project: Interstellar Meridian | 2026-11-26 | Final review — consolidated acceptance pending.
License: CC BY 4.0; official branding and separately identified third-party materials excluded.

Adopt D3 v1.0.0 and D4 v1.0.0 anchored f64 system coordinates and local offsets, camera-relative f32 projection. Alternatives: global f32, collapsed f64, fixed point, external floating-origin plugin. Preserve anchor/offset bits and identity; fixture envelopes and tolerances in D4 §10. No final physics or travel model. T4.6 tests; T6 actual flight verification. Consolidated owner approval pending.


## Closure disposition
Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. No production verification is claimed. V2/V2.5 deferred to T4.2–4.3; V3 to T4.7; see final checkpoint.
