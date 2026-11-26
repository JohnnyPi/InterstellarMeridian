# D4 — Simulation and Engine Architecture

**Project:** Interstellar Meridian\
**Version:** 1.0.0 — final T3 review baseline
**Date:** 2026-11-26\
**Status:** Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending.
**Repository:** <https://github.com/JohnnyPi/InterstellarMeridian> (`main`)\
**Starting reference commit:** `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b`\
**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights; <https://creativecommons.org/licenses/by/4.0/>. Official branding and separately identified third-party material excluded.

## 0. Authority, scope and terminology

### 0.1 Governing baseline

- **D0 v1.1.1:** pilot-first, persistent causal universe; deterministic material foundations; selective simulation detail; visual effects need not be deterministic.
- **D1 v2.1.1 §§5–8:** authoritative simulation separate from commands, presentation, definitions and persistence; no graphics/audio requirement for headless execution; durable identity distinct from ECS handles; validated state mutation.
- **D10 v0.1.2 §4–5:** T3.2 specifies application/state boundaries and T3.3 specifies clock/scheduling needed for T4; T4 remains blocked until all T3 contracts and the T4 plan are accepted.
- **ADR-002 v1.0.0 and accepted T3.1 technical contract v1.0.0:** Rust **1.100.0**, edition **2024**, target **x86_64-pc-windows-msvc**, Bevy exactly **0.20.1**. Disable Bevy defaults; baseline features `std` and `default_app`; opt-in `client` feature adds `bevy/3d` and `bevy/ui`. No additional direct dependency is authorized.
- **T3.1 acceptance checkpoint and licensing assessment v1.0.0:** disposable headless and graphical feasibility verified on the owner's Windows workstation; the probe is **not** production code. Staged licensing obligations and accepted ReShade exception remain in force.

**Status discipline:** Requirements inherited from D0/D1, accepted T3.1 technology decisions, and *proposed D4 contracts* are distinguished below. The proposed contracts acquire authority only after owner review. No source files, Cargo package, plugin topology, CLI syntax, or Bevy system API are selected by this draft.

### 0.2 Current increment

Sections 1–6 preserve all four working subdivisions of T3.2: A — application lifecycle and boot; B — headless and graphical boundaries; C — state ownership and mutation; D — command validation and schedule responsibilities. These labels are editorial subdivisions, not separate D10 gates. Owner authorized continuation without individual subsection signoffs; consolidated T3.2 acceptance is still required. Section 7 adds proposed T3.3 clock/pause/order/catch-up semantics; Section 8 adds T3.4 randomness; Section 9 adds T3.5 identity and lifecycle; T3.6 content is drafted in D9 v0.1.0; §10 adds T3.7 coordinates alongside D3 v0.3.0; T3.8 persistence remains next. Earlier checkpoint statements are historical.

## 1. T3.2A — Application lifecycle and boot

### 1.1 Purpose and owner

**Logical owner:** application orchestration. It establishes a valid session and selects its execution configuration; it does **not** own world facts, override simulation rules or render state as authoritative.

**Inputs:** the accepted build configuration, a startup request, configuration values, validated content/fixture inputs, and an initial material state or new-session seed as the later content/persistence contracts permit.

**Outputs:** either a **ready authoritative session**, available to the selected execution driver, or an actionable startup failure. Diagnostics report the failed stage and cause. The exact configuration schema, argument names and error type are deferred.

**Invariant:** no simulation advancement and no partially committed authoritative world are exposed from a failed boot.

### 1.2 Proposed lifecycle phases

| Phase | Responsibility | May advance the world? |
|---|---|---|
| **Created** | Application process or test harness begins; execution configuration is known. | No |
| **Preflight** | Establish diagnostics; read configuration; check required inputs and their supported versions; determine whether the requested mode is available. | No |
| **Initialize** | Validate definitions/fixture or restored state using the future D9 contracts; prepare a single authoritative session; assemble only the services required for that configuration. | No |
| **Ready** | All required core invariants hold; the selected driver may begin. | Not until explicitly driven |
| **Running** | Accept proposed commands, commit valid simulation transitions and expose committed observations. The driver controls when advancement is requested. | Yes, via the authoritative transition boundary only |
| **Stopping** | Stop accepting new advancement requests; finish or reject any in-progress operation at its defined atomic boundary; report an orderly termination or failure. | No new advancement |
| **Stopped / Failed** | Release transient resources and report outcome; no implicit further world progression. | No |

These are **logical phases**, not a mandate to use Bevy `States`, specific enums, a state-machine library or an event bus. Startup failure may occur before Ready, and a fatal runtime error may transition from Running to failure handling.

```text
Created
   |
Preflight --invalid input/unsupported mode--> Failed
   |
Initialize --invalid world or setup failure--> Failed
   |
 Ready
   |
explicit run/update driver
   |
 Running --fatal failure--> Stopping --> Failed
   |
stop request
   v
Stopping --> Stopped
```

### 1.3 Proposed boot sequence

1. **Select the execution configuration.** The accepted dependency baseline provides a core/headless build without `client`, and an opt-in client-capable build. The command-line interface and whether a client-capable executable also exposes a separate headless option are **not decided**.
2. **Start diagnostics and validate startup inputs.** Report input/configuration errors with enough context to identify their source. Do not depend on graphics initialization for actionable errors.
3. **Validate material initialization inputs.** Distinguish authored definitions, initial/generated world material and a possible restored snapshot; the exact validation schema and initialization order belong to T3.5/T3.6/T3.8. For T4, use only a small approved foundation fixture.
4. **Create one authoritative core session.** The session is the sole owner of mutable material state. Do not expose an initialized-but-invalid world as playable or advanceable.
5. **Attach a driver and optional services.** A headless driver can request explicit updates without a window; a graphical driver supplies user interaction and enabled visual presentation capabilities, without assuming an audio subsystem. The concrete Bevy plugin and runner composition is a T4 design/detail decision.
6. **Declare Ready and enter Running.** Only authorized core transitions can mutate the running material session, including autonomous transitions during intentional advancement. Produce diagnostics and committed observations for either execution configuration.

**Boot is not a world tick.** Starting the process, constructing Bevy resources or reopening a session must not by itself simulate elapsed real-world time. There is **no offline advancement** implied by D0/D1; T3.3 will define how simulation time is intentionally advanced.

### 1.4 Startup and shutdown failure policy

| Condition | Proposed response | Authority protected |
|---|---|---|
| Invalid/missing configuration or unsupported execution mode | Abort preflight with a descriptive error; no session advance. | Startup inputs |
| Invalid content reference, incompatible snapshot or inconsistent initial state | Reject initialization; no partially available authoritative session. Detailed diagnostics and supported versions deferred to D9. | Material world state |
| Failure to construct required headless services | Abort before Ready; non-success status and diagnostic evidence. | World integrity |
| Window, graphics device or UI initialization failure in graphical configuration | Fail graphical startup cleanly; do not bypass simulation validation or claim a usable client. Automatic fallback is **not promised**. | Presentation isolation |
| Rejected gameplay command during Running | Return an explicit rejection; keep authoritative material state unchanged; normally continue the session. | Command boundary |
| Fatal simulation invariant failure | Stop further advances, report the failing boundary and preserve evidence; do not silently repair state or continue. | Authoritative core |
| Normal shutdown | Stop new updates; end at a committed transition boundary; release transient resources. Automatic save behavior is **not specified**. | Session consistency |

**Shutdown does not constitute persistence.** Only a future explicitly specified save operation can promise durable restoration, and that boundary belongs to T3.8/D9. Window close or process exit cannot be treated as a committed save without an accepted contract.

### 1.5 Minimum T4-oriented review checks (not executed tests)

| ID | Scenario | Expected result |
|---|---|---|
| **D4-A01** | Valid minimal core fixture and supported startup configuration | Core reaches Ready; no material advance before explicit driver request. |
| **D4-A02** | Invalid configuration | Error identifies input and stage; no Running phase or material world exposure. |
| **D4-A03** | Invalid initial material input | Initialization fails atomically; no partial authoritative session. |
| **D4-A04** | Normal explicit stop after committed update | No new updates accepted; session terminates at a consistent boundary. |
| **D4-A05** | Start without player input | Boot itself causes no world-time advancement; later unattended advancement remains possible when explicitly driven. |

These checks define **expected behavior for future T4 tests**, not existing test names, implemented APIs or evidence of passing tests.

## 2. T3.2B — Headless simulation and graphical presentation boundaries

### 2.1 Single authority, two execution configurations

The architectural invariant is **one source of material truth**, regardless of whether the application runs with a window.

```text
                   Validated definitions / initial material state
                                      |
                             Authoritative core
                    (committed state + validated transitions)
                            /                    \
             explicit headless driver        graphical driver
             advances and observes          requests advancement
                       |                     and submits intents
                 test/diagnostics                   |
                                            read-only observations
                                                   |
                                          camera / scene / UI
```

The diagram describes **logical dependencies**, not required crates, Rust types, channels, threads or ECS schedules. Graphical presentation is a consumer of committed observations and a producer of **requests**, never a second writer of world truth.

### 2.2 Proposed boundary contract

| Concern | Authoritative core (both configurations) | Headless configuration | Graphical configuration |
|---|---|---|---|
| World facts | Owns all mutable material state and validated transitions | Uses the same core | Uses the same core |
| Time/updates | Advances only through an approved advancement boundary; details deferred to T3.3 | Explicitly driven by a test or application runner | Driven by the client application; frame timing cannot silently redefine simulation time |
| Commands | Validates requests before committing changes; rejection must not partially mutate state | Test/harness submits requests directly to the same boundary | Input/UI translates player actions to requests to the same boundary |
| Observations | Exposes committed read-only observations/results | Assertions, diagnostics and future replay verification | Cameras, meshes, UI and visual feedback consume observations |
| Graphics, windows, audio | **Not required** for core construction or transitions | Must not initialize any graphics/window/audio backend | Enables only required presentation plugins; sound is not currently selected |
| Transient resources | Neither render-only objects nor GPU handles own persistent facts | None required | Views, interpolation buffers, cameras and UI state are disposable |
| Errors | Domain failures reported with no partial material commit | Machine-observable errors and process/test outcome | User-facing error/feedback plus diagnostics; no bypass of core rules |

A **read-only observation** is a logical interface. It does not require a full copy of all world state on each frame, or prohibit carefully scoped immutable ECS queries. Detailed view data formats and caching will be designed when T4/T6 use cases justify them.

### 2.3 Headless path

**Logical owner:** the headless execution driver, which starts the same core and explicitly requests advancement for fixtures, diagnostics and automated tests.

**Inputs:** valid startup request, deterministic fixture as later specified, accepted commands and explicit advancement requests.

**Outputs:** transition results, committed material observations, diagnostic reports and a success/failure outcome.

**Invariants:**
- Works with **no display, window, audio device or GPU requirement**.
- Does **not initialize** rendering, native window or audio backends.
- Does not simulate merely because wall-clock time passes while no advancement is requested.
- Does not contain simplified or alternative gameplay authority that could diverge from the graphical configuration.
- Can be exercised in an automated test without a presentation process.

**Implementation constraint inherited from T3.1:** the core/headless build uses the accepted `bevy` configuration with defaults disabled and features `std` and `default_app`; `client` remains opt-in. The T3.1 probe used `MinimalPlugins` and manual App updates, but that observation **does not lock** the T4 plugin or runner implementation. Headless *runtime initialization* and inclusion in the *compile-time dependency graph* are separate questions; the accepted core graph is the starting constraint.

**Failure behavior:** invalid input, invalid command and fatal invariant failures are visible to the harness; no window or renderer is needed to diagnose them. Tests must fail on unexpected outcomes rather than only printing a message.

### 2.4 Graphical presentation path

**Logical owner:** presentation/application interaction. It may own camera, UI selection, input mapping, visual effects and other ephemeral display state. It cannot own persistent ship, cargo, faction, clock or world facts.

**Inputs:** committed observations and command results from the core; user/device input; presentation configuration and independently licensed assets where applicable.

**Outputs:** images/UI and *intent requests* directed to the core. Exact input devices, HUD layout, render graph, audio and art assets are not selected in T3.2.

**Invariants:**
- Rendering reads **committed authoritative state** (or an explicitly non-authoritative interpolation of it); it does not decide material outcomes.
- User input is normalized into a request and reaches the **same validator** used by headless tests. A UI affordance cannot grant capabilities the domain rejects.
- Frame updates and cosmetic interpolation may vary; they cannot change committed material results simply because a different monitor or rendering rate is used.
- Presentation resources may be recreated or discarded without creating, deleting or duplicating authoritative world identities.
- If an authoritative request is rejected, presentation shows the rejection or remains consistent with the authoritative state; it does not optimistically commit world facts.

**Accepted technology boundary:** Bevy 0.20.1 and opt-in `client = ["bevy/3d", "bevy/ui"]` are the only T3.1-selected graphical dependency additions. The graphical probe demonstrated a basic scene and window, not a production renderer or official UI design. It used Vulkan on the reference workstation, but Vulkan is **not** a mandatory backend decision. No audio feature or additional direct dependency is introduced here.

### 2.5 Interaction and observation flow

```text
User / automation / test
          |
       intent
          |
   authoritative command validation
      /                       \
  reject                    accept
    |                          |
diagnostic/result       ordered material transition
    |                          |
    +------------ committed result/observation
                                      |
                           headless assertion or UI view
```

A rejected request leaves the material session unchanged. An accepted request has one authoritative write boundary; the resulting observation may subsequently be used by any number of presentation or diagnostic consumers. Section 4 specifies the logical commit contract. Concrete atomicity mechanism, queue ordering and command types remain deferred to T3.3 and the T4 plan as applicable.

**Example (illustrative only):** A graphical control may request a ship maneuver. Only the flight/physics authority will later change the ship's canonical motion; the renderer may interpolate its visual pose without becoming a second owner. The T4 foundation does **not** need to implement a ship, flight control or physics system to prove this boundary.

### 2.6 Minimum T4-oriented review checks (not executed tests)

| ID | Scenario | Expected result |
|---|---|---|
| **D4-B01** | Start a headless foundation fixture in a graphics-free environment | Initialization and explicit updates work without graphics, window or audio runtime. |
| **D4-B02** | Observe a core state before and after explicit advancement | Material observations change only as permitted by accepted core transitions. |
| **D4-B03** | Submit invalid request from a headless fixture | Request rejected; authoritative state unchanged and failure diagnosable. |
| **D4-B04** | Submit the equivalent invalid request through graphical interaction (when supported by T4 fixture) | Same domain validation outcome; UI does not bypass authority. |
| **D4-B05** | Change only camera/UI/presentation state | No authoritative material change. |
| **D4-B06** | Change only frame pacing or render update count | No simulation advancement attributable solely to rendering; exact cross-run equivalence is specified later by T3.3/T3.4. |

These are acceptance **intent**, not test results. T3.9 must turn the relevant subset into executable Windows commands and fixture definitions; T3.10 must map them to ordered T4 tasks.

## 3. T3.2C — State ownership and mutation

### 3.1 Contract

**Owner:** the authoritative core owns the session; each material domain has one designated logical writer. Application orchestration controls session lifecycle, not domain facts. **Inputs:** validated definitions, committed material state, validated requests, and authorized autonomous transition work. **Outputs:** committed material changes, pending effects and observations. **Invariant:** one authority per fact at a time, including across ECS/record representations. **Failure:** reject invalid proposed changes without exposing partial writes; on unexpected invariant failure, quarantine the session and stop advancement (§4.4).

### 3.2 Classify before storing

| Class | Examples | Write policy | Snapshot treatment |
|---|---|---|---|
| Material | Clock, stock, motion, ownership | Designated simulation authority | Include as required for continuation |
| Validated definitions | Ship class, commodity definition | Content validation and activation boundary | Version/reference binding per D9 |
| Pending material work | Scheduled delivery, accepted future effect | Owning simulation domain and scheduler | Preserve if required for continuation |
| Application | Boot phase, mode, stop request | Application orchestration | Not material |
| Derived | Lookup index, computed summary | Rebuild from authoritative inputs | Normally reconstruct; never competing truth |
| Presentation | Camera, selection, interpolation | Presentation | Not material |
| Diagnostics | Errors, timing, debug output | Diagnostics | Separate evidence, not material truth |

Persistent settings are not necessarily material; authoritative state is not necessarily durable. For example, a lifecycle phase is authoritative for orchestration but need not be saved. An RNG cursor or allocator counter may be material if necessary for continuation. D9 specifies exact capture requirements.

### 3.3 Ownership register

All rows describe logical ownership, not existing modules. Only the narrow foundation subset is implemented at T4. Readers receive committed observations or scoped immutable access; cross-domain writes follow §3.4. The persistence adapter reads committed facts and never independently edits live state.

| Fact | Exclusive logical writer / write boundary | Typical readers | First relevant implementation |
|---|---|---|---|
| Simulation clock and execution order | Clock/scheduler at authorized advancement | All simulation domains, diagnostics | T4.3 |
| Material random stream state | Designated stream owner under RNG contract; consumption only in authorized transitions | Owning domain, replay diagnostics | T4.4 |
| Durable ID allocation, registration, retirement | Identity authority at lifecycle transition | Domain references, lookup, persistence | T4.4 |
| Definition data and version binding | Content authority on validated activation | Generator and simulation domains | T4.5 |
| Generated topology and site facts | Universe authority at initialization or validated world change | Navigation, economy, agents | T4 primitives/T5.2 |
| Canonical location, local pose and motion | Spatial/flight or transit authority according to representation; exclusive handoff | Navigation, logistics, presentation | T4 primitives/T5/T6 |
| Inventory and stock quantities, economic balances | Inventory/economy authority on validated transaction | Production, logistics, trade | T5.2–T5.4 |
| Shipment manifest/progress/outcome | Logistics authority on dispatch/progress/resolution; quantity transfer coordinated with inventory | Economy, operations | T5.4 |
| Asset legal ownership/control | Relevant ownership/faction authority on validated transfer; no duplicate inventory owner record | Economy, faction, missions | T5 and later |
| Faction relationship, jurisdiction and permissions | Faction authority on validated policy/authorization changes | Agents, operations, gameplay validation | T5 and later |
| Readiness, damage, fitted equipment | Ship/asset authority on validated readiness, damage or fitting transition | Operations, combat, economy | Abstract readiness T5; other detail later |
| Objectives, allocations, assignments | Mission/operation authority on eligibility, allocation and resolution | Agents, faction, presentation | Abstract operation T5; broader missions later |
| Actor knowledge and reputation | Knowledge/reputation authority on evidenced observation or consequence | Agents, factions, UI | Later relevant contract |
| Significant causal records | Transition recording authority as part of commit | Diagnostics, missions, player-facing reports | T5 |
| Pending material events | Owning domain determines meaning/outcome; scheduler owns ordering and dispatch bookkeeping | Owning domain, persistence | T4 clock primitives/T5 |
| Durable snapshot representation | Persistence adapter captures committed state; restore only via validated initialization | Load/restore, diagnostics | T4.6/T5.7 |

The register is a proposed refinement of D1 §6, not a complete schema. Overlapping concepts must be split by fact: shipment status is not cargo quantity; ship identity is not current pose; legal ownership is not operational assignment. If a future contract exposes ambiguity, settle the owner before implementing the write. No second canonical field for the same fact merely for convenience.

### 3.4 Cross-domain mutation

1. Read current authoritative facts and identify the affected owners.
2. Validate all relevant preconditions and calculate the intended changes before mutating.
3. Coordinate writes through the authorized transition boundary; each owner remains responsible for its own facts. Apply the whole logical change or reject it without material effects.
4. Publish committed results and significant causal records only after the logical transition succeeds. A future causal reaction is distinct from the original commit and must be explicitly scheduled or executed according to T3.3.

Illustrative T5 transfer: a shipment arrival validates the shipment's unresolved status and cargo manifest; logistics resolves the shipment, inventory transfers accountable quantities to destination, and the committed outcome records the causal connection. Failure must not mark arrival while leaving cargo undelivered or permit a second credit. No shipment implementation is required at T4.

### 3.5 ECS and representation guardrails

Bevy ECS may hold authoritative active objects as well as transient views. Authority is determined by the semantic ownership of a component/resource, not by whether it is in ECS. Compact remote records may be authoritative where appropriate. An ECS entity handle is a runtime reference, never a durable identity. A visual projection can be destroyed without retiring the underlying actor. Active/remote handoff must explicitly transfer authority, retain identity, quantities and pending work, and prevent simultaneous writers. Section 9 specifies the proposed handoff and identity contracts. Prefer ordinary scoped Rust/Bevy access and small modules; no generic repository layer, global mutable world singleton, separate simulation process or universal event store is required.

### 3.6 Proposed review/test evidence

| ID | Scenario | Expected result |
|---|---|---|
| D4-C01 | Inspect each T4 material field and mutation path | Exactly one designated logical owner; no unclassified field or competing writer |
| D4-C02 | Rebuild/discard a derived observation or view | Canonical material fixture unchanged |
| D4-C03 | Reject a small multi-field fixture transition | All affected material facts and pending effects unchanged |
| D4-C04 | Restore an invalid foundation fixture when T4.6 supports restore | No partial live session published |
| D4-C05 | Review identity/representation handoff | Exclusive writer and stable reference policy specified in T3.5; actual material handoff verified at T5.6 |

## 4. T3.2D — Commands, commits and schedule responsibilities

### 4.1 Contract

**Owners:** interaction adapters submit intent; domain authority validates and commits; scheduler controls eligibility and ordering; application orchestration admits advancement; presentation observes results. **Inputs:** current committed state, request payload, relevant actor/context, and an authorized execution opportunity. **Outputs:** explicit rejection or committed result (or acknowledged pending request, if supported later), material changes and observations. **Invariants:** submission is not authorization or completion; no partial material effect on expected rejection; no presentation writer; autonomous work does not require a player command. **Failures:** ordinary invalid input returns a reason; unexpected invariant failure stops advancement and invalidates the affected session until explicitly recovered.

### 4.2 Request and outcome semantics

| Stage | Responsibility | Material writes? |
|---|---|---|
| Submit | Normalize UI, fixture or agent intent; supply necessary actor/target context | No |
| Admit | Check session state and supported request shape; accept for consideration or reject | No material domain changes |
| Validate | Resolve authoritative references and check permissions, preconditions, current values and conflicts at execution | No |
| Prepare | Calculate all affected changes; ensure fallible checks complete before writing where practical | No externally visible material writes |
| Commit | Apply authorized changes as one logical transition, including required pending effects and causal records | Yes, designated owners only |
| Observe | Return outcome and expose committed facts | No |

No generic command bus, network protocol, queue format, universal command enum or transaction library is selected. A synchronous function or a minimal ECS-mediated boundary can satisfy T4. Commands that will execute later must be revalidated when due; acceptance into a queue is not proof of success. Request correlation is required only to the extent needed to distinguish outcomes in the chosen T4 fixture. Duplicate/retry semantics for material transfers belong to the relevant domain contract; an already resolved operation cannot be applied twice. Domain authorization uses current facts, not UI flags or caller assertions. Test fixtures exercise the same validators as graphical input; they may construct an initial state via a separate validated setup path, not bypass runtime rules.

### 4.3 Schedule responsibilities

These are logical execution responsibilities, not names of Bevy schedules or a commitment to a particular API.

| Responsibility | Trigger / input | Permitted output | Prohibited behavior |
|---|---|---|---|
| Boot and lifecycle | Startup/stop requests | Readiness, lifecycle status | Implicit world advancement |
| Input/interaction | Device input, headless fixture, autonomous intent | Proposed requests | Direct material writes |
| Advancement admission | Driver requests under future clock policy | Authorized simulation work | Advancement solely because rendering ran |
| Authoritative execution | Authorized work, ordered eligible commands/events | Validated material transitions, pending work | Unordered competing writes to same fact |
| Commit visibility | Successful logical transition | Committed observations and results | Expose intermediate invalid state |
| Presentation | Committed observations, frame time | Visual/UI state | Change simulation clock, physics, stock, RNG or identity |
| Persistence/diagnostics | Explicit request at safe boundary | Snapshot or evidence | Read incomplete transition as committed truth |

Minimum dependency ordering: initialize before advance; validate before mutation; complete the material transition (including any required deferred ECS mutations) before publishing observations; snapshot only a committed boundary. This is a partial order, not a full tick algorithm. T3.3 chooses timing, pause/step behavior, tie-breaks, simultaneous input policy, event ordering and catch-up. Presentation may run while the simulation is not advancing. A graphical frame may drive an explicit advancement request via the later clock policy; it is not itself a material tick. Autonomous work may advance without user input. No Bevy default schedule ordering or parallel execution behavior is assumed to guarantee determinism.

### 4.4 Commit and failure semantics

For T4 use the smallest coherent mutation boundary: perform fallible validation and calculations first, then apply a small set of changes without exposing intermediate values to consumers. Where ECS structural writes are deferred, completion of those writes must precede publication. No universal rollback engine, database transaction system, per-tick world clone or event sourcing is required. Expected domain rejection must leave affected material state, material RNG consumption and pending effects unchanged; diagnostic counters may change. If an unexpected failure occurs after mutation begins, do not report a successful commit, expose the damaged state as valid, or permit further advancement/snapshot capture from it. Report the failure and discard/quarantine the session; recovery from a previously valid snapshot is a later D9 contract. This is fail-stop containment, not a guarantee of rollback from panic or process termination. A previously committed transition is not undone by later presentation/diagnostic failure.

### 4.5 Minimal T4 proof

Use a non-gameplay fixture with a small material value and an associated related value, an accepted bounded change and an invalid change that would violate a fixture invariant. Exact initial values and request names are fixture proposals for T3.9/T3.10, not gameplay content. Prove that the headless driver and client adapter use the same transition boundary, rejection leaves both values unchanged, a valid change is visible only after completion, and presentation-only work leaves the fixture unchanged. Do not implement trade, flight, faction authorization or a universal event system to demonstrate this.

| ID | Scenario | Expected result |
|---|---|---|
| D4-D01 | Submit valid foundation request | One coherent commit; explicit outcome and matching observation |
| D4-D02 | Submit invalid request | Reason returned; no material change or pending material effect |
| D4-D03 | Request becomes invalid before execution (if deferred admission selected) | Revalidate and reject; no stale approval bypass |
| D4-D04 | Observe before and after transition | No partial material result visible; deferred structural writes complete before publication |
| D4-D05 | Run presentation work without advancement admission | No material transition or simulation clock advance |
| D4-D06 | Exercise fatal invariant failure using a controlled fixture | No further advancement or valid snapshot publication; diagnostic failure |
| D4-D07 | Advance eligible autonomous fixture work without player request | Progress occurs through authorized core boundary; T4.3 timing proof, richer T5 causality later |

## 5. Cross-boundary design rules

1. **Authority is independent of Bevy entity handles.** Authoritative facts may eventually be backed by ECS components or non-ECS aggregates, but persistent identity and handoff details are deliberately assigned to T3.5.
2. **Configuration does not bypass domain validation.** Changing execution mode or presentation features does not change which material commands are legal.
3. **The runtime is not the save format.** Application phases, window state, render entities, input devices and GPU objects are transient. D9/T3.8 will define the material snapshot and pending-effect boundary.
4. **No parallel simulation architecture is selected.** The proposal does not require threads, async runtimes, channels, servers or IPC. Bevy may perform internal work as configured; deterministic ordering is a separate T3.3 contract.
5. **Avoid premature modularization.** Logical responsibilities are not required to become Cargo crates, plugins or elaborate traits in T4. Choose the smallest maintainable implementation once T3 is accepted.
6. **No automatic real-time catch-up.** Startup and graphical frame pacing do not imply wall-clock advancement while the program was closed. T3.3 governs intentional advancement.

## 6. Consolidated T3.2 review and historical checkpoint

### 6.1 Traceability and design disposition

| Source requirement | D4 response | Review disposition |
|---|---|---|
| D0 P2/P3, D1 §8 — independent causal progression | §§1, 4: explicit advancement and autonomous eligible work | Addressed at boundary level |
| D0 P9, D1 §§6, 8 — deterministic foundations and selective detail | §§3–4: one writer, explicit order dependencies; T3.3–5 refine | Addressed; algorithms deferred |
| D1 §6 — separation, validated mutations and durable IDs | §§2–4: ownership register, request/commit and view boundaries | Addressed; concrete IDs deferred |
| D1 §7 — application responsibilities | §§1, 4: lifecycle, diagnostics and schedule separation | Addressed; clock semantics T3.3 |
| D10 v0.1.2 §4 — T3.2 deliverables | §§1–4: boot, headless/client, ownership, validation, schedule responsibilities | All design deliverables drafted |
| ADR-002/T3.1 — exact technology, opt-in client and licensing | §§0, 2: unchanged features and dependency policy | No change proposed |
| D10 §5 — T4 scope | Small foundation fixture; no gameplay or dependency additions | Addressed; executable plan T3.10 |

No substantive conflict identified with the inspected governing baseline. Earlier A/B review draft is retained in substance, with scope/status updated and the autonomous transition wording clarified. The proposed logical phases, ownership splits, fail-stop policy and schedule responsibilities are new D4 decisions for owner acceptance, not previously accepted implementation details. No separate ADR is proposed at this point: record these decisions in D4; owner may request an ADR for significant choices before banking.

### 6.2 Review and evidence checkpoint

| Item | Status |
|---|---|
| T3.2A lifecycle/boot | Draft complete; review pending |
| T3.2B headless/client | Draft complete; review pending |
| T3.2C ownership/mutation | Draft complete; review pending |
| T3.2D commands/schedule responsibilities | Draft complete; review pending |
| Test scenarios | Proposed; no execution evidence |
| Production code/dependencies | No changes |
| Documentation | Consolidated v0.1.0 review copy; no commit/push |
| Next work | T3.3 draft authorized by owner continuation; formal acceptance and banking of T3.2 still pending |

T3.9 maps applicable D4-A/B/C/D checks to executable fixtures, commands and evidence; T3.10 determines exact file tasks and T4 budgets. No numerical performance thresholds are selected here. Measure and justify foundation timing budgets in T3.9/T3.10 before T4 coding, with gameplay scale budgets at T5/T6. All mandatory invariant failures must be zero in acceptance tests. No additional API claims beyond the accepted probe are made; verify concrete APIs before selecting implementation in the later contracts/plan.

### 6.3 Evidence and limitations

The prior T3.2A/B turn inspected remote `main` at `a09251b8482894b9e2ae01b9c5afc1281b0e6a4b`, including accepted D10 v0.1.2, ADR-002 and T3.1 documents. That tree was documentation-only and contained no D4, Cargo manifest or source tree. This revision uses the supplied D4 draft and the same inspected baseline; local checkout and current remote head have not been reverified in this revision. T3.1 probe evidence is historical owner-reported evidence, not newly executed tests. Proposed repository path `docs/architecture/IM_D4_Simulation_and_Engine_Architecture_v0.1.0.md` follows the established directory convention; creation and publication remain owner-controlled.

**Source references:** D0 v1.1.1 §§3, 5.3, 11.2, 12; D1 v2.1.1 §§4–8, 11, 13–14; D10 v0.1.2 §§4–5, 10–11; ADR-002 v1.0.0; accepted T3.1 technical contract, checkpoint and licensing assessment.

**Checkpoint:** T3.2 remains ready for consolidated owner review, not accepted. The owner authorized drafting T3.3 below; this does not waive prerequisite acceptance before banking or authorize T4.

## 7. T3.3 — Simulation clock and scheduling

**Disposition:** proposed contract for owner review; no runtime evidence. T3.2 remains an unbanked prerequisite. The owner authorized continued design, not a waiver of formal acceptance or T4 authorization.

### 7.1 Clock ownership and time domains

The simulation clock/scheduler is the exclusive writer of committed simulation time. Inputs are a validated initial clock state, explicit advancement requests, eligible ordered intents and scheduled operations. Outputs are committed tick/time, transition results and pending work. Application and presentation may request work or read observations but cannot set the clock directly.

| Domain | Meaning | Authority / persistence |
|---|---|---|
| Wall time | Monotonic elapsed real time used to pace a running client | Driver; transient; never a world fact |
| Simulation tick | Integer count of successfully completed base steps since session epoch | Simulation; material, restored with state |
| Simulation duration | Tick multiplied by the agreed base quantum, relative to session epoch | Derived; checked arithmetic; no independent mutable clock |
| Presentation time | Frame delta and interpolation fraction | Presentation; transient |
| Fictional calendar | Human-readable mapping of simulation time into setting | Deferred to D3/content; no canonical epoch chosen |

Propose a nonnegative `u64` tick index and a fixed positive quantum expressed in integer nanoseconds, with checked arithmetic (widen multiplication or reject overflow). These are representation proposals, not Rust APIs. Tick 0 denotes the initial committed fixture; processing step 1 advances from 0 to 1. No time is consumed by boot, validation alone, rendering, rejected advancement, or process shutdown. Save/restore must preserve the quantum/version and committed tick under T3.8; changing the quantum is a compatibility decision, not a user setting.

**T4 fixture quantum:** 100 ms (10 steps per simulated second), chosen solely for simple exact arithmetic and readable tests, not a flight or production rate. T6 must validate flight timing and any subdivision; T5 must validate coarse remote operation timing. Do not treat 10 Hz as a final world or physics rate. T3.10 must explicitly ratify the T4 fixture and any runtime default before coding.

### 7.2 Driver and advancement admission

A ready session enters Running with an explicit advancement control state. Proposed states: running and paused, orthogonal to the application lifecycle. Headless fixture starts paused and can request an exact number of steps; a graphical foundation session starts paused until explicitly started. Pause governs material advancement, not UI, diagnostics or lifecycle. No automatic advancement while closed, loading, or paused; no wall-clock timestamps enter material rules. A paused session may accept control requests, but material intents are either queued for a named future step or rejected; T4 uses explicit fixture inputs for the next step only. No hidden mutation on pause.

| Request | Rule |
|---|---|
| Run | Enter running; reset wall pacing anchor; no implicit elapsed-time advance |
| Pause | Stop admission after current committed step; clear transient pacing debt; no further ticks |
| Step | While paused, request exactly one complete tick; remain paused afterward |
| Advance N | Headless explicit bounded request while paused; equivalent to N single steps, stopping on fatal failure |
| Stop | Reject new admissions; finish already admitted transition at safe boundary or fail-stop |

For T4, multi-step advancement is synchronous and bounded by a finite caller-supplied count; zero is a no-op, negative/unrepresentable counts rejected at input parsing, overflow rejected before starting. No unbounded run-until operation required. A domain command rejection does not cancel an otherwise valid tick; an invalid advancement request does not change the clock. Fatal failure during a tick quarantines the session under §4.4; no tick completion or valid snapshot is published for that failed tick. Earlier committed ticks remain historical results; no rollback guarantee for damaged in-memory state.

### 7.3 Graphical pacing and bounded catch-up

The client driver accumulates monotonic elapsed time only while running. At each driver service opportunity, request one base step per whole quantum available, subject to a cap; subtract only admitted/completed steps. Retain sub-quantum remainder. Rendering may run zero, one or many times between steps and never mutates material state. Headless explicit stepping bypasses wall pacing entirely.

**Proposed T4 fixture policy:** cap at 4 ticks per driver service and clamp accumulated wall debt to 400 ms (4 fixture quanta). Excess wall time is discarded with a diagnostic counter, not converted into skipped simulation ticks. Example: a 1-second stall requests at most four 100 ms steps, drops 600 ms wall debt, and advances exactly 400 ms of simulation time. The simulation runs slower than real time under overload rather than silently skipping operations or doing unbounded catch-up. This is an intentional responsiveness trade-off for the foundation, not a final gameplay speed policy. Pause/resume clears remainder and anchor so paused time never accrues. Reject invalid or backwards elapsed samples without changing material state; record diagnostics. Use integer durations and checked/saturating clamp at the driver boundary. Do not compare graphical runs by equal wall time; compare equal committed ticks and ordered inputs.

No time acceleration, offline catch-up, analytic remote catch-up, focus-loss pause policy, adaptive step size or interpolation API is selected. Remote progression during admitted game time is a later D3/T5 contract; it must not depend on camera visibility. T6 establishes flight responsiveness and pacing budgets; T3.9/T3.10 establish foundation measurement conditions and thresholds before implementation.

### 7.4 Deterministic step responsibilities

For each admitted step with target tick k = committed tick + 1:

1. Check readiness, advancement permission and tick arithmetic. Freeze the finite eligible input batch for k. Inputs arriving after the cutoff are considered no earlier than k+1; no wall timestamps determine ordering.
2. Process eligible commands in explicitly specified order, validating against the latest material state including earlier successful transitions in this step. Expected rejections consume no material RNG or pending effects and produce diagnostic/outcome records.
3. Process eligible scheduled operations in deterministic order; revalidate each against current facts. Process a bounded autonomous fixture phase even without commands. A future domain contract may refine the phase structure only through explicit dependency/order review.
4. Complete required writes, including deferred structural mutations, and verify fixture invariants. Commit tick k and publish the completed tick observation and ordered material results. No intermediate observation is exposed to presentation or persistence. Individual logical transitions may commit internally to provide the current facts for subsequent operations; a fatal tick failure invalidates the session rather than promising whole-tick rollback.

Proposed T4 ordering key: (target tick, phase ordinal, unique sequence within phase). Commands precede scheduled operations, then autonomous fixture work. Sequence is assigned from the explicitly ordered admitted input list or deterministic operation creation order, never thread arrival, ECS iteration order, entity handles, pointer address, hash-map iteration or wall time. Equal complete keys are invalid; reject malformed batches before mutation. Autonomous candidates must be enumerated in a stable defined order before assigning sequence. Persistent tie-break identity and pending-event sequence allocation are refined in T3.4/T3.5/T3.8; no ID format or queue storage chosen here. T4 may use a small ordered vector of fixture inputs. Any future concurrent producer requires an explicit deterministic merge contract; parallelism is not necessary to prove T4.

Due means target tick equals k; pending work targeting a past tick is an integrity/restore error, not silently executed late. Work produced during k for scheduled execution targets k+1 or later; immediate required effects belong to the originating coherent transition, not a recursive same-tick queue. This prevents unbounded same-step cascades. The eligible batch is finite; supported fixtures bound autonomous work and pending operations. Future domains specify resource limits and dispositions before scaling. An operation that is no longer eligible resolves according to its domain rejection/cancellation contract; it must not disappear silently or retry forever. T4 fixture removes the resolved operation and reports its outcome. Tick time advances once even with no commands or due operations.

### 7.5 Bevy integration and reproducibility boundary

Use one shared authoritative step entry for headless and graphical drivers. Application startup, device input, rendering and diagnostics may use engine facilities, but the authoritative schedule must be explicitly invoked only by admitted work. Do not assume Bevy's default fixed update, virtual time, automatic deferred application or executor ordering meets this contract. The accepted T3.1 probe verified explicit App updates and MinimalPlugins feasibility, not this schedule. Prefer a small synchronous ordered implementation for T4; selection of concrete Bevy 0.20.1 APIs, schedule labels, plugin placement and test commands is deferred to T3.9/T3.10 and requires version-specific verification. No new dependency or API claim is introduced.

Determinism claim is limited to the same initial material state, quantum, ordered inputs, supported build/configuration and number of committed ticks; compare canonical material results and ordered material events, excluding presentation, timing diagnostics and raw ECS handles. T3.4 specifies RNG/version and canonical comparison; T3.8 defines continuation including pending effects. No cross-platform bitwise guarantee is made.

### 7.6 T4 fixtures and expected outcomes

All values below are invented non-canonical test data. Proposed fixture: tick=0, scalar balance=10, autonomous counter=0, positive bounded balance [0,20]; every completed tick increments counter by 1. Commands adjust balance only when result is in range; scheduled operation uses same validation rule. No randomness needed. Each row starts from initial fixture unless specified. Diagnostics are not material events. Values/sequence are exact, not benchmark targets.

| ID | Input / scenario | Expected committed result |
|---|---|---|
| D4-E01 | Boot, then no advance | tick=0, balance=10, counter=0 |
| D4-E02 | Paused, service arbitrary wall delta and render | tick=0, balance=10, counter=0 |
| D4-E03 | Step once paused | tick=1, counter=1, duration=100 ms; still paused |
| D4-E04 | Advance 5 vs five single steps | tick=5, counter=5, duration=500 ms; same balance and ordered outcomes |
| D4-E05 | At tick 1 commands +8 then +5 | +8 commits, +5 rejected; balance=18, counter=1, tick=1 |
| D4-E06 | At tick 1 commands +5 then +8 | +5 commits, +8 rejected; balance=15; verifies explicit order matters |
| D4-E07 | At tick 1 command +8, scheduled +5 due 1 | command commits, operation rejected/resolved, balance=18, tick=1; no pending retry |
| D4-E08 | Scheduled +2 due tick 3; advance 2 then 1 | balance=10 through tick 2; balance=12 at tick 3, pending removed once |
| D4-E09 | No player inputs, advance 3 | tick=3, counter=3; autonomous work progresses |
| D4-E10 | Running pacing samples 40, 60, 250 ms | ticks 0,1,3 respectively, remainder 50 ms, duration 300 ms |
| D4-E11 | Running pacing 1000 ms stall from zero debt | tick=4, counter=4, remainder=0; 600 ms dropped wall time reported |
| D4-E12 | Run 50 ms, pause, service 5 s, resume, run 50 ms | tick=0, remainder=50 ms; no paused or pre-pause debt replay |
| D4-E13 | Zero steps; invalid request; overflow target | no material change; zero succeeds, invalid/overflow rejects |
| D4-E14 | Duplicate complete ordering keys or past-due work at startup | reject invalid fixture/batch without advancement |
| D4-E15 | Controlled fatal failure within admitted step | no committed tick observation for failed step; no further advance or valid snapshot |
| D4-E16 | Same ordered inputs and N admitted ticks with differing render cadence/headless | identical fixture material values and ordered material results |
| D4-E17 | Schedule follow-up during k for k+1 | no execution in k; resolves once in k+1 |
| D4-E18 | Stop after committed tick | no new advancement; last committed tick unchanged |

T3.9 defines concrete fixtures, test names and evidence capture; T3.10 maps checks to T4.2/T4.3 tasks. D4-E16 can use a client adapter without opening a window for the deterministic test; native graphical launch and backend absence require separate T4.1/T4.2 evidence. T3.8 adds restore/continuation cases; T5 tests actual causal operations. No tests have been executed.

### 7.7 Review and open decisions

| Decision | Proposed disposition |
|---|---|
| Integer tick and fixed quantum | Adopt for T4 fixture; production rates deferred |
| Pause/step and explicit headless advance | Adopt; no offline advancement |
| Client bounded catch-up | Adopt fixture cap 4 / 400 ms; overload slows simulation; revisit T6 |
| Ordered finite phases and next-step deferred work | Adopt T4 contract; later domain schedules must refine explicitly |
| API selection and performance budgets | T3.9/T3.10, before T4 implementation |
| RNG, identity, persistence | T3.4/T3.5/T3.8, no premature selection |

Traceability: D0 P2/P3/P9 and §§5.3, 11.2, 12; D1 §§6–8, 14; D10 v0.1.2 §4 T3.3 and §5 T4.3; D4 §§1–5. No conflict identified with T3.2 draft or accepted T3.1 dependency policy. No new ADR issued; the clock/pacing choices are proposed in D4 and may warrant an ADR at owner acceptance if elevated beyond fixture scope.

**Stop:** T3.2 and T3.3 remain ready for owner review, not accepted or banked. T3.4 requires accepted prerequisites or explicit authorization for non-binding exploration; T4 remains blocked. Remote main rechecked at a09251b8482894b9e2ae01b9c5afc1281b0e6a4b; local checkout not inspected. No repository writes, dependency changes, builds or runtime tests.


## 8. T3.4 — Deterministic randomness

**Status:** Proposed for owner review. Continued design authorized; T3.2/T3.3 remain unbanked. Governing basis: D0 P9 and §12, D1 §§6, 8, 9, 14, D10 v0.1.2 §4 T3.4 / §5 T4.4, ADR-002 and T3.1 contract. Companion: D3 v0.1.0 §1. No additional dependency or production implementation authorized.

### 8.1 Authority and reproducibility scope

The simulation randomness authority owns material random stream creation, derivation, consumption and continuation. Inputs: explicitly supplied master seed, RNG contract version, stable domain/context identifiers, committed stream state and ordered eligible operations. Outputs: deterministic raw words and bounded samples, updated committed stream state, and sufficient version/context information for diagnostics. No wall time, renderer activity, transient ECS handles, randomized hash iteration or thread arrival order is a material entropy source. The application receives a seed explicitly for T4; automatic seed selection and user-facing seed syntax remain deferred. Proposed seed representation: unsigned 64-bit integer; zero is valid.

For the same supported Windows x64 build/configuration, algorithm and generation versions, validated definitions, initial state, ordered inputs and committed tick count, canonical material state and ordered material outcomes must agree. Headless/client configurations must agree on the same fixture even with differing render cadence. No promise of cross-version, cross-platform floating-point, or visual bitwise equivalence. Integer RNG golden vectors are exact, independently of presentation.

### 8.2 Stream ownership and isolation

| Class | Owner | Context and lifetime | Material continuation |
|---|---|---|---|
| Procedural generation | Generation authority (D3) | Versioned domain and stable generation key; reconstruct from seed | Generation inputs/version; generated committed facts remain authoritative |
| Ongoing material resolution | Simulation domain authority | Named domain and stable context; stateful stream where sequential draws are needed | Store current stream state and context at committed boundary |
| Cosmetic effects | Presentation | Independent ephemeral randomness | Excluded from material snapshots and comparisons |

A stream has one writer; no mutable global RNG shared by unrelated domains. Use context-derived generation streams rather than one traversal-dependent stream across independently generated regions. Material streams may initially be per domain, with finer context introduced only when needed. Stable instance IDs are specified in §9 for review: existing T4 RNG fixtures retain explicit integer context keys, never ECS handles. Distinct keys identify distinct streams, but a 64-bit derivation cannot guarantee collision-free or statistically independent sequences; do not use seed values as identity. No cryptographic or adversarial unpredictability guarantee.

### 8.3 Proposed minimal algorithm for T4

Select for review a small, explicitly versioned SplitMix64-based integer generator, implemented with standard Rust unsigned integer wrapping operations only. This is a proposed foundation choice, not a claim that a dependency/API has been validated or that this is the final procedural noise or combat algorithm. It is easy to audit and test and needs no new crate. A high-quality library generator may be considered later through ADR/dependency review if measured needs justify it. Do not copy third-party source; implement from the mathematical specification below. Reference provenance and license review of any external implementation would be required before reuse.

Define mix(x) over unsigned 64-bit words with all multiplications and additions modulo 2^64:

```text
z = x
z = (z XOR (z >> 30)) * 0xBF58476D1CE4E5B9
z = (z XOR (z >> 27)) * 0x94D049BB133111EB
return z XOR (z >> 31)
```

For each raw word, add 0x9E3779B97F4A7C15 to the stream state modulo 2^64, then return mix(state). Shift is logical unsigned. Zero state is valid. Proposed RNG contract version 1 specifies this transition, derivation, sampling, and stream consumption rules. Do not use language-default hashing or unpinned random APIs for material output.

For the T4 fixture, derive an initial state by a fixed sequence of mixing operations:

```text
s = mix(master_seed XOR 0x494D524E47563100)
s = mix(s XOR rng_version)
s = mix(s XOR scope_version)
s = mix(s XOR domain_id)
s = mix(s XOR context_key)
return s
```

Each field is a u64; fixed field count and order are part of the contract. scope_version is the generation recipe version for generation streams and the material rules version for simulation streams, not a clock tick. Separate the namespaces by reserved domain IDs: 1 = T4 generation fixture, 2 = T4 material fixture A, 3 = T4 material fixture B. These IDs are test-only; maintain an explicit non-reused domain register for future production domains before use. No names, case folding, string encodings or arbitrary hashing are implicit in v1. Multi-part persistent context keys require a later explicit encoding/derivation contract; do not compress a durable identifier silently. Identical keys deliberately reconstruct the same initial stream, not a fresh random stream.

### 8.4 Sampling and consumption

T4 supports raw u64 and uniform integer sample in [0,n), n a positive u64. Reject n=0 before drawing. For n=1 return 0 without drawing. Otherwise define threshold = 2^64 mod n (equivalently wrapping unsigned 0-n modulo n); draw raw words until r >= threshold, then return r mod n. This removes modulo bias. Raw draw count can vary; it is part of state continuation, not presumed to be one per sample. For T4 bound rejection sampling to 128 attempts and treat exhaustion as unexpected integrity failure, not a biased fallback; this cap is proposed for bounded fixture work. Consume speculative draws in temporary stream state and commit only with the accepted transition. Expected command rejections, stale requests and canceled operations leave all material RNG state unchanged unless a future approved domain contract explicitly defines a committed stochastic outcome (e.g. failed attempt with material consequences). Distinguish rejection from an authorized attempt that randomly fails. Fatal failures follow D4 fail-stop policy; no snapshot of incomplete tick.

Do not select float sampling, weighted distributions, noise functions, normal distributions or procedural algorithms until a real domain requires them. Probability comparisons can use explicit integer ranges in future fixtures. Record sampling changes as version changes.

### 8.5 Ordering and persistence handoff

Draw only within the authoritative validated transition boundary, following §7's admitted tick/phase/sequence ordering. Never draw to validate a request, render a preview, inspect a read model, or produce diagnostics. Autonomous candidate order must be stable before material draws. No unreviewed concurrent stream writers. If two eligible operations intentionally share a stream, their explicit order determines their draws. Separate stream state must be included in the committed material projection and future snapshot along with seed, version, domain/context key and pending effects as applicable; generation inputs alone cannot reconstruct consumed material streams. T3.8 specifies encoding, validation, continuation and unsupported-version errors; T3.5 specifies stable identifiers. Restore must not silently reseed or upgrade unknown algorithms. Unsupported version, invalid domain/context registration or duplicate stream ownership fails validation before advancement.

### 8.6 Canonical comparison and golden fixture policy

Compare explicitly selected material fields, including committed tick, seed, versions, stable stream keys and states, generated fixture facts, pending effects and ordered material outcomes. Canonical ordering uses explicit stable keys; exclude raw ECS handles, allocation order, wall pacing debt, diagnostic timestamps, GPU state and cosmetic draws. T3.5/T3.8 complete the persistent projection and T3.9 specifies executable comparison and evidence. No serialization format or digest library selected; direct structured equality is sufficient for T4. Golden integer vectors are exact constants derived independently of production implementation; do not regenerate expectations from the implementation under test or update them merely to clear a regression. Intentional changes require version review, rationale and new golden vectors, retaining old evidence. T4 fixture is not fictional canon.

### 8.7 Proposed acceptance fixtures

Raw generator reference vectors (initial state 0):

| Draw | State after draw | Output |
|---|---|---|
| 1 | `0x9E3779B97F4A7C15` | `0xE220A8397B1DCDAF` |
| 2 | `0x3C6EF372FE94F82A` | `0x6E789E6AA1B965F4` |
| 3 | `0xDAA66D2C7DDF743F` | `0x06C45D188009454F` |
| 4 | `0x78DDE6E5FD29F054` | `0xF88BB8A8724C81EC` |

Derivation vectors (master seed 42, RNG version 1, scope version 1):

| Domain | Context | Initial state | First raw word |
|---|---|---|---|
| 1 | 0 | `0xD96D5B15F3607CFF` | `0xDEE0F95F4FB3C83C` |
| 1 | 1 | `0x6F0E6B56B926705C` | `0xA2634125300C15F7` |
| 2 | 0 | `0x370AA4F810B2633F` | `0x3A67B6B626CDF82A` |
| 3 | 0 | `0x7F7006A8480D8CDB` | `0xBE6F836C3F72F265` |

| ID | Scenario | Expected result |
|---|---|---|
| D4-F01 | Raw state 0, four draws | Exact raw vectors above |
| D4-F02 | Derive fixture streams | Exact derivation vectors above |
| D4-F03 | Same seed, domain, version and context twice | Same stream and generated values |
| D4-F04 | Consume extra draws in fixture domain B | Domain A sequence and state unchanged |
| D4-F05 | Generate contexts 0 and 1 in reverse order | Each context retains its own result |
| D4-F06 | n=0; n=1 | Reject without draw; return 0 without draw respectively |
| D4-F07 | Inject raw sequence 5, 16 for n=10 | Threshold=6; reject 5; return 6; two draws |
| D4-F08 | n=u64::MAX, injected raw 0 then max | Threshold=1; reject 0; return 0; two draws |
| D4-F09 | Expected rejected command after speculative draw | No committed material or RNG mutation |
| D4-F10 | Accepted stochastic attempt with failed objective | Commit specified outcome and consumed stream state |
| D4-F11 | Same ordered inputs and committed ticks under headless/client; extra cosmetics | Equivalent canonical material projection |
| D4-F12 | Unknown RNG version / duplicate stream key | Reject before advancement |
| D4-F13 | Rejection sampler exhausts cap via injected raw zeros, n=10 | Fatal integrity failure; no valid partial snapshot |
| D4-F14 | Capture stream state after two raw draws; reconstruct and continue | Next two raw words equal uninterrupted stream (in-memory foundation check; full save/restore T3.8/T5) |
| D4-F15 | Initial state u64::MAX, draw once | State wraps to 0x9E3779B97F4A7C14; no debug overflow panic |

Golden constants were calculated from the proposed mathematical definition using independent Python integer arithmetic during document preparation; no Rust, Bevy or production tests were executed. T3.9/T3.10 specify actual test commands and fixture mapping. Test injection is a test seam, not a production random override. No distribution-quality or speed target selected; revisit in T5/T6 when representative workloads exist.

### 8.8 Decisions and stop point

Owner review required for seed width, SplitMix64-based v1, derivation namespace, bounded sampling, isolated stream policy and exact integer golden vectors. Significant algorithm decision should be recorded in an ADR upon acceptance (identifier to be assigned after checking repository). No external source code reused, new crate, dependency version change, production implementation, or repository write. D3 v0.1.0 begins the generation side of this contract; Section 9 now defines proposed durable identity and lifecycle; this is the historical T3.4 checkpoint. T3.2–T3.4 are drafts, not formally accepted; no banking or T4 authorization implied. Remote reference remains the last verified T3.1 commit; not rechecked in this increment, local checkout not inspected. D10 v0.1.2 and T3.1 records were directly retrieved in the earlier T3.2 review; local Project roadmap copy v0.1.1 is superseded.

## 9. T3.5 — Persistent identity and lifecycle

**Disposition:** Proposed contract; T3.2–T3.4 are unbanked prerequisites. No implementation authorized. D1 §§6, 8–9 and D10 v0.1.2 §4 require durable identity, validated creation/retirement, lookup and exclusive authority across representation changes.

### 9.1 Ownership and identity domains

The identity authority owns allocation, registration, lifecycle status and resolution of durable instance identifiers within one material session. Domain authorities own the facts of their objects, including whether a lifecycle transition is permitted. Inputs: validated creation/retirement requests, committed allocator state, definition references, ordered operations and restore candidates. Outputs: registered identities, resolution results and committed lifecycle changes. Invariant: each live ID resolves to exactly one authoritative representation; no ID reuse in a world lineage. Failure: expected invalid requests leave material state and allocator unchanged; unexpected registry inconsistency triggers §4.4 fail-stop.

| Identity class | Meaning | Lifetime / authority |
|---|---|---|
| Definition key | Reference to validated reusable content (e.g. hull model) | Content authority; exact representation and version binding T3.6 |
| Instance ID | Particular material object (e.g. owned ship, station, shipment, component) | Identity authority; stable across ownership, location and detail changes |
| Generation context key | Explicit stable input to a generation recipe | D3 generation authority; not automatically an instance ID |
| ECS entity handle | Runtime handle for an ECS object | ECS runtime only; never serialized as durable identity |
| Presentation handle | Camera, mesh, UI or other view object | Presentation only; may be recreated freely |

Definition identity is not instance identity; two objects using the same definition must have distinct instance IDs. Display names, owner, location and classification are not identity. Not every ECS entity requires a durable ID; only objects with persistent material identity do. Not every durable object requires an ECS entity.

### 9.2 Proposed T4 instance ID and allocator

Use a single world-local namespace of nonzero unsigned 64-bit instance IDs, represented by a distinct Rust domain type rather than passing arbitrary integers or ECS handles. This is a proposed semantic representation, not a selected Rust API or persistence encoding. Zero is invalid, not a live object or wildcard. No UUID crate, random allocation, type-bit partition, global registry or distributed ID service is necessary. Independent worlds may allocate equal numbers; identifiers alone do not imply cross-world identity. Session replacement clears transient references; cross-world import, merge and branch identity are deferred to D9. A restored continuation preserves its lineage and IDs; divergent copies are not automatically mergeable.

The allocator maintains a committed u64 high-water mark H, initially 0. For n requested new identities, validate n and checked H+n before any material mutation. Assign H+1 through H+n in the declared creation order; commit H+n together with the complete successful creation transition. H=u64::MAX is exhausted, not a wraparound opportunity. A rejected creation consumes no ID; retired IDs are never returned to a free list. Restore H explicitly rather than inferring it from currently live objects; require every known allocated identity to be nonzero and <= H. Do not reset H when a region is unloaded. No persistent per-object tombstone is required merely to prevent reuse; history and references may require retirement records (see §9.4).

Allocation is permitted during validated boot or an admitted authoritative transition (§§1, 4, 7). Boot generation uses stable ordered candidate keys, then allocates and binds references in that order; publish only after all required references resolve. Generation contexts are not allocation sequence numbers; the same independent context can generate the same token in different traversal orders, while IDs are assigned in a canonical registration order for the same complete T4 fixture. Lazy generation across different discovery histories need not allocate identical IDs; its stable identity policy is deferred to D3/T5. Never derive IDs from RNG output or assume seed derivation is injective. Once an instance has an ID, a material RNG stream may use its u64 value as context when its registered domain explicitly defines that convention (§8); stream keys retain their versions and domain. Do not reseed on handoff or retirement; domain contracts decide retention of retired stream state and pending effects.

### 9.3 Registry and reference resolution

The authoritative session must provide a lookup from instance ID to its current authoritative location/representation, or explicit non-live resolution. A derived runtime index may be rebuilt from validated records; it cannot invent objects, override authority, or become competing material truth. A minimal ordered collection or standard Rust map is sufficient for T4; no storage structure is mandated. Map iteration order is never a scheduling or allocation order: sort by numeric ID where a domain has no other approved canonical ordering. All mutations route through §4, not arbitrary registry access.

Resolution distinguishes live and unresolved IDs; if retained evidence exists, distinguish retired from unknown. A known live object in a compact remote record is live, not missing because it has no ECS handle. A missing ID at or below H cannot by itself prove retirement. A reference must declare the expected object kind and validate it at use; strong required references must resolve to a compatible live object; explicitly historical/optional references may be absent or retired under their domain contract. Do not silently substitute a default entity, resurrect an object, or retarget a reference. D9 defines persisted reference encoding and diagnostics. Runtime caches and ECS handles must be invalidated or re-resolved when authority moves, a session ends, or an object retires; handle reuse must never alias another durable ID.

### 9.4 Lifecycle transitions

| Transition | Preconditions | Committed effect |
|---|---|---|
| Create/register | Valid definition if applicable, eligible request, capacity and unique allocation, all required references resolved in candidate batch | New live ID and canonical facts; allocator advances atomically |
| Update/transfer ownership | Live ID, authorized domain transition | Facts change; ID remains unchanged |
| Promote/demote | Live source, compatible destination representation, explicit ownership transfer | Same live ID and material facts; one authoritative writer after commit |
| Create/remove projection | Live target for new view; no material mutation | Disposable view only; ID allocation and lifecycle unchanged |
| Retire | Live ID and valid domain reason; reconcile required references and pending work | Remove live authority; ID never reused; retain required historical evidence |
| Restore | Candidate validated as a whole, including unique identities, allocator and references | Publish one replacement session; rebuild runtime handles/indexes without allocating replacement IDs |

Retirement is not unload, despawn of a view, docking, capture, or transfer. Domain contracts decide destruction, consumption, salvage and other sinks; retire does not implicitly erase inventory or resolve obligations. Before retirement, required live references and pending effects must be reconciled in the same transition or rejected; historical references may retain the ID and explanation. Scheduled work targeting a retired object must receive an explicit domain disposition (canceled with reason or other authorized outcome), not silently disappear or resolve against a replacement ID. T4 needs only a fixture retirement with a required reference blocking it and a simple cancellation result, not a universal reference graph or history database. Duplicate registration/retirement requests are explicit rejection with unchanged state, unless a future contract defines idempotency semantics.

### 9.5 Active/remote handoff and projection

The relevant domain authority owns the material facts; representation ownership is explicitly transferred, not copied as a second writable truth. A compact record may represent an inactive object; an active ECS representation may own the same material facts when activated. Neither is selected as universally canonical. Presentation may read committed facts and maintain disposable proxies at either detail level; proxy creation/removal is not promotion/demotion. Camera visibility is not the criterion for simulation relevance.

Logical handoff protocol (T4 synthetic proof; real domain rules T5.6):

1. Validate source ID, live status, current authority and eligibility of target representation at an admitted transition boundary. Capture the complete relevant material facts, including pending references and stream context/state where applicable.
2. Prepare and validate target representation without granting it write authority or publishing it. Do not allocate a new durable ID. If preparation fails normally, leave the source and allocator unchanged.
3. Commit the authority switch, registration/lookup change and source deactivation as one coherent transition. Apply required deferred structural work before publication (§7.4). Only the new representation may write after commit; a stale writer is rejected or treated as an integrity violation if it bypasses validation.
4. Publish the committed observation. Invalidate obsolete runtime handles; rebind projections through the durable ID. Preserve identity, canonical material facts, pending work, stream state and tick semantics. No implicit random draw, time advance beyond the admitted step, delivery or destruction occurs merely because the representation changed.

If an unexpected failure occurs after mutation starts, quarantine the session and refuse valid snapshot publication (§4.4); no universal rollback claim. Re-entering an active area resolves the existing live ID, not a regenerated instance. Full flight/remote conversion, relevance criteria, coordinate conversion and resource accounting remain T3.7/T5.6/T6. A T4 synthetic scalar record can demonstrate the authority switch without a ship, actual scene, physics or remote scheduler.

### 9.6 Persistence and deterministic continuation handoff

Canonical comparison extends §8.6 with H, live IDs in numeric order, each object's canonical kind/facts and required durable references, and any material lifecycle records or pending effects needed for continuation. Exclude raw ECS handles, view IDs, runtime indexes and storage layout. Representation-specific flags are included only if they affect future material behavior; a synthetic handoff with no such effect compares equivalent material facts. Restore must reject duplicate IDs, zero IDs, IDs above H, incompatible required references, conflicting authoritative copies and unsupported versions before publishing a session. Preserve H even when the highest allocated ID was retired. T3.8 defines exact snapshot encoding and restoration tests; T4 identity tests may use in-memory candidate records. No save format or new dependency chosen.

### 9.7 T4 fixture and review evidence

Use a non-canonical fixture with H=0 and two objects of the same illustrative definition key; create in declared order A then B. Expected IDs 1,2 and H=2. Each has an integer material value (A=10, B=20); A may have a required reference to B. No actual ship or content schema required; T4.5 later supplies validated definitions. Each case starts from its declared fixture, not a previous test's implicit state. Lifecycle requests are applied at admitted transitions; compare command-level material effects separately from ordinary successful tick advancement.

| ID | Scenario | Expected result |
|---|---|---|
| D4-I01 | Create A then B | IDs 1,2; H=2; same definition, distinct instances |
| D4-I02 | Repeat same ordered initial fixture | Same IDs and canonical facts |
| D4-I03 | Reject invalid creation before commit | No new object, no allocator advance |
| D4-I04 | Candidate duplicate ID or zero ID | Reject entire initialization; no live session |
| D4-I05 | H=MAX-1; request two new IDs | Reject before mutation; no wrap; one new ID remains possible |
| D4-I06 | Retire B without required references; create C | C=3, H=3; B not live; no reuse |
| D4-I07 | Retire B while A requires live B | Reject unchanged; after authorized reference removal retirement may succeed |
| D4-I08 | Missing or wrong-kind required reference | Reject candidate/request; no partial material change |
| D4-I09 | Remove and recreate A's view | A=1, value=10, H=2 unchanged |
| D4-I10 | Promote A from record to active representation | ID/value unchanged, exactly one authority; old writer unavailable |
| D4-I11 | Demote then re-enter A | Same ID, committed value, references, pending work and stream state |
| D4-I12 | Invalid target handoff preparation | Source remains live and unchanged; H unchanged |
| D4-I13 | Restore candidate with two authoritative copies of A | Reject before publication |
| D4-I14 | Resolve stale runtime handle after representation removal | No access to unrelated instance; durable lookup is authoritative |
| D4-I15 | Restore H=2 with B retired; allocate | New ID=3, even though greatest live ID was 1 |
| D4-I16 | Restore live ID above H | Reject candidate before publication |
| D4-I17 | Scheduled request targets retired ID | Explicit cancellation/rejection outcome; no mutation of replacement object |
| D4-I18 | Different runtime handles, same ordered fixture | Canonical material comparison equal; no handle-based ordering |
| D4-I19 | Unexpected handoff integrity failure after writes begin | Stop advancement; no valid snapshot or committed tick observation for failed tick |

All expected results are design fixtures, not executed tests. T3.9/T3.10 assign concrete APIs, commands, evidence and implementation order. Identity/reference tests belong primarily to T4.4; synthetic handoff scope must be explicitly confirmed in T3.10 against T4.6, with full material handoff reserved for T5.6. No performance budget for registry size or lookup latency is invented; establish representative measurements at T3.10/T5. No new Bevy API claim is made.

### 9.8 Review and stop point

Owner decisions requested: world-local nonzero u64 instance namespace, monotonic high-water allocator, canonical creation ordering, no reuse, explicit reference validity and exclusive handoff with fail-stop containment. Alternatives considered: UUID/random IDs (unnecessary for single-world T4 and introduces random/collision policy); ECS handles (not durable); slot reuse with generations (more complexity and lifetime risks); content-derived IDs for every object (cannot naturally represent arbitrary runtime creation). Revisit only for concrete cross-world or scaling requirements; record accepted significant choices in ADR(s) as appropriate. D3 v0.1.0 remains compatible: generation keys are explicitly distinct from runtime IDs. T3.6 next defines content definitions and reference validation in D9; T3.7 spatial representation and T3.8 snapshots remain deferred. No production source, dependency changes, tests or repository writes. Remote reference is last verified T3.1 commit; current remote and local checkout not rechecked in this increment. T3.2–T3.5 remain drafts; formal acceptance and banking still required, T4 unauthorized.


## 10. T3.7 — Spatial coordinate architecture

**Disposition:** proposed contract; no owner acceptance or runtime evidence. D3 v0.3.0 §4 defines topology and geographic semantics; D4 owns numeric and representation interfaces. D9 v0.1.0 owns definition admission; T3.8 will specify snapshot encoding. D1 §§6, 7.1, 8 and D10 v0.1.2 §§4–5 govern. No dependency or concrete Bevy API added.

### 10.1 Representation and ownership

Propose three-component binary64 (`f64`) Cartesian vectors in meters for authoritative spatial primitives. Each canonical location has a durable system ID, a system-relative anchor vector A, and a local offset vector L. The pair is the authoritative position, not a cache of A+L; system-relative coordinates alone are represented by L=0. Local frame axes are parallel to system axes in T4; anchor is a numeric reference origin, not automatically attached to a moving entity. Store the pair without collapsing it to a single absolute vector during handoff or persistence. The spatial authority validates and writes location through §4; flight and transit authorities will acquire exclusive write responsibility under §9.5 where required. Orientation, velocity, angular velocity, time-dependent frames and integration are deferred to D5/T6; no physics library selected.

The T4 representation is deliberately bounded: each anchor component magnitude <= 10^13 m, each local offset component magnitude <= 10^6 m. These are a synthetic precision envelope, not a universe-size or travel-distance limit. Reject nonfinite values, invalid system IDs, and out-of-range values at construction/restore/transition. Normalize negative zero to positive zero at admission; never clamp invalid coordinates. Operations use checked finite results and explicit frame context. For T4, use ordinary scalar Rust arithmetic, no external math library. No implicit conversions between units or coordinate domains. Candidate types and concrete storage layout remain T3.10/T4 decisions.

### 10.2 Relative conversion and precision

For positions P=(system,A,L) and Q=(system,B,M), compute P relative to Q per axis as `(A - B) + (L - M)`, in that order, using f64. Require equal system IDs; reject otherwise. Avoid `(A+L)-(B+M)`, which can lose local precision. Equal anchors allow local subtraction without astronomical cancellation. Do not claim centimeter accuracy for arbitrary far-separated results: anchor input quantization and subtraction error still apply. For T4, close-range relative comparisons are limited to same anchor or representable fixture anchors. A system-relative approximation A+L may be used for coarse inspection, never round-tripped into authoritative state automatically.

Precision evidence (IEEE binary spacing, per component at the stated magnitude, not measured engine errors):

| Magnitude | f32 spacing (m) | f64 spacing (m) |
|---|---:|---:|
| 1,000 m | 0.00006103515625 | 1.1368683772161603e-13 |
| 1,000,000 m | 0.0625 | 1.1641532182693481e-10 |
| 149,597,870,700 m (1 AU reference) | 16384 | 0.000030517578125 |
| 10^13 m | 1048576 | 0.001953125 |

At 10^13 m even f64 absolute positions have millimeter-scale spacing; preserving offsets separately avoids quantizing small local movements into that anchor. This does not make arbitrary frame conversion exact or guarantee future physics precision.

### 10.3 Local re-anchoring and handoff

For a new anchor B in the same system, compute L'=(A-B)+L in f64, validate both new components and target envelope, and retain the same durable object ID. T4 re-anchoring is limited to exactly representable integer-meter anchor shifts of magnitude <= 10^6 m per component within the admitted anchor envelope. Compare relative position before/after against a common reference within the same local neighborhood; require per-axis error <= 1e-6 m for the declared fixtures. Reject out-of-envelope target offsets before mutation. Do not automatically re-anchor based on camera position; future policy for moving objects leaving a local frame is deferred. Preserve all other material facts, pending work, time and RNG state. Representation promotion/demotion under §9.5 may retain the pair unchanged; no need to flatten coordinates to transfer ownership. Unexpected partial mutation uses fail-stop (§4.4). T5.6 and T6 refine actual reference-frame transitions and conservation of motion.

### 10.4 Render-relative projection

Presentation supplies a camera reference Q in the same system. Derive P relative to Q using §10.2 in f64, then narrow each component to f32 for nearby rendering. For T4 fixture projection, require magnitude <= 100,000 m per component; reject/omit out-of-envelope projections without changing material state. This is a near-view test envelope, not a visibility cutoff for the game; distant celestial imagery and layered rendering are deferred. A camera shift rebuilds projections; it does not move authoritative objects, consume RNG, allocate identities or advance simulation time. No inverse conversion from rendered pose to material state; user actions are validated requests. A headless fixture can exercise the projection arithmetic without a GPU. T6 verifies actual engine transforms, jitter, camera behavior and handling with the pinned Bevy version; no version-specific transform API claimed here.

### 10.5 Canonical comparison and persistence

Canonical snapshot fields include system ID, anchor and local offsets; preserve finite f64 values losslessly (including canonical zero). D9 chooses encoding and versioning. Canonical equality for identical ordered inputs and representation uses exact normalized component bits under the supported fixture environment, not approximate equality or renderer coordinates. Equivalent re-anchored representations may differ in bits; compare physical displacement using the explicitly specified tolerance only in conversion tests, not as a general substitute for replay equality. The chosen anchor and offset pair is persisted since future computations can depend on it. Exclude camera origin, view transforms and derived indexes. Do not claim cross-platform bitwise determinism for general future floating-point physics; T3.9 specifies supported execution evidence.

### 10.6 T4 spatial test fixtures

Use system A=1, B=2; per-axis examples on x, y=z=0 unless otherwise stated. Let anchor a=10^13 m, P=(A,a,0.125), Q=(A,a,0.124); expected relative x=0.001 m (tolerance 1e-12 m). For re-anchor fixture use anchor a-1000 (exactly representable), offset 1000.125; compare against Q. Projection tests use camera-relative values directly constructed in the same frame. Numerical tolerances are absolute per component, not relative or vector norm. All inputs are non-canonical test values.

| ID | Scenario | Expected |
|---|---|---|
| D4-S01 | Valid zero/negative positions in valid system | Accepted; -0 normalized |
| D4-S02 | NaN, ±infinity in anchor or offset | Rejected before mutation |
| D4-S03 | Anchor/offset beyond admitted envelope | Rejected; no clamp |
| D4-S04 | Unknown or invalid system ID | Reject candidate or transition |
| D4-S05 | P minus Q at same astronomical anchor | 0.001 m ±1e-12 m |
| D4-S06 | Add 0.001 m to local offset at same anchor | Relative displacement 0.001 m ±1e-12 m; ID unchanged |
| D4-S07 | Same numeric vectors in different systems | Direct subtraction rejected |
| D4-S08 | Re-anchor P to a-1000 and back | Relative position error <=1e-6 m per component, same ID |
| D4-S09 | Re-anchor would exceed offset envelope | Reject; canonical pair unchanged |
| D4-S10 | Project near-relative vector (0.125,-0.25,1.5) m | Exact f32 values |
| D4-S11 | Project (99999.997,0,0) m | f32 narrowing error <=0.004 m per component |
| D4-S12 | Project beyond 100,000 m component magnitude | No near-view projection; material state unchanged |
| D4-S13 | Shift camera by 100 m within supported frame | Projection moves -100 m (within 0.004 m); canonical state, tick and RNG unchanged |
| D4-S14 | Synthetic active/remote handoff retaining canonical pair | Exact normalized component bits, ID and pending state preserved; one writer |
| D4-S15 | Restore candidate with invalid coordinate/system | Reject before publication |
| D4-S16 | Repeat same ordered fixture in headless/client-enabled core | Exact canonical pair and same observations; no GPU required |

Rationale: the f32 spacing at 100,000 m is 0.0078125 m, so half-spacing <=0.00390625 m justifies the 0.004 m narrowing tolerance. The 1e-6 m re-anchor tolerance is limited to the declared integer-shift fixtures, not arbitrary system-scale translation. T4.6 implements only primitives and approved synthetic fixture tests; T3.9/T3.10 specify commands, test placement and performance measurements. T6 sets actual motion/visual precision, throughput and frame budgets before coding. No runtime tests executed.

### 10.7 Alternatives and review disposition

Single global f32 space rejected as a proposal because local precision degrades at astronomical offsets. Single absolute f64 vector simpler but loses small offsets at the upper envelope; not chosen for the fixture. Integer sector grids, fixed-point coordinates, rotating hierarchical frames and third-party floating-origin plugins are deferred until measured need. The proposed anchored f64 pair plus f32 render-relative projection adds explicit context and validation in exchange for precision and independent presentation. Owner approval is needed for this representation, envelopes, tolerance, re-anchor restriction and directed topology fixture. An ADR is appropriate at consolidated acceptance. No new dependency, crate hierarchy, production code, or repository changes. T3.2–T3.7 remain drafts; T3.8 snapshot foundation is next. Current remote/local state was not reverified; last observed commit is the banked T3.1 baseline.

## 11. T3.9 — Testing, continuous integration and compliance

**Disposition:** Proposed for owner review. D10 v0.1.2 requires exact commands, Windows runner/toolchain, headless tests, fixtures, diagnostics, lint and benchmark policy; T4.7 implements CI, automated licensing enforcement and notice generation. This section supplies that architecture, not a workflow implementation. D3 v0.3.0 and D9 v0.3.0 are companion drafts. T3.10 must map all applicable fixture IDs to ordered T4 tasks and approve concrete performance budgets and unresolved codec/tool selections before T4 starts.

### 11.1 Ownership and verification boundaries

Engineering owns test fixtures, automation and evidence collection; domain authorities define expected material outcomes; owner approves contracts, exceptions and milestones. Inputs: exact source SHA, accepted contracts, pinned toolchain/lockfile, explicit fixture data, ordered requests, supported feature profile and recorded environment. Outputs: independently reproducible pass/fail results, canonical differences, build and dependency records, performance measurements and compliance reports. Invariants: no claim of pass without execution evidence; a skipped, conditional or unavailable check is not a pass; no fixture expectation changed merely to conceal a regression; no material advancement based on test wall time. Failure: stop the affected gate, preserve diagnostic evidence, correct and rerun targeted plus relevant regression tests, or record an explicit owner-approved bounded exception. Tests must not mutate a real player's universe or depend on network, GPU, arbitrary sleeps or uncontrolled clock/entropy for material assertions.

| Layer | Responsibility | Evidence |
|---|---|---|
| Unit | Pure arithmetic, sampling, ID, coordinate, schema and validation rules | Exact expected values; negative and boundary cases |
| Headless | Shared production simulation path, explicit steps, command/operation order, authority and fail-stop | Canonical material assertions and ordered outcomes |
| Integration | Content activation, application readiness, handoff, snapshot validation and continuation | Complete fixtures; before/after state; error categories |
| Client-enabled core | Same material suite with client feature enabled, no window required | Both profiles independently satisfy same expectations |
| Native graphical smoke | Real window startup, simple projection, interaction and shutdown | Owner observation and build/log identity; not replaced by compile-only check |
| Performance | Reproducible measurement, not correctness by timing | Raw samples, workload, profile, machine and approved thresholds |
| Compliance | Dependency inventory, feature scope, policy and generated notices | Exact tool versions, graph, policy, exceptions and output |

Use Rust's built-in test harness initially. No new direct dependencies or test framework authorized. Keep synthetic test-only seams narrow; invoke the same authority/validation path, not a duplicate test implementation. Use independent fixture construction for expected values. Test-only injection of invalid state or controlled RNG is not a production bypass. Exact Bevy APIs and package/test paths belong to T3.10/T4 and must be verified against 0.20.1 before use. No mandatory custom runner or generalized replay engine.

### 11.2 Traceability and canonical comparison

Retain all existing D3/D4/D9 fixture IDs. T3.10 assigns each to one or more named tests, feature profile, gate and expected result; multiple criteria may share a test but none may disappear. Suggested test groups (names, not actual repository paths): application_boundaries, command_rejection, clock_ordering, random_vectors, identity_lifecycle, content_validation, spatial_precision, snapshot_continuation. Run common material tests in both feature profiles. Domain cases include valid, invalid, boundary/exhaustion and unexpected integrity failure where feasible; never rely solely on a happy-path launch. D9-P22 requires an approved codec; D9-P23 is a future disk adapter test; neither can pass via typed round trip. D9 external loading requirement remains unresolved and cannot be waived by this section.

Canonical comparison includes committed tick/quantum, master seed/versions, stream keys and exact states, identity and sequence high-water marks, live instances and material fields, required references, canonical spatial pairs, exact content binding, pending operations and ordered material outcomes (compare outcomes separately when not snapshot fields). Sort unordered collections by explicit stable keys; preserve meaningful operation order. Exclude ECS handles, derived indexes, camera/transforms, cosmetic randomness, driver pacing debt, elapsed test time and diagnostic timestamps. Compare integers, enums, strings and normalized finite coordinate component bits exactly where required; use only the explicit spatial conversion tolerance in conversion fixtures. Do not use approximate comparison as a general replay substitute. Direct structured equality suffices for T4; no digest or serializer dependency required for it. Report first differing field/key/tick and enough context to reproduce; no raw memory dumps as canonical data. Golden vectors from D4 §8 are independently specified constants, changed only after intentional versioned contract revision and approval.

Replay fixture: construct independent initial sessions from same validated definitions/seed; submit same finite ordered input batch at each admitted tick; compare after each committed step, final state and ordered outcomes. Repeat from a committed snapshot at the designated split using D9 §8.6. Compare batched versus single-step advancement, cosmetic work versus none, and headless versus client-enabled core. Tests drive explicit steps and inject elapsed durations for catch-up; no sleep-based assertions. Supported equivalence is pinned Windows x64/compiler/contract configuration for the declared fixtures; cross-machine, cross-platform and arbitrary physics bitwise equivalence are not guaranteed. Running both profiles on one runner is distinct from proving a real graphical session equivalence; native smoke is separate.

### 11.3 Environment and exact proposed commands

Accepted baseline: Rust/Cargo 1.100.0, edition 2024, target x86_64-pc-windows-msvc, Bevy =0.20.1 with defaults off, base std/default_app, opt-in client adds bevy/3d and bevy/ui. Toolchain components rustfmt, clippy, rust-src. T4.1 commits rust-toolchain.toml and Cargo.lock. Use Developer PowerShell locally; CI verifies actual MSVC/SDK availability and does not hardcode VS paths. Proposed hosted x64 runner `windows-2025-vs2026`; runner label is not immutable image. Record image identifier and actual compiler/SDK versions. Owner workstation reference is T3.1 checkpoint, not minimum specification.

```powershell
# Proposed commands, after T4.1 package exists; not executed in T3.9.
git branch --show-current
git rev-parse HEAD
git status --short
rustup show active-toolchain
rustc -vV
cargo -V
$t = 'x86_64-pc-windows-msvc'
cargo fmt --all -- --check
cargo build --locked --target $t --no-default-features
cargo test --locked --target $t --no-default-features
cargo clippy --locked --target $t --no-default-features --all-targets -- -D warnings
cargo build --locked --target $t --no-default-features --features client
cargo test --locked --target $t --no-default-features --features client
cargo clippy --locked --target $t --no-default-features --features client --all-targets -- -D warnings
cargo tree --locked --target $t --no-default-features --edges normal,build
cargo tree --locked --target $t --no-default-features --features client --edges normal,build
cargo metadata --locked --format-version 1 --filter-platform $t --no-default-features
cargo metadata --locked --format-version 1 --filter-platform $t --no-default-features --features client
git diff --exit-code -- Cargo.lock
git status --short
```

Check `$LASTEXITCODE` immediately after each native command; automation must throw/exit nonzero on failure rather than rely on PowerShell's default native error handling. Record logs and actual exit codes. Run targeted `cargo test --locked --target $t --no-default-features <test-filter>` (replace placeholder with actual test name), confirm expected test count is nonzero; follow with full suite. Use equivalent `--features client` when relevant. Do not use `--all-features` as substitute for headless. Once cache populated, repeat both build/test profiles with `--offline --locked`; offline does not mean fresh checkout needs no initial fetch. Do not assume metadata's resolved graph alone proves actual active feature scope: reconcile feature trees, target, build/dev dependency scopes and full locked inventory. No unexpected lockfile changes. Actual binary/run commands depend on T3.10-approved layout, not probe names; run headless executable and native client separately. All commands above are unexecuted production proposals derived from accepted T3.1 command evidence.

### 11.4 CI and graphical smoke

Proposed workflow path `.github/workflows/ci.yml` (not present/created here). On pull requests and pushes to main, one explicit Windows x64 runner family, separate headless and client jobs plus format/compliance checks. Build/test/lint both profiles with locked resolution; keep default test suite windowless even with client feature. Check expected graph exclusions (renderer/native window/audio backend absent headless; audio backend absent both). Compile graph verification and runtime absence of graphics initialization are separate requirements. Verify runtime plugin/service boundary in headless launch evidence. Start with no optional build caching; introduce cache only after observed build cost, with toolchain/target/lockfile/profile keys and no reliance on cache for correctness. Clean checkout online build and cached offline check are distinct. Pin reviewed action SHAs, least-privilege contents: read, no secrets for untrusted PRs, no publishing; avoid executing untrusted code in privileged contexts. Retain failure artifacts when possible without masking failed jobs. Branch protection requiring checks is proposed, requires owner setup and actual CI verification. No hosted GPU/display assumption; native Windows smoke includes launch, visible fixture, input/projection behavior within approved scope, no unexpected material advancement, normal exit, logs and owner observation. Record adapter/driver and known ReShade exception; do not silently generalize exception to a new failure. T3.10 specifies exact launch invocation after binary layout selection.

### 11.5 Performance and evidence policy

Measure foundation release-mode headless step throughput with a finite, explicit synthetic workload; warm up before timed repetitions, record elapsed duration outside authoritative state, report sample count, median and maximum (p95 only with adequate sample count), steps/second and fixture size. Measure snapshot capture/restore separately from I/O and compile time. Record debug/release profile, compiler, feature flags, machine CPU/RAM/OS, runner image, workload, seed and commit; retain raw values. Avoid arbitrary CI timing assertions on shared runners; correctness is a hard gate. T3.10 must approve a scoped workload, run counts and quantitative budgets on reference Windows hardware before implementation; T5/T6 set representative universe/flight budgets. No synthetic measurement is evidence of full galaxy scalability. No benchmark dependency or threshold invented in this section. Record cold versus warm builds separately if measured. Report actual samples, not estimates. Proposed test evidence record: criterion ID, command, profile, expected/actual, count, exit code, SHA, clean/dirty status, environment, log location, elapsed duration, issues, disposition and owner acceptance. Failed/blocked/not-run/conditional/deferred are explicit statuses. Redact local personal paths/secrets before public artifacts; preserve relevant third-party notices under their own terms.

### 11.6 Licensing automation and exceptions

Inherit the owner-approved staged policy from T3.1 licensing assessment v1.0.0. T4 must implement automated policy enforcement and notice generation, not simply list dependencies. Candidate external development tools: cargo-deny (policy check) and cargo-about (notice generation). Upstream references checked 2026-11-26: https://github.com/EmbarkStudios/cargo-deny (documents 0.19.9), https://github.com/EmbarkStudios/cargo-about (main manifest 0.9.3, MIT OR Apache-2.0, Rust minimum 1.88), https://github.com/actions/runner-images (Windows runner). These are candidates, not approved pins: exact release artifacts, license, MSRV, Windows installation, profile filtering, flags and output validation require verification/approval at T4.7 before tooling implementation. No additional application dependency, tool installation or CI action authorized by this draft. Avoid assuming main-branch metadata proves a released binary's properties.

Pipeline: enumerate full lock resolution and per-profile active normal/build/dev graphs; reconcile exact package name/version/source/license and changed feature sets against accepted inventory; evaluate SPDX expressions (OR choose supported permitted path, AND honor every mandatory term, WITH preserve exception); reject unknown/disallowed mandatory terms or unreviewed additions unless an approved narrowly scoped exception applies; generate candidate notices for both supported profiles and reconcile with actual redistributed code/assets/native runtimes at release. Proposed initial permissive policy candidates MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0; not a final allowlist or automatic approval; evaluate actual graph and source-level exceptions before selecting. OFL fonts require asset-specific review and notices; no blanket asset permission. No blanket copyleft/custom license acceptance. Tool-generated notices and metadata do not certify legal compliance. Keep third-party terms, attribution and provenance; project source MIT OR Apache-2.0, docs/lore CC BY 4.0, branding excluded. Reconcile all inherited LIC records including FiraMono subset and missing standalone notices; inactive issues reviewed before activation; unresolved affected rights block redistribution. Exception record: ID, exact package/version/source/feature/platform, evidence, rationale, scope (selection/build/distribution), owner, approval date, expiry or review trigger, remediation, disposition. No new blanket waiver; fail on expired or out-of-scope exception. Verify negative policy fixture (unapproved license rejects), known permitted expression, AND/OR and scoped exception behavior, generated notice existence and reconciliation; no need to add actual problematic dependency to production for tests. Tool candidates pinned for evaluation; final CLI/config/templates and output verification at T4.7. Release clearance remains separate from development CI acceptance.

### 11.7 Proposed acceptance scenarios

| ID | Scenario | Expected result |
|---|---|---|
| D4-V01 | Clean Windows checkout with accepted toolchain and lockfile | Both profiles build; no lock change |
| D4-V02 | Full default/headless suite | All applicable tests pass, nonzero count; no GPU/window/audio init |
| D4-V03 | Full client-enabled suite | All applicable material tests pass without GUI requirement |
| D4-V04 | Equivalent ordered replay in both profiles | Exact canonical equality and ordered outcomes |
| D4-V05 | Restore continuation fixture | D9-P04 expectations; exact material equality |
| D4-V06 | Known mismatch injected into comparison | Test fails with field/key/tick context |
| D4-V07 | Golden RNG expectations | Exact D4 §8 values; no automatic rewrite |
| D4-V08 | Formatting/lint | Format check and both Clippy profiles exit 0 with warnings denied |
| D4-V09 | Deliberate failing test/command in isolated verification fixture | Nonzero gate status; logs retained |
| D4-V10 | Feature graph review | Headless excludes prohibited backends; both exclude audio backend |
| D4-V11 | Cached offline locked build/test both profiles | Success without resolution changes; cache precondition documented |
| D4-V12 | Native graphical smoke | Visible approved fixture, correct observation boundary, clean exit |
| D4-V13 | CI push/PR verification | Exact SHA, runner, commands, results and required check statuses |
| D4-V14 | Dependency/license inventory | Production graph reconciled; additions/unknowns reviewed |
| D4-V15 | Negative and positive license policy fixtures | Unapproved terms rejected; scoped approved paths accepted |
| D4-V16 | Notice generation and exception review | Notices generated/reconciled; exceptions bounded; no distribution clearance claim |
| D4-V17 | Foundation measurement run | Raw samples, environment and approved T3.10 budget disposition recorded |
| D4-V18 | Gate evidence completeness | Every applicable criterion mapped; conditional/future/unexecuted explicitly distinguished; owner disposition recorded |

### 11.8 Review and stop point

New proposals: built-in harness, two feature-profile CI jobs, explicit Windows runner, canonical differential evidence, test/benchmark recording and license policy pipeline. D10 v0.1.2 already mandates these responsibilities; a proposed D10 status/acceptance revision belongs to T3.10 consolidated review, not a silent edit to the accepted roadmap. D3/D9 remain unchanged. Open T3.10 prerequisites: exact dependency/tool pins and licenses (including external loading/codec), concrete Bevy APIs and binary layout, fixture-to-task traceability, quantitative workloads/budgets, final compliance allowlist and exception disposition, CI action pins and native commands. T3.2–T3.9 remain unaccepted drafts; no implementation, installation, runtime testing, CI runs or repository changes. Accepted D10, ADR-002, technical contract and licensing assessment were retrieved at the T3.1 SHA; current remote HEAD/local checkout not reverified. T4 remains blocked pending consolidated owner acceptance.


## T3.10 closure review addendum (historical; final disposition below)

The preceding increment checkpoints are historical. Current remote main was verified at a09251b8482894b9e2ae01b9c5afc1281b0e6a4b; local checkout not inspected. This revision retains all scenario IDs. The T3.10 plan v0.2.0 and closure decision records govern proposed clarifications below; none amend accepted ADR-002 until owner approval. T4 remains blocked.

Use a synchronous ordered core entry for the foundation, shared by drivers. Pause/resume clears old pacing debt; scheduled rejection resolves and removes the operation exactly once. Retirement must reconcile or explicitly cancel required live-reference obligations in the same transition. Synthetic handoff is T4.6; actual material handoff T5.6. The T3.10 plan supplies budgets and binary layout. Bevy API documentation confirms App::new, add_plugins, add_systems, insert_resource, update and run in 0.20.1; production integration and feature graph still require Windows evidence. Compliance pins are candidates only; see verification checklist.


## Final T3 closure disposition (normative on consolidated acceptance)

Final review baseline; owner approved streamlined verification on 2026-11-26; consolidated contract acceptance and banking pending. The earlier increment checkpoints and historical version references describe their original issue, not outstanding prerequisites. Active companion versions upon acceptance: D3/D4/D9 v1.0.0, D10 v0.1.3, T4 plan v1.0.0, ADR-002 v1.1.0 and ADR-003–006 v1.0.0. Earlier proposed conditions are superseded only as stated here. All acceptance scenario IDs retained.

Use the synchronous ordered Rust core entry shared by drivers; actual Bevy integration, custom scheduling if justified, headless and native graphical verification at T4.2–4.3. No V2 probe result claimed. V1 confirms combined dependency feasibility, not integration. T4.5 adds only verified parser pins subject to ADR-002 amendment acceptance. V3 tooling installation, exact CLI and scoped licensing validation move to T4.7, not a T3 prerequisite. D4-V01–V18 remain required as mapped. Performance budgets in T4 plan §6 require owner acceptance.
