# Architecture

Kist is intentionally split so the archive engine does not depend on the desktop UI.

## Crates

### `kist-core`

Owns archive concepts and operations:

- format detection
- archive inspection and entry listing
- create/add/remove operations
- extraction
- integrity testing
- preview extraction
- safe path handling
- compression options and progress events

It must not depend on egui/eframe.

### `kist-app`

Owns the desktop application:

- windows and panels
- file/archive browser UI
- dialogs and notifications
- keyboard shortcuts
- drag and drop
- background job presentation
- theme and reusable widgets

The app consumes `kist-core` through a narrow API and should not contain format-specific archive logic.

### Future `kist-cli`

A thin command-line frontend over `kist-core`.

## Concurrency

Archive work must never block the UI thread. Operations should run as cancellable jobs and report structured progress back to the application.

The first implementation can use standard threads/channels. Add an async runtime only if a concrete need appears; Kist should not carry infrastructure merely because software engineers enjoy collecting infrastructure.

## Archive backends

Prefer native Rust crates per format and hide them behind internal adapters. Do not expose backend-specific types to callers.

Initial direction:

- ZIP: Rust ZIP implementation
- 7z: Rust 7z implementation
- TAR: `tar`
- gzip: `flate2`
- zstd: `zstd`

RAR and broader compatibility are extraction-only candidates for a later isolated backend.

## Safety

Extraction must reject paths that escape the selected destination. Resource limits, conflict policy, symlink handling, and suspicious archive behavior belong in the core API rather than in UI code.

## UI

Use egui/eframe as a rendering and interaction foundation, but build Kist-specific reusable components and visual tokens. Default egui aesthetics are not the product design.
