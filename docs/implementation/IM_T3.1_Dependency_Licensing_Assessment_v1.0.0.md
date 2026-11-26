# T3.1 — Dependency Licensing Assessment

**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights. Official branding and separately identified third-party material excluded. <https://creativecommons.org/licenses/by/4.0/>
**Date:** 2026-11-26  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian (`main`)  
**Verified baseline:** `40e04d0747c565513cfddcbb092c1ae7616a4ab0`  

**Version:** 1.0.0 (accepted 2026-11-26)  
**Status:** Technology-selection licensing gate passed under owner-approved staged policy (2026-11-26); redistribution compliance not certified. Assessment accepted by owner 2026-11-26.

## Inventory and scope
567 third-party packages, 568 including probe, in locked resolution. 131 active headless, 364 active client, 203 outside both tested Windows graphs. Normal/build graphs include procedural macros and build-only packages, not necessarily redistributed binaries. No missing declared expressions; no incompatible mandatory declared license path identified. Prior review reconciled identities and expressions. 948 license-related files collected; 502 packages with discovered files, 65 without (18 active, 47 inactive); 12 font files inventoried. 71 targeted provenance records, 69 Git revisions recorded. Two GNU Windows winapi packages lack revision evidence and are inactive. Filename and keyword scans are incomplete by nature.

## Owner-approved staged policy
T3.1: reasonable compatibility, inventory, known obligations and documented unresolved issues; replaces original zero-unresolved-rights criterion for selection only. T3.9: design automated checks and exception management. T4: implement license policy enforcement and notice generation, reconcile production dependencies. Every dependency/feature change: review new scope and exceptions. Before redistribution: verify exact distributed code, assets and native runtimes, resolve affected rights and preserve notices. Before activating affected platforms: resolve known relevant rights issues. Do not distribute affected material with unresolved rights. No policy change to project-owned MIT OR Apache-2.0 code or CC BY 4.0 documentation/lore, branding excluded.

Recommended tooling candidates (not selected or installed): cargo-deny and cargo-about; evaluate exact versions/licenses in T3.9. Do not treat metadata or generated notices as legal certification. Prefer a permitted upstream alternative where actually offered; preserve all mandatory AND terms and source-level exceptions. Detailed license allowlist and selection preference remain proposed until approved; no blanket permission for copyleft or custom licenses.

## Issue register
| ID | Finding | Required disposition / gate |
|---|---|---|
| LIC-001 | bevy_mikktspace 1.0.0: Zlib AND (MIT OR Apache-2.0); full 2011 Morten S. Mikkelsen notice located | Preserve original Zlib notice and selected code terms before distribution |
| LIC-002 | unicode-ident: Unicode-3.0 AND dual code terms; texts located | Preserve both required notices |
| LIC-003 | dpi: MIT AND Apache-2.0; texts located | Preserve both terms and libm attribution |
| LIC-004 | r-efi 6.0.0: MIT/Apache/LGPL alternatives; AUTHORS contains full MIT permission and attribution | MIT path available; inactive; recheck if activated |
| LIC-005 | bevy_text default_font active through client/default_platform; FiraMono-subset.ttf included in source when enabled; full OFL-1.1 text located in bevy_feathers | Confirm exact subset notice/rights and retain OFL before binary release; not official UI font |
| LIC-006 | 18 active crates lack discovered standalone license files; AccessKit source references Chromium BSD and Kurbo; mutants copyright located but MIT text not collected | Resolve exact version notices and other gaps in production compliance workflow, before affected redistribution |
| LIC-007 | 47 inactive crates lack discovered standalone license files | Review before relevant activation/distribution; no automatic assumption of unlicensed status |
| LIC-008 | Final notices, embedded assets, native redistributables not audited as a release artifact | Mandatory release gate |
| LIC-009 | sctk-adwaita 0.10.1 (inactive) rounded_headerbar_shape comment identifies possible MPL-2.0 material inconsistent with declared MIT | Do not activate/distribute affected dependency until resolved; no current Windows blocker |

The source comment is evidence of a potential discrepancy, not a legal finding. The staged policy is not a waiver of actual rights. No dependency replacement recommended. No probe binaries or third-party assets authorized for redistribution.

## Evidence retained
`E:\RustProjects\IM_T3_1_Probe\evidence\P7`: metadata-all.json, dependency-license-inventory.csv, license-summary.csv, headless/client license trees, upstream-license-files.csv and ZIP, license-file-coverage.csv, packages-without-license-files.csv, bundled-font-inventory.csv, supplementary-license-evidence.txt, bevy-text-features.txt, bevy-font-source-references.txt, upstream-provenance.csv, targeted-license-notices.txt. Keep original third-party materials under their own licenses; do not publish raw metadata with local paths or author contact information unnecessarily. The targeted source excerpts support the findings above. No further manual T3.1 scans required under the approved policy.
