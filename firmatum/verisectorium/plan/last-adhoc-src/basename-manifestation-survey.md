---
slug: basename-manifestation-survey
type: survey
depends: []
---

# One callable name, five shapes on disk

*What the estate's operational surfaces actually look like when you sort them by the form the steward's own convention predicts — the material behind [[sidecar-conventions]].*

## The convention being surveyed

Joseph's programme-wide norms file (`~/src/arch/notes/NORMS.md`, a live brainstorm, provisional throughout and explicitly not yet ratified) proposes that an operational set of records in a directory is referred to by a **base name** alone — you say "the project's TODO" and never have to know what shape it currently takes. It names five forms, ordered as a maturation path: a top-level `BASENAME.md` (marked *deprecated*, "almost exclusively" what is found in the wild); `BASENAME.udon` (transitional); `BASENAME.<type>.udon` (compliant); `BASENAME.<type>.udon/` as a directory-as-doc-store; and a partitioned form `BASENAME.<cat>.<type>.udon` with multiple files in one directory. It attaches one warning, which is the load-bearing part: **the base name is the union for that directory** — two differently-typed manifestations of one base name are two parts of *one* thing, not two things, so a genuinely separate concern needs a separate base name.

It also names a sidecar rule: alongside every `BASENAME` there may be a `.BASENAME-side`, which *is its own base name*. How main and sidecar link is recorded as unspecified.

## What is on disk

Surveyed 2026-08-06 across `asf/`, `vivarium/`, `logos/`, `firmatum/relata/`, `firmatum/udon/`, `proprium/comproprium/`, and this project.

| Form | Live instances |
|---|---|
| Top-level `.md` *(deprecated)* | The overwhelming majority. ASF's root alone carries **24** such files — 22 distinct, since three are symlinks to one SOP: `TODO` · `PRACTICA` · `PROPOSALS` · `CHANGELOG` · `LOG` · `FINDINGS` · `LEXICON` · `NOTATION` · `OUTLINE` · … |
| Top-level `.udon` *(transitional)* | `vivarium/LEXICON.udon` (106 terms) |
| `.un` *(a sixth form, not in the convention)* | This project: `PRACTICA.un` · `CHANGELOG.un` · `ONTOLOGY.un`. `.un` is glossed in the norms file as Udon Notation, implying the compliant form once schemas exist |
| Compliant `BASENAME.<type>.udon` | `vivarium/DECISIONS.decision-log.udon` (1,697 lines, 160 decision blocks) — the only instance found |
| Directory-as-store | `asf/terminology/entries/` (176) and `asf/terminology/decisions/` (149 dirs) · `vivarium/changelog/` (12 dated narrative files + a README + an allow-list) · `asf/spikes/` |
| Partitioned | `asf`: `BIBLIOGRAPHY-TODO` · `FORMAT-TODO` · `INTEGRATION-CLEANUP-TODO` · `TERMINOLOGY-TODO` · `TODO-big-picture` · `JOSEPH-TODO`. `udon`: `TODO-META` · `TODO-PUBLISHING` · `TODO-UTILS`. `logos`: `LOG` + `STRATEGY` |

So exactly one surface in the estate has reached the convention's compliant form, one more is transitional, three trees have reached directory-as-store by their own route without using the naming scheme, and the deprecated form carries essentially everything else. The maturation path the norms file sketches is a real trajectory — instances *are* found at several of its stages — but no instance has walked it as a path.

## The two divergences worth naming

**The partition affix runs in both directions, in one estate.** ASF writes the qualifier first (`FORMAT-TODO`); udon writes it last (`TODO-META`). Under the union rule this is not cosmetic: `FORMAT-TODO` parses as a partition of `FORMAT`, and `TODO-META` as a partition of `TODO`. Both are called "the TODO for X" in conversation. Nothing enforces either direction and no document states one.

**The `.BASENAME-side` sidecar form is not found at all** — but the *function* is, under a different spelling. Dot-prefixed sibling directories carry the processed and set-down material for a base name throughout the estate: `spikes/.integrated/`, `spikes/.archived/`, `spikes/.routing-trail/`, `audits/.integrated/`, `plan/.integrated/`, `plan/.archive/`, `vivarium/.archive/SUPERSEDED.md`. Every one is a sidecar of exactly the kind the convention describes; none uses its naming. The convention's own note that main-to-sidecar linkage is unspecified holds — in every case above the link is positional (the sidecar sits inside or beside the thing it serves) rather than named.

## Method & scope

`ls` per repository root, filtered for the all-caps operational surfaces; `wc -l` and element counts as cited; the norms file read whole. This is a survey of **top-level** surfaces in seven trees on one day; nested per-subdirectory base names (which the convention explicitly also governs) were not enumerated, so the deprecated-form count is a floor.

The register limit matters here and is not a formality: `NORMS.md` is the steward's working brainstorm, carries its own unresolved questions in the body (*"I wonder if this section is irrelevant"*), and holds an open TODO to name the concept at all. It is cited above as **a proposal to measure against**, never as a rule the estate is failing to follow.

## Working Notes

- The cheap next measurement: how often a base name is used *in conversation and in briefs* versus how often a literal filename is. The convention's entire claim is about callability, and callability is observable in the transcript corpus, not on disk.
- Not surveyed: whether any tooling resolves a base name across forms. If none does, the convention is currently a discipline for humans and agents rather than an abstraction, which is what its own closing paragraph says it hopes to stop being.
