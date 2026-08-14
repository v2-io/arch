# Quiet columns — surprisal to *this* caller

*Direction seeded 2026-08-14; cold-baseline mechanics drafted same day (this pass). The lattice `quiet` office is the inventory; this file is the law of when quiet speaks. The warm baseline stays a later wave, gated on [[last-look|Last look]]'s after-image store — nothing below depends on it.*

A fact appears only when it surprises. The reason this is right and not a style choice: every glyph in an aspecta should carry surprisal for its reader — a fact that matches expectation carries ~zero bits, and the line budget should never spend width on it. Permissions `644`, clean git, owner-is-you: silence. (Eyes doc: design the nothing-case first; the something-case then carries surprisal by contrast.)

## Whose surprise?

Joseph, 2026-08-14, setting the direction:

> "The quiet and actual protections and nuance against sibling norms etc. need work. I think we need to follow the same basic idea — what will be surprisal *to this agent*; colloquially it's the tool personalizing itself to the caller, where any state is appropriate (the after-image of the glances… the perception-of-movement…)"

So there are (at least) two baselines, layered:

1. **Cold baseline — norms of the place.** No state needed: unusual against siblings (`755` among `644`s), unusual against convention (dirty git letter; a 900-line file among 40-line files). This is what quiet means on a first look. Specified fact-by-fact below.
2. **Warm baseline — the after-image.** With caller-keyed state ([[cache|Cache]]), the baseline becomes *what this caller saw last*: a fact is quiet when it matches the caller's after-image and speaks when it moved. The perception-of-movement: a size that grew, a file that appeared, a mtime newer than the last look. [[last-look|Last look]] is this same mechanism promoted to its own explicit fact, and its store is this baseline's substrate. The layering is **union**: warm adds reasons to speak; it never silences a fact the cold baseline would voice (a root-owned file stays surprising however many times you've seen it — see Open for the counter-leaning).

## The cold baseline, fact by fact

Two kinds of expectation feed cold surprise, and every per-fact law names which it uses: **convention** (world-level priors that need no context — `644` is normal, root-owner in your tree is not) and **sibling norms** (the level's own statistics). Sibling norms are always computed over the **full enumerated set** of the level (readdir is complete even under the walk bound — walk-bound law), never over the budget-surviving listed set: otherwise changing `--lines` would flicker quiet, and determinism dies. Facts unobtainable for a node are absent from its level's statistics, not zeros.

| Fact | Speaks when (cold) | Baseline kind |
|---|---|---|
| **permissions** | Mode differs from *both* the conventional pair for its node kind (file `644` / dir `755`, `755` also usual for files under a `bin`-like majority-executable level) *and* the level's same-kind majority mode. Special bits (setuid/setgid/sticky) and flags (immutable, append-only, `uchg`) **always** speak — no majority normalizes those. | convention ∧ siblings |
| **owner / group** | Owner is not the caller's euid *and* not the level's majority owner. (Both legs: in a shared tree the majority owner may not be you and is still unsurprising; a root-owned file among your own always speaks — root ≠ you and ≠ majority.) Group mirrors owner with the caller's group list in place of euid. | convention ∧ siblings |
| **size** (outlier) | Log-scale deviation from its cohort's median: `log10(size) − log10(median)` ≥ the sensitivity (default 1.0 — an order of magnitude), *and* size clears an absolute floor (default ~256 KiB) so tiny trees don't cry outlier. Cohort = same-suffix siblings when ≥ 3 exist, else all files at the level; < 3 files total → only the absolute floor can trigger (a lone 400 MB file still speaks). Median, not mean — one whale must not silence itself by dragging the norm. | siblings (+ floor) |
| **filekind** | The kind *word* appears when the node's kind (config suffix-map) differs from its level's file-kind plurality — the binary in a source dir, the huge image among `.md`. Levels with no plurality (≤ 2 files, or all singleton kinds) stay silent — no norm, no surprise. | siblings |
| **mtime** | Recent in absolute perceived terms: age below the recency window (default 1 day, SIGNA-scale rationale — recency is surprising against *now*, not against siblings, since with the recency sort position already carries the sibling comparison). Warm baseline replaces the window with "newer than my last look." | convention (now) |
| **git letter** | Dirty (shipped law). Clean prints nothing. Not sibling-relative — a dirty file in an all-dirty dir still speaks; the convention is cleanliness. | convention |
| **cloud / linkcount / prior name** | Their lattice `unique` notes are already the law (evicted-only; nlink > 1; recently-renamed-only): presence-of-condition facts, no statistics involved. | convention |

**Sensitivity is channel tuning** — from the caller stack only, never the place. Leaning (lattice Open, answered here for ratification): **one dial** (`quiet.sensitivity`, default 1.0, scaling the statistical thresholds — size's log-deviation, mtime's window) **plus per-fact override keys** for the few callers who need one fact retuned. Convention legs (special permission bits, dirty git, root-owner) do not scale — they are laws, not thresholds.

## Protections (why "nuance" is load-bearing)

- **Determinism within a state.** Given the same tree and the same after-image, the look is byte-identical. Quiet must not flicker on re-runs, or two aspecta stop being diffable and the look becomes diff-noise. Corollaries now concrete: norms over the full enumerated level (above); thresholds from the caller stack; **no wall-clock leaks except mtime's own window** — and mtime-quiet does change across runs as time passes, which is truthful, not flicker (the fact *is* about now; the header timestamps the look).
- **Cold state is honest.** No after-image → the cold baseline, never a fabricated "nothing changed."
- **Quiet hides commentary, never existence.** Quiet is for *facts about* listed things. What exists — children, censuses, mass, `denied` — is governed by [[summarization|Summarization]] and is not quietable.
- **Thresholds are channel tuning** — they come from the caller stack, never from the place.
- **A quiet fact that speaks claims a column only on its line** — no reserved gap on silent lines ([[columns|Columns]] law); placement per the lattice/shorthand classes.

## Foundations (clauses)

| Clause | Where |
|---|---|
| Presence/absence is the surprisal channel; nothing-case first | [[../../../principles/src/form-agentic-eyes.md\|form-agentic-eyes]] |
| Which facts are quiet-capable | [[aspect-lattice\|Aspect lattice]] (`quiet` office) |
| Level enumeration is complete under bounds | [[walk-bound\|Walk bound]] |
| Warm substrate and cold-start honesty | [[last-look\|Last look]] · [[cache\|Cache]] |
| Tuning rides the caller stack | [[config\|Config]] · [[../../../principles/src/norm-caller-tunes-the-channel.md\|caller-tunes]] |

## Subfeatures (cold wave)

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Usual is silent | All-`644` files, `755` dirs, owner-you, clean git: no permissions/owner/size/kind glyphs anywhere. | Fixture; snapshot. |
| 2 | Odd mode speaks | One `600` among `644`s shows its mode; the `644`s stay silent. | Fixture. |
| 3 | Majority normalizes | A `bin/` of all-`755` files: silent (sibling-usual), even though `755` files are conventionally odd. | Fixture. |
| 4 | Special bits always | A setuid file speaks even in an all-setuid level. | Fixture. |
| 5 | Root-owned speaks | One root-owned file in the caller's tree shows owner. | Fixture (needs privileged fixture setup or a mocked stat layer). |
| 6 | Size whale | 40 files ~2 KiB and one 80 MiB: the whale's size prints; others silent. Median guards: two whales both still speak. | Fixture. |
| 7 | Tiny level floor | Two files, one 10× the other, both under the floor: silent. | Fixture. |
| 8 | Kind intruder | A `.png`/binary among 20 `.md`: kind word appears on it only. | Fixture. |
| 9 | Budget-independent | Same tree at `--lines 10` vs `--lines 0`: a node listed in both carries identical quiet facts (norms from the full level). | Diff shared lines. |
| 10 | Sensitivity dial | `quiet.sensitivity = 2.0` silences a 10× outlier; a per-fact override re-voices it. | Config fixtures. |
| 11 | Determinism | Same tree, same config, same state: byte-identical (mtime window pinned via a fixed fake clock in tests). | Diff runs. |
| 12 | JSON unaffected | JSON carries the facts regardless of quiet — quiet is a text-rendering law, not a data cut ([[json\|JSON]]; machine formats get facts, not affordances). | Parse fixture output. |

## Open

- **Warm∪cold vs warm-overrides-cold.** The union leaning above (warm only adds voice) keeps every cold surprise alive forever — arguably wrong for a *stable* oddity this caller has now seen five times (habituation is how real perception works). Counter-lean: warm may *demote* sibling-statistical legs it has witnessed before, never convention legs. Genuinely undecided; needs Joseph (it is the personalization philosophy question).
- **Defaults ratification:** the numeric constants (log-deviation 1.0, size floor, recency window 1 day, cohort minimum 3) — reasoned starting points, Joseph corrects on contact.
- Whether **mtime-quiet is redundant** under the recency default sort (position already says "recent"; the column re-says it). Leaning: keep QUIET but let the warm baseline be its real payoff; cold mtime-speak may prove noise and drop to OFF. Lived-use question.
- JSON: should it carry *why* a fact would have spoken (`quiet_reasons`)? Leaning no — that is affordance, not fact. Flag only if an agent consumer asks.
