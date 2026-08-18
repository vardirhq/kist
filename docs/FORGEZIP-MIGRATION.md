# ForgeZip migration notes

Kist is a clean Rust successor to ForgeZip, not a source-level migration.

## Preserve and improve

ForgeZip already proved the useful archive workflow. Kist should reproduce and improve:

- list archive entries
- add files to an archive
- extract all or selected entries
- delete selected entries
- test archive integrity
- preview entries through temporary extraction
- compression level controls
- archive statistics

## Reconsider rather than port

ForgeZip used a bundled 7-Zip process as the primary backend. Kist should prefer native Rust implementations and isolate format-specific adapters behind `kist-core`.

## Do not port into v1

The old roadmap included cloud storage, analytics, automation, plugins, a local DB, smart profiles, and archive workspaces. These are deliberately excluded from the initial Kist product.

Some ideas may return after the archive manager itself is mature:

- lightweight reusable presets
- watched-folder automation
- broader format compatibility

## UI reset

No ForgeZip renderer code is carried over. Kist is built as a native Rust desktop application with its own component system and interaction model.
