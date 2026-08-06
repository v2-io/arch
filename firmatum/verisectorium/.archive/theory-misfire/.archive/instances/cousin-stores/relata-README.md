<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: firmatum/relata/README.md (epistemic-state-primary bibliography; data external)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/arch/firmatum/relata/README.md
  Do not edit here expecting to update the live original.
-->

# RELATA
*Cross-project bibliography source-of-truth — the epistemic state is the primary object*

Relata is not a reference manager. The saturated market (Paperpile,
Zotero, Mendeley, …) makes the *formatted record* primary and competes
on polish. Relata makes the **epistemic state** primary: graded,
provenanced, defeasible belief about what each work is and what it
supports, plus the relation graph between works, as the first-class
object. Every interaction is supposed to leave the bibliography more
true and more sure — the North Star's felt test (TODO-ingest, top):
*drop anything, anywhere — never rename, never file — and it knows what
it is, tells you exactly how sure and why, and gets measurably smarter
every time anyone touches it.*

The installed command is **`relata`**. It is multi-agent-safe by
construction (atomic writes, append-only audit trails, a write-membrane
in front of the canonical data) and is shared across consuming projects
(ASF, logos, the NeurIPS workspace, future consumers).

> **Where the data is — read this first.** This repo holds **only code**
> (CLI + library + tests + docs). The bibliography itself — per-entry
> YAML files, append-only verification events, calibration events, the
> anonymization deny-list, PDFs, derived markdown — lives in an
> **external data tree** at `$RELATA_DATA_DIR` (default
> `~/.local/share/relata/`; §11.10, externalized 2026-05-20). There is
> no `entries/` directory in this repo, and nothing under
> `test/fixtures/` is real bibliography data. `relata --help` prints
> the resolved paths for the current run, on its first lines.

**How to orient.** Three documents divide the labor:

- `relata --help` — the **verb-by-verb reference**, always current,
  with resolved data paths. Trust it over any remembered verb list.
- This README — the **concepts and workflows**: what a fresh agent or
  human needs *before* the verb list makes sense.
- `TODO-ingest.md` (repo root) — the **design of record**: the North
  Star, the §7 evidence core, the §11 decisions log, the §16 session
  log. When this README and TODO-ingest disagree, TODO-ingest is right.
  Section references below (§7, §11.17, …) point into it.

---

## Concepts

Five ideas carry the whole surface. Each is decided doctrine with a §
pointer; verify there, not here.

### 1. Designators and the resolution ladder (§11.17)

A **designator** is anything that attempts to resolve to one specific
viewable and/or citable work: a bibkey, a sha256, a DOI or arXiv id, a
URL, a local file path, an author+year string ("wecker 2025"), a whole
guessed `.bib` entry. The term is chosen for the Kripke lens it earns:
DOI/bibkey/sha are (near-)**rigid** designators; "name + year" is a
**definite description** that may fail to designate uniquely — exactly
the confidence spectrum the resolver must be honest about. A **unique
designator** is one detailed enough to identify exactly one existing
entry (bibkey, DOI already in the corpus, sha of a registered blob);
*resolution* is the migration fuzzy designator → unique designator,
with a stated confidence.

Every resolving command degrades through five tiers, **never silently**:

1. **Automatic** — exactly one very-high-confidence answer; just do it.
2. **Blocking interactive** — on a TTY, a rich choice prompt with the
   evidence; non-TTY (script/agent/pipe) gets the same choices as
   **JSON on stdout + a non-success exit code**, so an agent decides
   and re-invokes with the choice as a flag. The caller's nature is
   auto-detected (TTY vs not); `--json` / `--color` / `--no-color`
   remain explicit overrides.
3. **Batch choice list** over many designators at once.
4. **A runnable pending report** — `relata pending`: everything
   awaiting a decision, each item's stage, headline reason, and the
   exact command to decide it. Nothing waits invisibly.
5. **The queue** — the §15 spool itself; at minimum the system knows
   what stage each designator reached. The spool is the *bottom* tier
   of the ladder, not the center of the UX (§16.25 — that inversion
   was a live bug, corrected).

Three distinct acts at any wait-state, never conflated (§11.17):
**rerun** (the same command re-presents the pending decision from its
stored memo — no blind pipeline redo) ≠ **`--retry`** ("yes it's
waiting, but recompute the earlier stages anyway — things may have
changed") ≠ **decide** (actually making the choice, via prompt or
`decide --choose`).

### 2. The write-membrane, and what may write automatically (§15, §11.19)

Nothing external writes canonical `entries/` directly — ever. External
writers (agents, scripts, Joseph) drop files into
`$RELATA_DATA_DIR/ingest/`; only `relata ingest` validates and promotes.
An unguarded `entries/` is the exact corruption vector the North Star
forbids (a hallucinated YAML silently becoming canonical truth). The
membrane keeps the happy path frictionless — drop it, it's gone, it's
done — and keeps two non-happy outcomes **distinct because the
distinction is itself truth-honoring**:

- `.rejected` + sidecar — the *submitter* erred (malformed, schema-
  invalid, deny-listed, colliding). Fix and re-drop.
- `.needs-review` + sidecar — the submission is fine; the *system* is
  honestly uncertain, and the sidecar carries the ranked candidates
  with their evidence ledgers. Conflating this with `.rejected` would
  mislabel system-uncertainty as user-error.

(`.skipped-nonbib` marks drops that aren't bibliographic at all.)

A parked document is **a question the system is asking**, so at a
terminal `ingest`/`prep`/`inspect` ask it immediately — the same
choices `decide` shows, while you still have the context that makes
them cheap to answer (`--no-confirm` opts out; a non-TTY caller is
pointed at `relata decide` and never waits invisibly). And the sidecar
says **why**, in terms of something actually observed: a DOI relata
cannot resolve because it has no rung for that registration agency is
reported as *relata's* gap, by name — never as a defect in your file
(§16.29).

What earns the **automatic tier** (ladder tier 1) is drawn at
**identifier-grade vs judgment-grade evidence**, not
automatic-vs-manual (§11.19):

- A document whose *own* extracted strong identifier (DOI/arXiv from
  masthead-scoped extraction, never reference-list regions) resolves
  online to a record that **passes a consistency check against the
  document's extracted text** → the entry is auto-created from the
  authoritative record and the in-hand blob attached (we are holding
  the very bytes; re-fetching them would be procedural theater).
- A document whose ledger accepts an *existing* entry on
  identifier-grade evidence (doi-exact / arxiv-exact, ≥ τ, no standing
  refutation) → auto-attach.
- Anything resting on judgment-grade evidence (title/author/filename
  channels) **parks for `decide`** until the calibration loop earns
  auto-attach open (§11.17 arc iv). Consistency-check failure is the
  wrong-DOI trap (a masthead DOI on a book review can name the
  reviewed book) — it parks with the mismatch shown, never auto-mints.

Online identification is **registry-plural and identifier-keyed**:
arXiv → Crossref → **DataCite** (Zenodo/figshare/Dryad/Dagstuhl and
most institutional repositories mint DataCite DOIs — asking only
Crossref made a third of the world's DOIs look nonexistent). Never
title-guessed (§12.1(b)). When no registry can place a document, the
confirm tier offers a **provisional scaffold read from the document's
own front matter** — marked unverified, editable before it is written,
and never eligible for the automatic tier. That is what gives the ~63%
of real papers that carry no identifier (§14c) a real `create` choice
rather than only *reject* and *skip*.

**relata acts by default** (§11.21). The verbs do the thing they name —
`ingest` ingests. What protects canonical is the *evidence*, not a flag:
only identifier-verified evidence acts on its own, and everything else
parks for `decide`. `--dry-run` (`-n`) prints the identical plan and
writes nothing at all — not an entry, not a rename, not a sidecar.

### 2b. Where candidates come from (§16.31, §16.33)

The ledger can only choose among the hypotheses it is handed, and for most
documents it used to be handed almost none — measured: **33% of documents raised
zero candidates, 61% raised exactly one.** A ledger with one hypothesis is
rubber-stamping.

`relata candidates <file.pdf | designator…>` populates the hypothesis set and then
narrows it:

```bash
relata candidates ~/Downloads/unknown.pdf     # gather + weigh
relata candidates "smith 2019 causal"         # a search, not an identification
```

Three sources, one decision:

- **Identifiers the document prints** — every DOI/arXiv id in a safe region, each
  with its *provenance*. Measured: **87% of them name a different work** (cited
  papers, datasets), so where an identifier sits is part of the observation. A
  masthead-primary DOI carries woe **+4.5**; one in the reference list carries
  **−3.4**, because a DOI in a bibliography is evidence the paper *cites* that
  work — very nearly the opposite of evidence that it *is* it.
- **A bibliographic search** (Crossref `query.bibliographic`) — for the ID-poor
  majority (§14c) that prints no identifier at all. Scored with its *own* measured
  factors, because a search's confusers are topical neighbours, not cited works.
- **The corpus itself** — and if a candidate turns out to be a work you already
  have, its evidence is **summed into that entry's ledger**, not scored as a rival.
  The same work is one hypothesis however many routes reach it.

The reference-list identifiers that are *not* the document are not waste: they are
the works it **cites** — free, curated relation data, which is what a bibliography
of *relata* is for.

### 2c. Trust, re-checked (§16.32)

`relata validate --deep` checks the **evidence**, not just the claims: every
registered blob must exist, be a document (type by *content*, never extension),
and still hash to what was recorded. `relata audit` is the standing,
**non-destructive** recheck — it reports and hands you the command, and never
writes:

```bash
relata audit          # blobs that vanished or were never documents;
                      # a PDF whose masthead disagrees with its entry;
                      # an entry for the published version holding the preprint
```

This exists because relata validated its claims *at the moment they were made and
never again* — which is how five entries spent two months asserting they had a PDF
when the blob was an HTML login wall. **A system that only validates a claim when
it is made accumulates silent lies.**

### 3. Evidence ledgers and calibration (§7, §7.7, §16.27)

One decision rule, no tier tables: per candidate,
`logodds = log prior-odds + Σ weight-of-evidence`, accept iff the
posterior ≥ τ_high. Priors (base rates, filesystem context) are held
strictly separate from likelihoods (identifier comparisons, title
semantics, fingerprints). A **refutation is a large-but-finite negative
term** in the same sum — a soft veto, overcomable only by abundant
independent corroboration, and a bar to early-stopping. **Absence is
never refutation** (no DOI extracted ≠ DOI conflicts). LLMs, when the
ladder ever reaches them, adjudicate *over* evidence — never source
citation facts.

Two disciplines a future reader would otherwise erode:

- **The independence rule (§16.23, load-bearing):** correlated signals
  are ONE evidence channel. All filename-derived reasons emit at most
  one factor; the title family emits at most one. Two "signals" from
  the same string counted separately would let a lone renamed file
  clear τ_high by itself — this was *measured* as a live false-accept
  and fixed, not hypothesized.
- **Seeded priors carry defended chains (§7.7):** every hand-set
  magnitude records the base rate it estimates, the reasoning, and
  what evidence would revise it. No bare constants.

The loop closes at `relata decide`: **every** decision (attach /
create / reject / skip) writes §7.7 calibration events — one per
(source, candidate) pair, full ledgers attached — into
`$RELATA_DATA_DIR/calibrations/`. That store was empty until
2026-07-10; it is filling now (§16.27). This is the North Star's "gets
measurably smarter every time anyone touches it," made literal: the
factor weights are refit from the labeled outcome trail, and the
τ-gated automatic tier expands only as the data earns it. The rigor is
deliberate beyond the PDF-matching stakes: the calibrated
belief-update discipline is the deliverable, rehearsed here in a safe
sandbox (§7.8).

### 4. work · expression · item — aliases and `same_work_as` (§12, §11.18)

FRBR is the lens (not a catalog mandate): a relata **entry = one
citable Expression**; the file(s) backing it are **items**. The
resolving principle: *two files belong to the same entry iff a careful
reader following a citation to either reaches the same content at the
same locators.* Hence:

- **Equivalent or provisional copies → one entry, an item set**
  (`pdfs:` list; exactly one `canonical: true`; per-item `coverage:`
  flags partial stand-ins; upgrades re-point `canonical` and never
  delete the superseded item — it records what historical
  verifications ran against).
- **Citation-distinct versions (preprint vs journal, editions,
  translations) → distinct entries, explicitly linked** via
  `same_work_as:` — **siblings link, never merge**. Silent merge would
  corrupt citations, the one thing relata exists to protect.
- **Retired keys live forever as `aliases:`** (§11.18). A dedup merge
  or rename records the absorbed key as an alias on the survivor;
  `Entry.resolve` falls back to the alias index; **`emit` writes the
  entry under the *cited* name**, so consumer LaTeX/pandoc keys never
  break, with zero consumer edits. `validate` enforces alias
  uniqueness corpus-wide, and every key-minting site checks the alias
  index (learned the hard way: a re-drain once recreated merged twins
  within hours, §16.25).

### 5. View artifacts (§16.20)

`markdown/<sha256>/` under the data tree is a **derived** cache, keyed
by source-blob sha: the converter's output directory stored
**verbatim** (relative links untouched) plus a provenance sidecar
(tool, version, argv, the force-OCR probe's signals and verdict) and
the full conversion log. Link absolutization and figure caption-lift
happen **at emit time only** — the stored form stays portable, and
cache invalidation stays honest when the converter improves.
Conversions run serially (OCR saturates the CPU) and are expensive
(minutes) but regenerable; `$RELATA_MARKDOWN_DIR` relocates the tree.

---

## The five workflows

### 1. See anything: `relata inspect`

```bash
relata inspect wecker-2025-asf          # bibkey
relata inspect 10.5194/gmd-19-5343-2026 # DOI
relata inspect ~/Downloads/mystery.pdf  # unknown local file
relata inspect "aguilera 2022 particular" # free text
```

One command runs the whole cascade (§16.26): **resolve** the
designator → if it's an unknown file, run the **ingest membrane**
inline (§11.19 automatic tier live) and re-resolve → if the matched
entry has no document, attempt the **open-access fetch** (logged as
always) → **convert** on first touch → **markdown on stdout**. Each
stage narrates on stderr; `--no-fetch` / `--no-ingest` opt out;
`--path` prints the artifact dir instead of content.

Exit codes ARE the machine contract: `0` markdown on stdout · `2`
multiple candidates (choices as JSON when stdout isn't a TTY) · `3`
stalled, with the stage named (needs-identification /
needs-document + attempt outcomes) · `4` no match · `5` conversion
failed (see conversion.log). Agents branch on the code and re-invoke
with a choice; humans get prompts.

### 2. Drop a messy PDF into the library

```bash
relata ingest ~/Downloads/gmd-19-5343-2026.pdf              # identify + file it
relata ingest ~/Downloads/gmd-19-5343-2026.pdf --dry-run   # show the plan; write nothing
```

Identifier-verified documents auto-create/attach (concept 2 above).
**Disposition of the original** (§11.17): the
default **removes** it — but only after the corpus copy is
sha-registered and byte-verified (relata becomes the consolidated
source; origins make it recoverable). `--keep` renames the original in
place to its bibkey; `--leave-alone` touches nothing. Anything
judgment-grade parks; then:

```bash
relata pending                 # the visible queue: stage, reason, how to decide
relata decide                  # walk it — TTY prompt with ranked candidates + ledgers
relata decide <file> --choose attach:<key>   # agent form (JSON + exit 2 guided you here)
```

Every choice writes calibration events. Re-running `ingest` on a
parked file re-presents the stored decision memo; `ingest --retry`
recomputes the pipeline stages first.

### 3. Import a reference list (Undermind report CSV)

```bash
relata ingest survey-report.csv            # import: per-row outcomes committed
relata ingest survey-report.csv --dry-run  # the same plan, written nowhere
relata fetch --report <name> --top 20      # OA PDF sweep, rank order
relata fetch --escalations               # the manual-fetch queue that remains
```

Rows match against the existing corpus first (dup detection, enrich
instead of create at title collisions — the §11.18 regrowth gate);
fetch tries arXiv → Unpaywall → doi.org with strict PDF detection,
logging every attempt append-only under `pdf-attempts/`. What can't be
fetched surfaces as an escalation queue rather than failing silently —
typically paywalled rows for a human/browser pass. (One fully-guided
verb for this walk is decided direction, §16.21; today it is these
three steps.)

### 4. Batch view-readiness: `relata prep`

```bash
relata prep --report <name>     # convert every entry in a report, rank order
relata prep <key> <key> …       # or explicit designators
```

Strictly one conversion at a time; already-converted entries skip
instantly; per-entry failures are reported and don't halt the batch.
Exit `0` = everything viewable; `1` = some entries still lack a
viewable document. Run it before a reading session so `inspect` is
instant.

### 5. The paper-build loop: `cited` / `emit` / `lint`

```bash
relata cited <paper-dir>        # keys cited where (scans <paper-dir>/src/**/*.md;
                                #   LaTeX natbib + pandoc syntaxes)
relata emit <paper-dir> --output <path>.bib   # write exactly the cited entries
relata lint <paper-dir>         # deny-list + missing keys + unverified + schema
REFS_LINT_STRICT=1 relata lint  # exit non-zero on any finding (CI / preflight)
```

`emit` writes each entry **under the name the paper cites** — including
retired keys, via aliases — so merges and renames in the canonical
corpus never break a consumer build. Consumers commit the emitted bib
as a derived snapshot (self-contained for readers without the data
tree); canonical edits go through relata and the next build re-emits.
The ASF monograph build (`asf/bin/build-monograph`) and the logos build
(`logos/bin/build`) call `relata emit` directly.

---

## Layout

Two trees, deliberately orthogonal:

**Code repo** at `~/src/relata/` (this directory):

```
relata/
├── README.md                      this file
├── TODO-ingest.md                 design of record (§ references resolve here)
├── exe/relata                     the CLI source (gem-packaged; PATH command `relata`)
├── lib/relata/                    requireable modules (entry, designator, convert,
│                                  evidence ledger, decider, ingest, safe_write, ...)
├── test/                          minitest suite + hermetic characterization goldens
├── tools/                         dx-runner project-local extensions
├── script/                        ad-hoc maintenance scripts
├── docs/                          agent-nav docs (generated via `dx docs`)
├── relata.gemspec                 packaging
└── _emitted/                      build-time outputs (gitignored)
```

**Canonical data tree** (§11.10), default `~/.local/share/relata/`,
configurable via `RELATA_DATA_DIR` (`relata --help` prints the resolved
paths):

```
$RELATA_DATA_DIR/
├── entries/<bibkey>.yml           one file per entry; filename is the canonical key
├── verifications/<bibkey>/        append-only verification events
│   └── <ts>-<verifier>-<criterion>.md
├── calibrations/                  §7.7 labeled-decision store (append-only;
│                                  written by every `decide` choice)
├── pdf-attempts/<bibkey>/         append-only fetch-attempt log per entry
│   └── <ts>-<source>.md           (outcome: acquired / no-oa-pdf / http-error / ...)
├── ingest/                        §15 write-membrane spool (drop-box; only
│                                  `relata ingest` promotes into entries/)
├── deny-list.yml                  anonymization vocabulary + DOIs that must
│                                  not be cited
├── pdfs/                          registered document bytes; $RELATA_PDFS_DIR
│                                  to put them on a different mount
│                                  (default $RELATA_DATA_DIR/pdfs)
└── markdown/<sha256>/             derived view artifacts (concept 5):
                                   content.md + provenance sidecar +
                                   conversion.log + extracted images.
                                   $RELATA_MARKDOWN_DIR to relocate.
```

The two trees are independent: the code repo carries only the CLI +
library + tests + docs; the data tree is Joseph-backed-up (no
redundancy/resiliency obligation on relata itself). Entries hold an
opaque `pdfs:` item list that never encodes the physical external
location — consumers resolve `pdfs/<key>.pdf` through
`$RELATA_PDFS_DIR`, never a hard path.

## Why the structure looks like this

**Per-entry files, not one shared `.bib` file.** Three (or more) agents
add citations independently. With one shared `refs.bib`, every
concurrent edit is a potential bibtex syntax conflict; the fix is
brittle and the failure mode is silent (a malformed entry compiles but
mis-cites). With per-entry files, two agents adding distinct entries
touch distinct files — git-style conflicts reduce to a clean
single-file question. The shared `.bib` is recovered as a *generated*
artifact via `relata emit`.

**Append-only verification events, not mutable status fields.**
Citation hallucination is a real reviewer-facing risk; we need to know
*who* verified *what* against *which criterion* and *when*. Mutable
status fields lose that history silently. Append-only files preserve
it, and timestamped-per-verifier filenames mean concurrent verifiers
never collide. The same discipline extends to `pdf-attempts/` and
`calibrations/`.

**YAML, not TOML.** YAML is in the Ruby stdlib (no extra gems); agents
already read/write the format everywhere adjacent. The structure is
shallow enough that whitespace fragility isn't a real concern.

**Anonymization is first-class.** `deny-list.yml` enumerates DOIs /
authors / proper nouns that must not appear in submitted papers.
`relata lint` runs it against every entry plus every key cited in any
paper — surfaced before submission, not at PDF audit.

## Storage form — OPEN, decide fresh

The earlier trade study advocating files-over-sqlite was entangled with
an unsound framing and has been **removed**. Storage **form** (files vs
DB) is an open question (§11.10): do not inherit a prior conclusion or
reconstruct the argument from memory; weigh it fresh when it is next
worked. (The **location** question — in-repo vs external — is settled:
external, above. §11.10 also records that the *calibration* store's
operational layer is Sequel + PostgreSQL, with canonical truth staying
in the document tree.)

Neutral current state: canonical data is per-entry `*.yml` +
append-only `*.md` event files; there is no DB. `safe_write` (below)
gives single-write crash-atomicity regardless of that choice.

## Atomicity contract (`safe_write`)

All entry writes, event writes, and emitted-bib writes go through
`safe_write` (`lib/relata/safe_write.rb`):

1. Write to a sibling tempfile `<dest>.tmp.<pid>.<rand>` with
   `O_WRONLY | O_CREAT | O_EXCL`.
2. `fsync` the body to disk.
3. `File.rename(tmp, dest)` — POSIX `rename(2)` is atomic on the same
   filesystem (APFS, ext4, xfs all honor this).
4. On any error, the tmp is unlinked; the destination is untouched.

A reader concurrent with a writer sees either the prior content or the
new content — never a half-written file. A crash between fsync and
rename leaves a `.tmp.*` artifact (harmless; never the destination);
`relata validate` sweeps such artifacts older than 60s (the floor
protects concurrent in-flight writes). The same 60s stability guard
keeps the ingest spool from processing half-written drops.

**Concurrent writes to the same key are intentionally not serialized by
a lock.** Last-writer-wins at the filesystem level. The contents-side
question — "which agent's DOI value is right?" — isn't solved by a lock
anyway; it's a content disagreement that needs a decision either way.
(Same-key concurrent writes are vanishingly rare given
`firstauthor-year-shortword` keying — and external writers go through
the membrane besides.)

**What is *not* defended against:** silent media-level corruption past
the rename barrier. Joseph's data-tree backup is the recovery path —
relata itself has no redundancy/resiliency obligation (§11 #13,
disavowing the old §7.9 framing).

## Schema — `$RELATA_DATA_DIR/entries/<bibkey>.yml`

```yaml
key: anderson-1985-bursting           # MUST match filename basename
type: article                         # article / book / inproceedings / incollection / misc / techreport / phdthesis / mastersthesis / unpublished
title: Adaptive systems, lack of persistency of excitation, and bursting phenomena
authors:                              # list — no "and" joiners; importer/emitter handles BibTeX form
  - Anderson, Brian D. O.
year: 1985                            # 4-digit integer
journal: Automatica                   # type-dependent (journal / booktitle / publisher; `venue:` accepted)
volume: '21'
number: '3'
pages: 247--258                       # BibTeX en-dash form
doi: 10.1016/0005-1098(85)90058-5     # bare DOI, no URL prefix
url: ...                              # optional; for entries with no DOI
isbn: 978-1-4739-7111-0               # optional
howpublished: "IEEE Std 7000-2021"    # optional; @misc-style description (standards etc.)
publisher: ...                        # optional
note: ...                             # optional; SCHOLARLY-ONLY — emitted to bibtex, renders in the bibliography
internal_note: ...                    # optional; AGENT/WORKING metadata — never emitted

# Identity & relations (§12 / §11.18):
aliases:                              # retired keys (merges, renames) that resolve here forever;
  - anderson-1985-persistency         #   emit writes the entry under the CITED name
same_work_as: anderson-1986-bursting-journal   # citation-distinct sibling (preprint/edition) — linked, never merged

# Recall & fingerprint fuel (§12.1):
abstract: ...                         # sourced metadata with provenance (abstract_source / abstract_fetched);
                                      #   matching fuel, never mistaken for having the work

# Citation-status fields (2026-05-14):
citation_status: published            # pre-publication | in-review | preprint | published | withdrawn | accepted-not-yet-published
citation_status_venue: ...            # free-form; names the venue when in-review / accepted
citation_status_updated: 2026-05-13   # ISO date last set

# Anonymization handling (2026-05-14):
applicable_anonymity: true            # citing this in a blind submission needs careful handling
applicable_anonymity_reason: ...      # why

# PDF-acquisition status (§16.14, written by `relata fetch`):
pdf_acquisition_status: escalated     # OPTIONAL; unattempted | tried-no-pdf | paywalled | format-mismatch | escalated | acquired
                                      # absence = fetcher never touched this entry; the per-attempt
                                      # trail is $RELATA_DATA_DIR/pdf-attempts/<key>/

# Item set (§12; schema-enforced: a list, exactly one canonical):
pdfs:
  - path: pdfs/anderson-1985-bursting.pdf   # OPAQUE token; resolved as $RELATA_PDFS_DIR/<key>.pdf
    hash: sha256:abc123...                  # content identity; dedups across entries; verifies local copies
    original_filename: anderson_1985.pdf    # what it was called in the wild (trace-back + evidence)
    pdf_meta_title: Adaptive systems, ...   # PDF document-properties Title (sanity probe vs entry title)
    source: https://doi.org/...             # where the bytes came from (URL or free-form)
    coverage: full                          # LOAD-BEARING: full | chapter-1-only | excerpt | preprint | ...
    added: '2026-05-14'                     # ISO date registered
    added_by: claude                        # provenance of the registration act
    canonical: true                         # exactly one per list: the copy citations resolve to
```

Field notes that carry recorded decisions:

- **`note:` vs `internal_note:`.** `note:` is emitted to BibTeX and
  renders as trailer text in the published bibliography — reserve it
  for genuinely scholarly content ("Russian original 1969", "Originally
  posted as arXiv 2010.08380"). `internal_note:` is agent-side working
  metadata and never reaches the rendered PDF. Agent-meta bleeding into
  a published bibliography is a real failure mode (caught live
  2026-05-06 across three NeurIPS papers).
- **`coverage:`** — a registered file may back only part of the cited
  work (publisher sample chapter, extended abstract, differing
  preprint). Without this field, an agent verifying a claim against
  the PDF would reasonably assume it holds the whole work. `full` is
  the default safe-claim; anything else is free-form and specific
  (`chapter-1-only`, `excerpt`, `sample`, `pre-camera-ready`, …). The
  ingest pipeline must never *silently* write `coverage: full` when a
  sample-cue fires (§7.4) — it sets coverage honestly or escalates.
- **`pdfs:` shape is enforced** — `Entry#schema_errors` (hence
  `validate`/`lint`) flags a regression to the legacy singular `pdf:`
  map, a split-brain (both keys), or a list without exactly one
  `canonical: true`. `pdf_acquisition_status` and `aliases` are
  likewise enum-/uniqueness-enforced. (Enum enforcement for
  `citation_status` and the anonymity-reason pairing is still open —
  see Status.)
- **`applicable_anonymity` vs the deny-list.** The deny-list is the
  *absolute-prohibition* mechanism (never cite, regardless of
  context); `applicable_anonymity: true` is the *conditional-handling*
  mechanism — citing the work could deanonymize the submitter
  (co-authorship in active blind review; author identity indelibly
  tied to a framework name), so cite with care (third-person form,
  alternative citation, possibly omission). `relata lint` consults
  both.
- **Institutional authors** are BibTeX-braced single-author lists
  (`authors: ["{IEEE}"]`) so BibTeX doesn't name-parse "IEEE";
  institutional bibkeys use `<institution>-<stdnum>-<year>-<shortword>`
  (e.g. `ieee-7000-2021-ethical-design`). **IEEE standards** use
  `type: misc` + `howpublished:` (`VALID_TYPES` has no `standard`;
  adding one is reasonable future work).
- **DOIs are left blank when not verifiable from the primary source** —
  blank is safer than fabricated (the hallucinated-citation
  anti-pattern); `internal_note:` flags such gaps for follow-up.
- **Conventions:** bibkeys `firstauthor-year-shortword` (lowercase,
  hyphenated; multi-author extension allowed); pages `--` en-dash;
  DOIs without URL prefix; titles stored in canonical published case —
  `emit` double-braces (`title = {{...}}`) so case-mangling BibTeX
  styles leave acronyms alone; no manual brace-protection needed in
  the YAML.

## Verification — `verifications/<bibkey>/<ts>-<verifier>-<criterion>.md`

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

**Outcomes:** `verified` / `failed` / `uncertain` / `n/a`. The latest
event per criterion wins; older events stay as the audit trail (never
delete).

**Criteria** (defined in `lib/relata/verification_event.rb`):

| Criterion          | What it asserts                                                             |
|--------------------|-----------------------------------------------------------------------------|
| `bib-fields`       | Authors / year / title / venue match the published record.                  |
| `doi-resolves`     | DOI resolves to the cited paper (not redirected, not 404).                  |
| `claim-supported`  | The cited paper's text actually supports the claim it's used for.           |
| `page-ref`         | Specific page / section reference is correct.                               |
| `anonymization`    | Citing this entry does not violate anonymization (deny-list clean).         |
| `no-self-cite`     | Entry is not a self-citation (does not appear on the self-cite deny-list).  |

**Overall verified** = `bib-fields` + `doi-resolves` + `anonymization`
all latest-verified. `claim-supported` and `page-ref` are per-paper and
surface separately (multiple papers cite one entry for different
claims; each pass is its own event, scoped in the note).

```bash
relata verify stuart-2010-acta bib-fields --by joseph \
  --note "DOI 10.1017/S0962492910000061 resolves to Acta Numerica 19:451-559; matches."
relata unverify burda-edwards-storkey-klimov-2018-rnd bib-fields \
  --note "Year is 2019 (ICLR), not 2018; arXiv preprint is 2018. Fix then re-verify."
# (real historical example — the corpus now carries the corrected -2019- key)
```

---

## Status

*Counts are pointers, not numbers: `relata --help` prints the resolved
data paths; `relata validate` walks and counts every entry;
`relata pending` shows the decision queue; `relata fetch --escalations`
shows the manual-fetch queue. Dated snapshot for orientation only — as
of 2026-07-10: ~2,189 entries validate clean, 207 with registered
document items, 31 alias-carrying survivors from the §11.18 dedup, 15
`same_work_as` links, ~1,539 entries carrying sourced abstracts.*

### Repo-visibility assumption — load-bearing

> **This repo is private.** As of 2026-05-14 relata is held strictly
> private — no public visibility, no public clone, no public hosting.
> Decisions that depend on this assumption are marked
> **`[private-repo assumption]`** so a future "generalize relata for
> public sharing" sweep can find every load-bearing instance with
> `grep -n "private-repo assumption" README.md`. (The one previously
> marked decision — copyrighted PDFs in git history — was *resolved* by
> the §11.10 externalization: the bytes are no longer in git at all.
> New decisions that lean on privacy must carry the marker.)

### Where the loop stands

The open critical path is the **§7.7 calibration loop**: the confirm
surface (`decide`) shipped 2026-07-10 and `calibrations/` began
filling the same day (§16.27). Auto-attach on *judgment-grade* evidence
stays gated until that data earns it open (§11.17 arc iv). The bulk
sweep of Joseph's document directories (~2,500 docs censused in
§11.17) is deliberately *last* in the arc — the membrane and confirm
loop absorb it, not the other way around.

### Genuinely open (verified against code + §16.26–§16.27, 2026-07-10)

- **`decide` gaps caught live (§16.27):** no honest choice yet for "this
  is an equivalent COPY of an already-backed entry" (`covered:<key>`,
  the §12 Case-1 item-append); explicit-TTY `ingest` doesn't yet
  auto-enter `decide` for a just-parked file.
- **Ladder tier 3** (batch choice lists) and **resolution memos +
  `--default`** (fuzzy-designator narrowings recorded for the next
  person) are designed (§11.17), not built.
- **The family sweep** (§11.19's ambiguity oracle — Stage D resolving a
  work's whole preprint/edition family, shrinking the human tier to
  true conflicts) is decided direction, unbuilt.
- **`origins[]` schema** (plural tracked origins with redistribution,
  §11.17) — the doctrine is decided; the per-item field isn't live.
- **EPUB registration** — conversion handles epub; `pdfs[]`
  registration is still pdf-shaped.
- **LLM adjudication rungs (Stages E/F)** — designed cost-order
  fallbacks, unbuilt; everything live is deterministic/heuristic +
  online identifiers.
- **Schema validation gaps** — `validate` does not yet check
  `citation_status` against its enum, require
  `applicable_anonymity_reason` when the flag is true, or verify item
  `hash` against file bytes on demand. Values are honored as opaque
  strings until it does.
- **Conditional rendering** — an emitter seeing
  `applicable_anonymity: true` under an anonymized build target should
  render soft-form. Lives in consumer build pipelines; not implemented.
- **Consumer layout generality** — `cited`/`emit`/`lint` scan
  `<paper-dir>/src/**/*.md` (LaTeX natbib + pandoc, §16.17); a consumer
  with a different layout still needs its own integration.
- **Consumer wiring** — ASF (`bin/build-monograph`) and logos
  (`bin/build`) call `relata emit` in their builds. The NeurIPS
  workspace still builds against its own `refs/`
  (`~/src/neurips/PIPELINE-TODO.md §F7`); embeddings still cites its
  self-contained `refs.bib` with pre-relata keys — both migrate when
  those projects are next worked.
- **DOI / Xplore lookups for the seven IEEE 7000-series entries** —
  `internal_note:` still flags these.
- **A standing same-DOI dedup check in `lint`** — on-demand
  `possible-duplicates` and the import-time regrowth gate (§16.23)
  exist; a corpus-wide standing lint for identifier twins does not.
- **Content-vs-claim tooling** — the markdown layer now exists
  (concept 5), but nothing yet *searches* document text against claims
  (`claim-supported` anchoring) or structures `page-ref` beyond a
  free-text note.
- **logos 01/03 cite 5 never-entered philosophy refs**
  (aaronson/brandom/chalmers/graham/taylor) — pre-existing, a natural
  early confirm-loop exercise (§16.22).

### History (dated snapshots; superseded by live commands)

Founding migration, 2026-05-13/14 — 354 entries at close of day:
neurips per-entry YAML (187), synthese-paper per-entry YAML (92 incl.
2 richer-version collision-overwrites), embeddings `refs.bib` (34,
keys rewritten to hyphenated form, originals preserved in
`internal_note`), causal-language + behavioral-floor sweeps (26), 16
hand-curated, 1 separate addition; 31 entries then had registered
PDFs. ASF bibliography ingested in tiers 2026-05-20 (§16.6–§16.10);
Undermind report imports + OA fetch sweeps 2026-05-20 → 2026-07-10
(§16.12, §16.21); corpus-wide dedup with aliases 2026-07-10 (30
merges, §16.22); identifier-verified auto-creation live 2026-07-10
(§16.25). The old NeurIPS `common/refs.bib` verification statuses were
*not* carried over — legacy-imported entries re-verify in relata.
