# T3.1 — Toolchain and Bevy Technical Contract

**Document license:** CC BY 4.0 — © 2026 JohnnyPi and Interstellar Meridian contributors, to the extent they hold rights. Official branding and separately identified third-party material excluded. <https://creativecommons.org/licenses/by/4.0/>
**Date:** 2026-11-26  
**Repository:** https://github.com/JohnnyPi/InterstellarMeridian (`main`)  
**Verified baseline:** `40e04d0747c565513cfddcbb092c1ae7616a4ab0`  

**Version:** 1.0.0 (accepted 2026-11-26)  
**Status:** Feasibility verified; baseline accepted 2026-11-26. Supersedes proposal v0.1.  
**Authority:** D10 §4; D1 §§5–6; ADR-001 v1.0.1; companion ADR-002.

## Scope and ownership
Owner approves technology selection and exceptions. Engineering maintains pins, configuration, evidence and validated updates. Inputs: accepted requirements, Windows toolchain, manifest and locked graph. Outputs: independently buildable headless and graphical configurations and diagnostics. Invariants: no renderer/window/audio backend in headless graph; no unintended lockfile changes; no production changes during T3.1. Failures stop the relevant gate and require documented correction or owner-approved disposition.

## Exact reference environment
| Component | Observed baseline |
|---|---|
| OS | Windows 11 Home 10.0.26300 x64 |
| Rust | 1.100.0 (0ef1bc386, 2026-11-09), LLVM 23.1.5 |
| Cargo | 1.100.0 (3acc79c0c) |
| Target/host | x86_64-pc-windows-msvc |
| Rust edition | 2024 |
| Bevy | 0.20.1; declared MSRV 1.97.1; edition 2024; MIT OR Apache-2.0 |
| Visual Studio Community | 2026 18.12.1 |
| MSVC | 14.52.37025; cl 19.52.37029; link 14.52.37029.0 |
| Windows SDK | 10.0.26100.0 |
| CMake / Git | 4.3.3 / 2.51.2.windows.1 |
| Hardware | i7-12700K, 63.8 GB RAM, RTX 3070 Ti, Intel UHD 770 |
| Graphics | NVIDIA driver 617.42, Vulkan, wgpu 30.0.2 |

Cursor with rust-analyzer is the reference editor. Install Community Desktop development with C++, x64 MSVC and Windows SDK; CMake available. Use Community Developer PowerShell, not an assumed PATH. Community licensing eligibility and any redistributed native runtimes remain subject to their terms. Reference hardware/OS is not a minimum support specification; Windows 10 and other adapters untested.

## Tested configuration
The probe manifest uses package `im_t3_1_probe`, version 0.1.0, publish=false, edition=2024, rust-version=1.100.0, license=MIT OR Apache-2.0. The following dependency portion is the accepted production dependency baseline, not authorization to create a manifest now:

```toml
[features]
default = []
client = ["bevy/3d", "bevy/ui"]
[dependencies]
bevy = { version = "=0.20.1", default-features = false, features = ["std", "default_app"] }
```

Tested probe toolchain:
```toml
[toolchain]
channel = "1.100.0-x86_64-pc-windows-msvc"
profile = "minimal"
components = ["rustfmt", "clippy", "rust-src"]
targets = ["x86_64-pc-windows-msvc"]
```

Separate probe binary `graphical_probe` has `required-features = ["client"]`. This is a fixture, not a production executable layout. Client includes UI, default font, gamepad and scene infrastructure; neither graph includes active audio. Feature selection and runtime plugin selection are distinct. Headless uses `MinimalPlugins` and explicit App updates for feasibility only.

## Reference commands (probe only)
```powershell
Set-Location 'E:\RustProjects\IM_T3_1_Probe'
$t = 'x86_64-pc-windows-msvc'
cargo fmt --all -- --check
cargo tree --locked --target $t --no-default-features --edges normal,build
cargo tree --locked --target $t --no-default-features --features client --edges normal,build
cargo build --locked --offline --target $t --no-default-features --bin im_t3_1_probe
cargo test --locked --offline --target $t --no-default-features
cargo clippy --locked --target $t --no-default-features --all-targets -- -D warnings
cargo build --locked --offline --target $t --no-default-features --features client --bin graphical_probe
cargo test --locked --offline --target $t --no-default-features --features client
cargo clippy --locked --target $t --no-default-features --features client --all-targets -- -D warnings
```
Inspect each exit code; stop on failure. Offline assumes populated cache. Run headless executable and verify 42, 2 updates, exit 0; run graphical executable and observe rotating lit cube/floor and normal exit. Check repository and lockfile before/after. Commands were exercised in the owner-reported probe (format and Clippy in P4/P5; offline build/test P6).

## Dependencies and change control
Preserve exact Bevy pin, commit toolchain/production Cargo.lock when T4 authorized, use --locked in CI. Review feature and version changes; no unreviewed forks or additional direct dependencies. Approved staged licensing assessment governs T3.1-09 and release obligations. Do not automatically vendor or distribute probe dependencies/assets.

## Version-specific reference register
- Rust release: https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/
- Bevy release: https://github.com/bevyengine/bevy/releases/tag/v0.20.1
- Bevy API: https://docs.rs/bevy/0.20.1/bevy/ ; feature manifest: https://github.com/bevyengine/bevy/blob/v0.20.1/Cargo.toml
- MSVC target: https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html
- Cargo tree: https://doc.rust-lang.org/cargo/commands/cargo-tree.html ; metadata: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
- Rustup pins: https://rust-lang.github.io/rustup/overrides.html
References carried forward from proposal v0.1; not independently re-fetched in closure turn. Local exact metadata confirms Bevy version, edition, MSRV and license; compile/run evidence confirms fixture APIs including corrected PointLight field.

## Limits
No clean second-machine build, byte-identical reproducibility, release build, peak memory measurement, minimum hardware validation, physics or simulation determinism established. See checkpoint for tests, actual measurements and exceptions. T3.2 design authorized 2026-11-26; T4 remains unauthorized.
