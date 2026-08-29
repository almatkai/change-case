# Change Case for Tezbar

Transform a string between camelCase, snake_case, CONSTANT_CASE, and more from Tezbar on Windows or macOS.

## Commands

- **Change Case** shows all 22 transformations for selected text or clipboard with live preview, pin and recent support.
- **Convert to camelCase / snake_case / …** (22 no-view commands) convert clipboard/selection directly and copy or paste the result.

Included transformations: camelCase, Capital Case, CONSTANT_CASE, dot.case, Header-Case, lower case, lower First, no case, kebab-case, KEBAB-UPPER-CASE, PascalCase, Pascal_Snake_Case, path/case, AlTeRnAtInG/rAndOm cAsE, Sentence case, snake_case, sWAP cASE, Title Case, UPPER CASE, Upper first.

## Compatibility

- Windows: built and exercised in Tezbar on Windows 11, including clipboard and selection round-trips.
- macOS: uses only cross-platform `@raycast/api` and pure JS case libraries and retains the upstream macOS-compatible design.
- Linux: not currently declared.

## Local verification

```powershell
pnpm install
pnpm typecheck
pnpm test
pnpm build
```

## Credits

Adapted for Tezbar from Raycast's MIT-licensed `change-case` extension at upstream commit `6734d85992290066fd31a7f5bee3cca3d3545a1a`. Original author `erics118` and contributors remain credited in `package.json`.
