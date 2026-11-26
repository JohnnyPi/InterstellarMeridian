# D1 — High-Level Game and Engine Design

**Project:** Interstellar Meridian  
**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; see [license](https://creativecommons.org/licenses/by/4.0/). Official branding and separately identified third-party material excluded.  
**Version:** 2.1.1 — greenfield migration; review copy  
**Date:** 2026-11-26  
**Status:** T1 substantive baseline accepted; v2.1 migration for owner review; no implementation authorization  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian  
**Platform:** Windows x64 first; Rust + Bevy  
**Code license:** MIT OR Apache-2.0 (project-owned code)

## 0. Authority and reading guide

D0 Project Vision and Design Principles v1.1 carries the accepted vision and subsequent owner decisions on independent development. D1 v2.0 was accepted on 2026-11-26; this v2.1 migration retains its substantive requirements and is proposed to supersede it upon owner review. Appendix A records the migration. D0 owner-established direction governs; candidate mechanics remain candidates. D6A and D6B v0.2.1 provide institutional and historical context. D6B supersedes inconsistent earlier historical proposals in D6A. D10 establishes implementation milestones; ADR-001 records independent project establishment. Both await final owner acceptance.

**Terminology:** SHALL denotes a normative high-level D1 requirement carried forward from the accepted baseline or established owner direction. Candidate denotes an approach for evaluation, not an approved mechanic. Deferred means no implementation commitment in the current gate. Gate criteria describe required evidence, not evidence already obtained.

## 1. Overview and identity

Interstellar Meridian is an independent, open-source, pilot-first spacefaring simulation. It emphasizes a connected, evolving region of space, meaningful individual agency, enjoyable flight, consequential discovery, and a luminous original visual identity. The project is greenfield, with original code, native content and original or independently licensed assets. No inherited code, assets, repository history, formats or compatibility requirements are part of its baseline. The title is owner-selected; formal trademark clearance remains outstanding.

The player begins as an independent pilot with modest resources and can pursue self-directed careers, own ships, command capital ships and fleets, and invest in existing enterprises. The world evolves during game-time advancement without requiring player participation. There is no mandatory central storyline, faction membership or combat career. Piloting remains central; player construction of stations and industrial installations is excluded.

## 2. Product experience and gameplay loops

The core loop is observe opportunities → choose a goal → prepare ship and cargo → launch and navigate → interact with real world actors or sites → resolve outcomes → dock, trade, repair or refit → observe consequences and choose again. A player can decline opportunities and the world continues. News, records, markets, traffic and communications should make important consequences intelligible without granting omniscient knowledge.

Long-term careers include trading/logistics, mining/prospecting, bounty hunting, piracy, diplomacy and exploration. Salvage, escort, smuggling, scientific investigation, rescue and military contracting are candidate extensions (D0 §6). Careers share resource, inventory, travel, security, knowledge and reputation systems rather than independent minigames. Early implementation proves a narrow loop, not all careers.

## 3. Design pillars and requirement traceability

| ID | Required direction | Player-facing evidence | D0 anchor |
|---|---|---|---|
| P1 | Density over diameter | Distinctive systems have economic, political or geographic relationships | §3 P1, §4 |
| P2 | Independent, understandable causality | Shortages and conflict outcomes have inspectable causes | §3 P2, §5 |
| P3 | Individual agency without guaranteed centrality | Intervention changes conditions; absence does not freeze events | §3 P3 |
| P4 | Satisfying flight at meaningful scales | Responsive maneuvering and meaningful traversal | §3 P4, §8 |
| P5 | Consequential discovery | Sites can create resources, knowledge or new opportunities | §3 P5, §7 |
| P6 | Connected careers | Professions affect shared world state | §3 P6, §6 |
| P7 | Luminous legible frontier | Distinct environments and readable action | §3 P7, §9 |
| P8 | Self-directed progression | Multiple viable careers; no compulsory storyline | §3 P8 |
| P8A | Captain first, owner second | Fleet ownership expands but does not replace flight | §3 P8A, §6.1 |
| P9 | Deterministic foundations and selective detail | Persistent outcomes and reproducible material transitions | §3 P9, §5.3 |
| P10 | Native extensibility | Validated content definitions and documented extensions | §3 P10 |

## 4. Scope and non-goals

| Scope | Deliverable intent |
|---|---|
| Foundation | Native Windows executable, headless test path, time, identity, deterministic generation, validated data and diagnostics |
| Causal sandbox | Small headless world with production, inventory, persistent shipments and readiness-dependent consequences |
| Flight graybox | Responsive flight, camera, scale, minimal transit; evidence before locking handling |
| Playable loop | Launch, navigation, docking, trade, inter-system travel, autonomous activity, intervention and save/load |
| Subsequent expansion | Combat, exploration and mining, additional careers, faction operations, mission binding, advanced content |
| Long-term product | Multi-capital-ship fleets, large battles, rank-dependent command, named tuning stamps, specialized parts, investment and richer living world |

First-slice exact content and counts are not fixed. Two systems and one production chain are proposed test fixtures, not final universe limits. D0's 6–12 system / 3–4 regional faction exploration range is nonbinding and is not the count of governments in the setting.

Early non-goals: multiplayer, seamless planetary landing, on-foot station or planet exploration, photorealism, full-fidelity remote ship physics, complete career catalog, universal procedural quest generator, full scripting platform. Full ship construction/freeform hull assembly is not planned; upgrading existing hulls is the direction. Player-built stations or industrial installations are a product exclusion, not merely deferred. Secondary Linux/macOS support remains future evaluation, not a release gate.

## 5. Technology and engineering choices

Rust and Bevy on Windows x64 are established. Cargo, Rust automated testing, Git and GitHub Actions remain the proposed development foundation. Serde, tracing, wgpu and WGSL are retained as technology candidates or engine-associated facilities subject to version/API review; no new version is selected here. Physics, spatial indexing, content serialization, scripting, audio extensions, UI tooling, asset conversion and profiling tools are evaluated when a concrete need exists. Assess compatibility, licenses, maintenance, build reproducibility, platform support and performance. T2 establishes the independent repository and documentation baseline; T3 validates the toolchain, dependency versions and APIs before implementation. Do not prescribe a crate tree before concrete requirements justify it.

## 6. Architecture and state ownership

The engine SHALL separate authoritative simulation, gameplay command/interaction handling, presentation, content definitions and persistence. Headless execution SHALL not require graphics or audio. ECS composition is preferred for active runtime objects; distant state may be represented by compact records or aggregates outside live ECS entities. Logical boundaries do not mandate separate Cargo crates.

Conceptual flow: validated definitions + seed + saved material state + accepted inputs → simulation transitions → committed state and events → presentation/read models and persistence. Gameplay issues requests subject to validation; presentation never independently decides authoritative combat, trade or mission outcomes. Persistence captures committed state and restores validated state, not transient renderer or ECS handles.

| State domain | Proposed logical authority | Mutation boundary |
|---|---|---|
| Simulation time and event order | Simulation clock/scheduler | Explicit advancement and ordered steps |
| World definitions and topology | Validated content/generator | Versioned generation or explicit world change |
| Ship/item/actor identity | Persistent world identity model | Creation, transfer, retirement |
| Active pose and motion | Local simulation | Flight/physics steps and handoff |
| Stocks and ownership | Economy/inventory authority | Validated transactions, production, transfers |
| Shipments and transit | Logistics authority | Dispatch, progress, terminal outcome |
| Faction control and permissions | Faction/command authority | Validated policy, rank, delegation and control changes |
| Objectives and assignments | Mission/operation authority | Eligibility, allocation and outcome evaluation |
| Knowledge and reputation | Actor/faction knowledge model | Observation, communication and evidenced consequences |
| Historical records | Committed material-event history | Record significant state changes and causal references |
| Presentation | Non-authoritative view state | Read state and interpolate/display |

The table is a proposed ownership contract, not a selection of storage engines or ECS APIs. Each material fact has one authority at a time; cross-domain changes require explicit reconciliation. Durable IDs SHALL be distinct from transient ECS Entity handles. Avoid implicit system-order dependencies and global mutable ownership. Failed commands must leave consistent state; error reporting and recovery paths are defined in D4/D9.

## 7. Subsystem responsibilities

| Subsystem | Responsibility and integration | Detailed owner |
|---|---|---|
| Application core | Boot, configuration, game states, schedules, pause, diagnostics, shutdown; separate wall/presentation time | D4 |
| Universe and geography | Versioned procedural generation, authored constraints, systems, sites, topology, stable references | D3 |
| Spatial architecture | Galactic topology, system coordinates, local motion, render-relative transforms; precision tests | D3/D4 |
| Entity lifecycle | Identity, content instantiation, local membership, safe retirement, references and handoff | D4 |
| Flight and travel | Responsive controls, motion, navigation, system transit, jump validation, launch/dock | D5 |
| Combat | Targeting, weapons, damage, defenses, destruction and persistent consequences | D5/D7 |
| Ship/equipment | Hull definitions, physical components, inventory, compatible upgrades and tuning | D5/D9 |
| Economy | Stock, production, consumption, prices, transactions and access | D6 |
| Logistics | Persistent routes, services, shipments, progress, security, delivery/loss/diversion | D6 |
| Factions and law | Relationships, asset control, jurisdiction, legal status, reputation, conflict, investment | D6 |
| Agents and fleet AI | Intent, planning, assignment, steering, autonomous operations and command | D7 |
| Missions/events | Authored templates bound to real needs and assets, eligibility, outcome and history | D8 |
| Exploration | Discovery, resource sites, hazards, evidence, knowledge and opportunities | D3/D6 |
| Persistence | Versioned material snapshots, restore validation, migration and pending effects | D9 |
| Content/modding | Native schemas, validation, dependencies and documented extensions | D9 |
| Rendering | Ship/station/celestial visuals, cameras, lighting, effects; consume simulation state | D4/D5 |
| UI/input | Configurable controls, cockpit HUD, navigation, market, communications, accessibility | D4/D5 |
| Audio | Music, interface, spatial effects and communications; not authoritative state | D4 |
| Testing/tooling | Headless fixtures, replay, profiling, content tools and gate evidence | D4/D10 |

### 7.1 Flight, spatial scales and travel

Flight SHALL balance responsive local dogfighting with convincing scale, between arcade and strict Newtonian handling. Mass, thrust and equipment should matter; exact equations, flight assist, inertia and controls are open. Evaluate tactical flight, system transit and interstellar transit as a candidate three-scale model (D0 §8). Travel must offer intelligible choices rather than prolonged empty waiting. Coordinate domains must not be silently conflated. Evaluate double precision, hierarchical coordinates and floating origin with a graybox; no representation is chosen. Launch, docking and travel must preserve identity, cargo, state and time. Exact clearance, fuel, interruption and jump mechanics are D5 decisions.

### 7.2 Economy, supply and security

Production, consumption, transport, trade and loss share material state. Markets must respond to real availability and demand; pricing formulas are to be designed and validated natively. A candidate logistics vocabulary is route (corridor), service (recurring transport obligation), shipment (actual movement). Civilian, commercial and military supply lines persist; discoverable lanes can be used or attacked and are generally safer through policing, not invulnerability. Record stock creation/consumption/transfer/loss and prevent double credit. NPC owners, not players, build infrastructure. Investments in existing enterprises are a long-term capability; ownership rights and return formulas are open.

### 7.3 Factions, agents and command

Factions act without player input using available resources, assets, relationships and information. Separate strategic intent, operational allocation and tactical execution conceptually; no AI framework is selected. Rank governs command scope over faction assets and is separate from reputation, wealth and private ownership. Established progression direction is assigned pilot → formation leadership/selection → target changes → strategic decisions within authorized scope. Titles and exact permission matrix are deferred. Orders must be validated; autonomous agents execute them. Player command replaces eligible decisions, not the simulation rules. Knowledge-based enforcement avoids omniscient retaliation. Large battles and multi-capital-ship fleets are long-term product goals; exact counts and tactical UI require prototypes.

### 7.4 Equipment and progression

Players acquire existing hull designs, upgrade components, attach specialized parts and reuse named tuning stamps. Distinguish physical component instances from reusable configuration recipes; a stamp does not duplicate items. Proposed validation: compatible family, supported tuning limits, available parts and explicit failure reporting. Exact sockets, costs, trade-offs, version semantics and fitting mechanics remain D5/D9 decisions. Fleet autonomy and investment must not make management compulsory. A single-ship career should remain viable.

### 7.5 Missions and exploration

Authored military mission types SHALL bind to actual available ships, formations, targets and operational needs. Candidate template fields: prerequisites, capabilities, target predicates, objectives, completion/failure conditions and text slots. Stable references must resolve and become invalid or reassigned under explicit rules if assets disappear. Distinguish operational success from individual participation; no battle waits for acceptance. Broader procedural missions are a candidate blend of authored framing and world facts, not a universal quest generator commitment. Exploration finds resource fields, wrecks, hidden installations and other consequential sites. Candidate loop: notice, investigate, interpret, decide, change. Knowledge can be distinct from world truth; secrecy, claims, sensors and scientific rewards remain open.

## 8. Simulation model and causal invariants

Selective simulation detail is required; the exact tier definitions are proposed. D0 §5.3 suggests Active (immediate flight/combat), Operational (local routes, production and patrols), Strategic (remote logistics, readiness and control), Dormant/Deferred (stored state with scheduled or analytical catch-up). Choose relevance by gameplay and causal dependencies, not camera visibility. No offline real-world advancement is implied. D3/D4 shall define promotion/demotion, pause, advancement and event ordering.

Proposed minimum invariants for T3/T5:

- Same generation version, seed, content and ordered inputs produce equivalent material outputs under documented supported conditions. Rendering effects need not match bit-for-bit.
- Every persistent actor, item and shipment has a stable identity; local projection never creates a second authoritative copy.
- Material transfers conserve quantities except explicit production, consumption, destruction or other recorded sources/sinks; dispatch and delivery occur at most once.
- A lost shipment cannot later deliver its lost cargo. Partial deliveries, captures and diversions reconcile explicitly if supported.
- Transition between simulation levels preserves identity, progress, resource quantities and pending events.
- Unattended world advancement resolves eligible operations from world conditions, not player participation flags.
- Save/restore preserves material state and pending effects; replay comparisons have a defined canonical state projection.
- Significant consequences retain enough causal references to explain them; history is not required to be a full event-sourced database.

Illustrative chain (D0 §5): shipment loss → input shortage → delayed repair → diminished readiness → changed conflict/security and mission needs. A carrier-war scenario may be used as an abstract fixture, not mandatory fiction or a requirement for rendered fleet combat. At least one intervention should produce persistent effects in two related subsystems in the causal proof.

## 9. Universe, native content and setting

Generate an original connected region with geographic, economic, political, exploratory and social differentiation. A fixed seed and version identify reproducible generated definitions; persistent changes are distinct from generation. Authoring can constrain generation; exact generator grammar, system counts, topology and discovery placement are open.

Native content definitions should cover systems/sites, ships, stations, commodities, equipment/parts, governments/factions, routes, missions, behavior parameters, dialogue, visual/audio resources and extension metadata. Stable IDs, versions, validation and explicit reference resolution are requirements. Runtime instances are distinct from definitions. Schema formats, override rules and scripting runtime await D9 and concrete extension needs.

Setting constraints from D6B §§2–5: humanity originated on Earth and settled another galaxy; Earth is not an ordinary destination; contemporary civilizations are human-origin; other life and remains of extinct/disappeared civilizations exist. Seven governments descend from distinct generation-ship expeditions. Their established names are The Concordancy, Albion, Codex, The Gold League, Choral, Bastion and The Rhizome. A devastating war precedes the fragile peace. Bastion's emergency awakening is established; exact chronology, calendar, other expedition origins, war names and precursor mysteries remain provisional. The 5,000-year history is established, but its epoch interpretation is open. No requirement to instantiate every government in the initial region. D6A v0.2.1 is the institutional reference: established direction includes diversified competing megacorporations, independent unions, contrasting pirate cultures, an Academy attempting neutrality, and bounded neutral authorities. Detailed profiles and additional organizations remain proposed; D6A v0.2.1 reconciles founding histories with D6B.

Visual direction: luminous space opera with credible scale and readable action. Distinct palettes, silhouettes and landmarks; preserve dark negative space and instrument contrast. Art grammar, shaders and audio design remain candidates. Accessibility includes configurable controls, scalable interface and avoiding color-only information.

## 10. Independent development and content provenance

The project SHALL use independently authored systems and native content contracts. No inherited source code, data, assets, scripts, saved games, compatibility adapters or repository history are required or admitted from the former project. Historical research and previous documents remain outside the active development baseline. No migration or parity investigation is a prerequisite to implementation.

Third-party libraries and independently licensed assets may be integrated only when their provenance, license compatibility, required notices and distribution obligations are documented. Unresolved rights block inclusion. This does not prohibit ordinary use of external technical documentation or standards. Do not imply affiliation or endorsement. See §15 for licensing policy.

## 11. Repository organization

The independent repository is `JohnnyPi/InterstellarMeridian`, working branch `main`. The owner supplied evidence of a fresh three-commit history, synchronized local and remote branches, tracked README, Rust `.gitignore` and two license files. Local documentation directories exist under `docs/design`, `docs/architecture`, `docs/worldbuilding`, `docs/decisions` and `docs/implementation`; empty directories are not tracked by Git. No Rust manifest or source tree was present in the supplied tracked-file listing.

Start with the smallest maintainable Rust application and headless test boundary justified by requirements; introduce modules and crates when ownership/build boundaries warrant them. Preserve documentation, assets/data, tests and decision records with actual implementation paths determined at T3/T4. Recheck the repository before modifying it. Package and executable conventions are `interstellar_meridian` and `interstellar_meridian.exe`; these are identifiers, not claims of existing build artifacts.

## 12. Development workflow

Use small focused commits, test-first changes where practical, automated formatting/linting/tests, reproducible Windows builds, CI, explicit dependencies and license review, diagnostics, profiling-led optimization and versioned ADRs. Validate each milestone before advancing. Use main as the working branch; verify actual branch and tag state before changes. Significant decisions record alternatives, consequences, evidence and superseded requirements. Failed tests or unresolved rights are blockers, not waived by document completion.

## 13. Roadmap and transition gates

These T labels are development gates. D10 defines implementation phases and task dependencies; quantitative budgets remain to be established at the relevant gates.

| Gate | Evidence required | Status |
|---|---|---|
| T0 | D0 vision disposition; established/candidate separation | Accepted at vision scope; v1.1 migration reviewed separately |
| T1 | D1 high-level requirements and owner approval; v2.1 migration | v2.0 accepted; v2.1 review pending |
| T2 | Independent repository and clean history; tracked licenses and README; reconciled D0/D1/D6A/D6B, D10, ADR-001, new Project configuration and synchronization | In progress; repository structure verified, documentation and final review pending |
| T3 | Minimal D3/D4/D9/D10 contracts for time, RNG, identity, state, content, tests, coordinates; validated dependency APIs and approved first implementation criteria | Pending |
| T4 | Native Windows build/run, headless tests, clock, seeded output, validated content errors, diagnostics, CI and dependency/license evidence | Pending |
| T5 | Causal sandbox: stocks reconcile, exclusive shipment outcomes, unattended progression, intervention, replay/save/load and handoff | Pending |
| T6 | Flight graybox: responsiveness, readability, convincing scale and meaningful transit via Windows playtest | Pending |
| T7 | Launch/travel/intervention/dock/trade/save/reload loop with persistent cross-system consequence; headless tests pass | Pending |

T0–T4 are prerequisites. T5 and T6 are complementary evidence before T7; a narrow handling spike may inform sandbox planning without accepting T6 early. Avoid building a production economy before tactile flight validation. T7 is not final product completion. Subsequent phases expand combat, mining/exploration, careers, fleets, faction authority, mission binding, tuning, investment and modding according to D10. No implementation starts from T1 alone.

## 14. Testing and acceptance strategy

Unit tests: algorithms, schema validation, calculations, command preconditions. Integration: schedule ordering, lifecycle, transitions, cross-domain transactions, content and persistence. Deterministic tests: fixed version/seed/content/inputs, stable tie ordering and material digest comparisons. Causal tests: conservation, exclusivity, unattended progress and traceable outcomes. Persistence: round trip, invalid/missing references, migration and failure recovery. Performance: measured simulation throughput, memory, loading, frame times and active/remote scale; budgets set by D10 after evidence. Playtests: handling, travel, navigation, readability, accessible controls and meaningful choices. Tests SHALL target native contracts and documented supported platforms.

Proposed T5 matrix (retained from accepted D1 v2.0):

| ID | Scenario | Pass evidence |
|---|---|---|
| CS-01 | No intervention | Production, transport, demand and operation advance |
| CS-02 | Delivery | Origin debit/reservation and destination credit once; identity retained |
| CS-03 | Loss/capture/diversion | No duplicate arrival; shortage and downstream effect |
| CS-04 | Player intervention | Material outcome changes through shared facts |
| CS-05 | Active/remote handoff | Cargo, progress, identity, pending outcome conserved |
| CS-06 | Save mid-transit | Restored continuation equivalent to uninterrupted run |
| CS-07 | Replay | Equivalent canonical state and ordered material events |
| CS-08 | Explanation | Inspect causal references to stock/readiness/conflict |

Pass requires all approved invariants and recorded repeatable evidence, at least one intervention affecting two related subsystems, and unattended progression. Exact quantities and seeds are fixtures to define in D3/D6/D10. Prototype does not establish final economic formulas or combat rules.

T6 playtest criteria must be operationalized before coding: specified control tasks, representative traversal, readable target/HUD, observed precision problems, reproducible build/configuration and recorded feedback. No arbitrary FPS or entity target is declared. T7 requires a complete interaction loop, persistent causal effect and regression suite. Later acceptance must demonstrate rank boundaries, valid mission bindings, stamp reuse without duplication, multiple capital ships, large-battle persistence and viable single-ship careers before claiming those features.

## 15. Licensing and attribution

Project-owned source code is dual-licensed under MIT OR Apache-2.0, at the recipient's option. Maintain LICENSE-MIT and LICENSE-APACHE and use `license = "MIT OR Apache-2.0"` when Cargo metadata is introduced. Project-owned documentation and original fictional lore are licensed under Creative Commons Attribution 4.0 International (CC BY 4.0), https://creativecommons.org/licenses/by/4.0/. Project-owned source code remains MIT OR Apache-2.0. The game name, official logos and other source-identifying branding are excluded from the CC grant as branding; no trademark or endorsement permission is granted. Artwork, music and audio not expressly released as documentation or lore require separate licensing before distribution. Third-party materials retain their applicable terms.

Verify actual third-party license texts and provenance before integration; document source, version or hash as appropriate, rights, modifications, notices and intended distribution. Preserve applicable copyright, attribution and license notices. Conduct comprehensive distribution review before release. Use independently authored placeholders where rights are unresolved. No inherited materials are part of the project. Formal trademark clearance is outstanding; the T2 audit inspected both code license texts and README, without constituting a legal audit.

## 16. Governance and document hierarchy

| ID | Authority / responsibility |
|---|---|
| D0 | Vision and established product direction |
| D1 | High-level requirements and boundaries |
| D2 | Retired identifier; no active specification |
| D3 | Universe, time, simulation detail and world causality |
| D4 | Validated Rust/Bevy technical architecture |
| D5 | Flight, ships, equipment, combat and handling prototypes |
| D6 | Economy, logistics, factions and investment |
| D6A/D6B | Faction/worldbuilding reference; established vs proposed content |
| D7 | Agents, AI, rank permissions and command |
| D8 | Mission and operation templates/binding |
| D9 | Persistence, content and modding |
| D10 | Implementation milestones, tests and budgets |
| ADRs | Significant decisions, evidence and supersession |

Downstream documents refine, not silently contradict, accepted direction. Conflicts require recorded owner decisions and updates to affected documents. Keep superseded material in the former project archive, not in the active repository or new Project Sources. Active sources contain current authority and relevant references. ADR-001 records independent project establishment; Appendix B summarizes the decisions it must carry forward.

## 17. Success criteria

Initial engineering success: native Windows build, testable headless boundaries, reproducible generation and material transitions, stable identity, validated data, diagnosable failures, and reliable restore. Initial gameplay success: enjoyable controllable flight, navigable scale, docking and trade, autonomous transport, cross-system travel, meaningful intervention and persistent consequences. Long-term product success: connected careers and exploration, deep faction/economic causality, rank-based operations, fleets with multiple capital ships and large battles, customization and investment without player infrastructure construction, original art identity, native extensibility and compliant distribution. These are staged outcomes, not claims of completed functionality. D10 translates them into measurable release criteria.

## 18. Immediate next steps and acceptance

T0 and T1 substantive scope remains accepted. This migration requires owner review; it does not authorize implementation. T2 is open until the documentation package, new Project configuration and synchronization are validated. T3 technical contracts follow only after T2 acceptance.

| Review check | Disposition |
|---|---|
| Identity consistent with D0 v1.1.1 | Addressed §§0–3, 10–11 |
| Early vs long-term scope separated | Retained §§4, 13, 17 |
| Architecture principles without premature choices | Retained §§5–9 |
| Native acceptance and testing | Retained §§13–14, 17 |
| Independent licensing and provenance | Updated §§10, 15 |
| Setting and institutional boundaries | Reconciled §9; detailed lore migration prepared |
| Owner signoff | v2.0 accepted; v2.1 review pending |

Dependency APIs, builds, runtime tests and trademark clearance remain unverified. Logical ownership remains a high-level boundary, not a storage or crate design.

## Appendix A — v2.0 to v2.1 migration ledger

| Area | Disposition |
|---|---|
| §§0–1 | Update authority and original project identity |
| §§2–4 | Preserve loop, pillars, scope and exclusions; remove obsolete compatibility reference |
| §§5–8 | Preserve architecture, subsystem responsibilities, flight, economy, command, customization, missions and causal invariants; update T2/T3 dependency boundary |
| §9 | Preserve native generation and setting; acknowledge available D6A and D6B precedence for historical conflicts |
| §§10–12 | Replace reuse policy and branch-preservation obligations with greenfield provenance and actual repository baseline |
| §§13–14 | Replace T2 evidence requirements; retain T3–T7 and CS-01–CS-08 scenarios |
| §§15–16 | Record dual code licensing, retired D2 and new documentation governance |
| §§17–18 | Retain success criteria and distinguish prior acceptance from migration review |
| Former appendices | Historical supersession ledger and prior reuse decision retained in the old D1 v2.0 outside the active project; superseded reuse direction is not operative |

## Appendix B — Independent establishment decisions for ADR-001

Owner decisions: adopt Interstellar Meridian; establish a new independent repository with no inherited materials or history; retain Rust/Bevy and Windows x64; dual-license project-owned code under MIT OR Apache-2.0; retain the accepted game vision and high-level design; retire legacy compatibility and reuse prerequisites. ADR-001 records context, alternatives, consequences and verification separately. No new gameplay mechanic or dependency version is approved here.

## Appendix C — Source register

- IM_D0_Project_Vision_and_Design_Principles_v1.1.1.md: migrated authoritative vision, established versus candidate distinctions, independent direction and governance.
- D1_High_Level_Game_and_Engine_Design_v2.0.md: accepted substantive baseline; historical source for this migration only.
- IM_D6A_Factions_Institutions_and_Political_Identity_v0.2.1.md: active institutional direction and proposed profiles.
- IM_D6B_History_and_Civilization_v0.2.1.md: active established setting and provisional history.
- Owner decisions and supplied repository verification in the project establishment discussion.

**Stop point:** T2 in progress. Review this migration and complete the documentation and establishment criteria before T3. No implementation or repository changes performed in this revision.


**Reconciliation record (2026-11-26):** Editorial patch of 2.1 to 2.1.1; active references updated, owner-approved CC BY 4.0 documentation/lore policy incorporated, and outstanding T2 evidence distinguished from completed checks. Earlier migration records describe the preceding revision. No new gameplay, canon or technical implementation is authorized.
