# principles/src/ — carved suite atoms

One atom, one clause, one file. Atoms are cited from utility outlines (first consumer: [`../../utils/aspectus/ASPECTUS.outline.md`](../../utils/aspectus/ASPECTUS.outline.md)) and supersede any influx gather they descend from. Cite the atom, not the gather.

## Frontmatter vocabulary (honest tiers)

- **`form`** — `norm` (a law of behavior) · `form` (a concrete formulation: vocabulary, stack, table) · `claim` (an assertion about the world).
- **`type`** — the influx/outline type carried through: `normative` / `formulation` / `demand` / `claim`.
- **`state`** — `influx` means the atom is carved but the corpus is still in gather-flow: expect revision without ceremony. Nothing here is frozen canon yet.
- **`max`** — the *ceiling* of decidedness the atom claims. `decided` means the clause is operative law for the suite **as decided by someone** — and the body's provenance line says by whom and when. It does not mean Joseph read this exact text.

## Decision provenance (the honest part)

Every atom carries a provenance line naming its ground. Authority is a **different axis** from evidence strength (the verisectorium reconciliation: norms and formulations are not truth-apt the way claims are — they are *decided*, and the weight of a decision is who stood behind it and how). This corpus uses the verisectorium template's decided-by vocabulary (`verisectorium/template/DECISIONS.ud`):

| decided-by | meaning |
|---|---|
| `steward` | steward made the call; agent or council ratified |
| `ratified` | agent made the call; steward ratified |
| `council` | agent call after red-teaming + unified validation from other agents |
| `supported` | agent call with provisional steward support; easier revisit |
| `defacto` | "decided" without really being decided; recorded so the record exists |
| `proposed` | from steward or any agent; not blocking anything yet |
| `transition` | rejected but still existing somewhere; defacto-but-being-fixed |

Current honest placement of this corpus: the caller-stack config clauses restate steward words (`steward`); the 2026-08-14 carves (caller-key, channel-tuning, SIGNA pair, config-atom rewrites) were authored under explicit grants he has not read line-by-line (`supported`); anything carved in working sessions without a specific ask is `defacto` until traced (the original repo-file config atoms were exactly this, worn as `decided` until they collided with a decision already made).

When an atom's clause collides with steward words, the words win and the atom gets rewritten — no decided-by value is a defense against the primary. An atom whose provenance you cannot grade is `defacto`.
