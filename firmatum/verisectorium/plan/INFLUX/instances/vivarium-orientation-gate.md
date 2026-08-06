<!--
  Verisectorium notes gather — summary + extracts, not live authority.
  Sources: arch/vivarium/ORIENT.md (read whole 2026-08-05), arch/vivarium/CLAUDE.md
  (orientation-gate + telos sections), bin/try-me / bin/prove-me / bin/orient-rank
  (not read internally — behavior taken from the operator manual).
  Added 2026-08-05 at Joseph's request: the notes tree had no coverage of this
  mechanism. Claim home in vivarium: #ops-orientation-gate.
-->

# vivarium's orientation gate — enforced orientation before commit

## The problem it solves (why it belongs in verisectorium)

In vivarium the segments are **upstream from everything else** — claim segments in `core/src/` are the sole source of truth, and code is explicitly ranked as compliance debt / instrument, not adjudicator. But agents habitually treat segments like "docs" (downstream artifact you skim after coding) and jump straight to code. The gate makes orientation to the claim surface a **verifiable precondition of write access**, instead of a norm agents feel they've satisfied. CLAUDE.md names the failure mode directly: compaction (and ordinary overconfidence) leaves a *feeling* of orientation that is not the substance.

This is the P3-channel move applied to *reader state* rather than document state: orientation used to be unobservable (an honor-system claim); the quiz manufactures a cheap, low-noise check for it.

## The mechanism (from ORIENT.md)

1. **Free-read phase:** `core/OUTLINE.md` always; the **★ starred rows** (current high-importance set, refreshed by `bin/orient-rank --mark-outline`; most quiz mass drawn from these), read *in outline order*; plus whatever you're about to touch.
2. **`bin/try-me`** — seals `core/src` (chmod) and issues a quiz: N items, each a *unit* from a claim segment (an FE clause, a WN bullet, a paragraph), given as slug + section + `:start`/`:end` word anchors. Quiz and answers are **vanilla udon** — deliberately doubling as a first real-world udon exercise.
3. **`bin/prove-me --ready <answers>`** — grades (match ignores case/punctuation/whitespace) and prints an `orient:<session_id>` token that goes in the commit message.
4. **Graded outcomes:** 5/5 = unlimited commits while the session's compaction generation matches; 3–4/5 = **one-time** commit + a promise to free-read the missed sections; <3 = re-orient and retry. **Compaction invalidates the pass** — re-prove after compact.
5. **Soft rejects don't burn the attempt** (format errors, <3 substantial answers); a real graded attempt below bar does. Retrying after more reading is the intended path, no stigma. Bypassing needs Joseph's approval or a reason you can own with him.
6. **Post-pass norm:** the first legitimate uses of commit access are repairs the free-read/quiz made obvious — segment fixes, gate-tool fixes, or Working Notes on `#ops-orientation-gate` (the durable backlog).

## Portable design points for verisectorium

- **Orientation is gated on the *outline + starred segments*, not on docs** — the outline is doing double duty as exposition view *and* onboarding curriculum. Star-marking is a per-view importance annotation (edge attribute, in the logical-model reading).
- **The pass is scoped to a context lifetime** (session + compaction generation) — the gate models agent turnover explicitly, which is exactly the "100% turnover" problem verisectorium exists to solve. No other instance ties permissions to reader-context freshness.
- **The quiz units are addressed by slug + section + word anchors** — the same addressing that provenance spans use (cf. comproprium `bin/check-corpus`), applied prospectively (prove you read it) instead of retrospectively (prove the quote exists).
- **Incentive alignment:** misses convert into targeted reading; friction converts into a standing repair backlog on a named claim segment. The gate feeds the corpus rather than just guarding it.
- **Costs/known limits:** one sealed test per checkout (parallel agents on one tree clobber each other); seal is chmod-based (emergency `bin/orient-unseal`); quiz answerable-from-memory pressure shapes what kinds of units are quotable.

## Open question for the generalized toolset

Whether the gate generalizes per-deployment like everything else (stakes × reversibility): vivarium gates *commits to the whole repo* on orientation to `core/`; ASF has no equivalent (its FORMAT gates are per-segment promotion, not per-agent access). A standardized verisectorium kit plausibly offers the orientation gate as an optional profile item, with the quiz generator driven by the outline + FORMAT the deployment already has.
