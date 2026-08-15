# Specimen — one record, worked whole

*Register: **illustrative** — a fabricated-but-faithful example (the hydrology claim borrowed from the retired scaffolding's demonstrations), showing one record's whole life so a founding agent can see the machinery run instead of reading its schema. Everything below is what the model's objects look like inhabited.*

## The record

- **slug:** `claim-fill-residual-distance` — *"water-fill residual grows with distance from lakes"*
- **kind:** claim (*asserts*; fails by being false or overclaimed; write semantics: replacement)
- **foundation declared at minting:** derivable · in-vivo-testable · instance-of-an-established-model · witnessed · carried — which opens `evidence/derivation`, `evidence/in-vivo`, `evidence/instance`, `evidence/testimony`, `evidence/carriage`. (`evidence/wild-empirical` was *not* declared; no line opens for it, and no status will ever pretend one ran.)

## The trail (append-only; each row one event)

| # | date | dimension | channel | criterion *(commitment)* | actor | outcome | era |
|---|---|---|---|---|---|---|---|
| e1 | 08-02 | derivation | formal-expression-written | cadence-conformant FE *(n/a)* | agent-A | drafted | — |
| e2 | 08-03 | derivation | premises-named-and-checked | premises P1 (mass conservation), P2 (monotone drainage) named, each checked *(n/a)* | agent-A | deps-verified | — |
| e3 | 08-05 | in-vivo | first-run | "residual strictly increases in ≥95% of cells beyond 3 cells from any lake" *(pre-committed, e3a 08-04)* | agent-B | pass → run-once | **sim@k1** |
| e4 | 08-06 | instance | verify-correspondence-by-perturbation | mapping onto the diffusion-wave model must survive the near-horizontal case *(pre-committed)* | agent-B | **fails at the lake case → SCOPE-BOUND** — claim narrows to *away-from-lakes*; question minted: `rescope-near-lake-behavior` | — |
| e5 | 08-07 | testimony | account-captured | field observer's report *(n/a)* | steward | position: **reconstructed** (fixed); credibility 1 | — |
| e6 | 08-08 | carriage | convergent-secondaries | external basin study located via two independent secondaries | agent-C | search-corroborated (rung 2/5); source's own family: **wild-measurement** | — |
| e7 | 08-10 | derivation | independent-re-derivation | re-derive from P1–P2 without sight of e1–e2's working *(pre-committed)* | agent-D | pass → **re-derived** | — |
| e8 | 08-13 | *(era registry)* | era-bump | simulation kernel k1 → k2 | steward | e3's certificate **expires, visibly**; staleness question minted: `re-run-under-k2` | sim@k2 |

Note what the trail does *not* contain: no row says "status: good." No row ever will.

## The projections (computed, never stored — shown at two moments)

**As of e7:**

| dimension | computed position |
|---|---|
| `evidence/derivation` | re-derived — exact under P1–P2, *scoped away-from-lakes per e4* |
| `evidence/in-vivo` | run-once @ sim@k1 |
| `evidence/instance` | scope-bound (a result: the break narrowed the claim; it did not fail it) |
| `evidence/testimony` | reconstructed, credibility 1 |
| `evidence/carriage` | rung 2/5, capped at source tier |

- *strongest-leg*: exact-under-premises, away-from-lakes (derivation)
- *every-line-sustains*: credibility-1 testimony — the floor an adversarial reader may hold us to
- *independence-locked*: **ARMED** — derivation + in-vivo agree and their failure modes are independent families. Carriage did **not** arm it and could not have alone: had its source been another *derivation*, the lock would look through the carriage wrapper, see same-family, and refuse — carried material never corroborates a line it could have seeded.

**As of e8 (the era bumped):** `evidence/in-vivo` shows *expired @ k1 — re-run pending*; the lock **disarms** (one strong leg left); nothing was edited anywhere — the same trail simply computes differently, and the minted question routes the re-run. When e9 lands a fresh committed run under k2, the lock re-arms; that event's criterion is the *same pre-committed one from e3a*, which is what makes the re-run certifying rather than tuning.

## What the specimen teaches, in one breath each

- The record occupies exactly the positions its trail supports — and *reports* exactly those (the cardinal sin impossible by construction; its under-recording mirror visible too: had e7 never been appended, the claim would compute at deps-verified while actually re-derived — just as dishonest in the downward direction, and worse than it looks: a later reader who learns agent-D worked on this would have to wonder whether the re-derivation *failed* — which is why a declined lift gets its own event, and silence stays meaning *unrun*).
- A refutation is a result: e4 *improved* the claim (narrower and true beats broader and false), and the scope rides every later projection.
- Decay is visible, not silent: e8 expired e3 in public, minted the repair, and touched nothing else.
- The undeclared dimension never lied: no wild-empirical line, no wild-empirical status, no pretense.
- Identity held through all of it: the slug never changed while the claim's scope, strength, and era all did.
