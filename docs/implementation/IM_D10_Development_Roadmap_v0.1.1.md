# D10 — Development Roadmap and Acceptance Gates

**Project:** Interstellar Meridian  
**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; see [license](https://creativecommons.org/licenses/by/4.0/). Official branding and separately identified third-party material excluded.  
**Version:** 0.1.1 | 2026-11-26  
**Status:** Proposed implementation roadmap for owner review; no implementation authorization.  
**Authority:** D0 v1.1.1 and D1 v2.1.1 govern requirements; D6A v0.2.1 and D6B v0.2.1 govern worldbuilding at their stated canon/proposal status. D10 sequences work, specifies evidence and records gates; it does not supersede product requirements.  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian | `main` | Windows x64 | Rust + Bevy | MIT OR Apache-2.0 (project-owned code).

## 1. Delivery strategy

Prove the smallest meaningful causal universe and satisfying flight before expanding content, simulation scale or engineering complexity. A milestone is an evidence boundary, not a calendar deadline. Do not start the next gate until the current gate is accepted by the owner. A narrowly scoped exploratory spike may be explicitly authorized within the current gate, but does not constitute acceptance of a later gate. No dependency version, numerical performance budget, final crate structure or physics model is selected here.

Status vocabulary: **Pending**, **In progress**, **Blocked**, **Ready for review**, **Accepted**. Record exceptions as explicit decisions with impact, follow-up and owner approval; do not silently waive failed criteria. A proposed criterion becomes binding when the owner approves the relevant implementation contract. Preserve the distinction between a document review and an executed test.

## 2. Gate register and dependencies

| Gate | Deliverable | Prerequisite | Status at issue |
|---|---|---|---|
| T0 | Product vision | None | Accepted at original substantive scope; migration review separate |
| T1 | High-level design | T0 | Accepted at original substantive scope; migration review separate |
| T2 | Independent project establishment | T1 | In progress |
| T3 | Technical foundation contracts | T2 | Pending |
| T4 | Executable Rust/Bevy foundation | T3 | Pending |
| T5 | Headless causal sandbox | T4 | Pending |
| T6 | Flight graybox | T4; ordered after T5 by default | Pending |
| T7 | Integrated playable vertical slice | T5 and T6 | Pending |

T5 and T6 are complementary proofs. The owner may explicitly reorder them or authorize a limited handling spike before T5 acceptance, as contemplated in D1 §13. Do not lock a production economy or final world scale before flight feasibility is demonstrated. T7 is not release readiness.

## 3. T2 — Independent project establishment

**Goal:** A clean, independently authored project, with accepted documents and a reproducible handoff to the new ChatGPT Project.

| ID | Acceptance criterion | Evidence / current disposition |
|---|---|---|
| T2-01 | New independent remote exists | Verified via repository metadata in prior conversation |
| T2-02 | No inherited legacy history | Owner supplied three-commit log; recheck before closure |
| T2-03 | Local Windows clone works | Owner supplied PowerShell output from E:\\RustProjects\\InterstellarMeridian |
| T2-04 | Both code licenses tracked and correct | Pass: presence and complete text inspected during T2 audit; not legal certification |
| T2-05 | README declares project identity, status and dual licensing | Prior README text passed; updated non-code license statement prepared, publication pending |
| T2-06 | D0 v1.1.1 and D1 v2.1.1 reconciled and reviewed | Editorial reconciliation prepared; explicit final owner disposition needed |
| T2-07 | D6A v0.2.1 and D6B v0.2.1 migrated, canon/proposal distinction preserved | Editorial reconciliation prepared; owner review needed; canon preserved |
| T2-08 | D10 approved | This draft; pending review |
| T2-09 | ADR-001 accepted | Prepared; final owner acceptance pending |
| T2-10 | New ChatGPT Project configured with six active documents and instructions | Owner confirmed configuration and upload of previous versions; replace with reconciled versions |
| T2-11 | Local and remote synchronized with documentation committed | Initial setup synchronized; final check pending |

Known baseline from owner output: `d4899aa` identity/licensing, `50ef0e2` README revision, `33cd6e7` initial commit. The prior direct audit confirmed HEAD d4899aa79dfc35bf9b8591b9a2f18a6fc6405352; final synchronization still requires a new check. GitHub metadata recognized Apache-2.0; both license files were present. Confirm README explicitly offers either license. Earlier README stash may remain; inspect before discarding. Empty folders are not tracked by Git. No Rust source or Cargo manifest existed in the supplied tracked file listing.

Closure checklist: review reconciled documents, updated README and licensing notice; retain lore proposals as proposals; approve D10, ADR and instructions; replace Sources and settings; commit documents; verify branch, remote, tracked files and clean status. Project-owned documentation and original fictional lore are licensed under Creative Commons Attribution 4.0 International (CC BY 4.0), https://creativecommons.org/licenses/by/4.0/. Project-owned source code remains MIT OR Apache-2.0. The game name, official logos and other source-identifying branding are excluded from the CC grant as branding; no trademark or endorsement permission is granted. Artwork, music and audio not expressly released as documentation or lore require separate licensing before distribution. Third-party materials retain their applicable terms. Formal title clearance and other asset licensing remain release-related obligations.

Suggested PowerShell evidence (run at repository root):

```powershell
git fetch origin
git remote -v
git branch -a
git log --oneline -10
git status
git ls-files
git stash list
git rev-parse HEAD
git rev-parse origin/main
```

**Stop:** T2 acceptance is required before T3. Do not initialize the Rust application yet.

## 4. T3 — Technical foundation contracts

**Goal:** Resolve only the contracts necessary for T4 and the planned proofs. Produce reviewable D3/D4/D9 sections, ADRs where warranted, and a T4 task/test plan. D5/D6/D7/D8 contracts may be developed at the later relevant gate; do not invent complete systems now.

| Package | Dependencies within T3 | Deliverable and measurable review evidence |
|---|---|---|
| T3.1 Toolchain and engine | T2 | Record exact Rust toolchain, target, Bevy version/features, supported Windows prerequisites, official documentation/API references, compatibility and licenses; verify local toolchain availability and minimal build feasibility in an explicitly authorized disposable probe if needed. |
| T3.2 Application and state boundaries | 3.1 | D4: minimal boot/headless/presentation boundary, state ownership matrix, command validation, schedule responsibilities; identify where each persistent fact is written. |
| T3.3 Simulation clock and scheduling | 3.2 | D4: explicit simulation time, pause/step, deterministic order and catch-up policy; specify input/output fixtures for advancement and simultaneous events. No offline advancement assumed. |
| T3.4 Deterministic randomness | 3.3 | D3/D4: seed/version, stream separation and ordering rules; define reproducibility scope, canonical comparison and golden fixture policy. |
| T3.5 Persistent identity and lifecycle | 3.2, 3.4 | D4: durable IDs, creation/retirement, lookup, projection and handoff; test duplicate IDs, dangling references, re-entry and retirement. |
| T3.6 Content definitions and validation | 3.5 | D9: minimal versioned definition schema, instance/definition separation, reference resolution, load ordering and actionable error behavior; valid/invalid fixtures. |
| T3.7 Spatial coordinate architecture | 3.2, 3.5 | D3/D4: topology, system/local frames, render-relative transforms and precision policy; specify representative ranges and round-trip tolerances before implementation. |
| T3.8 Persistence foundation | 3.3–3.7 | D9: committed snapshot boundary, versions, pending effects, validation and failure behavior; define canonical round-trip fixture and unsupported-version behavior. |
| T3.9 Testing and CI | 3.1–3.8 | D4/D10: exact commands, Windows runner/toolchain, headless tests, fixtures, diagnostics, formatting/lint and benchmark recording policy. |
| T3.10 T4 implementation plan | 3.1–3.9 | Ordered file-level tasks, tests before implementation, dependencies, acceptance matrix, scoped performance budgets and owner approval. |

T3 review criteria: every contract identifies owner, inputs, outputs, invariant, failure behavior and tests; dependency APIs are verified against selected versions; license and platform constraints documented; no speculative crates or dependencies; quantitative thresholds needed for T4 are selected and justified or explicitly deferred to a named gate. A disposable probe is not the production foundation and must not be presented as T4 acceptance.

**Stop:** No production foundation implementation until owner accepts T3 and the T4 plan.

## 5. T4 — Executable Rust/Bevy foundation

**Goal:** A small native Windows application with a testable authoritative core, reproducible builds and diagnostics. No gameplay economy or production flight system required yet.

| Package | Prerequisites | Deliverable / acceptance evidence |
|---|---|---|
| T4.1 Bootstrap | T3 | Minimal Cargo package, accepted dependency versions, lockfile, license metadata, native Windows build and application launch. |
| T4.2 Runtime boundary | 4.1 | Headless execution without graphics/audio runtime; presentation can start separately; headless smoke test. |
| T4.3 Clock and schedules | 4.2 | Tests for pause, explicit stepping and ordered transitions under fixed inputs. |
| T4.4 Seed and identity | 4.3 | Reproducible seeded fixture, durable IDs independent of ECS handles, duplicate/referential failure tests. |
| T4.5 Content validation | 4.4 | Load a minimal valid definition set; malformed/unknown references rejected with useful diagnostics and no partial authoritative state. |
| T4.6 Spatial and snapshot primitives | 4.4–4.5 | Implement only approved primitives needed by T5/T6; precision and minimal snapshot tests per T3 contracts. |
| T4.7 CI and closure | 4.1–4.6 | Clean Windows checkout build/run, headless suite, formatting/lint policy, CI evidence, dependency/license inventory, recorded diagnostics. |

Suggested Cargo commands, subject to T3 feature/target design: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build`. Record actual feature flags and runner in T3, not assumed here. Confirm that headless execution does not initialize graphics or audio; whether it shares compile-time dependencies is a separate architecture decision. Validate snapshot behavior at foundation scope; full material continuation belongs to T5.

**Stop:** Native run and automated evidence must pass; no broad content, combat, rendering or fleet work before the foundation is accepted.

## 6. T5 — Causal world simulation sandbox

**Goal:** Demonstrate that world operations and material consequences are real, persistent, reproducible and independent of the player. Headless first.

Candidate fixture from D0 §11.2, D1 §8 and D6A §16: two connected systems; contrasting factions; a few stations; one production chain; one civilian route; one military shipment and readiness-bearing asset; an impending contested operation. These are test-fixture choices, not canonical starting geography or mandatory faction relationships. Approve exact fixture, starting stocks, rates, routes, event order and expected outcomes before coding.

| Package | Prerequisites | Deliverable |
|---|---|---|
| T5.1 Contract and fixture | T4 | Minimal D3/D6/D7/D9 contracts, deterministic fixture and expected stock ledger; material event semantics, tolerances and performance budget agreed. |
| T5.2 Geography and stocks | 5.1 | Stable systems/sites/links and inventory ownership; validated stock transfer and explicit sources/sinks. |
| T5.3 Production and demand | 5.2 | Autonomous production/consumption with input constraints and recorded changes. |
| T5.4 Logistics | 5.3 | Persistent shipment dispatch, progress, exclusive terminal outcome and exactly-once destination credit. |
| T5.5 Operations and intervention | 5.4 | Readiness and contingent operation resolution from shared facts; unattended path plus supply, sabotage and direct combat-outcome inputs represented abstractly. |
| T5.6 Detail handoff | 5.4–5.5 | Active/remote representations reconcile identity, quantities, progress and pending effects without duplicate authority. |
| T5.7 Persistence and replay | 5.6 | Mid-transit snapshot/restore and canonical replay, including pending outcomes. |
| T5.8 Explanation and closure | 5.7 | Inspect causal references, run acceptance matrix, record benchmark and regressions. |

### 6.1 Required causal acceptance matrix

| ID | Test | Pass evidence |
|---|---|---|
| CS-01 | No intervention | Advance fixture without player input; production, consumption, transport and operation advance under declared conditions. |
| CS-02 | Delivery | Dispatch debit/reservation and destination credit reconcile exactly once; stable shipment identity; repeated resolution cannot credit again. |
| CS-03 | Loss/capture/diversion | Exercise supported exclusive outcomes; lost cargo cannot arrive; captured/diverted quantities reconcile; shortage changes a downstream fact. Define partial outcomes only if supported. |
| CS-04 | Intervention | Compare same seed/start state with and without accepted intervention; material outcome differs via shared facts and propagates to at least two related subsystems (e.g. repair readiness and security). |
| CS-05 | Detail transition | Compare continuous and promoted/demoted execution; identity, cargo, progress and pending outcome retained; no duplicate authoritative entity. |
| CS-06 | Save mid-transit | Save before resolution, restore and continue with identical inputs; canonical material state and pending effects equivalent to uninterrupted execution. |
| CS-07 | Replay | Same version, seed, definitions and ordered inputs produce equivalent canonical state and ordered material events in the documented reproducibility scope. |
| CS-08 | Explanation | Trace operation outcome to readiness, stock and shipment facts; distinguish recorded causes from presentation narrative. |

Invariant checks: no negative stock unless explicitly modeled and approved; quantity accounting across stock and in-transit ownership; explicit production/consumption/destruction; at most one terminal resolution; failed commands preserve state; no player-presence requirement; committed state only in persistence. Use per-step assertions where practical, not merely final balances. A scripted branch that bypasses shared facts fails the causal proof. Carrier survival/loss may be abstract, not rendered combat. Record evidence for all four D0 intervention scenarios, including non-participation.

**Stop:** All eight CS scenarios must pass with retained fixtures, reproducible commands and owner review. Do not claim a complete economy, AI, mission system or rendered fleet combat.

## 7. T6 — Flight graybox

**Goal:** Establish satisfying controllable flight, legibility and credible scale on Windows before production art or broad world expansion. Approve a minimal D5 contract with D3/D4 coordinate interfaces; exact handling model remains open until evaluated.

| Package | Prerequisites | Deliverable / evidence |
|---|---|---|
| T6.1 Handling contract | T4; T5 accepted by default | Define controls, camera, movement assumptions, test course, travel tasks, measurable precision and responsiveness criteria; owner approves rubric before coding. |
| T6.2 Flight prototype | 6.1 | One controllable ship, configurable inputs, basic motion and camera; automated motion/command checks plus Windows playtest. |
| T6.3 Navigation and scale | 6.2 | Readable target and HUD, representative local/system traversal, coordinate precision and travel transition tests; no arbitrary galaxy size. |
| T6.4 Approach and docking | 6.3 | Graybox approach and dock/launch interaction; valid/invalid commands and deterministic state transitions where required. |
| T6.5 Evaluation | 6.2–6.4 | Record build, hardware, settings, measured timings/precision/frame behavior, task completion and owner feedback. Revise handling until accepted. |

Proposed playtest rubric (operationalize in T6.1): start/stop and turn on command; acquire and maintain target; navigate between landmarks; approach and dock without confusing controls; launch; representative traversal with meaningful scale and comprehensible feedback; no unacceptable jitter or unreadable HUD. Record failed attempts and causes; test configurable controls and non-color-only cues. Define numerical tolerances, input latency and frame budgets with reference hardware before implementation; qualitative satisfaction requires owner review and cannot be replaced by a unit test.

**Stop:** Owner accepts both measured results and handling experience. No requirement for final ship art, atmospheric flight, large battles or full physics realism.

## 8. T7 — Integrated playable vertical slice

**Goal:** Join proven flight and causal simulation in one small persistent playable region. Demonstrate the design maxim through actual gameplay.

| Package | Prerequisites | Deliverable |
|---|---|---|
| T7.1 Integration contract | T5 + T6 | Approve minimal region, ship, stations, route, commodity, interface ownership, save boundary, expected player loop and performance budget. |
| T7.2 Shared world and travel | 7.1 | Graybox scene projects authoritative world; launch, transit and docking update shared state; no duplicate simulated ship. |
| T7.3 Market and interaction | 7.2 | Validated trade with real stocks, credits and inventory; readable feedback; rejected transactions leave state consistent. |
| T7.4 Consequential intervention | 7.3 | At least one accessible supply/security/prospecting interaction (select in contract) changes a persistent cross-system outcome via real causal facts. Minimal resource site and exploration scope assessed against D0 §11.3. |
| T7.5 Save and resume | 7.4 | Save/reload at meaningful points including transit if supported; preserve player/world IDs, inventory, operations and pending effects. |
| T7.6 Integrated evaluation | 7.5 | Complete Windows playtest, headless regressions including CS-01–08, performance capture, documented limitations and owner acceptance. |

### 8.1 Proposed end-to-end acceptance scenarios

| ID | Scenario | Pass evidence |
|---|---|---|
| VS-01 | Start and launch | New session starts from approved fixture; player launches controllably; state agrees with simulation. |
| VS-02 | Navigate and travel | Reach destination in another system through approved transit; stable identity and authoritative location. |
| VS-03 | Dock and trade | Dock at eligible station, execute valid trade, reject invalid trade; stocks/credits/inventory reconcile. |
| VS-04 | Autonomous world | During ordinary play or deliberate non-intervention, transport and operations continue according to simulation time. |
| VS-05 | Intervention | Player changes a real world fact and observes persistent cross-system consequence; compare with no-intervention fixture. |
| VS-06 | Save and reload | Restored player and universe retain relevant state; continuation produces equivalent material results for same accepted inputs. |
| VS-07 | Regression and usability | All applicable T4/T5/T6 evidence passes or is explicitly revalidated; owner completes loop and accepts usability under approved budget. |

T7 scope is intentionally small. D0 proposes a resource site and trading/prospecting/security loop; exact minimum combination is to be approved at T7.1 rather than silently omitted or expanded. Fleet ownership, advanced tuning, large battles, full mission library and investment are not prerequisites for this first slice.

**Stop:** No broader feature expansion until integrated proof accepted. T7 is an internal playable baseline, not a release claim.

## 9. Post-T7 expansion backlog (not authorized milestones)

Order below is a proposed dependency-oriented planning aid; define new gate IDs and criteria after T7 review. Prioritize features that deepen connected careers and observable causality over geographic size.

| Candidate increment | Required foundation | Evidence to define before implementation |
|---|---|---|
| Combat and damage | Flight, identity, persistence | Targeting, damage, destruction and salvage reconcile across detail levels. |
| Mining and exploration | Geography, travel, content, economy | Discovery changes knowledge and creates accountable resource/research opportunities; meaningful single-ship loop. |
| Faction policies, law and autonomous operations | Causal sandbox and actor knowledge | Distinct responses to same disturbance; bounded jurisdiction, resources and evidence. |
| Authored operational missions | Valid world facts and operations | Template binds to real eligible entities; NPC completion possible; destroyed target invalidates or resolves assignment. |
| Ship equipment and tuning | Ship/inventory contracts | Compatibility validation, physical parts conserved; named tuning stamps reusable without duplicating items. |
| Rank and delegated command | Faction authority and operations | Unauthorized orders rejected; rank and appointment bound scope; rank distinct from reputation and ownership. |
| Fleet ownership and capital battles | Combat, logistics, command, detail handoff | Multiple capital ships in player fleet; benchmark large-battle behavior; persistent losses and readiness, no duplicated assets. |
| Industrial investment | Economy, ownership and legal contracts | Existing operations can be invested in; returns and risks reflect real outcomes; no player construction of stations/factories. |
| Content/modding expansion | Versioned native schemas and validation | Independently authored extension loads with clear compatibility and failure reporting; deterministic behavior in declared scope. |
| Visual/audio identity and accessibility | Validated presentation boundaries | Original or licensed assets, readable luminous aesthetic, configurable interaction; measured performance. |

Mod-aware design and accessibility begin earlier; backlog entries expand capability, not defer basic requirements. No commitment to implement all 38 organizational concepts, all careers or all historical events at launch. D6A and D6B guide content selection, not simulation count or algorithms.

## 10. Measurement and performance budgets

Do not invent production targets before representative hardware, workloads and requirements exist. Before each implementation gate, record a budget table with metric, unit, workload, hardware, build profile, sample count, statistic, threshold, measuring command and failure action. Use Windows reference hardware and headless fixtures as appropriate. Establish baselines before optimization; avoid universal performance claims from a single workstation.

| Gate | Minimum metrics to operationalize |
|---|---|
| T3/T4 | Build success and reproducibility; headless runtime initialization; fixed-step test timing; content load/error behavior; spatial precision. |
| T5 | Simulated time per wall-clock time, tick/operation throughput, memory, canonical comparison, snapshot size/time, handoff cost; fixture counts and duration. |
| T6 | Frame-time distribution, control responsiveness, traversal/task completion, coordinate error and visible jitter; scene and settings. |
| T7 | Integrated frame/tick budget, save/reload latency and size, route/market interaction timing, loop completion and regressions. |
| Later fleet increment | Active and remote fleet counts, simultaneous combat operations, transition costs, material conservation and frame/tick metrics. |

Functional invariants such as duplicate-credit count = 0, unresolved references in accepted fixture = 0, failed required tests = 0 are non-negotiable unless the governing contract is formally revised. Numerical latency/FPS/scale thresholds remain unset; choose before relevant coding. Record exact test environment and distinguish regression thresholds from aspirations.

## 11. Implementation workflow and evidence record

Each work package follows: inspect actual repository → identify governing requirements → define acceptance test and failure case → implement smallest change → run targeted tests → run relevant regressions → inspect diff and dependency/license effects → commit → record evidence → obtain approval at gate boundary. Use precise file paths and insertion/replacement anchors for existing code; do not guess future files. If a test fails, report actual failure and fix within current scope. No claim that a command was run unless output is available.

For each gate, maintain a record under `docs/implementation/` (exact naming may be approved at T3):

```text
Gate / package:
Governing document versions and requirement IDs:
Scope and exclusions:
Commit SHA / branch / clean status:
Toolchain, dependencies, features and environment:
Fixture seed, version and ordered inputs:
Acceptance ID | command or review method | expected | actual | status | evidence path:
Performance budget and measured result:
Known failures, limitations and open risks:
ADRs / changed documents:
Owner disposition and date:
Next authorized package:
```

For GUI playtests, include build identity, control settings, scenario, observations and owner decision. For deterministic tests, include canonical comparison rules and event ordering; no bitwise rendering requirement. Use CI logs and test output rather than checkmarks without evidence. Tagging milestones is optional; record immutable commit SHA regardless.

## 12. Risks and decision register

| Risk / open choice | Mitigation | Decision gate |
|---|---|---|
| Dependency/API mismatch | Verify official versioned docs, build and platform compatibility | T3.1 |
| Premature ECS/crate architecture | Minimal boundaries, actual ownership and measured need | T3.2/T4 |
| Non-deterministic ordering or randomness | Explicit scheduling/streams, canonical fixture comparison | T3.3–4/T5 |
| Duplicate state across detail levels | Durable IDs, exclusive authority, reconciliation tests | T3.5/T5.6 |
| Spatial precision vs scale | Representative coordinate and motion tests | T3.7/T6 |
| Broken persistence or pending events | Versioned committed snapshots and continuation tests | T3.8/T5.7 |
| Economy appears scripted | Shared facts, intervention/control fixtures, inspectable causal links | T5 |
| Flight not enjoyable | Graybox and owner playtest before scaling | T6 |
| Scope growth | Tiny fixture and explicit non-goals; gate acceptance | All |
| IP/asset provenance | Dual code licensing; CC BY 4.0 documentation/lore; separate branding and other asset rights; verify dependencies and rights | T2 onward; before distribution |
| Title clearance | Formal trademark clearance outstanding; record owner disposition | Before relevant public distribution/branding commitments |
| Stale document references | Cross-check active versions and ADRs before T2 closure | T2 |

## 13. Documentation ownership and handoff

D0 vision; D1 engineering/product requirements; D3 universe/geography; D4 technical architecture; D5 flight/ship/combat; D6 economy/factions/logistics; D6A/B worldbuilding; D7 agents and command; D8 missions; D9 persistence/content/modding; D10 roadmap/evidence; ADRs record significant decisions. D2 retired. No requirement to produce empty full specifications before use. Document contracts in the relevant owner document when needed, link rather than duplicate requirements, and revise affected documents on approved changes.

Active initial Sources: D0 v1.1.1, D1 v2.1.1, D6A v0.2.1, D6B v0.2.1, D10 v0.1.1, ADR-001 v1.0. Project instructions are maintained separately and applied in Project settings. Old project documents remain archived and outside active authority.

## 14. Source register and revision limits

- D0 v1.1.1, especially §§5, 8, 11–17: causal proof, candidate simulation levels, early non-goals and accepted vision.
- D1 v2.1.1, especially §§5–9, 12–18 and appendices: engineering boundaries, gates, CS acceptance matrix, testing and open choices.
- D6A v0.2.1, especially institutional causality, phased activation and authority sections: candidate sandbox organizations and constraints.
- D6B v0.2.1: setting facts and provisional historical content; not an implementation mandate.
- Owner-provided repository setup output and prior direct GitHub verification in conversation: T2 evidence at time supplied.

This draft elaborates work-package dependencies, proposed acceptance details and post-T7 backlog without selecting technical implementations. No live repository inspection, dependency research, code modification or tests performed for this document. Numerical budgets, fixture details and later gate order require owner approval. The stale active worldbuilding references were reconciled in this editorial revision. No milestone is newly accepted by issuance of this document.

**Current stop point:** T2 remains open. Review and synchronize the reconciled documentation baseline, update Sources and settings, then accept T2 before beginning T3.1.


**Reconciliation record (2026-11-26):** Editorial patch of 0.1 to 0.1.1; active references updated, owner-approved CC BY 4.0 documentation/lore policy incorporated, and outstanding T2 evidence distinguished from completed checks. Earlier migration records describe the preceding revision. No new gameplay, canon or technical implementation is authorized.
