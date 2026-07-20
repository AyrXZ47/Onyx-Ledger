# Ponytail, lazy senior dev mode

Lazy means efficient. Stop at the first rung that holds:
1. YAGNI — does it need building at all?
2. Reuse what's already in this codebase.
3. Stdlib over third-party.
4. Native platform feature over lib.
5. Already-installed dep before a new one.
6. One line if possible.
7. Only then: minimum working code.

Bug fix = root cause. Grep every caller of the function you touch and fix the shared function once.

Rules: no unrequested abstractions, no new deps you can avoid, no boilerplate nobody asked for. Deletion > addition. Boring > clever. Fewest files. Shortest working diff. Mark deliberate simplifications with a `ponytail:` comment naming the ceiling and upgrade path.

Not lazy about: understanding the problem first, input validation, error handling that prevents data loss, security, accessibility, hardware calibration, anything explicitly requested. Non-trivial logic leaves ONE runnable check behind (assert-based demo or single test; no frameworks, no fixtures).

# Repository

**Monorepo** with two packages:

| Package | Path | Tech |
|---|---|---|
| `onyx-core` | `src-rust/` | Rust → WASM (`cdylib`, wasm-bindgen 0.2) |
| `obsidian-plugin` | `obsidian-plugin/` | Svelte 5 + TS 6 + esbuild 0.28 (pnpm) |

`src/` at root is a separate prototype/test area (may use stale API names).

## Build

```bash
# Rust core → WASM + npm package (outputs to src-rust/pkg/)
wasm-pack build src-rust/

# Obsidian plugin
pnpm --dir obsidian-plugin run build   # production
pnpm --dir obsidian-plugin run dev     # watch mode
```

The plugin depends on `onyx-core` via `"onyx-core": "link:../src-rust/pkg"` — rebuild the Rust crate after changing `src-rust/src/lib.rs` or the plugin won't pick up new WASM exports.

## Architecture

- **WASM loading**: esbuild loads `.wasm` as base64; the Svelte app decodes it at runtime (`Uint8Array.from(atob(...))`).
- **Obsidian API** (`obsidian` module) is marked `external` in esbuild — never bundled, provided by plugin host.
- **`obsidian-plugin/main.js`** must land at plugin root (esbuild `outfile: 'main.js'`). **`styles.css`** must land at plugin root too — esbuild emits `main.css` or `dist/main.css`, the build script renames it to `styles.css`.
- **Svelte 5 runes** active (`$state`, `$effect`). Do not use Svelte 4 `let` reactivity or `on:click` — use `onclick` with `{() => ...}`.
- **Spanish/English split**: Rust WASM exports are English (`init_system`, `create_workspace`, `validate_and_add_transaction`). The root `src/App.svelte` uses stale Spanish names (`inicializar_motor`) — do not use as reference for WASM API calls.
- Thread-local state (`RefCell<Option<Workspace>>`) in Rust — single-workspace in-memory, not persisted yet.

## Conventions

- **No tests, no CI, no linters** configured yet. Don't try to run them or add them unasked.
- **Multi-license**: `src/` `scripts/` → Apache 2.0, `hardware/` `kicad/` → CERN-OHL-P v2, `docs/` media → CC BY 4.0.
- **`pnpm-workspace.yaml`** has unresolved `allowBuilds` entries for `esbuild` and `svelte-preprocess` — resolve these before first `pnpm install` if prompted.
- Plugin works on mobile (`isDesktopOnly: false`).
- No Prettier/ESLint/Ruff/mypy configs — code style is whatever the editor provides.
- RTK command prefix (`rtk git status`, etc.) is required per system-level config (see `~/.config/opencode/AGENTS.md` or `CONVENTIONS.md`).
