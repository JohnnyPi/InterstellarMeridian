# D9 — Persistence and Modding

**Project:** Interstellar Meridian\
**Version:** 1.0.0 — final T3 review baseline
**Date:** 2026-11-26\
**Status:** Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending.
**Repository:** JohnnyPi/InterstellarMeridian (`main`)\
**Reference commit:** `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b` (last verified earlier)\
**License:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; <https://creativecommons.org/licenses/by/4.0/>. Official branding and separately identified third-party material excluded.

## 0. Authority and scope

D0 v1.1.1 P9/P10; D1 v2.1.1 §§6, 8–10, 14–15; D10 v0.1.2 §§4–5 (T3.6 and T4.5); ADR-002 v1.0.0 and accepted T3.1 contracts; proposed D3 v0.1.0 §1 and D4 v0.4.0 §§1, 3–4, 7–9. T3.2–T3.5 are cumulative drafts, not banked contracts. T3.6 elaborates them conditionally and does not accept them. T3.8 adds the snapshot contract in §8; D3 v0.3.0 and D4 v0.5.0 govern spatial and material primitives. No changes to accepted dependencies or source code are authorized.

## 1. T3.6 — Definition and instance contract

### 1.1 Owner, inputs, outputs and invariants

The content authority owns definition admission and the activated definition catalog. Inputs are candidate definition records, explicit schema version and source labels. Outputs are an immutable validated catalog and diagnostics, or a rejection with no partial activation. Application orchestration admits the catalog during boot; generation and simulation read it. The material domain owns instance facts and validated mutations. Content cannot create or mutate a live instance except through an authorized initialization or domain transition. Presentation cannot bypass either boundary. Identical supported definitions must have equivalent canonical meaning independent of record order. No source provenance, schema, dependency or reference failure may silently become a default value.

### 1.2 Distinct identity domains

| Domain | Meaning | Example (illustrative) |
|---|---|---|
| Definition ID | Stable authored identity | `core:ship.industrial_freighter` |
| Definition kind | Required expected type | ship, commodity, fixture |
| Instance ID | World-local nonzero u64 under proposed D4 §9 | 1042 |
| Generation context | Explicit stable recipe context under D3 §1 | context 0 |
| Display name | Human-facing label, not identity | Industrial Freighter |

Definitions describe permissible characteristics and defaults; material instances reference definitions and own current facts. Shared definition data must not become mutable per-instance state. Current damage, cargo, fitted parts, ownership and location never reset from defaults when the instance is observed or restored. A definition reference never resolves through an ECS handle or display name. A tuning stamp is a reusable recipe, not a physical instance; detailed equipment rules are deferred to D5/D9.

### 1.3 Proposed definition identifier grammar

Use case-sensitive ASCII identifiers `namespace:kind.name`, where namespace, kind and each dot-separated name segment match `[a-z][a-z0-9_]*`. The name contains at least one segment. Exactly one colon, at least one dot following the colon; no empty segments, whitespace, path separators or Unicode normalization. Example: `core:fixture.alpha`. Reserve `core` for project-owned content. Namespaces are labels, not security credentials or ownership proof. T4 accepts only `core` fixture records; other namespace registration and extension ownership are deferred. No automatic case folding or trimming. Full ID unique across the activated catalog; duplicate records, even identical ones, reject. Kind field must agree with ID kind. A rename is an identity change requiring an explicit future migration, not a display-name edit. No arbitrary size limit is selected; input size/resource budgets and untrusted content policy require later review.

## 2. Minimal versioned T4 schema

### 2.1 Logical schema v1

The following is a logical record model, not a selected disk encoding, Rust type signature or file layout. A fixture document contains one `schema_version` (unsigned integer, exactly 1) and a collection of definitions. Source labels are diagnostic metadata assigned by the loader, not trusted content authority. Only one record kind, `fixture`, is necessary for T4.

| Field | Logical type | Rule |
|---|---|---|
| id | string | Required; grammar in §1.3; `core:fixture.*` |
| kind | string | Required; exactly `fixture` |
| display_name | string | Required; nonempty after whitespace check; retain original text |
| initial_value | u64 | Required; inclusive range 0–100 for this fixture only |
| prerequisite | optional definition ID | Absent or one valid reference to another fixture definition |

Unknown fields, duplicate fields, missing required fields, invalid types, negative/overflow values and unsupported schema versions reject. No coercion, implicit defaults for required fields or silent unknown-field acceptance. Optional absence is explicitly meaningful; the encoding of absence is deferred. The prerequisite relation is acyclic for this synthetic fixture; this is not a universal prohibition on cyclic world relationships. No per-definition revision field required for T4: the session binds the exact validated catalog plus its schema version. A changed catalog is different content even at the same schema version. Schema version describes structure, not content revision, generation recipe or RNG version.

### 2.2 Format decision and T4 loading scope

Recommend a typed in-memory candidate input and strict validation for the first T4.5 proof; malformed typed records (missing required fields, unsupported version, invalid references) must be representable at the candidate boundary. A second narrow textual loading proof is needed if the accepted T4.5 criterion requires external file loading; T3.10 must explicitly select and approve the encoding/parser before implementation. Do not claim typed fixtures prove parsing malformed external bytes. Do not introduce Serde, JSON/TOML dependencies, a homemade general parser, scripting or filesystem discovery now. T3.8 may coordinate the format choice for snapshots, but content and snapshot schemas remain distinct. If no acceptable external loading design is selected, T4.5 external-input acceptance remains open, not waived. No dependency approval implied.

## 3. Validation and activation pipeline

1. Read candidate input and retain source/record location; report decoding errors if an approved decoder is present.
2. Validate envelope/schema version before interpreting records. Reject unsupported versions explicitly.
3. Validate required fields, field types, ranges, identifier syntax, namespace and kind agreement.
4. Build candidate index by exact ID; reject duplicates before resolving references. Do not use last-writer-wins.
5. Resolve required references against the entire candidate catalog; permit forward references. Check expected kind and fixture-specific self-reference/cycle rules. If a kind is unsupported reject at structural validation rather than relying on reference checks.
6. Canonicalize valid records by ASCII ID ordering; use explicit field order for comparison. Never depend on filesystem traversal, hash map iteration or parser order. Prerequisite-first evaluation, if needed, uses topological order with ASCII ID tie-breaks. Validation itself creates no material instances and consumes no material RNG or IDs.
7. Publish the complete validated catalog once at boot, then initialize the candidate material session under D3/D4. A failure in either leaves no ready partial session. The catalog is immutable for the session. Hot reload, overrides, unloading and live migrations are deferred.

For T4, fail-fast is acceptable if the reported error is actionable; collecting all errors is not required. For semantically invalid records, choose the first error using the declared stages and stable record/field ordering where keys are valid; malformed records without valid keys may use input position. Diagnostic order is not material state. Candidate catalog and indexes are disposable until activation; do not replace a valid live session with a failed candidate. Exact storage, Rust module and Bevy resource layout deferred to T3.10.

## 4. Reference and instance binding

References are exact definition IDs plus an expected kind from the referencing schema. No fallback by display name, nearest match, implicit namespace or unrelated definition. Optional absence is allowed only for a field explicitly declared optional; a supplied but missing target is an error. A definition existing in the catalog does not guarantee a live material instance or eligibility for a command. D4 §9 governs live instance references, retirement and allocator validation. When creating a material fixture, validate its definition binding and explicit initialization values, allocate an instance ID only at a successful creation commit, and copy only the material defaults required by the fixture. A rejected creation leaves material state, allocator and RNG unchanged. Restore must bind the same compatible catalog and preserve evolved facts; exact catalog identity/encoding and migration policy are T3.8 decisions. Until migration exists, mismatched catalog is rejected rather than silently substituted. Content changes are allowed for new sessions only under this T4 proposal.

## 5. Diagnostics and failure containment

| Failure class | Expected outcome |
|---|---|
| Unsupported version | Report received/supported version; reject catalog |
| Missing/invalid field | Report source, record, field and reason; reject |
| Duplicate ID | Report conflicting ID and both origins; reject |
| Unresolved/incompatible reference | Report referencing ID, field, target and expected kind; reject |
| Cycle in fixture prerequisites | Report involved IDs; reject |
| Failed instance binding | Reject creation/restore with no partial material change |
| Unexpected integrity failure after mutation | D4 fail-stop containment; do not publish invalid state |

Diagnostics carry a stable conceptual category, source label, record identifier or index, field path when applicable, and a human-readable explanation. Exact error enum and text are not selected. Headless and client use the same validation; GUI error handling must not bypass admission. Provenance and licenses remain subject to ADR-002 staged policy; successful schema validation does not prove distribution rights. Content inputs are data, not executable code.

## 6. Non-canonical foundation fixture and acceptance evidence

Valid envelope schema version 1 contains these records (logical table, not wire format):

| ID | Kind | Display name | Initial value | Prerequisite |
|---|---|---|---:|---|
| `core:fixture.alpha` | fixture | Alpha | 10 | absent |
| `core:fixture.beta` | fixture | Beta | 20 | `core:fixture.alpha` |

Feed beta before alpha and confirm the same canonical catalog as alpha before beta. For an otherwise empty material fixture with allocator H=0, create two alpha instances in explicit order: IDs 1 and 2, values 10 and 10, H=2. Change instance 1 to 15 via an admitted fixture command; instance 2 remains 10 and definition default remains 10. Use the accepted D4 tick boundary once approved. No gameplay or actual lore is represented.

| ID | Input / scenario | Required result |
|---|---|---|
| D9-C01 | Valid envelope and both records | Catalog contains exactly two definitions; canonical order alpha, beta |
| D9-C02 | Reverse input order, forward reference | Same canonical fields and resolved prerequisite |
| D9-C03 | Schema version 2 | Reject; report unsupported version |
| D9-C04 | Missing id or initial_value | Reject; identify required field |
| D9-C05 | String in numeric field; negative or overflow candidate | Reject without coercion |
| D9-C06 | initial_value 101 | Reject with range 0–100 |
| D9-C07 | Uppercase, whitespace, invalid separator or empty ID segment | Reject; no normalization |
| D9-C08 | Duplicate alpha, including identical record | Reject; report both origins |
| D9-C09 | Unsupported kind or mismatch with ID kind | Reject |
| D9-C10 | Beta references missing alpha | Reject; identify reference and target |
| D9-C11 | Alpha references itself, or alpha/beta cycle | Reject fixture prerequisite cycle |
| D9-C12 | Omit optional prerequisite | Accept; explicit absence |
| D9-C13 | Unknown or duplicate field in candidate input | Reject; if decoder needed, test at chosen input boundary |
| D9-C14 | Valid prefix followed by invalid record | No partial catalog or ready world |
| D9-C15 | Create two alpha instances and mutate one | IDs and values as above; default and second instance unchanged |
| D9-C16 | Create from missing/incompatible definition | Reject; no ID, RNG or material change |
| D9-C17 | Repeat validation in headless and client configurations | Same acceptance and canonical catalog; no renderer needed |
| D9-C18 | Attempt replacement with invalid candidate | Existing valid session unaffected; live reload unsupported |
| D9-C19 | Restore candidate bound to changed catalog | Reject until approved migration; T3.8/T4.6 integration test |
| D9-C20 | Malformed external bytes | Decoder rejection with location; mandatory at T4.5 under final closure scope |

All are proposed tests, not execution evidence. T3.9/T3.10 assign concrete commands, fixture representation and ownership. Required test failures, unresolved required references and duplicate active definitions must all be zero. No latency or throughput budget is invented; T3.10 must set necessary T4 input workloads and thresholds, later T5/T6 budgets apply to real content. C19 belongs to persistence integration; C20 cannot pass merely by typed-record validation.

## 7. Deferred contracts and review

T3.8: snapshot versioning, material state, exact catalog binding, compatibility and atomic restore; T3.9/T3.10: evidence and implementation plan, encoding/parser decision and production dependency review if justified. Later content contracts: actual ship/commodity/station/faction schemas, assets, mod manifests, dependency graphs, override policies, localization, migration, scripting, sandboxing and editor tooling. No gameplay content or new lore approved.

Owner decisions requested: namespaced ASCII IDs, immutable session catalog, strict v1 fixture schema, explicit reference resolution and whole-catalog admission; typed candidate proof plus separately approved external loading decision. No new dependency or serialization format selected. D4 v0.4.0 and D3 v0.1.0 remain unchanged; this document supplies their deferred content contract. D4's reference to future D9 is satisfied by this proposed section, subject to consolidated review. No production code, tests, repository writes or live repository verification performed in this increment. T4 remains unauthorized. Next design: T3.7 spatial coordinate architecture in D3/D4.


## 8. T3.8 — Committed snapshots and restoration

**Disposition:** proposed, not accepted. D10 v0.1.2 T3.8 requires a committed snapshot boundary, versions, pending effects, validation/failure behavior and canonical round-trip fixture; T4.6 implements minimal snapshot primitives, not a production save manager. D1 §§6, 8, 17 and D4 v0.5.0 §§4, 7–10 govern. D3 v0.3.0 governs topology and generation. Earlier draft decisions remain conditional on consolidated owner acceptance.

### 8.1 Authority and capture

Persistence owns the snapshot representation, not live simulation. Input: a ready, healthy session at a committed boundary (tick zero or completed tick), immutable validated catalog and explicit capture request. Output: a self-contained logical snapshot or diagnostic rejection. Capture is read-only, consumes no IDs or material RNG and advances no ticks. T4 capture is synchronous between steps; reject requests during an admitted step rather than invent asynchronous machinery. No capture of quarantined or partially initialized state. Complete deferred structural changes before capture. No autosave or shutdown-save guarantee. A successful in-memory capture is not a durable disk save.

### 8.2 Logical snapshot v1

| Section | Required T4 fields / rules |
|---|---|
| Compatibility | snapshot schema=1, content schema=1, RNG contract=1, generation recipe=1, material rules=1; fixture quantum=100000000 ns; all exact supported values |
| Content | exact canonical validated catalog records from §2; compare to externally supplied validated catalog field by field; no digest dependency |
| Clock | committed u64 tick; checked duration per D4 §7 |
| Generation | master u64 seed; material generated fixture facts, context keys and tokens when used; never regenerate evolved facts |
| Randomness | registered material stream keys (RNG version, scope version, domain, context), current u64 state; unique keys, explicit owner binding |
| Identity | allocator high-water mark H, live records with nonzero IDs and expected kinds, required retirement/history records if fixture uses them |
| Material | definition bindings, evolved fixture values, required references, authoritative representation/detail discriminator where relevant to continuation |
| Geography | system records, directed topology pairs, spatial locations as system ID and separate anchor/offset f64 triples |
| Pending effects | typed fixture operations and accepted deferred intents if supported, target tick, phase, sequence, target IDs and payload; ordering allocation state |
| History | only required material outcomes/history when part of fixture state; future domains define retention; comparison harness may concatenate observed outcomes externally |

Exclude transient ECS handles, derived indexes, wall time/debt, cosmetic state, diagnostics, GPU resources and unadmitted external input. Restore starts paused with zero pacing debt; no offline progression. If a fixture supports queued future material intents, preserve them; otherwise reject unsupported pending kinds rather than drop them. Capture cannot silently omit future-relevant state. Preserve normalized finite spatial f64 components losslessly; use exact bit equality for identical representations, not tolerance. A representation discriminator is semantic, never an ECS handle. Retired IDs remain unavailable via H; absence alone is not proof of retirement.

### 8.3 Pending operation ordering

Preserve D4 §7.4 key (target tick, phase ordinal, sequence). For T4 propose a single world-local monotonically increasing u64 high-water mark S for scheduled operation sequences, initially zero; assign S+1 on committed scheduling, checked overflow before mutation. This is a refinement of the earlier within-phase uniqueness requirement, not a change to phase order. Commands retain explicit admitted order; if deferred admitted commands are supported, preserve their keys and any separate allocator state. T4 fixture supports scheduled operations only in persisted queue; external unadmitted commands must be resubmitted by caller. Require unique scheduled sequences, 1..S, target tick strictly greater than committed tick, supported phase/kind, valid payload and required live references. Revalidate at execution. Remove resolved work exactly once on its due tick; rejected/canceled outcome explicit. No silent late execution, duplicate replay, automatic retry or same-tick recursive insertion. Preserve S even after queue empties. Scheduling failure leaves S and material state unchanged.

### 8.4 Restore pipeline and compatibility

1. Read candidate logical snapshot; validate envelope/version before interpretation. Reject unsupported version; no automatic downgrade, migration or reseed.
2. Validate supplied catalog using §§1–5 and compare every canonical field with snapshot binding; changed values at same schema version reject. Embedded records are comparison evidence, not permission to activate arbitrary content. Source paths/diagnostics excluded from semantic comparison.
3. Validate quantum/tick, seed/versions, stream ownership and keys, identities/H, material types/ranges, topology and finite coordinate envelopes, references, pending effects/S and cross-field invariants. Reject duplicate records before constructing indexes. Validate supported fixture domains explicitly; unknown fields/kinds reject at applicable input boundary.
4. Build isolated candidate material session and rebuild disposable indexes/runtime representations without allocating replacement IDs, drawing material random values or executing a tick. No saved ECS handles. Check invariants and compare canonical projection to validated candidate.
5. Publish candidate only after complete success, with advancement paused. Existing healthy session remains unchanged on rejection. For T4 replacement, require existing session paused or stopped; no concurrent replacement. Clear stale runtime references and pacing debt. Subsequent explicit inputs advance from restored tick+1.

Snapshot versions, content schema, content values, RNG version, generation recipe, rules version and quantum are independent compatibility dimensions. Build metadata may be diagnostic, not sole compatibility proof. T4 admits only exact supported contract values; no claim of compatibility across builds/platforms beyond tested supported configuration. Migration, cross-world merging, save branching metadata, compression and encryption deferred. Divergent restored copies need not be mergeable.

### 8.5 Serialization and disk policy

**Recommendation:** T4.6 first prove typed snapshot capture/restore and canonical structured equality, without introducing dependencies. This meets the proposed minimal snapshot primitive, not a byte-level round trip or crash-durable save/load. T4.5 external definition loading remains open under §2.2; T3.10 must resolve it before coding and must not waive D10 loading criteria. Recommend evaluating a single human-readable JSON encoding with a strict typed decoder for external content and eventual snapshot fixtures, subject to version/API/license verification and owner approval; no parser, Serde dependency, exact wire schema or disk format selected here. If JSON is selected later, preserve u64 exactly (no intermediary binary64 conversion), preserve f64 normalized bits through an explicit lossless representation, reject duplicate/unknown fields, overflow and trailing malformed data, and set input bounds. Do not infer bitwise equality from textual formatting. Alternative encodings remain open. T3.10 must explicitly establish whether disk serialization is in T4.6 scope and define tests accordingly; no production save claim from typed fixtures.

Future durable save adapter (T5.7/T7): stage new data separately from last known valid save; validate complete staged data; publish replacement only through a verified Windows-safe commit mechanism; on failure retain previous valid save or report recovery condition explicitly. Never claim atomic replace, flush-to-disk or power-loss durability without platform-specific tests. Keep capture, encode, write, validation and publish failures distinguishable. Reject truncated/malformed data, incompatible versions and unresolved references before activation. No overwrite of sole known-good save as first action. Actual file layout, checksums, backup retention, crash recovery and Windows APIs deferred; require approval and tests before implementation. A checksum is not semantic validation or authentication.

### 8.6 Canonical continuation fixture (non-canonical synthetic data)

Use D9 §6 alpha/beta catalog, D3 §4 systems IDs 1 and 2 with A→B, alpha instance ID 3 in A at anchor (10^13,0,0), offset (0.125,0,0), value 10. Allocate and retire an unreferenced fixture ID 4 during authorized initialization/test setup so H=4; no pending references to 4. Seed 42, all supported versions 1, quantum 100 ms; material stream A domain 2 context 3 using D4 §8 derivation; scope version 1. At committed tick 0 schedule a synthetic increment-by-2 effect due tick 3, sequence 1, target 3, S=1, as part of validated initial fixture. Fixture value arithmetic checked within 0..100 (fixture-specific rule). No autonomous material change in this isolated fixture; a no-op autonomous phase is permitted here. Step 1: accepted fixture command changes value to 15 and consumes exactly one raw draw from stream A as an explicitly recorded outcome. Step 2: no input. Capture at tick 2: value 15, H=4, S=1, pending effect present, stream state after one draw, canonical anchor/offset unchanged. Restore into fresh session paused. Tick 3 executes due effect exactly once, value 17; tick 4 accepted command consumes one raw draw and creates one alpha instance (ID 5, default value 10) with valid fixture location/binding. Same explicit input sequence on uninterrupted branch. Compare canonical state after each continuation tick, and ordered material outcomes after capture, including exact RNG outputs/state, H=5, S=1, empty pending queue, instance 3 value 17 and instance 5 value 10. No actual gameplay or lore. Golden RNG vectors from D4 remain independent; this integration fixture compares full canonical fields, not a digest or generated implementation-derived expected file. Use separately constructed expected scalar values. Initial retired ID evidence may be omitted if no historical reference is present; H must still be 4.

### 8.7 Proposed acceptance scenarios

| ID | Scenario | Required result |
|---|---|---|
| D9-P01 | Capture healthy tick zero or tick 2 | Complete committed logical snapshot; no clock/ID/RNG changes |
| D9-P02 | Capture during step or quarantined session | Reject; no valid snapshot |
| D9-P03 | Restore valid tick 2 snapshot | Canonical fields exactly equal; paused, zero wall debt |
| D9-P04 | Uninterrupted vs restored continuation | §8.6 exact scalar results, canonical equality and ordered outcome equality |
| D9-P05 | Restore with retired highest ID | H preserved; next creation ID 5 |
| D9-P06 | Pending effect due tick 3 | Executes once, removed; no replay at tick 4 |
| D9-P07 | Past/current due tick, duplicate key or sequence beyond S | Reject candidate |
| D9-P08 | Empty queue after prior scheduling | S preserved; next schedule sequence 2 |
| D9-P09 | Unsupported snapshot version | Explicit rejection before activation |
| D9-P10 | Unsupported RNG/recipe/rules version or quantum | Reject; no silent upgrade |
| D9-P11 | Changed catalog value under same schema | Reject exact content mismatch |
| D9-P12 | Duplicate live ID, zero ID or ID beyond H | Reject before publication |
| D9-P13 | Missing required instance/definition/system reference | Reject with field and target diagnostic |
| D9-P14 | Duplicate/unregistered stream or invalid owner binding | Reject; no reseed |
| D9-P15 | Nonfinite or out-of-envelope coordinate | Reject; normalized zero policy per D4 |
| D9-P16 | Rebuild runtime indexes and handles | Same material IDs/fields; no new allocations/draws |
| D9-P17 | Failed restore over paused valid session | Original canonical state remains unchanged |
| D9-P18 | Restore after arbitrary wall-time gap | No offline advancement; next tick exactly 3 |
| D9-P19 | Reorder valid candidate records | Same canonical ordering and equality |
| D9-P20 | Unsupported pending operation kind/payload | Reject; no silent discard |
| D9-P21 | Exhausted scheduling sequence | Reject scheduling without mutation |
| D9-P22 | External malformed/truncated encoding | Conditional on approved codec; reject before activation; not satisfied by typed tests |
| D9-P23 | Staged write/publish failure | Future disk adapter; preserve known-good save, diagnose failure; not T4 typed proof |

All scenarios are proposed, not executed. P22/P23 are conditional/future and must not be reported as passed by typed tests. T3.9/T3.10 establish concrete commands, coverage, resource budgets and gate assignment. T5.7 extends continuation to actual causal material systems.

### 8.8 Review and handoff

New owner decisions: exact catalog binding, snapshot v1 logical fields, scheduled sequence high-water mark, paused restore, synchronous committed capture, strict compatibility rejection and typed-first scope. JSON is an evaluation recommendation only, not dependency approval. D4 §7/§8/§9/§10 and D3 §4 need no substantive revision for these proposed refinements; cross-reference versions updated at consolidated review. D9 §2.2 external loading remains explicitly open. No code, dependencies, runtime tests or repository writes. D10 v0.1.2 was retrieved at the accepted T3.1 SHA; current remote HEAD and local Windows checkout not rechecked. Next T3.9 testing and CI; T4 remains unauthorized.


## T3.10 closure review addendum (historical; final disposition below)

The preceding increment checkpoints are historical. Current remote main was verified at a09251b8482894b9e2ae01b9c5afc1281b0e6a4b; local checkout not inspected. This revision retains all scenario IDs. The T3.10 plan v0.2.0 and closure decision records govern proposed clarifications below; none amend accepted ADR-002 until owner approval. T4 remains blocked.

T4.5 external input is mandatory: proposed bounded UTF-8 JSON envelope with schema_version and definitions, strict typed decoding, no duplicate/unknown fields, explicit null prerequisite rejected, 64 KiB maximum, 256 records maximum, ID <=128 ASCII bytes, display name <=256 UTF-8 bytes. Candidate Serde =1.0.228 (derive), serde_json =1.0.154; no dependency change until verified and explicitly approved. T4.6 typed snapshots only; D9-P22/P23 deferred to T5.7, not passed. Historical conditional phrasing for C20 is superseded by this proposed scope clarification.


## Final T3 closure disposition (normative on consolidated acceptance)

Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. The earlier increment checkpoints and historical version references describe their original issue, not outstanding prerequisites. Active companion versions upon acceptance: D3/D4/D9 v1.0.0, D10 v0.1.3, T4 plan v1.0.0, ADR-002 v1.1.0 and ADR-003–006 v1.0.0. Earlier proposed conditions are superseded only as stated here. All acceptance scenario IDs retained.

External UTF-8 JSON input mandatory at T4.5; exact serde =1.0.228 with derive and serde_json =1.0.154 proposed for approval based on V1 native tests. Strict schema/bounds from preceding addendum apply. Typed snapshots only at T4.6; D9-P22/P23 deferred to T5.7, not passed. Production validation and external file loading still required.
