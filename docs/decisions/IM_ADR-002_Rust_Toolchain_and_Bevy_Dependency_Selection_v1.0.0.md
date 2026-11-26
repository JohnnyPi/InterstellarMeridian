# ADR-002 — Rust Toolchain and Bevy Dependency Selection

**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights. Official branding and separately identified third-party material excluded. <https://creativecommons.org/licenses/by/4.0/>
**Date:** 2026-11-26  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian (`main`)  
**Verified baseline:** `40e04d0747c565513cfddcbb092c1ae7616a4ab0`  

**Version:** 1.0.0 (accepted 2026-11-26)  
**Status:** Accepted by owner 2026-11-26; staged licensing policy reaffirmed.  
**Authority:** D0 v1.1.1, D1 v2.1.1, D10 v0.1.1, ADR-001 v1.0.1; T2 accepted.

## Context
A greenfield, pilot-first persistent causal universe needs a headless authoritative simulation and a separately enabled presentation path. T3.1 tests engine feasibility, not production architecture. Owner authorized an isolated probe at `E:\RustProjects\IM_T3_1_Probe` on 2026-11-26.

## Accepted decision
- Rust 1.100.0, edition 2024, Windows x64 MSVC, Bevy exactly 0.20.1.
- Bevy defaults disabled; base features `std`, `default_app`; opt-in `client` activates `bevy/3d`, `bevy/ui`. No additional direct dependency.
- Cursor is the reference editor; Visual Studio Community 2026 supplies native tools (not a paid IDE requirement). Reference SDK 10.0.26100.0 and MSVC 14.52.37025.
- Commit pinned toolchain and production lockfile only at authorized T4.1; locked CI/test builds. Dependency updates require regression, license and platform review.
- Retain the tested bundled default font subject to redistribution checks; not a visual identity decision. Vulkan was observed, not mandated.
- Apply the owner-approved staged licensing policy in the accompanying assessment; no unresolved rights may be distributed.

## Alternatives
Floating stable compiler: rejected for baseline drift. Nightly: unnecessary. Older Bevy or minimum upstream Rust version: no demonstrated need; not tested. Default Bevy features everywhere: rejected for headless isolation. Individual Bevy subcrates: defer until measured need. GNU Windows ABI: not selected. Paid Visual Studio: not required; editor independent of native toolchain. Exhaustive manual licensing before code: superseded by approved staged checks.

## Evidence and consequences
Native compile/run and two tests, headless compile/run and six tests, graphical compile/run and owner visual verification, Clippy, cached offline locked builds all passed on the owner workstation. Dependency graph 131 headless / 364 client / 567 locked third-party packages. Exact lockfile and source inventory preserved outside repository. No production code was written. See technical contract, checkpoint and licensing assessment.

Bevy is evolving; initial graphical fixture required `shadow_maps_enabled` instead of `shadows_enabled`. Future upgrades require version-specific API verification. Headless isolation does not establish authoritative scheduling, deterministic replay, persistence or performance budgets. Reference Vulkan loader reported stale ReShade manifest references; owner accepted as a non-blocking exception on 2026-11-26. No other OS or graphics backend tested.

## Acceptance and scope
Owner accepted this ADR, technical contract, checkpoint and ReShade disposition on 2026-11-26 and authorized T3.2 only. T4 remains unauthorized. T4 requires acceptance of all T3 contracts and the T4 plan. No permanent crate topology, physics, serialization, assets or simulation clock selected.
