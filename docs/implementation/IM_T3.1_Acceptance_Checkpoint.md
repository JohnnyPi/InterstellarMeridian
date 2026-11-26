# T3.1 — Acceptance Checkpoint

**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights. Official branding and separately identified third-party material excluded. <https://creativecommons.org/licenses/by/4.0/>
**Date:** 2026-11-26  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian (`main`)  
**Verified baseline:** `40e04d0747c565513cfddcbb092c1ae7616a4ab0`  

**Status:** Accepted by owner 2026-11-26; technical probe completed.  
**Governing versions:** D0 1.1.1, D1 2.1.1, D6A/B 0.2.1, D10 0.1.1, ADR-001 1.0.1, repository Project Instructions 1.0.1 (current Project instructions 1.0.2). D10 0.1.2 and ADR-002/contract/assessment 1.0.0 accepted 2026-11-26; D10 0.1.1 governed the original probe. T2 accepted 2026-11-26.

## Authorization and evidence provenance
Owner authorized disposable probe on 2026-11-26. All Windows execution results are owner-supplied terminal output and visual verification, not commands executed by ChatGPT. Uploaded lockfile and metadata independently inspected in closure turn; previous inventory reviews recorded in conversation. Production repository remote was inspected at initial T3.1; final synchronization is owner terminal evidence. No production code changes or new commits.

## Acceptance matrix
| Criterion | Result / evidence |
|---|---|
| T3.1-01 | Pass: local/remote main exact baseline; 0 changes, 0 stashes, 13 tracked files |
| T3.1-02 | Pass based on prior proposal release register and installed exact versions; links not rechecked at closure |
| T3.1-03 | Pass: Rust/Cargo 1.100.0, MSVC x64, rustfmt/Clippy available |
| T3.1-04 | Pass: native compile/link/run, exit 0, 2 tests passed |
| T3.1-05 | Pass: Bevy metadata confirms 0.20.1, edition 2024, MSRV 1.97.1, MIT OR Apache-2.0 |
| T3.1-06 | Pass: headless graph excludes render, native window and audio backends |
| T3.1-07 | Pass: six ECS tests, explicit updates, 42 value and 2 updates, headless exit 0; no window confirmed in owner visual verification as recorded |
| T3.1-08 | Technical pass: client build/link, Clippy, Vulkan RTX 3070 Ti, window/scene and owner visual verification, normal exit 0; ReShade exception accepted 2026-11-26 |
| T3.1-09 | Pass under owner-approved staged policy; rights gaps remain restricted as documented |
| T3.1-10 | Pass: cached offline locked build/test under both profiles, unchanged lockfile |
| T3.1-11 | Pass: owner accepted milestone, documents and ReShade exception 2026-11-26 |

## Probe results and measurements
P1 native 2/2 tests; executable 135168 bytes; cached build 507 ms. P2 exact Bevy resolution; P3 isolated graphs 131/364 active third-party packages. P4 six tests (zero failures), clean Clippy; first headless build 46.098 s, execution 234 ms, executable 10863616 bytes. P5 initial graphical compile failed E0560 (158.383 s); corrected outdated field to shadow_maps_enabled; successful incremental build 19.851 s; graphical executable 167035904 bytes; headless regression 6/6; client Clippy passed; owner confirmed visual rendering. Vulkan loader reported missing ReShade JSON manifests but window/render/exit passed. P6 offline cached build 562 ms headless, 859 ms client; six tests passed in each profile, graphical harness zero tests; headless exit 0. P7 licensing inventory and staged acceptance. P8 final owner report: main, local/remote same SHA, 0 changes, 0 stashes, lockfile unchanged.

All timings are debug, workstation-specific observations, not comparative clean-build benchmarks or performance budgets. No peak memory recorded. No claim of cross-machine or bit-identical builds.

## Fingerprints (SHA-256)
- Cargo.lock: `C123EFC94D6D17BEF54B183B468E14B921510A7A27119FE54D3E28E631F6D671`
- P6 headless exe: `39C2B9C6CC898D2C015134E60AA598DDE8DAFD0F80A5F90BF716A99B7A46C67B`
- P6 graphical exe: `3DE8F7BEC277D093F90CEE572ACA4CCC034588C41B9385BFC3C37F600555719B`

## Evidence retention
Probe path `E:\RustProjects\IM_T3_1_Probe`. P8 directory counts P1=4, P2-P3=6, P4=4, P5=6, P6=0, P7=965 (985 total). P6 output retained in conversation and summarized here; original local log absent due to script omission. Retain probe and evidence, do not promote its code automatically. Record earlier formatting correction, API correction, PowerShell JSON case-key parsing and false success marker corrected, and excessive manual licensing effort replaced by approved staged policy.

## Exceptions and owner disposition
- ReShade stale Vulkan manifests: accepted non-blocking environment exception 2026-11-26; no workstation cleanup required for this gate.
- Licensing staged policy explicitly approved 2026-11-26. Follow-up obligations in licensing assessment; no rights waived.
- Cross-machine/byte-identical reproduction and minimum hardware not tested; no production performance guarantees.
- Accepted documents not yet committed. T3.2 design authorized; T4 production implementation unauthorized.

## Owner disposition (accepted)
On 2026-11-26 the owner stated:

> I formally accept T3.1, ADR-002 v1.0.0, the technical contract, acceptance checkpoint, licensing assessment and D10 v0.1.2. I accept the ReShade environmental exception, reaffirm the staged licensing policy, and authorize T3.2 — Application and State Boundaries. T4 remains unauthorized.

Documentation commit: pending local installation and commit. T3.2 is design only; T4 remains blocked.
