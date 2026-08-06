<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: neurips/refs/README.md (lineage ancestor of terminology + logos/refs + relata)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/neurips/refs/README.md
  Do not edit here expecting to update the live original.
-->

# refs/ — citation source-of-truth

Multi-agent-safe citation system for the NeurIPS umbrella. The CLI lives at `bin/refs`. This directory holds the data: per-entry YAML files, append-only verification events, the anonymization deny-list, and (eventually) cached PDFs.

## Layout

```
refs/
├── README.md                      this file
├── deny-list.yml                  anonymization vocabulary + DOIs that must not be cited
├── entries/<bibkey>.yml           one file per entry; filename is the canonical key
├── verifications/<bibkey>/        append-only directory of verification events
│   └── <ts>-<verifier>-<criterion>.md
├── pdfs/<bibkey>.pdf              registered full-text PDFs (gitignored)
└── _emitted/                      build-time outputs (gitignored)
```

## Why the structure looks like this

**Per-entry files, not one shared `.bib` file.** Three (or more) agents add citations independently. With one shared `refs.bib`, every concurrent edit is a potential bibtex syntax conflict; the fix is brittle and the failure mode is silent (a malformed entry compiles but mis-cites). With per-entry files, two agents adding distinct entries touch distinct files — git resolves trivially. Two agents editing the *same* entry surfaces as a clean single-file conflict. The shared `.bib` is recovered as a *generated* artifact via `bin/refs emit`.

**Append-only verification events, not mutable status fields.** Citation hallucination is a NeurIPS Code-of-Conduct concern; we need to know *who* verified *what* against *which criterion* and *when*. Mutable status fields lose that history (and lose it silently when overwritten). Append-only files preserve it. Filenames include a UTC timestamp + verifier + criterion, so two agents recording verifications for the same entry never collide.

**YAML, not TOML.** YAML is in the Ruby stdlib (no extra gems); the meta.md frontmatter is already YAML; agents already read/write the format. The structure is shallow enough that whitespace fragility isn't a real concern.

**Anonymization is first-class.** `refs/deny-list.yml` enumerates DOIs / authors / proper-nouns that must not appear in submitted papers. `bin/refs lint` runs the deny-list against every entry plus every key cited in any paper — surfaces before submission, not at PDF audit.

## Design decisions: why per-entry YAML, not sqlite

The original sketch was sqlite. Re-examined 2026-05-06 and re-decided in favor of hardened YAML-on-disk. The summary of the trade study:

| Concern                           | sqlite                                                | per-entry YAML (this design)                                     |
|-----------------------------------|-------------------------------------------------------|------------------------------------------------------------------|
| Crash-atomicity of single write   | WAL (journal-replay)                                  | `safe_write`: temp-file + fsync + `rename(2)`                    |
| Concurrent writes to distinct keys | serialized inside one DB connection                   | filesystem-disjoint; no contention                               |
| Concurrent writes to same key     | last-writer-wins, contents irrecoverable from history | last-writer-wins at filesystem; previous content recoverable via `git log -p` of the YAML file |
| Reviewability of audit trail      | binary; reviewer needs `sqlite3` CLI to inspect       | markdown frontmatter; `cat`, `git blame`, GitHub PR diffs all work |
| Code-of-Conduct provenance        | event rows in a table                                 | one file per event with diffable frontmatter + free-form note    |
| Build-pipeline interface          | export step → `<paper-dir>/refs.bib`                  | export step → `<paper-dir>/refs.bib` (same artifact, same `bin/build` contract) |
| Backup / restore                  | one binary file (must be quiesced)                    | `git` already does this, per-entry granularity                   |
| Dependencies                      | `sqlite3` gem (native build)                          | stdlib only                                                      |
| Indices / query planning          | built-in B-trees                                      | linear scan over 170 files (ms-scale; non-issue at this size)    |
| Multi-step transactions           | `BEGIN/COMMIT`                                        | not available; the data model has no operation that needs them   |

The decisive points: (i) the verification audit trail is *the* Code-of-Conduct surface for citation discipline, and a binary store would make it harder for reviewers — internal or post-hoc — to read; (ii) the data model is document-shaped, not relational (no joins, no foreign keys, no index-driven queries that aren't trivially fine at 170 entries scanned linearly); (iii) per-entry `git diff` / `git blame` / GitHub PR review of bib edits is genuinely load-bearing in the multi-agent workflow (the `internal_note:` migration on 2026-05-06 across all three papers happened by reviewing per-entry YAML diffs); (iv) `safe_write` closes the only legitimate gap that sqlite-with-WAL would have closed (truncated mid-write entries) for free.

What sqlite would have bought that we don't get: ACID multi-step transactions. The data model has no operation that needs them. `add` is a single write. `verify` is a single write to a fresh filename. `emit` is read-only on entries + a single write of the per-paper `.bib`. There is no "update entry and append verification event in the same transaction" operation, and there's no obvious pressure to introduce one.

## Atomicity contract (`safe_write`)

All entry writes, verification-event writes, and emitted-bib writes go through `safe_write` (defined at `bin/refs:114`):

1. Write `content` to a sibling tempfile `<dest>.tmp.<pid>.<rand>` using `O_WRONLY | O_CREAT | O_EXCL`.
2. `fsync` the body to disk.
3. `File.rename(tmp, dest)` — POSIX `rename(2)` is atomic on the same filesystem (APFS, ext4, xfs all honor this).
4. On any error, the tmp is unlinked; the destination is untouched.

A reader concurrent with a writer always sees either the prior content or the new content — never a half-written file. A crash between fsync and rename leaves a `.tmp.<pid>.<rand>` artifact (harmless; never the destination); `bin/refs validate` sweeps such artifacts older than 60s (the floor protects against yanking concurrent in-flight writes).

**Concurrent writes to the same key are intentionally not serialized by a lock.** Last-writer-wins at the filesystem level. The contents-side question — "which agent's DOI value is right?" — isn't solved by a lock anyway; it's a content disagreement that needs a human-in-the-loop decision either way. The collision surfaces as a pending change in `git status`, which is the right place for the resolution. (Same-key concurrent writes are also vanishingly rare in practice given `firstauthor-year-shortword` keying.)

**What is *not* defended against:** silent media-level corruption past the rename barrier (a bit-flip in a YAML file that's been on disk for weeks). For that, `git` is the recovery path — every committed entry is content-addressable via SHA-1 in the object store, and `git fsck` surfaces corruption. The window where a YAML edit is not yet committed is the one window where `safe_write` is the only line of defense; that window is small in practice (agents commit verification events as they go) and `safe_write` covers it.

## Schema — `refs/entries/<bibkey>.yml`

```yaml
key: anderson-1985-bursting           # MUST match filename basename
type: article                         # @article / @book / @inproceedings / @incollection / @misc / @techreport / @phdthesis / @mastersthesis / @unpublished
title: Adaptive systems, lack of persistency of excitation, and bursting phenomena
authors:                              # list — no "and" joiners; importer / emitter handles BibTeX form
  - Anderson, Brian D. O.
year: 1985                            # 4-digit integer
journal: Automatica                   # type-dependent (journal / booktitle / publisher)
volume: '21'                          # quoted-string for safety; YAML number coercion is fine too
number: '3'
pages: 247--258                       # BibTeX en-dash form (LaTeX renders as en-dash)
doi: 10.1016/0005-1098(85)90058-5     # bare DOI, no URL prefix
url: ...                              # optional; for entries with no DOI
note: ...                             # optional; SCHOLARLY-ONLY — emitted to bibtex; appears in rendered bibliography
internal_note: ...                    # optional; AGENT/WORKING metadata — never emitted to bibtex
```

**`note:` vs `internal_note:` — important distinction.** The `note:` field gets emitted to the BibTeX entry's `note = {...}` and `unsrtnat` (and most natbib styles) renders it as trailer text in the published bibliography. So `note:` is reserved for genuinely-scholarly content that belongs in the rendered bib: "Russian original 1969", "Originally posted as arXiv 2010.08380", "Reprinted in...", "Theorem 2 referenced in this work". The `internal_note:` field is for agent-side working metadata: "Cited in 03-llm-hallucinate §1 / §7.1 for VC-dim impossibility", "Year-of-record decision deferred per source OUTLINE", "Verified 2026-05-05 via OpenReview ID OwNoTs2r8e". `bin/refs emit` ignores `internal_note:` entirely — it stays in YAML for the agent's records and never reaches the rendered PDF. AUTHORING §3.9's chronicle-voice anti-pattern applies to bib `note:` fields too — agent-meta bleeding into the published bibliography is a real failure mode (caught 2026-05-06 across all three NeurIPS 2026 papers).

**Conventions inherited from the upstream `common/refs.bib`:**
- Bib keys: `firstauthor-year-shortword` (lowercase, hyphenated). Multi-author entries can extend (`boyd-ghaoui-feron-balakrishnan-1994-lmi`).
- Pages with en-dash via `--` (BibTeX rendering convention).
- DOIs without URL prefix.
- **Title is rendered verbatim** in the bibliography. `bin/refs emit` double-braces the title field (`title = {{...}}`) so BibTeX's title-case styles (`unsrt` / `unsrtnat` lowercase everything past the first letter) treat it as a no-case-change protected group. Store titles in their canonical published case — acronyms (`MDPs`, `RL`, `NeurIPS`, `DOVI`) and proper nouns stay uppercase as written. No need to manually brace-protect inside the YAML title string.

## Verification — `refs/verifications/<bibkey>/<ts>-<verifier>-<criterion>.md`

Each verification act is one file. Frontmatter + free-form note:

```markdown
---
key: anderson-1985-bursting
criterion: bib-fields
verifier: joseph
outcome: verified
timestamp: 20260506T004858Z
---

DOI resolves to Automatica 21(3):247-258. Authors / year / venue match the bib
entry. Title verified against publisher landing page.
```

**Outcomes:** `verified` / `failed` / `uncertain` / `n/a`. The latest event per criterion wins; older events stay as the audit trail (never delete).

**Criteria** (defined in `bin/refs`):

| Criterion          | What it asserts                                                                          |
|--------------------|------------------------------------------------------------------------------------------|
| `bib-fields`       | Authors / year / title / venue match the published record.                               |
| `doi-resolves`     | DOI resolves to the cited paper (not redirected, not 404).                               |
| `claim-supported`  | The cited paper's text actually supports the claim it's used for.                        |
| `page-ref`         | Specific page / section reference is correct.                                            |
| `anonymization`    | Citing this entry does not violate anonymization (deny-list clean).                      |
| `no-self-cite`     | Entry is not a self-citation (does not appear on the self-cite deny-list).               |

**Overall verified** = `bib-fields` + `doi-resolves` + `anonymization` all latest-verified. `claim-supported` and `page-ref` are per-paper and surface separately (a single bib entry may be cited by multiple papers, each with its own claim/page context — those are recorded as additional events with notes scoping the verifier's pass).

## Workflow

### Adding an entry

```bash
# Pipe BibTeX on stdin (paste from publisher landing page, etc.)
echo '@article{stuart-2010-acta, ... }' | bin/refs add stuart-2010-acta

# Or scaffold an empty YAML and fill in interactively
bin/refs add stuart-2010-acta   # creates a stub at refs/entries/stuart-2010-acta.yml

# Bulk import from an existing .bib file
bin/refs import path/to/refs.bib
```

### Verifying

```bash
bin/refs verify stuart-2010-acta bib-fields --by joseph \
  --note "DOI 10.1017/S0962492910000061 resolves to Acta Numerica vol 19 pp 451-559; matches entry."

bin/refs verify stuart-2010-acta doi-resolves --by joseph
bin/refs verify stuart-2010-acta anonymization --by joseph

# Per-paper claim verification (two papers cite the same entry for different claims)
bin/refs verify stuart-2010-acta claim-supported --by paper-agent-bn8 \
  --note "B-N8 §3.2 cites Stuart 2010 Theorem 4.1 for posterior contraction; verified."

# Surface a problem
bin/refs unverify burda-edwards-storkey-klimov-2018-rnd bib-fields \
  --note "Year is 2019 (ICLR), not 2018; arXiv preprint is from 2018. Update entry then re-verify."
```

### Per-paper bib emission (build-pipeline integration)

`bin/build` calls `bin/refs emit` automatically before each compile, with a path override that lands the emitted bib inside the build directory:

```bash
bin/refs emit 01-tragedy-confident-agent --output 01-tragedy-confident-agent/.build/full-paper/full-paper.references.bib
# (this is what `bin/build` invokes internally)
```

The build copies the result to `<paper-dir>/<stem>.extracted.bib` for repo-visibility — the `extracted` naming is explicit-on-purpose so authors don't hand-edit it. Canonical edits go through `bin/refs add` / `refs/entries/<key>.yml`; the next build re-emits.

Standalone use (no path override) writes the legacy `<paper-dir>/refs.bib` location:

```bash
bin/refs emit 01-tragedy-confident-agent
# wrote 01-tragedy-confident-agent/refs.bib  (47 entries; 0 missing)
```

That form is preserved for ad-hoc inspection but the build pipeline doesn't read `<paper-dir>/refs.bib` anymore.

### Linting before submission

```bash
bin/refs lint                       # all papers
bin/refs lint 03-llm-hallucinate-bound  # one paper
REFS_LINT_STRICT=1 bin/refs lint    # exit non-zero on any finding (CI / preflight)
```

Findings categorize as:
- `DENY` — entry violates anonymization deny-list
- `SCHEMA` — entry malformed
- `MISSING` — paper cites a key that has no entry
- `UNVERIFIED` — paper cites a key whose entry is not yet `overall verified`

### Concurrency

Two agents can run any combination of `add` / `verify` / `emit` / `lint` simultaneously without coordination as long as they don't both edit the *same* entry's YAML. Verification events carry timestamps and never overwrite — concurrent verifies serialize naturally.

## Migration from the legacy `common/refs.bib`

```bash
bin/refs import ~/src/neurips2026/common/refs.bib
# imported 164 entries; skipped 0 pre-existing.
```

The legacy bib's verification status (from `~/src/neurips2026/common/citation-verification-report.md`) is currently unverified in the new system — re-verification is needed against the actual published records. The deny-list catches anonymization issues that may have slipped the legacy pass.

## What is not (yet) here

- **Auto-fetch by DOI.** Phase 0 is manual — agents paste BibTeX on stdin or fill the YAML by hand. A future `bin/refs fetch <doi>` would call CrossRef.
- **Bib-style fingerprint dedup.** Two entries with different keys but the same DOI are not currently flagged. `bin/refs lint` could grow this.
- **PDF-to-text claim verification.** `bin/refs pdf <key> <path>` registers a PDF; nothing yet inspects the content. A future `bin/refs grep <key> <claim>` could anchor `claim-supported` verifications more rigorously.
- **Tooling for the page-ref criterion.** Currently a free-text note; could become a structured per-paper claim/page field.
