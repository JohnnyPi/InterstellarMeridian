# ADR-001 — Independent Project Establishment

**Project:** Interstellar Meridian  
**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; see [license](https://creativecommons.org/licenses/by/4.0/). Official branding and separately identified third-party material excluded.  
**Version:** 1.0.1 | 2026-11-26  
**Status:** Records accepted owner decisions; record and remaining governance conventions awaiting final owner review.  
**Gate:** T2 — Independent Project Establishment (open)  
**Scope:** Project identity, independence, technology direction, licensing, governance and handoff.  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian

## 1. Context

The project began as an exploration of modernizing an existing space simulation. The owner subsequently chose a spiritual successor with independent gameplay and architecture, then a completely independent greenfield project without inherited source, assets, Git history, compatibility obligations or active legacy references. The owner selected Interstellar Meridian as the title and approved dual MIT / Apache 2.0 licensing for project-owned code.

The accepted product vision and high-level design carry forward, with their original distinctions between requirements, proposals and open questions. D0 v1.1.1 and D1 v2.1.1 reconcile those requirements with the new identity; D6A and D6B v0.2.1 migrate the original fictional setting. D10 v0.1.1 establishes the proposed delivery sequence and acceptance gates. The former project remains a separate historical archive, not an implementation dependency.

The purpose of this ADR is to make the boundary explicit so future contributors cannot accidentally restore discarded obligations or mistake a documentation decision for an implemented capability.

## 2. Decision register

| ID (local to this ADR) | Decision | Disposition |
|---|---|---|
| DEC-001 | Adopt Interstellar Meridian as the title | Owner accepted; trademark clearance outstanding |
| DEC-002 | Establish a new independent repository | Owner accepted; repository established |
| DEC-003 | Exclude inherited code, assets, history and compatibility obligations | Owner accepted |
| DEC-004 | Use Rust and Bevy | Owner accepted; exact versions deferred to T3 |
| DEC-005 | Prioritize Windows x64 | Owner accepted; other platforms not guaranteed |
| DEC-006 | License project-owned source under MIT OR Apache-2.0 | Owner accepted; full texts inspected in prior T2 audit |
| DEC-007 | Preserve accepted gameplay vision and original setting | Owner accepted; migration review separate |
| DEC-008 | Establish the active documentation hierarchy described below | Carried forward from D0/D1; final migration review pending |
| DEC-009 | Use incremental, test-first, evidence-gated development | Established development practice; D10 detail pending approval |
| DEC-011 | License project-owned documentation and original lore under CC BY 4.0; separate branding rights | Owner accepted 2026-11-26; publication pending |
| DEC-010 | Establish a dedicated ChatGPT Project with current sources and instructions | Owner confirmed configuration with prior document versions; refreshed Sources/settings pending |

### Identity conventions

| Attribute | Value |
|---|---|
| Title | Interstellar Meridian |
| Abbreviation | IM |
| Repository | JohnnyPi/InterstellarMeridian |
| Working branch | main |
| Local clone (owner workstation, observed) | `E:\RustProjects\InterstellarMeridian` |
| Intended Cargo package | `interstellar_meridian` |
| Intended Windows executable | `interstellar_meridian.exe` |
| Code SPDX expression | `MIT OR Apache-2.0` |

Package and executable names are adopted working conventions, not existing build artifacts or claims of registry availability. “Chart your own course” is proposed branding guidance; no logo or in-universe interpretation of the title is established by this record.

## 3. Alternatives considered

| Alternative | Assessment / disposition |
|---|---|
| Modernize within inherited history and preserve behavioral parity | Rejected as baseline: constrains architectural and creative choices and carries inherited obligations. |
| Spiritual successor with selective legacy reuse or compatibility | Superseded by the owner's stronger independence decision; no reuse assessment or compatibility adapters required. |
| Independent greenfield Rust/Bevy project | Selected: preserves creative goals while allowing independently designed systems and clean provenance. Requires building the foundation anew. |
| C# or other technology stack | Rust/Bevy selected in earlier project discussions; no comparative benchmark or claim of universal superiority. Reopen only through a justified decision. |
| Single permissive code license | Dual MIT / Apache 2.0 selected by owner, allowing recipients to choose either. No relicensing of third-party materials implied. |

Prior candidate titles were superseded by the owner's selection. Naming searches were preliminary; absence of an exact match is not legal clearance.

## 4. Independence and provenance policy

- Independently author the implementation and setting; do not import legacy source, data, assets, scripts, save formats, historical branches or commit history into the active repository.
- No requirement to reproduce original galaxies, algorithms, interfaces, random sequences or mod formats of another game.
- Ordinary external technical documentation, standards and appropriately licensed third-party dependencies/assets remain available subject to documented provenance and rights. This is not a prohibition on external learning or a claim of formal clean-room certification.
- Keep the former project and its technical research outside the active implementation and active ChatGPT Project sources. D2 is retired; do not fill the identifier merely for numbering continuity.
- Do not imply affiliation or endorsement by another project.
- Record source, version/hash where relevant, license, notices, modifications and distribution rights for third-party materials. Unresolved rights block inclusion.

## 5. Technology and architecture consequences

Rust and Bevy are the chosen foundation, with Windows x64 as the primary development platform. Preserve the D1 logical boundaries among authoritative simulation, commands/gameplay, presentation, content and persistence. Support headless execution, deterministic material transitions, persistent identity, validated commands and selective simulation detail. Prefer idiomatic Rust and Bevy ECS where appropriate; no premature crate decomposition.

T3 must verify the actual Rust toolchain, Bevy version and features, Windows prerequisites, dependency compatibility, APIs and license obligations. No specific engine version, physics library, serialization format, scripting runtime, coordinate representation or performance budget is approved by this ADR. T4 production implementation follows acceptance of T3.

## 6. Licensing and distribution

Project-owned source code is offered under either the MIT License or Apache License, Version 2.0, at the recipient's option. Maintain `LICENSE-MIT` and `LICENSE-APACHE` in the root, and an explicit dual-license statement in the README. When Cargo metadata is introduced, use:

```toml
license = "MIT OR Apache-2.0"
```

Both files were retrieved and inspected in the T2 audit, including attribution and README dual-license wording. GitHub's earlier metadata recognized Apache-2.0 alone; that does not replace the explicit dual-license policy. Verify the updated README and licensing notice after publication before T2 closure. Contributor attribution and third-party notices must reflect actual rights and obligations; no blanket relicensing of external contributions or assets is implied.

Project-owned documentation and original fictional lore are licensed under Creative Commons Attribution 4.0 International (CC BY 4.0), https://creativecommons.org/licenses/by/4.0/. Project-owned source code remains MIT OR Apache-2.0. The game name, official logos and other source-identifying branding are excluded from the CC grant as branding; no trademark or endorsement permission is granted. Artwork, music and audio not expressly released as documentation or lore require separate licensing before distribution. Third-party materials retain their applicable terms. Formal title/trademark clearance is outstanding and must be addressed before relevant public branding/distribution commitments. This record is a project policy, not a legal audit.

## 7. Product scope preserved

D0 and D1 retain a pilot-first, persistent causal universe: meaningful flight, exploration and discovery, connected careers, autonomous factions, accountable commerce and military logistics, and player intervention with durable consequences. Long-term progression includes multi-capital-ship fleets, delegated faction authority, component tuning stamps and specialized parts, and investment in existing installations; player construction of stations or factories is excluded.

D6A/D6B preserve the established seven governments, generation-ship heritage, ancient remains, historical war and fragile contemporary peace at their documented canon status. Candidate histories, organizations, mechanics, numerical targets and technical approaches remain proposals. Independence does not authorize changes to these requirements or create new lore.

## 8. Documentation and change control

| Source | Responsibility | Initial path under repository root |
|---|---|---|
| D0 v1.1.1 | Authoritative vision | `docs/design/IM_D0_Project_Vision_and_Design_Principles_v1.1.1.md` |
| D1 v2.1.1 | Authoritative high-level requirements | `docs/design/IM_D1_High_Level_Game_and_Engine_Design_v2.1.1.md` |
| D6A v0.2.1 | Institutional and faction reference | `docs/worldbuilding/IM_D6A_Factions_Institutions_and_Political_Identity_v0.2.1.md` |
| D6B v0.2.1 | Historical setting reference | `docs/worldbuilding/IM_D6B_History_and_Civilization_v0.2.1.md` |
| D10 v0.1.1 | Roadmap, evidence and acceptance gates | `docs/implementation/IM_D10_Development_Roadmap_v0.1.1.md` |
| ADR-001 v1.0.1 | Significant establishment decisions | `docs/decisions/IM_ADR-001_Independent_Project_Establishment_v1.0.1.md` |
| Project Instructions | Collaboration workflow; apply in ChatGPT Project settings | `docs/ChatGPT_Project_Instructions.md` (planned) |

D0 governs product vision; D1 governs high-level requirements; downstream contracts refine rather than contradict them. D6B's established historical facts take precedence over inconsistent provisional accounts in D6A. D10 governs delivery, not product scope. ADRs record decisions and their rationale; a later ADR does not silently override governing requirements. Explicit owner approval and corresponding document updates are required when decisions change. D3–D9 (excluding retired D2) are developed incrementally according to D10; D6A/B are content references, not substitutes for the future D6 technical specification.

For subsequent ADRs, record identifier, title, date, status, context, decision, alternatives, consequences, evidence, outstanding obligations and affected documents. Retain superseded records with explicit links to successors. This lightweight convention is proposed for review; no ADR tooling is required.

## 9. Observed repository evidence

Owner-provided PowerShell output showed the local working directory, clean `main` synchronized with `origin/main`, and this history at the time of verification:

```text
d4899aa Establish Interstellar Meridian project identity and licensing
50ef0e2 Revise README for clarity and additional details
33cd6e7 Initial commit
```

Tracked files observed: `.gitignore`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`. Local directories observed: `docs/architecture`, `docs/decisions`, `docs/design`, `docs/implementation`, `docs/worldbuilding`. Empty directories are not tracked by Git. Prior remote inspection confirmed both license files on `main` and Apache-2.0 repository metadata. An earlier README stash was observed; final disposition is not confirmed. No Cargo manifest or Rust source appeared in the supplied tracked listing.

The subsequent T2 audit directly retrieved the same three commits, four root files and complete license and README texts at d4899aa79dfc35bf9b8591b9a2f18a6fc6405352. The owner confirmed the new Project configuration. Final remote and local checks remain pending. No repository modifications, builds or tests were performed in preparing this ADR.

## 10. Consequences and trade-offs

**Benefits:** independent architecture and creative direction; small auditable history; explicit ownership and licensing boundaries; fewer irrelevant compatibility obligations; coherent documentation for contributors and AI-assisted work.

**Costs:** foundational systems must be implemented and tested independently; no inherited content library or compatibility test suite; original asset production and provenance tracking are required; documentation migration and validation consume effort before coding; title and separate content rights need attention.

**Mitigation:** build the smallest meaningful simulation and flight prototypes first, use licensed dependencies where justified, maintain acceptance evidence and avoid speculative systems. Reassess significant technical choices using measured evidence at the appropriate gate.

## 11. T2 closure obligations

| D10 criterion | Remaining action |
|---|---|
| T2-01–03 | Retain independent repository/clone/history evidence; recheck at closure |
| T2-04–05 | Prior audit inspected full texts and README; verify updated licensing notice and README after push |
| T2-06–07 | Review editorially reconciled documents; no change to established canon |
| T2-08 | Approve D10 roadmap and its proposed acceptance procedures |
| T2-09 | Review and accept this ADR |
| T2-10 | Owner confirmed Project setup; replace Sources and instructions with reconciled revisions |
| T2-11 | Commit and push approved documentation; verify branch, remote, tracked files and clean synchronized state |

Inspect the earlier README stash before discarding it if still present. Track formal trademark clearance and separate artwork/music/audio licensing as open governance items with owner disposition and release-related obligations. Do not represent them as completed. Do not advance to T3.1 until the owner accepts T2 against D10; no runtime tests are required by this ADR itself.

## 12. Source basis and disposition

Prepared from the owner-approved naming, independence, technology and licensing decisions in the project establishment conversation; the owner-provided Git/PowerShell evidence and prior remote inspection; D0 v1.1.1 §13–14 and closing disposition; D1 v2.1.1 §§10–16 and Appendix B; D6A/D6B v0.2.1 authority and setting sections; D10 v0.1.1 §§1–3 and closure requirements. All five migrated source documents were available for inspection. The historical decisions are recorded here without promoting provisional mechanics, lore or implementation contracts. Local decision identifiers, ADR format and proposed file naming are editorial conventions, subject to owner review.

**Next action:** Review the reconciled baseline, replace Sources/settings, publish and verify the repository, and complete T2 acceptance. This ADR does not close T2 or authorize T3/T4.


**Reconciliation record (2026-11-26):** Editorial patch of 1.0 to 1.0.1; active references updated, owner-approved CC BY 4.0 documentation/lore policy incorporated, and outstanding T2 evidence distinguished from completed checks. Earlier migration records describe the preceding revision. No new gameplay, canon or technical implementation is authorized.
