# Reproducible validation and limits

Run from `v2/`. Implementation source is own commit **b33e675ea3945e08f87dc5ffc8428f5b7dc9d12c**, freshly fetched from origin/main before `feat/native-v2-rust` was created. Beta fixtures are exact bytes from **fd300e276356587726566026f072563eb9c34cb1**. Upstream icon/credits are **6734d85992290066fd31a7f5bee3cca3d3545a1a**, `extensions/change-case`.

Recorded on macOS arm64, Rust stable **1.98.0**, MSRV **1.85.0**, Node **26.7.0**, Unicode **17.0**, default locale **en-US**:

- **52,668 actual own-TypeScript/locked-library conversion vectors pass**, across all 21 transforms, six preference sets, full-input and per-line paths, fixed edge strings and 180 seeded randomized strings. Random tapes test 0, exactly 0.5, above 0.5 and other samples, including consumption counts. Normal Rust tests read the generated vector data offline; expected values are produced by executing original TS and locked JS, not handwritten Rust expectations.
- **8 actual own TS source-selection scenarios** and **168 actual direct-command action/hideHUD/popToRoot combinations** pass under module API stubs.
- Rust library: **2 tests**. Concrete adapter/beta common/SDK fixture suite: **15 tests** (including 2 repeated engine tests and 5 unchanged beta common/SDK tests). Actual unchanged beta native session dispatcher: **2 integration tests**, with separately labeled Tauri, registry, runtime-registration and recording-router stubs. The original unrelated native_tests.rs file is preserved as reference, not claimed as executed. Helper library unit/doctests are disabled to avoid compiling that unrelated full-host test graph.
- Adapter tests cover all 22 commands, every enabled-case toggle, primary action ordering, source/fallback priority, context conversion, exact bound outputs, forged/stale/cross-session/discarded-search IDs, refresh with discarded clone, pin/unpin/clear/remove/history persistence, corrupt storage and unknown-key preservation, strict malformed/contradictory effect results, transport errors, explicit uncertain retry/cancel, and acknowledgement stages across storage/hide failures.
- Actual beta session tests exercise installation/payload dispatch, search/refresh/action/dispose/clear, canonical IDs and cancellation of an in-flight paste future. Stubs never access the host profile or OS clipboard/selection/focus/paste/browser.
- Actual pinned renderer TSX List/Grid/Detail/Form parsers execute under headless React/import stubs. A real adapter-produced list parses all 21 rows/details/action IDs; actual action-panel click wiring, form full-map submission, field callbacks and queued-draft reconciliation pass. Change Case itself introduces no form or grid; these are beta contract checks. No browser screenshot or mounted real DOM interaction is claimed.
- Full pinned draft-2020-12 JSON Schema validation and **37 negative mutations** pass, with separate 22-command/platform/replacement/verification assertions.
- **78 provenance entries** pass file/hash/size/Git-visibility auditing, including exact source fixtures, vendored library dist files, licenses and icon. The original icon is 218405 bytes with SHA-256 `9c2ae5a995722826df828565dc991aff954d30b376fc3fce6014372c4528ff6b`.
- Strict formatting and Clippy all targets pass on stable and 1.85. Windows GNU `check` and strict Clippy all targets pass; no Windows binary or linker execution is claimed. Separate actual-session fixture integration test and Clippy checks pass on stable/MSRV.
- Cargo package verification and extracted **offline** root/session tests pass. Include paths deliberately omit disposable npm environments, Python venvs, build directories and materialized nested Cargo packages. Template manifests, sources and lockfile materialize the separate session test package after extraction. Every provenance path must appear in the crate archive.
- Disposable exact V1 checkout copy: original **4 tests** and TypeScript typecheck pass. Original root lockfile bytes remain unchanged.

Core and adapter offline commands (dependencies must first be fetched into the local Cargo cache; offline is not a promise to vendor crates):

```sh
cargo fetch --locked
cargo fmt --check
rustfmt --check --edition 2021 adapter.rs
cargo test --locked --offline --all-targets
cargo clippy --locked --offline --all-targets -- -D warnings
cargo +1.85.0 fmt --check
cargo +1.85.0 test --locked --offline --all-targets
cargo +1.85.0 clippy --locked --offline --all-targets -- -D warnings
cargo check --locked --offline --target x86_64-pc-windows-gnu --all-targets
cargo clippy --locked --offline --target x86_64-pc-windows-gnu --all-targets -- -D warnings
cargo +1.85.0 clippy --locked --offline --target x86_64-pc-windows-gnu --all-targets -- -D warnings
python3 validation/materialize.py
cargo fetch --locked --manifest-path validation/materialized/Cargo.toml
cargo test --locked --offline --manifest-path validation/materialized/Cargo.toml --test session
cargo clippy --locked --offline --manifest-path validation/materialized/Cargo.toml --test session -- -D warnings
cargo +1.85.0 test --locked --offline --manifest-path validation/materialized/Cargo.toml --test session
cargo +1.85.0 clippy --locked --offline --manifest-path validation/materialized/Cargo.toml --test session -- -D warnings
python3 validation/provenance.py
```

Node/oracle validation uses a disposable npm environment **inside v2**, never root npm install. Create it from shipped exact source/lock fixtures; only validation needs Node/TypeScript:

```sh
python3 - <<'PY'
from pathlib import Path
import shutil
source=Path('validation/oracle');out=Path('validation/v1');out.mkdir(exist_ok=True)
for name in ['package.json','package-lock.json','tsconfig.json','raycast-env.d.ts']:
    shutil.copy2(source/name,out/name)
for name in ['src','test']:
    shutil.copytree(source/name,out/name,dirs_exist_ok=True)
PY
npm ci --ignore-scripts --prefix validation/v1
npm test --prefix validation/v1
npm run typecheck --prefix validation/v1
node validation/oracle/behavior.mjs
node validation/oracle/generate.mjs
node validation/renderer.mjs
python3 -m venv validation/pyenv
validation/pyenv/bin/pip install -r validation/schema-requirements.txt
validation/pyenv/bin/python validation/schema.py
```

`validation/oracle/loader.mjs` transpiles the byte-identical original TypeScript with the locked compiler and redirects only Raycast API and the three libraries to explicit stubs/exact vendored JS. The upstream icon/license downloads happened during implementation; normal tests perform no network requests. To regenerate casing facts, use the recorded Node/locale and `node validation/generate-unicode.mjs`, then `rustfmt --edition 2021 src/unicode.rs`. Hashes are audited by `provenance.py`; do not rewrite provenance just to conceal a mismatch.

Package/extraction procedure:

```sh
cargo package --allow-dirty --locked --offline
mkdir -p validation/extracted
# Extract target/package/tezbar-change-case-0.1.0.crate there, then enter its directory.
cargo test --locked --offline --all-targets
python3 validation/materialize.py
cargo test --locked --offline --manifest-path validation/materialized/Cargo.toml --test session
python3 validation/provenance.py
```

Remaining gaps are intentional and visible: compiled host registration/rebuild, actual install/catalog/coverage CLI checks, and **real macOS and Windows UI/clipboard/selection/focus/paste QA are not-run**. Quicklink creation is explicitly unavailable in this pinned beta; no fake storage or open route is substituted. Native success messages adapt HUD; source pop/close outcomes share beta no-view/hide semantics. No frontmost-app naming or arbitrary Raycast shortcuts/icons is claimed. en-US casing is pinned; Turkish/Azeri/Lithuanian OS-default locale variants are unimplemented. Exact initial recent-row selection and visual Raycast detail fidelity need host work. No runtime JS fallback is hidden behind a Rust manifest.

XERJ used canonical root prefix **ax-2020b9e4f087** and isolated workspace state. Initial index reported all **29/29 source files indexed**, 35 files/184 records, exit 3 `completed-with-junk`; four non-source files were skipped, so it is not an exhaustive all-file index. Source discovery succeeded; exact/exhaustive checks used `rg`, Git blobs and hashes. No global search server was restarted.

Final bounded XERJ refresh aborted immediately with a frozen-schema error for new `v2/Cargo.toml` (exit 1: no frozen dataset accepts family docs). The final index is therefore incomplete/stale for V2. No new prefix, global restart or external server mutation was used; exact V2 audits used current files, `rg`, Git blobs and provenance hashes.
