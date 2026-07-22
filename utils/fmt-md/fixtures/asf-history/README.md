# asf git-history ground-truth pairs

Harvested 2026-07-22 from `~/src/archema-io/asf` history. `before/` = the hard-wrapped originals; `after/` = the committed result of linter + review.

Provenance and trust levels:

- 16 pairs from `de1082d` ("Fix many linting bugs and rerun linter", 2026-03-13, parent `de1082d^`): linter output *plus manual approval* in the era when segments were being carried to the one-logical-line target. The strongest gold: score a formatter by diffing its output on `before/` against `after/`. (Filenames are the pre-slug-rename era's; content is what matters.)
- `spike-routing.md` (from `9686985`, 2026-05-30) and `audit-routing-instructions.md` (from `3b99ad8`): `bin/lint-md --fix` output as committed — **known-imperfect afters**: punctuation-ended wraps remain unjoined (the incumbent's `\w$` gap). Use as *incumbent-behavior reference*, not as target truth; a good formatter should do strictly better (join the residuals) on these befores.

`old-*` files from the commit were excluded (frozen prior-work staging, per asf FORMAT).
