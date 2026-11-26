\# T4.1 — Bootstrap Checkpoint



\*\*Project:\*\* Interstellar Meridian  

\*\*Date:\*\* 2026-11-26  

\*\*Status:\*\* Verified; ready for owner acceptance and banking.  

\*\*License:\*\* CC BY 4.0; official branding and separately identified third-party material excluded.



\## 1. Governing baseline



\- D0 v1.1.1

\- D1 v2.1.1

\- D3 v1.0.0

\- D4 v1.0.0

\- D9 v1.0.0

\- D10 v0.1.3

\- ADR-002 v1.1.0

\- ADR-003 through ADR-006 v1.0.0

\- T3 Closure Checkpoint

\- T4 Implementation Plan v1.0.0



Starting branch: `main`.



Starting commit: `38138cfc8544a43e2b8c40ff420afe3664075b55`.



T3 formally accepted and banked. T4.1 explicitly authorized.



\## 2. Implementation



Established the production Rust package and library `interstellar\_meridian`, version 0.1.0, with a native executable and two automated bootstrap integration tests.



Files introduced:



\- `Cargo.toml`

\- `Cargo.lock`

\- `rust-toolchain.toml`

\- `src/lib.rs`

\- `src/main.rs`

\- `tests/bootstrap.rs`



Removed one trailing blank line from the T3 Closure Checkpoint as authorized housekeeping.



No disposable verification projects were created.



\## 3. Toolchain and dependency configuration



\- Rust 1.100.0, commit 0ef1bc386.

\- Cargo 1.100.0, commit 3acc79c0c.

\- LLVM 23.1.5.

\- Host and target: x86\_64-pc-windows-msvc.

\- Rust edition 2024.

\- Bevy exactly 0.20.1.

\- Bevy default features disabled.

\- Baseline Bevy features: std, default\_app.

\- Optional client feature: bevy/3d, bevy/ui.

\- Project code license: MIT OR Apache-2.0.

\- Package publication disabled.



Verification performed by the owner against the actual Windows production checkout.



\## 4. Verification evidence



| Check | Actual result |

|---|---|

| Formatting | Passed |

| Headless initial compilation and tests | 44.86 s; 2/2 passed |

| Headless Clippy | 24.08 s; exit 0 |

| Headless build | 0.61 s |

| Native launch | Expected bootstrap message; exit 0 |

| Client-enabled initial build | 193 s |

| Client-enabled tests | 2/2 passed |

| Client-enabled Clippy | 73 s; exit 0 |

| Locked offline headless build | 0.70 s; passed |

| Locked offline headless tests | 0.50 s; 2/2 passed |

| Locked offline client build | 0.90 s; passed |

| Locked offline client tests | 0.85 s; 2/2 passed |

| Lockfile stability | Unchanged |

| Staged whitespace validation | Passed |



Two distinct integration tests executed four times across online and offline feature configurations, yielding eight successful test executions and zero failures.



Native executable output:



`Interstellar Meridian | T4.1 bootstrap`



Production working-copy Cargo.lock SHA-256:



`B88E2B3AF03BF45A8E77E88592CF0EC53078FBBEBDC5F0E6C855F8A9141482C1`



Evidence consists of owner-supplied PowerShell console output. No independent execution on the owner's workstation by the assistant is claimed.



\## 5. Dependency observations and limitations



Bevy is the sole direct dependency.



Observed transitive versions include serde 1.0.229, serde\_json 1.0.154, blake3 1.8.8, constant\_time\_eq 0.6.2 and wgpu 30.0.2.



Default-feature Cargo metadata returned 325 third-party package records. This is not a complete lockfile inventory.



Existing licensing obligations and staged compliance policy remain unchanged. Full inventory reconciliation, license policy enforcement and notice generation remain assigned to T4.7.



Cargo reports an unused Bevy dependency warning because actual engine integration is deferred to T4.2. Clippy completed successfully. No lint suppression or dependency removal was introduced.



Tests were authored before production verification, but an initial failing test run was not recorded.



Git reported a prospective LF-to-CRLF conversion for Cargo.lock. Working-copy byte fingerprints may therefore depend on line-ending configuration.



No graphical runtime, authoritative simulation, clocks, content loading, persistence, gameplay, CI, or production performance budget has been implemented or verified.



\## 6. Acceptance and handoff



T4.1 functional evidence is complete at bootstrap scope. Final staged source inspection, owner acceptance and Git banking remain required.



T4.2 — Runtime Boundary is the next work package after explicit owner authorization.



No later work package is authorized by this checkpoint.

