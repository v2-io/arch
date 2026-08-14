# Last look

I can see what changed since *my* last aspecta of this locus (origin: *"show diff from last time (via uid or something?)"*). This is the perception-of-movement made explicit: the recency sort already shows movement by position with zero state; last-look adds the caller-keyed **after-image** ([[cache|Cache]]'s second store) so a returning agent perceives *appeared / gone / changed* rather than re-deriving it. [[quiet-columns|Quiet]]'s warm baseline is this same after-image consumed silently; last-look is the same mechanism promoted to a visible fact (the lattice's `last-look delta` row: INFO, QUIET, formats `marker* / delta / signa`).

## Cold start first (the honest floor this row stands on)

No after-image — first ever look, a new caller key, `--no-cache`, a cleared cache, or a store lost — means **no basis for surprise**:

- The look is the ordinary unpersonalized look. No delta marks at all.
- Nothing is fabricated: not "everything is new" (a lie — the place predates the look), not "nothing changed" (a lie — we cannot know). Absence of the fact, never a faked value ([[quiet-columns|Quiet]]: *"cold state is honest"*).
- Losing the store costs memory, not honesty (cache.md's law) — a degraded look is the cold look, byte-identical to a genuine first visit.
- The uncached behavior is the *shipped* behavior until the store lands; everything below layers on without changing it.

## The after-image

Written by the look itself, to the tool's own state dir — never the locus (purity, [[cache|Cache]]). Keyed by **(caller key, locus identity)**:

- **Caller key:** [[../../../principles/src/form-caller-key.md|form-caller-key]] — one caller's after-image never retunes another's channel. Two agents on one machine each have their own "last time."
- **Locus identity — recorded leaning, sub-ratification:** the canonicalized absolute root path. It matches how a caller actually returns to a place (by path); a moved/renamed locus honestly cold-starts. Device/fs ids and repo ids add rename-tracking cleverness at the cost of surprising identity claims — refused for now. (This is the "identity of a look" open the lattice and cache both carry; the leaning resolves the *locus* half. Ratify.)
- **Per-node identity — recorded leaning:** path-within-locus. A rename shows as gone + appeared, which is the truth at the filesystem grain; git rename-follow ([[heat|Heat]]'s obtain) may later refine a rename into `prior name`, which is that row's fact, not a reason to build inode-tracking here.
- **Contents:** for every node the walk actually **statted** last time — not just the printed ones — the identity plus the facts deltas are computed from (mtime, size, git letter, existence; growable per lattice). Plus the look's own UTC stamp and the walk's coverage marks (bounds/denied), because honesty about coverage must persist with the image.
- Every completed look **replaces** the after-image (the newest look is the new baseline). `--no-cache` neither reads nor writes it. Two quick calls: the second honestly reports "nothing moved."

## The delta, rendered

QUIET by nature — unchanged prints nothing; the something-case carries the surprisal:

- **changed** (mtime/size/git letter moved): a delta mark on the line (format `marker*`), or the moved fact's old→new in `delta` form, or the elapsed half in `signa` ([[phenom-format|SIGNA]] — this is its "perception of movement at the right emotional weight" use case).
- **appeared:** a node present now, absent from the after-image *and within what the last look statted* — see the coverage law below.
- **gone:** a node in the after-image with no present counterpart. Existence information, so it cannot be silently quiet — but a full line per ghost fights the budget. Leaning: a typed count on the parent line (`2 gone`, names when budget allows), same census family as everything else. Ratify with the spellings.
- **Census forms** (lattice: `changed-count`): an unexpanded dir carries `N changed beneath` — movement visibility without expansion, the aliveness twin of mass.
- **Header:** the overview line may carry "last look: `2.1d ago`" (SIGNA delta) — the single cheapest orientation fact a returning caller can get, and the boundary-marker use the notation was born for. Leaning ON-when-warm; ratify.

**The coverage law (what makes deltas claims, not guesses):** a delta is asserted only where the after-image can testify. A node outside the last look's walk bound / depth / denied region is not "appeared" — it is merely *unseen before*; either it stays unmarked or it carries a distinct honest form (spelling open). Bounded prior coverage degrades delta claims exactly the way `≥` degrades counts — the after-image's stored coverage marks are what decide this per node.

Determinism: the look is a pure function of (tree, caller state) where the after-image *is* caller state — outline working note, verbatim: *"given the same tree and same after-image state, a look is byte-identical — two aspecta must be diffable."*

## Foundations (clauses)

| Clause | Where |
|---|---|
| Two stores; losing the after-image costs memory, not honesty | [[cache\|Cache]] |
| Warm baseline consumer; cold state is honest | [[quiet-columns\|Quiet columns]] |
| Caller key; only the caller tunes the channel | [[../../../principles/src/form-caller-key.md\|caller-key]] · [[../../../principles/src/norm-caller-tunes-the-channel.md\|caller-tunes]] |
| Absent, never faked; quiet hides commentary, never existence | [[../../../principles/src/form-agentic-eyes.md\|form-agentic-eyes]] |
| SIGNA delta register | [[phenom-format\|Agentic phenom formatting]] |

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | Cold | First look: no delta marks; byte-identical to a `--no-cache` look. | Fresh state dir; diff. |
| 2 | Nothing moved | Look twice, unchanged tree: second look shows no deltas (and is not the cold look — header may say "last look: …"). | Fixture. |
| 3 | Changed | Touch a file between looks: its line carries the mark. | Fixture. |
| 4 | Appeared / gone | Add one file, delete another: appeared marked; gone reported in its parent's typed form. | Fixture. |
| 5 | Coverage honesty | Last look bounded so `deep/x` was never statted; unbounded now: `deep/x` is not marked appeared. | Bounded-then-unbounded fixture. |
| 6 | Caller isolation | Caller A looks, tree changes, caller B's first look is cold — B sees no deltas; A's next look sees them. | Two `--caller` keys. |
| 7 | Replace semantics | Three looks with a change before the third: third reports only the second→third delta. | Fixture. |
| 8 | `--no-cache` | Neither reads nor writes: shows cold, and a subsequent normal look deltas against the pre-`--no-cache` image. | Fixture. |
| 9 | Determinism | Same tree, same after-image: byte-identical. | Copy state dir; diff runs. |
| 10 | JSON | Deltas are fields (`changed`, `appeared`, `gone` counts/booleans), never glyphs ([[json\|JSON]]). | Parse fixture output. |

## Open

- **Locus identity** — the canonical-absolute-path leaning above. Joseph ratifies (it is the recorded open from the founding session's "via uid or something?").
- **Mark/spelling family:** the delta glyph(s), the `gone` parent form, the unseen-before form, the header line. Interface vocabulary — ratify before it ossifies.
- **After-image size discipline:** per-node facts for big loci is a real store (~100k nodes × a small record). Probably fine as a compact binary per (caller, locus); pruning policy (drop images unvisited for N days?) undecided.
- Whether a look can **peek without updating** the baseline (an agent mid-investigation may want "vs my session start," not "vs 30 seconds ago"). Leaning: not until asked-for by lived use; `--no-cache` approximates it poorly. Recorded, not designed.

## Not in this row

Cache freshness of *derived facts* ([[cache|Cache]] — recomputables are the other store). When quiet speaks off this baseline ([[quiet-columns|Quiet columns]]). `prior name` rename facts ([[heat|Heat]] family). Cross-caller sharing (refused by caller-key law, not deferred).
