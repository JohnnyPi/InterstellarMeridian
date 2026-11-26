# D3 — Universe and World Simulation

**Project:** Interstellar Meridian\
**Version:** 1.0.0 — final T3 review baseline
**Date:** 2026-11-26\
**Status:** Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending.
**Reference:** `main` at `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b` (last verified prior increment)\
**License:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; <https://creativecommons.org/licenses/by/4.0/>. Official branding and separately identified third-party material excluded.

## 0. Authority and scope

D0 v1.1.1 P1/P2/P9, §§4–5 and 11–12; D1 v2.1.1 §§6, 8–9, 14; D10 v0.1.2 §§4–5; D4 v0.5.0 §8 (proposed RNG contract). D6B/D6A govern setting at their documented canon/proposal status. Sections 1 and 4 address seeded generation and spatial domains for T4. No canonical galaxy, topology generation algorithm or final simulation tiers are selected.

## 1. T3.4 — Seeded generation contract

### 1.1 Ownership and input/output boundary

Generation authority consumes an explicit master seed, generation recipe version, RNG contract version, validated authored constraints/definitions and stable generation context keys. Outputs are candidate generated definitions and initial world facts with stable references (identity format T3.5). Validate the entire required fixture before publishing to the authoritative session. Reject unsupported versions, invalid definitions and unresolved references without partial authoritative state. T3.6 defines validation/schema; T3.8 defines persisted encoding.

### 1.2 Generation is not ongoing simulation

Generation creates initial conditions; material transitions evolve those conditions. Revisiting a region cannot reset existing stocks, ownership, discoveries, destruction or pending operations to their generated defaults. A seed is not a save file. Persist changes and necessary generated facts, and reconstruct only explicitly approved immutable derived facts using matching versions and inputs. No offline advancement implied. Lazy generation, discovery and simulation-detail handoff must later preserve exclusive authority and causal state; T4 need not implement them.

### 1.3 Context and version separation

Use D4 §8's proposed domain/context derivation, with the generation recipe version as scope_version and a reserved generation domain. Each independently generated context starts from its own derived stream. Generation of one context does not advance another context's stream. This supports order-independent generation where contexts have no causal dependencies; explicitly dependent generation must consume the same validated prerequisite facts in a defined order. Adding or changing content can legitimately alter generated facts; reproducibility requires the same content and versions. Distinct seeds need not guarantee distinct outputs. RNG version and recipe version are separate: changing geography rules need not change the RNG algorithm, but changes to generated results require recipe version review. Unknown versions fail explicitly; do not silently regenerate older worlds under newer rules.

### 1.4 Minimal T4 fixture (non-canonical)

Master seed 42, RNG version 1, generation recipe version 1, fixture domain 1, explicit context keys 0 and 1. Generate one raw u64 token per context using D4 §8.3. These are meaningless test tokens, not system IDs, coordinates or lore. Use the D4 derivation golden vectors as exact expected values. Store observations ordered by context key, not generation order. The fixture proves reproducible independent generation, not topology or world quality.

| ID | Input | Expected result |
|---|---|---|
| D3-G01 | Generate context 0 twice | Exact matching D4 golden token |
| D3-G02 | Generate contexts 0,1 versus 1,0 | Identical sorted context/token records |
| D3-G03 | Extra draws in unrelated material or cosmetic streams | No change in generated tokens |
| D3-G04 | Unsupported recipe or RNG version | Reject initialization; no partial published world |
| D3-G05 | Evolve a fixture fact after generation; observe it again | Observation reads committed changed fact, not initial generated default |

T3.9/T3.10 map to concrete fixtures and executable tests. D3-G05 may share the existing T4 scalar state fixture; no world geography or save system required here. All scenarios are proposed, not executed.

## 2. Later sections (historical handoff, updated)

T3.7: see §4 and D4 v0.5.0 §10. T5.1: approved causal sandbox geography, material rates, simulation detail and generation needs. T5.6: handoff. T6: travel and flight validation. D9/T3.6/T3.8: definitions, schema and persistence. No content count, generator grammar, physics library, procedural noise implementation or new dependency chosen.

## 3. Review and limitations

The document introduces a proposed generation recipe/version boundary, context-isolated streams and a minimal exact fixture; all require owner acceptance with D4 §8. No repository writes or runtime tests performed. No new setting facts are established. D4 owns the algorithm and sampling specification; D3 references rather than duplicates it. T3.2/T3.3 remain unbanked and T4 remains unauthorized.


## 4. T3.7 — Universe topology and spatial domains

**Disposition:** proposed, not accepted. D4 v0.5.0 §10 owns numeric representation, conversion, validation and precision. D3 owns geographic meaning and connection semantics. D1 §§7.1, 8–9 and D0 §§4, 5.3, 8 govern this division.

### 4.1 Topology

Universe authority owns system identities and explicit navigable connections. A system is a durable world instance, not a generation context key, display name or ECS handle. Links reference valid system IDs; topology, physical separation and travel eligibility are distinct. A connection does not imply that travel is currently permitted, instantaneous, symmetric or cost-free. The future D5/D6 contracts specify traversal and restrictions. No global Cartesian position is required for authoritative interstellar navigation in T4; galactic map layout can be derived independently and is not travel authority. Authored physical distances, if needed later, require explicit units and validation; do not infer them from map pixels.

### 4.2 Spatial hierarchy

| Domain | Meaning | Ownership |
|---|---|---|
| Region topology | Systems and explicit directed links | Universe authority |
| System coordinates | Meter offsets from a stable system-specific reference origin | Spatial authority within system |
| Local frame | Meter offsets from an explicit anchor inside the same system; axes parallel for T4 | Spatial authority; future flight authority on handoff |
| Render-relative | Camera-relative disposable positions | Presentation only |

A position is meaningful only with its system and frame context. Different systems cannot be subtracted as if they shared a Cartesian origin. An intersystem transition changes system membership through an authorized travel transition, preserving ID and other material facts. Generation supplies initial geographic facts; revisiting must not reset evolved locations. The system reference origin is a mathematical datum, not necessarily a star or barycenter; selection of physical origin, orbital motion and moving/rotating frames is deferred. No new lore or system-count commitment.

### 4.3 Minimal non-canonical topology fixture

Two existing durable system instances A=1, B=2 (fixture-local IDs, allocated by D4 §9), with one explicit directed A→B link. No reverse link, route length, travel time or drive requirements are assumed. Link records may use endpoint pairs for this minimal fixture; multiple parallel links and their persistent identity are later design decisions. Reject dangling endpoints, self-links and duplicate ordered pairs in this fixture; these are not universal future travel rules. Normalize iteration by (source ID, destination ID). Candidate validation and activation are atomic; no second authority is introduced by a map or index.

| ID | Scenario | Expected |
|---|---|---|
| D3-S01 | A→B valid | A can discover the directed adjacency to B; reverse not inferred |
| D3-S02 | Missing destination | Reject candidate topology before publication |
| D3-S03 | Duplicate pair or self-link | Reject candidate fixture |
| D3-S04 | Reorder valid records | Same canonical topology |
| D3-S05 | Two identical numeric positions in different systems | Distinct locations; direct subtraction rejected |

T3.10 confirms minimal topology fixture placement within T4.6; no procedural graph generation or actual travel implementation required. All tests proposed. D9/T3.8 must preserve system references and canonical coordinates; D4 §10 specifies exact foundation range and tolerances. T5.1/T5.6 define actual geography and handoff; T6 validates scale and handling. No dependencies or production code changed; latest repository observation is the earlier T3.1 baseline, not a fresh checkout audit.


## T3.10 closure review addendum (historical; final disposition below)

The preceding increment checkpoints are historical. Current remote main was verified at a09251b8482894b9e2ae01b9c5afc1281b0e6a4b; local checkout not inspected. This revision retains all scenario IDs. The T3.10 plan v0.2.0 and closure decision records govern proposed clarifications below; none amend accepted ADR-002 until owner approval. T4 remains blocked.

Generation and topology scope unchanged. Generated tokens are not durable instance IDs. Directed links are fixture semantics only. Actual world generation and detail handoff remain T5 work.


## Final T3 closure disposition (normative on consolidated acceptance)

Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. The earlier increment checkpoints and historical version references describe their original issue, not outstanding prerequisites. Active companion versions upon acceptance: D3/D4/D9 v1.0.0, D10 v0.1.3, T4 plan v1.0.0, ADR-002 v1.1.0 and ADR-003–006 v1.0.0. Earlier proposed conditions are superseded only as stated here. All acceptance scenario IDs retained.

Generation and topology contracts unchanged; D4 v1.0.0 owns RNG and numeric spatial behavior. T4 fixture only; no canonical geography selected.
