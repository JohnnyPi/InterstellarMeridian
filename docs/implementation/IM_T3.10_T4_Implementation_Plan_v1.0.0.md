# T3.10 — Consolidated review and T4 implementation plan

Project: Interstellar Meridian | 2026-11-26 | v1.0.0 — final review baseline; consolidated acceptance pending.
License: CC BY 4.0, project documentation; branding and third-party material excluded.

## 1. Baseline and authority
Remote main verified at a09251b8482894b9e2ae01b9c5afc1281b0e6a4b; recursive tree contains documentation/licensing only. Owner V1.1 output confirmed local and remote main at the baseline; draft documents untracked and historical D10 v0.1.1 locally deleted. No later clean status asserted. Governing D0 1.1.1, D1 2.1.1, D10 0.1.2, ADR-002 1.0.0 and accepted T3.1 contract/checkpoint/licensing assessment. Final review D3 1.0.0, D4 1.0.0, D9 1.0.0 read for this plan. T3.1 alone is accepted/banked. No production edits, tests or installations performed. T4 remains blocked pending consolidated acceptance. All paths below are proposed new paths, not existing source.

## 2. Review and decision register

| Decision | Recommendation | Approval/evidence required |
|---|---|---|
| A | Adopt D4 §§1–6 lifecycle and single logical writer, fail-stop; only committed observations | Owner approval |
| B | Adopt D4 §7 100 ms quantum, initially paused, 4-step/400 ms clamp, explicit phase order | Owner approval; not final physics rate |
| C | Adopt D4 §8 SplitMix64 v1, fixed derivation, 128-attempt rejection bound; D3 generation isolation | Owner approval; independent golden tests |
| D | Adopt D4 §9 nonzero world-local u64, monotonic H, no reuse, exclusive handoff | Owner approval |
| E | Adopt D9 strict immutable catalog and namespaced IDs; select JSON for narrow external content fixture | Owner approval and exact parser pins/license/feature verification |
| F | Adopt D3 topology and D4 §10 anchored f64 pairs with f32 near-view projection | Owner approval; fixture limits only |
| G | Adopt D9 §8 typed snapshot and exact catalog matching; external snapshot encoding/durable disk adapter outside T4 | Owner approval; D9-P22/P23 remain deferred, not passed |
| H | Adopt D4 §11 separate Windows feature jobs, canonical comparisons, staged compliance and budgets below | Owner approval; tooling pins and verification pending |

Audit clarifications: D4 §0.2 stale reference to T3.8 as next is editorial, update on consolidation. D4-E12 clears debt at pause/resume; 50 ms remaining is new time, not old debt. D4-E07 requires scheduled rejection to resolve and remove once; D9-P13 rejects invalid required references at restore, while D4-I17 concerns retirement after scheduling; domain must explicitly reconcile or cancel. Choose cancellation within retirement transition for required live refs; test that subsequent advancement cannot execute canceled work. D4-C05 real handoff deferred to T5.6, synthetic handoff belongs T4.6. D9-C20 requires external decoder, not typed-only test. D9-P22/P23 are not evidence of disk durability. D4 §7.5 explicit schedule or synchronous ordered core entry must not depend on frame scheduling; choose plain Rust synchronous core for fixture and invoke from Bevy drivers, verify actual APIs before code. D3 generated tokens are not IDs. No new universal architecture, physics, content editor or actual gameplay. No discrepancy between exact canonical bit comparisons and tolerance-based re-anchoring tests: they test different properties.

## 3. Proposed minimal package layout

Single package/library with two executable targets; no workspace or extra crates. Keep Cargo package name `interstellar_meridian` and library same name; binaries `interstellar_meridian` (headless default) and `interstellar_meridian_client` (required-features client). No API stability promised. File allocation only at relevant gate.

| Work | Proposed paths | Test first / gate |
|---|---|---|
| T4.1 | Cargo.toml, Cargo.lock, rust-toolchain.toml, src/lib.rs, src/main.rs | Bootstrap metadata, native launch and locked build; preserve existing licensing |
| T4.2 | src/application.rs, src/simulation.rs, src/bin/interstellar_meridian_client.rs, tests/boundaries.rs | Ready/invalid boot, coherent change/rejection, no graphics headless, client visible smoke |
| T4.3 | src/clock.rs, extend simulation.rs, tests/clock.rs | Exact D4-E expectations, bounded pacing, explicit ordered batch, fatal quarantine |
| T4.4 | src/randomness.rs, src/identity.rs, tests/randomness.rs, tests/identity.rs | Golden vectors, allocation/exhaustion, duplicate/reference rejection; generation tokens |
| T4.5 | src/content.rs, tests/content.rs, tests/fixtures/content/valid.json and malformed fixtures | Strict typed validation then real file decode; verify no partial activation |
| T4.6 | src/spatial.rs, src/snapshot.rs, tests/spatial.rs, tests/snapshot.rs | Topology, arithmetic, synthetic handoff and D9-P04 continuation |
| T4.7 | .github/workflows/ci.yml, scripts/verify.ps1, scripts/compliance.ps1, deny.toml, about.toml, about.hbs, docs/implementation/IM_T4_Acceptance_Checkpoint.md | CI, inventory, notices, negative checks, benchmarks and native smoke |

Names and file boundaries may be simplified after inspection at each gate. Do not create placeholder modules. Each step: inspect local status and governing versions; add targeted failing test when feasible; implement smallest change; run targeted and full relevant suite; inspect diff/license changes; record results and obtain owner disposition. Bootstrap may require minimal harness before a test can compile. No destructive Git operations.

## 4. Exact accepted baseline and candidate changes

T4.1 pins Rust 1.100.0-x86_64-pc-windows-msvc, minimal profile, rustfmt/clippy/rust-src, target x86_64-pc-windows-msvc; edition 2024, rust-version 1.100.0, publish=false, license MIT OR Apache-2.0. Bevy =0.20.1, default-features=false, features std/default_app; client=[bevy/3d,bevy/ui]. Commit lockfile. No audio, no other direct dependencies at bootstrap. Verified parser candidates serde =1.0.228 (derive) and serde_json =1.0.154; V1 native evidence supports proposed ADR-002 amendment; introduce only at T4.5 after consolidated approval. serde_json 1.0.154 documentation inspected as a candidate (https://docs.rs/crate/serde_json/1.0.154); V1 subsequently supplied native compatibility evidence. T4.5 may add approved pins only. Do not write a general parser to evade review. Proposed wire envelope {schema_version:1,definitions:[...]}; UTF-8 JSON object, prerequisite omitted or string (reject null), reject duplicate/unknown fields at every record/envelope, noninteger values, overflow, malformed/trailing non-whitespace input. File ceiling 64 KiB, at most 256 definitions, identifier <=128 ASCII bytes, display_name <=256 UTF-8 bytes; bounds checked before activation. No dynamic discovery or mod packages. Verify strict duplicate detection in typed decoder (do not parse into generic map first); explicit null rejection test. Snapshot remains typed, exact normalized f64 bits and u64 values preserved, no encoder claim. Final JSON snapshot wire schema deferred to T5.7.

Bevy: accepted probe exercised MinimalPlugins and explicit App updates, not production topology. Candidate core uses sequential ordinary Rust function for each step, invoked by both drivers; presentation reads committed projection only. At T4.1/2 verify exact App/plugin/resource/input/run APIs against https://docs.rs/bevy/0.20.1/bevy/ and pinned compile probes, before selecting actual insertion code; no unverified API is a requirement. Keep client feature gated. Headless executable exits after finite fixture; client starts paused, explicit step/run/pause input, visible original primitive, clean exit; actual UI bindings verified at T4.2/3.

## 5. Verification commands (proposed, not executed)

Use Developer PowerShell. After every native command check $LASTEXITCODE immediately; stop on nonzero. Record exact command/output and SHA.

```powershell
Set-Location E:\RustProjects\InterstellarMeridian
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
cargo run --locked --target $t --no-default-features --bin interstellar_meridian
cargo build --locked --target $t --no-default-features --features client
cargo test --locked --target $t --no-default-features --features client
cargo clippy --locked --target $t --no-default-features --features client --all-targets -- -D warnings
cargo run --locked --target $t --no-default-features --features client --bin interstellar_meridian_client
cargo tree --locked --target $t --no-default-features --edges normal,build
cargo tree --locked --target $t --no-default-features --features client --edges normal,build
git diff --exit-code -- Cargo.lock
```

Initial lock generation at bootstrap is explicit and reviewed before locked builds. Repeat build/test both profiles offline after cache populated. Targeted tests use actual filter names and nonzero executed counts. Client test suite must not require display; native smoke separate. Proposed CI runner windows-2025-vs2026; verify label and image when configuring; action SHAs reviewed/pinned, contents read only, no privileged untrusted PR execution. Compliance candidate cargo-deny 0.19.9 (release list https://github.com/EmbarkStudios/cargo-deny/releases), cargo-about 0.9.3 (main manifest https://github.com/EmbarkStudios/cargo-about/blob/main/Cargo.toml; release metadata confirmed by owner cargo info; not installed). Exact installation commands, policy flags, profile filters, Windows compatibility and license evidence are mandatory T4.7 tasks, not T3 closure blockers. Use external tools, not application deps. Inventory locked graph and active scopes; preserve T3.1 LIC issues. No blanket license allowlist or redistribution certification. Review real graph before final configuration. Generate notices under proposed target/compliance/{headless,client}/; retain evidence separately and publish only reviewed notices. Verify policy positive/negative fixtures, scoped exceptions and failing exit handling. No action or tool installation authorized here.

## 6. Proposed quantitative budgets

Reference T3.1 owner Windows workstation, release build, headless, idle machine, no graphics or disk in timed loop. Seed 42; 100 fixture instances, 2 systems, 1 directed link, 2 material RNG streams, 100 scheduled increments spread deterministically over first 100 ticks, initial value 10, each target receives one +2, no further queued work; per tick iterate all 100 in stable order and read/check values (no extra material updates beyond fixture rules). Warm-up one independent 1000-tick run; five independent timed runs of 10,000 ticks, each from same initial state, no console inside timed section. Proposed median total <= 10 s (>=1000 steps/s), worst sample <=20 s. Snapshot benchmark same fixture at tick 50 with pending work; one warm-up, 20 separate capture and restore samples, median <=10 ms each, max <=50 ms each; record exact raw samples, no disk/serialization. Capture/restore verify canonical equality outside timing. These are modest foundation regression budgets, not gameplay throughput promises; require owner approval. No timing assertions on shared CI. If missed, record and resolve or explicitly approve exception before T4 closure; no optimizing without profile evidence. T5/T6 establish representative workloads and actual gameplay budgets.

## 7. Traceability matrix

Each row below is generated from the actual draft scenario IDs; placement is a proposed primary gate, earlier unit coverage may be added. Deferred rows are not waived. T4.7 runs all applicable regressions. No tests executed.

| Criterion | Primary gate | Scenario | Expected result |
|---|---|---|---|
| D3-G01 | T4.4 | Generate context 0 twice | Exact matching D4 golden token |
| D3-G02 | T4.4 | Generate contexts 0,1 versus 1,0 | Identical sorted context/token records |
| D3-G03 | T4.4 | Extra draws in unrelated material or cosmetic streams | No change in generated tokens |
| D3-G04 | T4.4 | Unsupported recipe or RNG version | Reject initialization; no partial published world |
| D3-G05 | T4.4 | Evolve a fixture fact after generation; observe it again | Observation reads committed changed fact, not initial generated default |
| D3-S01 | T4.6 | A→B valid | A can discover the directed adjacency to B; reverse not inferred |
| D3-S02 | T4.6 | Missing destination | Reject candidate topology before publication |
| D3-S03 | T4.6 | Duplicate pair or self-link | Reject candidate fixture |
| D3-S04 | T4.6 | Reorder valid records | Same canonical topology |
| D3-S05 | T4.6 | Two identical numeric positions in different systems | Distinct locations; direct subtraction rejected |
| D4-C01 | T4.2 | Inspect each T4 material field and mutation path | Exactly one designated logical owner; no unclassified field or competing writer |
| D4-C02 | T4.2 | Rebuild/discard a derived observation or view | Canonical material fixture unchanged |
| D4-C03 | T4.2 | Reject a small multi-field fixture transition | All affected material facts and pending effects unchanged |
| D4-C04 | T4.6 | Restore an invalid foundation fixture when T4.6 supports restore | No partial live session published |
| D4-C05 | T4.6 synthetic / T5.6 actual | Review identity/representation handoff | Exclusive writer and stable reference policy specified in T3.5; actual material handoff verified at T5.6 |
| D4-D01 | T4.2 | Submit valid foundation request | One coherent commit; explicit outcome and matching observation |
| D4-D02 | T4.2 | Submit invalid request | Reason returned; no material change or pending material effect |
| D4-D03 | T4.2 | Request becomes invalid before execution (if deferred admission selected) | Revalidate and reject; no stale approval bypass |
| D4-D04 | T4.2 | Observe before and after transition | No partial material result visible; deferred structural writes complete before publication |
| D4-D05 | T4.2 | Run presentation work without advancement admission | No material transition or simulation clock advance |
| D4-D06 | T4.2 | Exercise fatal invariant failure using a controlled fixture | No further advancement or valid snapshot publication; diagnostic failure |
| D4-D07 | T4.3 | Advance eligible autonomous fixture work without player request | Progress occurs through authorized core boundary; T4.3 timing proof, richer T5 causality later |
| D4-E01 | T4.3 | Boot, then no advance | tick=0, balance=10, counter=0 |
| D4-E02 | T4.3 | Paused, service arbitrary wall delta and render | tick=0, balance=10, counter=0 |
| D4-E03 | T4.3 | Step once paused | tick=1, counter=1, duration=100 ms; still paused |
| D4-E04 | T4.3 | Advance 5 vs five single steps | tick=5, counter=5, duration=500 ms; same balance and ordered outcomes |
| D4-E05 | T4.3 | At tick 1 commands +8 then +5 | +8 commits, +5 rejected; balance=18, counter=1, tick=1 |
| D4-E06 | T4.3 | At tick 1 commands +5 then +8 | +5 commits, +8 rejected; balance=15; verifies explicit order matters |
| D4-E07 | T4.3 | At tick 1 command +8, scheduled +5 due 1 | command commits, operation rejected/resolved, balance=18, tick=1; no pending retry |
| D4-E08 | T4.3 | Scheduled +2 due tick 3; advance 2 then 1 | balance=10 through tick 2; balance=12 at tick 3, pending removed once |
| D4-E09 | T4.3 | No player inputs, advance 3 | tick=3, counter=3; autonomous work progresses |
| D4-E10 | T4.3 | Running pacing samples 40, 60, 250 ms | ticks 0,1,3 respectively, remainder 50 ms, duration 300 ms |
| D4-E11 | T4.3 | Running pacing 1000 ms stall from zero debt | tick=4, counter=4, remainder=0; 600 ms dropped wall time reported |
| D4-E12 | T4.3 | Run 50 ms, pause, service 5 s, resume, run 50 ms | tick=0, remainder=50 ms; no paused or pre-pause debt replay |
| D4-E13 | T4.3 | Zero steps; invalid request; overflow target | no material change; zero succeeds, invalid/overflow rejects |
| D4-E14 | T4.3 | Duplicate complete ordering keys or past-due work at startup | reject invalid fixture/batch without advancement |
| D4-E15 | T4.3 | Controlled fatal failure within admitted step | no committed tick observation for failed step; no further advance or valid snapshot |
| D4-E16 | T4.3 | Same ordered inputs and N admitted ticks with differing render cadence/headless | identical fixture material values and ordered material results |
| D4-E17 | T4.3 | Schedule follow-up during k for k+1 | no execution in k; resolves once in k+1 |
| D4-E18 | T4.3 | Stop after committed tick | no new advancement; last committed tick unchanged |
| D4-F01 | T4.4 | Raw state 0, four draws | Exact raw vectors above |
| D4-F02 | T4.4 | Derive fixture streams | Exact derivation vectors above |
| D4-F03 | T4.4 | Same seed, domain, version and context twice | Same stream and generated values |
| D4-F04 | T4.4 | Consume extra draws in fixture domain B | Domain A sequence and state unchanged |
| D4-F05 | T4.4 | Generate contexts 0 and 1 in reverse order | Each context retains its own result |
| D4-F06 | T4.4 | n=0; n=1 | Reject without draw; return 0 without draw respectively |
| D4-F07 | T4.4 | Inject raw sequence 5, 16 for n=10 | Threshold=6; reject 5; return 6; two draws |
| D4-F08 | T4.4 | n=u64::MAX, injected raw 0 then max | Threshold=1; reject 0; return 0; two draws |
| D4-F09 | T4.4 | Expected rejected command after speculative draw | No committed material or RNG mutation |
| D4-F10 | T4.4 | Accepted stochastic attempt with failed objective | Commit specified outcome and consumed stream state |
| D4-F11 | T4.4 | Same ordered inputs and committed ticks under headless/client; extra cosmetics | Equivalent canonical material projection |
| D4-F12 | T4.4 | Unknown RNG version / duplicate stream key | Reject before advancement |
| D4-F13 | T4.4 | Rejection sampler exhausts cap via injected raw zeros, n=10 | Fatal integrity failure; no valid partial snapshot |
| D4-F14 | T4.4 | Capture stream state after two raw draws; reconstruct and continue | Next two raw words equal uninterrupted stream (in-memory foundation check; full save/restore T3.8/T5) |
| D4-F15 | T4.4 | Initial state u64::MAX, draw once | State wraps to 0x9E3779B97F4A7C14; no debug overflow panic |
| D4-I01 | T4.4 | Create A then B | IDs 1,2; H=2; same definition, distinct instances |
| D4-I02 | T4.4 | Repeat same ordered initial fixture | Same IDs and canonical facts |
| D4-I03 | T4.4 | Reject invalid creation before commit | No new object, no allocator advance |
| D4-I04 | T4.4 | Candidate duplicate ID or zero ID | Reject entire initialization; no live session |
| D4-I05 | T4.4 | H=MAX-1; request two new IDs | Reject before mutation; no wrap; one new ID remains possible |
| D4-I06 | T4.4 | Retire B without required references; create C | C=3, H=3; B not live; no reuse |
| D4-I07 | T4.4 | Retire B while A requires live B | Reject unchanged; after authorized reference removal retirement may succeed |
| D4-I08 | T4.4 | Missing or wrong-kind required reference | Reject candidate/request; no partial material change |
| D4-I09 | T4.4 | Remove and recreate A's view | A=1, value=10, H=2 unchanged |
| D4-I10 | T4.6 | Promote A from record to active representation | ID/value unchanged, exactly one authority; old writer unavailable |
| D4-I11 | T4.6 | Demote then re-enter A | Same ID, committed value, references, pending work and stream state |
| D4-I12 | T4.6 | Invalid target handoff preparation | Source remains live and unchanged; H unchanged |
| D4-I13 | T4.6 | Restore candidate with two authoritative copies of A | Reject before publication |
| D4-I14 | T4.4 | Resolve stale runtime handle after representation removal | No access to unrelated instance; durable lookup is authoritative |
| D4-I15 | T4.6 | Restore H=2 with B retired; allocate | New ID=3, even though greatest live ID was 1 |
| D4-I16 | T4.6 | Restore live ID above H | Reject candidate before publication |
| D4-I17 | T4.6 | Scheduled request targets retired ID | Explicit cancellation/rejection outcome; no mutation of replacement object |
| D4-I18 | T4.4 | Different runtime handles, same ordered fixture | Canonical material comparison equal; no handle-based ordering |
| D4-I19 | T4.6 | Unexpected handoff integrity failure after writes begin | Stop advancement; no valid snapshot or committed tick observation for failed tick |
| D4-S01 | T4.6 | Valid zero/negative positions in valid system | Accepted; -0 normalized |
| D4-S02 | T4.6 | NaN, ±infinity in anchor or offset | Rejected before mutation |
| D4-S03 | T4.6 | Anchor/offset beyond admitted envelope | Rejected; no clamp |
| D4-S04 | T4.6 | Unknown or invalid system ID | Reject candidate or transition |
| D4-S05 | T4.6 | P minus Q at same astronomical anchor | 0.001 m ±1e-12 m |
| D4-S06 | T4.6 | Add 0.001 m to local offset at same anchor | Relative displacement 0.001 m ±1e-12 m; ID unchanged |
| D4-S07 | T4.6 | Same numeric vectors in different systems | Direct subtraction rejected |
| D4-S08 | T4.6 | Re-anchor P to a-1000 and back | Relative position error <=1e-6 m per component, same ID |
| D4-S09 | T4.6 | Re-anchor would exceed offset envelope | Reject; canonical pair unchanged |
| D4-S10 | T4.6 | Project near-relative vector (0.125,-0.25,1.5) m | Exact f32 values |
| D4-S11 | T4.6 | Project (99999.997,0,0) m | f32 narrowing error <=0.004 m per component |
| D4-S12 | T4.6 | Project beyond 100,000 m component magnitude | No near-view projection; material state unchanged |
| D4-S13 | T4.6 | Shift camera by 100 m within supported frame | Projection moves -100 m (within 0.004 m); canonical state, tick and RNG unchanged |
| D4-S14 | T4.6 | Synthetic active/remote handoff retaining canonical pair | Exact normalized component bits, ID and pending state preserved; one writer |
| D4-S15 | T4.6 | Restore candidate with invalid coordinate/system | Reject before publication |
| D4-S16 | T4.6 | Repeat same ordered fixture in headless/client-enabled core | Exact canonical pair and same observations; no GPU required |
| D4-V01 | T4.7 | Clean Windows checkout with accepted toolchain and lockfile | Both profiles build; no lock change |
| D4-V02 | T4.7 | Full default/headless suite | All applicable tests pass, nonzero count; no GPU/window/audio init |
| D4-V03 | T4.7 | Full client-enabled suite | All applicable material tests pass without GUI requirement |
| D4-V04 | T4.7 | Equivalent ordered replay in both profiles | Exact canonical equality and ordered outcomes |
| D4-V05 | T4.7 | Restore continuation fixture | D9-P04 expectations; exact material equality |
| D4-V06 | T4.7 | Known mismatch injected into comparison | Test fails with field/key/tick context |
| D4-V07 | T4.7 | Golden RNG expectations | Exact D4 §8 values; no automatic rewrite |
| D4-V08 | T4.7 | Formatting/lint | Format check and both Clippy profiles exit 0 with warnings denied |
| D4-V09 | T4.7 | Deliberate failing test/command in isolated verification fixture | Nonzero gate status; logs retained |
| D4-V10 | T4.7 | Feature graph review | Headless excludes prohibited backends; both exclude audio backend |
| D4-V11 | T4.7 | Cached offline locked build/test both profiles | Success without resolution changes; cache precondition documented |
| D4-V12 | T4.7 | Native graphical smoke | Visible approved fixture, correct observation boundary, clean exit |
| D4-V13 | T4.7 | CI push/PR verification | Exact SHA, runner, commands, results and required check statuses |
| D4-V14 | T4.7 | Dependency/license inventory | Production graph reconciled; additions/unknowns reviewed |
| D4-V15 | T4.7 | Negative and positive license policy fixtures | Unapproved terms rejected; scoped approved paths accepted |
| D4-V16 | T4.7 | Notice generation and exception review | Notices generated/reconciled; exceptions bounded; no distribution clearance claim |
| D4-V17 | T4.7 | Foundation measurement run | Raw samples, environment and approved T3.10 budget disposition recorded |
| D4-V18 | T4.7 | Gate evidence completeness | Every applicable criterion mapped; conditional/future/unexecuted explicitly distinguished; owner disposition recorded |
| D9-C01 | T4.5 | Valid envelope and both records | Catalog contains exactly two definitions; canonical order alpha, beta |
| D9-C02 | T4.5 | Reverse input order, forward reference | Same canonical fields and resolved prerequisite |
| D9-C03 | T4.5 | Schema version 2 | Reject; report unsupported version |
| D9-C04 | T4.5 | Missing id or initial_value | Reject; identify required field |
| D9-C05 | T4.5 | String in numeric field; negative or overflow candidate | Reject without coercion |
| D9-C06 | T4.5 | initial_value 101 | Reject with range 0–100 |
| D9-C07 | T4.5 | Uppercase, whitespace, invalid separator or empty ID segment | Reject; no normalization |
| D9-C08 | T4.5 | Duplicate alpha, including identical record | Reject; report both origins |
| D9-C09 | T4.5 | Unsupported kind or mismatch with ID kind | Reject |
| D9-C10 | T4.5 | Beta references missing alpha | Reject; identify reference and target |
| D9-C11 | T4.5 | Alpha references itself, or alpha/beta cycle | Reject fixture prerequisite cycle |
| D9-C12 | T4.5 | Omit optional prerequisite | Accept; explicit absence |
| D9-C13 | T4.5 | Unknown or duplicate field in candidate input | Reject; if decoder needed, test at chosen input boundary |
| D9-C14 | T4.5 | Valid prefix followed by invalid record | No partial catalog or ready world |
| D9-C15 | T4.5 | Create two alpha instances and mutate one | IDs and values as above; default and second instance unchanged |
| D9-C16 | T4.5 | Create from missing/incompatible definition | Reject; no ID, RNG or material change |
| D9-C17 | T4.5 | Repeat validation in headless and client configurations | Same acceptance and canonical catalog; no renderer needed |
| D9-C18 | T4.5 | Attempt replacement with invalid candidate | Existing valid session unaffected; live reload unsupported |
| D9-C19 | T4.6 | Restore candidate bound to changed catalog | Reject until approved migration; T3.8/T4.6 integration test |
| D9-C20 | T4.5 | Malformed external bytes | Decoder rejection with location; conditional on approved external loading contract; open until T3.10 |
| D9-P01 | T4.6 | Capture healthy tick zero or tick 2 | Complete committed logical snapshot; no clock/ID/RNG changes |
| D9-P02 | T4.6 | Capture during step or quarantined session | Reject; no valid snapshot |
| D9-P03 | T4.6 | Restore valid tick 2 snapshot | Canonical fields exactly equal; paused, zero wall debt |
| D9-P04 | T4.6 | Uninterrupted vs restored continuation | §8.6 exact scalar results, canonical equality and ordered outcome equality |
| D9-P05 | T4.6 | Restore with retired highest ID | H preserved; next creation ID 5 |
| D9-P06 | T4.6 | Pending effect due tick 3 | Executes once, removed; no replay at tick 4 |
| D9-P07 | T4.6 | Past/current due tick, duplicate key or sequence beyond S | Reject candidate |
| D9-P08 | T4.6 | Empty queue after prior scheduling | S preserved; next schedule sequence 2 |
| D9-P09 | T4.6 | Unsupported snapshot version | Explicit rejection before activation |
| D9-P10 | T4.6 | Unsupported RNG/recipe/rules version or quantum | Reject; no silent upgrade |
| D9-P11 | T4.6 | Changed catalog value under same schema | Reject exact content mismatch |
| D9-P12 | T4.6 | Duplicate live ID, zero ID or ID beyond H | Reject before publication |
| D9-P13 | T4.6 | Missing required instance/definition/system reference | Reject with field and target diagnostic |
| D9-P14 | T4.6 | Duplicate/unregistered stream or invalid owner binding | Reject; no reseed |
| D9-P15 | T4.6 | Nonfinite or out-of-envelope coordinate | Reject; normalized zero policy per D4 |
| D9-P16 | T4.6 | Rebuild runtime indexes and handles | Same material IDs/fields; no new allocations/draws |
| D9-P17 | T4.6 | Failed restore over paused valid session | Original canonical state remains unchanged |
| D9-P18 | T4.6 | Restore after arbitrary wall-time gap | No offline advancement; next tick exactly 3 |
| D9-P19 | T4.6 | Reorder valid candidate records | Same canonical ordering and equality |
| D9-P20 | T4.6 | Unsupported pending operation kind/payload | Reject; no silent discard |
| D9-P21 | T4.6 | Exhausted scheduling sequence | Reject scheduling without mutation |
| D9-P22 | T5.7 deferred | External malformed/truncated encoding | Conditional on approved codec; reject before activation; not satisfied by typed tests |
| D9-P23 | T5.7 deferred | Staged write/publish failure | Future disk adapter; preserve known-good save, diagnose failure; not T4 typed proof |

Extracted 151 unique scenario rows; verify placement at each work package. T3.1 accepted evidence remains separate.

## 8. Acceptance and banking checklist

- Owner has approved streamlined verification, not yet explicitly accepted every final contract. Review and accept decisions A–H, budgets, ADRs and dependency amendment together; no further disposable test required.
- Preserve all scenario IDs; production evidence is mandatory at each assigned gate. Verify version-specific APIs when implementing T4.2–4.3; tooling and notice generation at T4.7. No tests waived.
- Install finalized files in docs/architecture, docs/decisions and docs/implementation. Replace temporary addendum and draft checkpoint with full D10 and final checkpoint. Historical tracked files remain unless owner explicitly chooses removal.
- Review diff, explicitly approve final baseline, commit documentation, verify remote/local SHA and clean status; synchronize Project Sources. Authorize T4.1 only after acceptance and banking.

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
