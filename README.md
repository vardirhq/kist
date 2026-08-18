# Kist

Kist is a fast, focused desktop archive utility built entirely in Rust.

The goal is simple: make creating, inspecting, extracting, and maintaining archives feel like a first-class desktop workflow rather than a legacy utility full of tiny toolbar buttons.

## Direction

- Pure Rust desktop application
- `egui` / `eframe` UI
- Archive logic isolated in `kist-core`
- Native Rust archive implementations where practical
- No Electron, React, Tauri, or webview runtime
- Cross-platform target: Linux, Windows, and macOS

## Workspace

- `crates/kist-core` — archive domain model and compression/extraction engine
- `crates/kist-app` — desktop UI
- future `crates/kist-cli` — command-line frontend over the same core

## Initial format target

Create and extract:

- ZIP
- 7z
- TAR
- TAR.GZ
- TAR.ZST

Additional extraction formats can be added through isolated backends without leaking format-specific behavior into the UI.

## Product principles

1. Archive operations first. No cloud platform, accounts, analytics, or plugin marketplace before the core utility is excellent.
2. Show useful information such as compression ratio, progress, throughput, and integrity without turning the interface into a cockpit.
3. Keep archive operations independent from the GUI so the same engine can power a future CLI and integrations.
4. Treat safe extraction, cancellation, conflicts, and failure recovery as core behavior rather than polish.

## Development

```bash
cargo check --workspace
cargo test --workspace
cargo run -p kist-app
```

See [`docs/PRODUCT.md`](docs/PRODUCT.md), [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), and [`docs/FORGEZIP-MIGRATION.md`](docs/FORGEZIP-MIGRATION.md) for the roadmap and design constraints.
