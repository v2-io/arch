---
slug: cousin-store-lineage
type: survey
depends: []
---

# The cousin-store lineage

**Summary.** Four stores in this estate — `neurips/refs`, `asf/terminology`, `logos/refs`, and `relata` — form a documented copy-and-refine lineage sharing one spine (per-key file, append-only event directory, generated view, atomic single-write). Across those four generations the spine stayed fixed while the *enforcement* around it diverged sharply, and the most-evolved member is the one that left its consuming repository.

These stores are not outline+segment corpora — none has a pedagogical outline — but they carry the same identity/eventing/projection split, which is why they are worth surveying beside the claim corpora.

## The four generations

| Generation | Store | Atom | Event directory | Generated view |
|---|---|---|---|---|
| 1 (2026-05) | `~/src/neurips/refs/` | bibliographic entry (`entries/<bibkey>.yml`) | `verifications/<bibkey>/<ts>-<verifier>-<criterion>.md` | `.bib` via `bin/refs emit` |
| 2 (2026-05-08) | `~/src/arch/asf/terminology/` | term (`entries/<slug>.md`) | `decisions/<slug>/<ts>-<decider>-<action>.md` | `LEXICON.md` via `bin/term render` |
| 3 (2026-05-09) | `~/src/arch/logos/refs/` | bibliographic entry | same as gen 1 | `.bib` via `bin/refs emit` |
| 4 (2026-05 →) | `~/src/arch/firmatum/relata/` (data at `$RELATA_DATA_DIR`, default `~/.local/share/relata/`) | work + its epistemic state | `verifications/`, `calibrations/`, `pdf-attempts/` | `.bib` via `relata emit`; `markdown/<sha256>/` view cache |

The lineage is self-declared, not inferred: `asf/terminology/README.md` says its design was "modelled on the `~/src/neurips/refs/` pattern that landed in May 2026"; `logos/refs/README.md` carries an "Adaptation provenance (2026-05-09)" note naming neurips as its source and listing exactly what did *not* transfer (the LaTeX build pipeline); relata's README frames itself against the same per-entry/append-only rationale in the same words.

## What is shared, verbatim

Generations 1, 3, and 4 repeat the same four rationale paragraphs — *per-entry files not one shared file* · *append-only events not mutable status fields* · *YAML not TOML* · *anonymization is first-class* — and the same `safe_write` atomicity contract (tempfile with `O_WRONLY|O_CREAT|O_EXCL` → `fsync` → `rename(2)`; a 60-second floor before sweeping stale `.tmp` artifacts). Generation 2 restates the same contract in its own words for a different atom. The shared reasoning in all four is concurrency: per-key files make independent writers filesystem-disjoint, and same-key contention is deliberately left to surface as a git-level question rather than being hidden by a lock.

## What diverged — measured

**Verification density (re-run 2026-08-05).** Two stores with byte-identical machinery and near-identical READMEs:

| Store | Entries | Entries with a verification directory | Share |
|---|---|---|---|
| `neurips/refs` | 188 | 13 | **7%** |
| `logos/refs` | 97 | 70 | **72%** |

The schemas cannot explain the gap; the consuming deployment can. `logos/refs/README.md` states in bold that "the linter is the anonymization gate before submission," and logos papers go to venues where a deanonymizing citation is fatal. The neurips README contains the same anonymization machinery but describes the linter as surfacing findings, with `REFS_LINT_STRICT=1` as an opt-in for CI.

*Honest scope on this pair, worth stating precisely because the numbers are quotable.* The two READMEs are near-copies — the same four rationale paragraphs, the same lint findings vocabulary (`DENY` / `SCHEMA` / `MISSING` / `UNVERIFIED`), the same opt-in `REFS_LINT_STRICT=1`. Neither linter blocks a build on its own. So the "gate versus warn" difference lives in **one sentence of declared intent and in the measured behaviour**, not in two different mechanisms: logos's README says in bold that the linter *is* the anonymization gate before submission and its operators verified 72% of entries; neurips's says the deny-list surfaces findings before submission and its operators verified 7%. The 7%-vs-72% figure is the durable part of this row. Reading it as *two schemas, one strict* would overstate it; reading it as *one schema, two deployments, a tenfold difference in exercise* is what the files support. This is the measured instance behind [[verification-provenance]]'s claim that gating is the consuming deployment's call.

**Machinery only the fourth generation has** (verified live, 2026-08-05): 2,277 entries and 219 verification directories; a `calibrations/` store (15 events) recording every accept/reject decision with its full evidence ledger; a `quarantine/` directory (one dated case, `2026-07-13-false-pdf-registrations`); a `deny-list.yml`; and an `ingest/` write-membrane spool holding 300 files that no external writer may bypass. Its data tree lives outside its code repository entirely.

## Method & scope

- Counts by directory listing against the live trees on 2026-08-05: `ls entries | grep -c '\.yml$'` and `ls verifications | wc -l` for the two refs stores; `ls` under `~/.local/share/relata/` for relata. Re-running supersedes them.
- Lineage claims are read from each store's own README (live paths in the table above), which is how the estate records them; no independent reconstruction from commit history was attempted.
- **Shared-authorship caution.** All four stores were built by Joseph and agents in the same estate, largely by deliberate copying. Their agreement on the spine is therefore evidence that the spine is *reusable and was reused*, and **not** evidence of independent convergence on a correct design. The one genuinely independent-ish signal is the divergence: the same machinery, handed to two deployments with different stakes, was exercised at ten times the rate in one of them.

## Working Notes

- Not surveyed here: whether the spine is also present in stores outside this estate (Zotero/BibDesk-style tools make the formatted record primary, per relata's own framing) — that would be the outside-authorship leg this survey lacks.
- The lineage's fourth generation being the one that externalized its data is suggestive for [[substrate-independence]] but is a single case.
- `asf/terminology` counts as of 2026-08-05: 176 entries, 149 decision directories — a third density profile, not analyzed here.
