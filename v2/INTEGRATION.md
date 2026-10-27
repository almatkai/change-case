# Required compiled beta integration (not performed here)

Reference: Tezbar `fd300e276356587726566026f072563eb9c34cb1`. The host has not been edited, rebuilt, staged, or deployed. This is compiled Rust dispatch; copying a JSON marker cannot install executable code, and no dynamic loader is assumed.

1. Vendor this `v2/` directory at **`src-tauri/native-extensions/change-case/`**, preserving its relative layout, `src/`, `adapter.rs`, manifest, icon, notices and licenses. Validation fixtures can remain for reproducible tests. Do not vendor it under an extra `v2` subdirectory.
2. In `src-tauri/src/native_core/extensions/mod.rs`, beside the existing SDK module:

   ```rust
   #[path = "../../../native-extensions/change-case/adapter.rs"]
   mod v2_change_case;
   ```

   The adapter calls sibling `super::v2_sdk` and actual `crate::native_core::common::{CoreContext, CoreResult}`. It embeds the offline engine with `#[path="src/lib.rs"]`; no Cargo dynamic plugin is involved. The host needs `serde_json = 1.0.145` compatible APIs, `getrandom = 0.2.16` (add that exact normal dependency if absent), and Tokio with `macros`/runtime features. Adapter tests compile against the byte-identical beta fixtures, not a redesigned SDK.
3. Update **all three** dispatch seams in `v2_runtime.rs`: include `"change-case"` in `slug`'s accepted match, add `"change-case" => super::v2_change_case::commands()` to `commands`, and add the concrete `handle` arm:

   ```rust
   "change-case" => super::v2_change_case::handle(ctx, command, operation, payload, state).await,
   ```

   Existing normalization accepts both `com.tezbar.native.change-case` and `raycast.change-case`. Canonical ID is `com.tezbar.native.change-case`; V1 source's own ID is `com.tezbar.change-case`, but the requested catalog replacement is **raycast.change-case**. Do not rename or remove the V1 package.
4. Append **the complete `manifest.json` object** to `src-tauri/src/native_core/extensions/v2_catalog.json`. Keep all 22 command names, `runtime:"rust"`, both platform declarations, preferences, original icon URL and both verification states `not-run`. The root schema rejects `author`, so credits are in description/notices/provenance. Keep the existing V1 catalog entry and command inventory. Update the pinned catalog assertion from **11 to 12** bundled V2 entries, and any related static coverage expectations. Do not report QA pass based on compilation.
5. Copy `assets/change-case.png` unchanged to **`src/renderer/public/extensions/icons/change-case.png`**, exposed at **`/extensions/icons/change-case.png`**. Assert byte count 218405 and SHA-256 `9c2ae5a995722826df828565dc991aff954d30b376fc3fce6014372c4528ff6b`.
6. Build/test the actual host and renderer. Add coverage assertions that the new catalog entry has exactly `v2_change_case::commands()` (22 names), every command supports the canonical and legacy catalog IDs on macOS/Windows, every declared command reports compiled Rust dispatch, and canonical install leaves V1 intact. Run `scripts/extensions-v2-list.mjs` against the newly built native CLI (`TEZBAR_NATIVE_CORE_CLI` if needed); static JSON is insufficient evidence. Retain both `not-run` verification entries until actual macOS and Windows UI/selection/clipboard/focus/paste QA artifacts exist.

Actual operations are `extension:run-command`, `extension:invoke-action`, `extension:search-text-changed`, `extension:refresh-session`, `extension:load-more`, `extension:dispose-session`. Preferences come from **`payload.preferences`**, not individual root fields. The own V1 commands declare no launch arguments; the host launcher uses **`argumentValues`**, which must not be confused with selected text or fallback text. Optional own-source `fallbackText` and `launchContext.case` are handled if forwarded, but the pinned typed launcher request supplies neither a quicklink builder nor routing support to create them. Quicklink action therefore reports an explicit beta capability error, not fictitious success.

Routes used: `clipboard:read [] -> string`, `core:platform:selected-text [] -> string`, SDK `copy` (`clipboard:write [text]`), `core:platform:paste-text [text]`, and `window:hide []`. Paste-text already writes clipboard, hides, restores focus, waits and pastes: the adapter never calls a second paste or hide for that path. All effect results require strict success objects. No browser/open route, network request, real OS call, or host profile mutation is performed by tests.

Native session state holds canonical converted output and fresh action bindings. A process-wide monotonic nonce makes IDs unique across sessions and discarded staged searches; refresh/load-more return unchanged without generating IDs. A dropped effect leaves `uncertain` state before its await; only a deliberate retry can repeat it. An acknowledged action records subsequent storage/hide stages so resuming cannot repeat copy/paste. Direct command launch failures that never establish a renderer still require a fresh deliberate user launch; no durable success is asserted.

Beta API gaps and UI adaptations are listed in README. In particular, no true HUD, frontmost-app query, quicklink creation, or supported per-action icon/shortcut API is fabricated. The implementation does not modify global launcher navigation, import unknown legacy cache formats, or add text-editing forms.
