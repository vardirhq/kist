# Product direction

Kist is a focused desktop archive manager. Its job is to make archive operations fast, clear, and pleasant.

## Core v1 workflow

- Drop files to create an archive.
- Open an archive to inspect its contents.
- Extract all or selected entries.
- Add and remove entries where the format supports it.
- Test archive integrity.
- Preview supported files without permanently extracting them.
- Show progress, throughput, ETA, size reduction, and useful errors.

## Initial create formats

- ZIP
- 7z
- TAR
- TAR.GZ
- TAR.ZST

## UX requirements

- Proper desktop layout, not a dashboard of cards.
- Keyboard-friendly file table with sorting and multi-selection.
- Breadcrumb navigation inside archives.
- Drag and drop for both creation and adding entries.
- Explicit conflict handling during extraction.
- Cancellable background jobs.
- Remember common choices without hiding them behind a profile-management product.
- Light, dark, and system theme support.

## Later, only after the core is excellent

- reusable compression presets
- compression comparison/sample benchmark
- watched-folder automation
- shell/context-menu integration
- CLI frontend
- broader extraction format support

## Explicit non-goals for early releases

- accounts
- cloud sync
- analytics
- collaboration
- plugin marketplace
- AI features
