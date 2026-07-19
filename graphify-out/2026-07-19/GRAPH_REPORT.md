# Graph Report - Onyx Ledger  (2026-07-18)

## Corpus Check
- 16 files · ~4,880 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 111 nodes · 111 edges · 18 communities (17 shown, 1 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d064aa34`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Rust Core Library
- Obsidian Plugin Core
- TypeScript Configuration
- Package Configuration
- Build Tooling
- Project Concepts
- Plugin Manifest
- Dependencies
- Opencode Plugin
- License Model
- Code of Conduct
- Contributing
- Graphify JS Plugin
- Obsidian Plugin

## God Nodes (most connected - your core abstractions)
1. `compilerOptions` - 9 edges
2. `OnyxView` - 7 edges
3. `Transaction` - 6 edges
4. `Ledger` - 5 edges
5. `Workspace` - 5 edges
6. `scripts` - 4 edges
7. `validate_and_add_transaction()` - 4 edges
8. `OnyxLedgerPlugin` - 3 edges
9. `WorkspaceType` - 3 edges
10. `Entry` - 3 edges

## Surprising Connections (you probably didn't know these)
- `RTK (Rust Token Killer)` --semantically_similar_to--> `RTK (Rust Token Killer)`  [INFERRED] [semantically similar]
  AGENTS.md → CONVENTIONS.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Multi-License Scheme** — readme_multi_license_model, readme_apache_2_0, readme_cern_ohl_p_v2, readme_cc_by_4_0 [EXTRACTED 1.00]

## Communities (18 total, 1 thin omitted)

### Community 0 - "Rust Core Library"
Cohesion: 0.24
Nodes (12): JsValue, Result, Account, create_workspace(), Entry, Ledger, Transaction, validate_and_add_transaction() (+4 more)

### Community 1 - "Obsidian Plugin Core"
Cohesion: 0.16
Nodes (4): OnyxLedgerPlugin, OnyxView, onyx-core/onyx_core_bg.wasm, svelte/transition

### Community 2 - "TypeScript Configuration"
Cohesion: 0.14
Nodes (13): compilerOptions, esModuleInterop, forceConsistentCasingInFileNames, module, outDir, skipLibCheck, strict, target (+5 more)

### Community 3 - "Package Configuration"
Cohesion: 0.17
Nodes (11): author, description, keywords, license, main, name, scripts, build (+3 more)

### Community 4 - "Build Tooling"
Cohesion: 0.18
Nodes (11): esbuild, esbuild-svelte, devDependencies, esbuild, esbuild-svelte, svelte-preprocess, @types/node, typescript (+3 more)

### Community 5 - "Project Concepts"
Cohesion: 0.50
Nodes (4): Graphify, Ponytail Lazy Senior Dev Mode, RTK (Rust Token Killer), RTK (Rust Token Killer)

### Community 6 - "Plugin Manifest"
Cohesion: 0.22
Nodes (8): author, authorUrl, description, id, isDesktopOnly, minAppVersion, name, version

### Community 7 - "Dependencies"
Cohesion: 0.29
Nodes (7): obsidian, dependencies, obsidian, onyx-core, svelte, onyx-core, svelte

### Community 8 - "Opencode Plugin"
Cohesion: 0.50
Nodes (3): plugin, $schema, .opencode/plugins/graphify.js

### Community 9 - "License Model"
Cohesion: 0.50
Nodes (4): Apache 2.0 License, CC BY 4.0 License, CERN-OHL-P v2 License, Multi-License Model

### Community 10 - "Code of Conduct"
Cohesion: 0.67
Nodes (3): Contributor Covenant Code of Conduct v2.0, Code of Conduct Enforcement Contact, Mozilla Code of Conduct Enforcement Ladder

### Community 11 - "Contributing"
Cohesion: 0.67
Nodes (3): GitHub Issues, Contributing Guidelines, Pull Requests

### Community 17 - "Obsidian Plugin"
Cohesion: 0.67
Nodes (3): esbuild, Obsidian Plugin, svelte-preprocess

## Knowledge Gaps
- **50 isolated node(s):** `$schema`, `.opencode/plugins/graphify.js`, `id`, `name`, `version` (+45 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `devDependencies` connect `Build Tooling` to `Package Configuration`?**
  _High betweenness centrality (0.038) - this node is a cross-community bridge._
- **Why does `dependencies` connect `Dependencies` to `Package Configuration`?**
  _High betweenness centrality (0.025) - this node is a cross-community bridge._
- **What connects `$schema`, `.opencode/plugins/graphify.js`, `id` to the rest of the system?**
  _50 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `TypeScript Configuration` be split into smaller, more focused modules?**
  _Cohesion score 0.14285714285714285 - nodes in this community are weakly interconnected._