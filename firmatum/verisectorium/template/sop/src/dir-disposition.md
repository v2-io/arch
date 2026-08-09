---
slug: dir-disposition
form: directive
status: proposed
max: ratified
state: [template]
depends: []
---

# Disposition — general praxes for every verisectorium instance

Owned and maintained upstream by verisectorium (state: `template`); copied at deployment. When the upstream text changes, this copy is temporarily un-ratified until the change is adopted / adapted / declined-with-record through `sop/influx/`. Local deltas belong in [[dir-orient]] and [[dir-domain-disposition]], not edits here.

- **Truth over completion, always.** This is a living collection — nothing here "ships," and a session's finish-line is a truthful segment drafted, corrected, or strengthened, or an item honestly discharged — never emptied queues or moved files. Honest incompleteness is a complete discharge: honest tier + working notes saying what is open + release to PRACTICA.
- **Segment discipline:** one atom per file; slug = filename; form-kind prefixes only (`def-` `post-` `claim-` `form-` `disc-` `obs-` `dir-` `ref-` — never trajectory-kinds); the exemplars' cadence (frontmatter → title → summary → Formal Expression → Epistemic Status → Discussion → Working Notes); statuses conservative — never borrow rigor from source confidence; same-author estate agreement is coherence, not corroboration; state flags are resettable, never ratchets — on edit, reset the checks the edit invalidates.
- **Register discipline:** derived / evidenced / decided / proposed, each in its own voice; a convention honestly stated is a complete answer, never dressed as a derivation; no absolutes ("the deepest", "the one thing") — precision instead; absence claims carry their search.
- **Definitions defined once.** Terms live in `def/` segments; `LEXICON.md` is generated from them and never edited by hand; other segments link and gloss in one line, never restate a definition.
- **Self-contained, breadcrumb-free canon.** Math, notation, and definitions stand alone in their segments. Very few pointers out; **zero** meta-commentary about how things changed — that is what `CHANGELOG.md`, `DECISIONS.ud`, and git are for. `ref/` exists so the rare legitimate external lean has one stable citable home — cite `ref/` entries, not scattered estate paths.
- **Dispatch truly.** Items leave live surfaces only through the delete-test (assume the item disappears — is everything it carried landed or truly disposable?), into `.integrated/` (content landed, verified first-hand) or `.archive/` (consciously set down, reason recorded) — never breadcrumbed. The two surfaces are truth-claims, not storage bins.
- **File hygiene:** `md-press --check` every touched `.md` before reporting it done; never run md-press on `.ud`/`.un`/`.udon` files (no canonicalizer exists).
- **Steward relationship:** the steward holds the valve. Genuine forks go up as real briefs (context + options + recommendation + honest uncertainty); steward brainstorms are captured verbatim first, organized second; fiat is marked as fiat. Delegation in peer voice — `~/src/arch/AGENTIC-DELEGATION.md` binds here as everywhere.
- **Commits:** one attributable thing per commit, batching plan stated first; this corpus's history is studied by future agents.
