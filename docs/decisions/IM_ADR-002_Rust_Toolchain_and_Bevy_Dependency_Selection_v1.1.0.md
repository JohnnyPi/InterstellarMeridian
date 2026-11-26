# ADR-002 — Rust Toolchain and Bevy Dependency Selection

**Version:** 1.1.0 — proposed amendment to accepted v1.0.0
**Date:** 2026-11-26
**Status:** Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending.
**License:** CC BY 4.0, branding and third-party terms excluded.
**Authority:** D0 v1.1.1, D1 v2.1.1, D10 v0.1.3 on acceptance; ADR-001 v1.0.1.
**Accepted predecessor:** v1.0.0 at `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b`; retain historical evidence.

## Decision and scope
Retain Rust 1.100.0, edition 2024, x86_64-pc-windows-msvc, Bevy exactly 0.20.1, defaults disabled, features std/default_app; client adds bevy/3d and bevy/ui. Cursor reference editor, VS Community 2026 MSVC 14.52.37025 and SDK 10.0.26100.0 reference environment. No audio. Commit toolchain and lock only at authorized T4.1. No other direct dependency at bootstrap. Amend the no-additional-dependency rule solely at T4.5 to permit serde =1.0.228 with derive and serde_json =1.0.154 on consolidated owner acceptance. Review actual production resolution and license effects before activation. No external RNG, UUID, physics or persistence codec selected. Keep staged licensing policy, inherited LIC obligations, font restrictions and ReShade exception unchanged. Vulkan observed, not mandated.

## Rationale and alternatives
Strict typed JSON supports authoring and avoids a custom parser; TOML/RON and handwritten parsing not selected for initial external definitions. Typed snapshots remain separate. No new dependency for snapshot encoding. Original v1.0.0 alternatives and evidence remain applicable, including native T3.1 headless and graphical probe, 131/364 active and 567 locked third-party packages, corrected shadow_maps_enabled API and no cross-platform claim. V1 demonstrates decoder and combined compilation, not native graphical integration or production correctness. No global toolchain change. Future changes require regression, platform and license review.

## Final verification disposition — 2026-11-26

Owner approved ending disposable verification after V1 and transferring remaining implementation checks to T4. This supersedes earlier statements requiring V2/V3 before T3 closure. It does not waive any production acceptance criterion. Final consolidated design approval and banking remain explicit.

| Evidence | Actual result / disposition |
|---|---|
| Repository | Owner git fetch and local/remote HEAD both `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b`, main; draft documents untracked, historical D10 v0.1.1 deletion observed; no later clean checkout asserted |
| Toolchain | Rust 1.100.0 (0ef1bc386), Cargo 1.100.0 (3acc79c0c), MSVC 14.52.37025, VS Community 2026 18.12.1, host/target x64 confirmed in final owner output |
| V1.3 | cargo info confirmed serde 1.0.228 (MSRV 1.56), serde_json 1.0.154 (1.71), cargo-deny 0.19.9 and cargo-about 0.9.3 (both 1.88.0); all declared MIT OR Apache-2.0 |
| V1.4 | Native decoder build 10.08 s, 28/28 tests; initial collapsible_if lint corrected; locked offline 28/28, fmt, Clippy -D warnings, execution `Validated 2 definitions` passed |
| V1.5 | Bevy 0.20.1 + exact parser pins; headless build 59.82 s, client build 3m43s; 28/28 tests each; Clippy completed both (manifest unused Bevy warning expected in decoder-only fixture); targeted graphics/audio exclusions passed; 567 locked third-party packages reported; counts do not prove identical inventory |
| Evidence integrity | Owner generated five standalone decoder baseline hashes after excluding checksum output itself; combined graph/build logs retained locally; no independent byte verification of workstation files |
| V2 / V2.5 | Not executed; explicit shared core driver, ordered steps and native graphical integration verified at T4.2–4.3; prior T3.1 graphical probe remains evidence only |
| V3 | Not executed; candidate compliance tool install/CLI checks, scoped policy, positive/negative fixtures, notice generation and changed package reconciliation at T4.7; review dependency changes at introduction |
| Performance / production | No production tests, benchmark or CI run; all matrix scenarios remain unexecuted as production acceptance tests |

Evidence location: owner workstation `E:\RustProjects\IM_T3_Verification\evidence\V1` (decoder-baseline, combined, decoder-tests-final.txt, decoder-clippy-final.txt, decoder-run-final.txt); T3.1 evidence remains under its original probe. Owner-supplied console output in project conversation is the evidence for the statements above. The combined V1 graphs were not built with offline flags in that run; production cached offline verification remains mandatory. New blake3/constant_time_eq versions observed; reconcile exact name/version/source/license, inherited LIC issues, font and notice gaps. No redistribution clearance. No further disposable probes required for design closure.
