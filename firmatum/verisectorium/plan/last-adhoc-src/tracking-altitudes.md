---
slug: tracking-altitudes
type: form
depends:
  - tracking-layer-census
---

# Tracking layers are distinguished by the question they answer

*A corpus's non-claim surfaces stratify by altitude — which areas matter, what is open inside one, what moves cut across all of them, what is reserved for a person — and a layer earns its existence by answering a question none of the others does, not by being bigger, smaller, or more detailed.*

## The claim

Every living corpus grows surfaces that are not claims: navigators, backlogs, proposal portfolios, decision queues, audit status. Left alone they multiply and overlap, and the overlap is what makes them stop being read. The organizing principle that survives contact is **altitude**, and the operative test is a question, not a size:

- **Navigator** — *which areas of work are live, and in what priority?* One screen. It is the entry point and it names areas, never items.
- **Items** — *what is open inside a named area?* This is where actionable work lives, and it is the layer that grows fastest.
- **Portfolio** — *what structural moves cut across areas?* Separate because a structural move is evaluated on its own merits and against the others, not slotted into whichever area it happens to touch first.
- **Steward queue** — *what genuinely requires a particular person?* Pointers only; the work item stays in its home tracker.
- **History**, split in two — *what happened, forward-going* and *what happened before the discipline changed*, the second frozen with a reading rule attached.

**The test that keeps this from being decoration: if two surfaces answer the same question, they are one surface partitioned, not two altitudes.** ASF's four subject-partitioned TODO files all answer *what is open here* — they are one layer split for write-concurrency and browsing, and treating them as separate altitudes would be a category error. Its `PROPOSALS.md` answers a different question and is therefore genuinely a different layer, even though its items look superficially like large TODO items.

**Each altitude needs a consumer, and the failure mode of a missing altitude is that its load lands on the nearest available surface — where nothing watches it.** This is the sharpest observed failure in the estate and it is worth stating as the general shape: when there is no surface at the right altitude, material does not stop arriving; it silts into whatever store is closest to hand. ASF's own lifecycle review found load-bearing theory-correctness findings — a possibly-vacuous bound, a schema convention — parked in per-segment Working Notes awaiting steward routing, with no queue semantics and no owner: *"load-bearing theory-correctness items are parked where nothing watches them."* The notes store was not misused out of carelessness; it was the only per-atom place to put something, and the item's real altitude (a decision awaiting a person) had no per-atom expression. Read the same way, an overgrown sidecar is often a symptom of a missing tracker rather than of sloppy note discipline ([[working-notes-sidecar]]).

**The steward queue is the one layer whose value is inversely proportional to its size,** and pointers-only is what keeps it that way. It is measured at 27 lines against a 537-line item layer in the census. Two failures it prevents at once: it stops the steward becoming a routing bottleneck for work that others can carry, and it stops the queue forking its home trackers — a copy of an item is a second thing to keep in step, and it will not be kept.

**Audience is a second cut, independent of altitude, and it is easy to conflate.** The same corpus marks six surfaces auditor-hidden, three auditor-safe, four priming-heavy — and in this instance the safe set happens to be the navigator plus generated public surfaces, which makes altitude *look* like it predicts safety. It does not: one surface is hidden because it primes, another because it names live decisions. Two different properties with two different reasons, and a corpus that infers one from the other will expose the wrong file the first time a high-altitude surface turns priming-heavy ([[priming-discipline]]).

**A last, smaller move worth naming because it recurs: a steward's own artifact is kept verbatim, with the reconciliation placed beside it rather than edited in.** One live instance keeps Joseph's checkbox list reproduced as typed — *"Here it is, as you typed it"* — followed by a dated agent paragraph reconciling each block against the live tree and naming which boxes are known-stale because written from a superseded understanding. Editing the primary would have destroyed the only record of what the steward actually asked for; the adjacency keeps both facts.

## Strength & grounds

**A formulation generalized from one worked instance, with one measured specimen and one recorded failure.** The ladder, the audience markings, and the sizes are measured first-hand on a single mature corpus ([[tracking-layer-census]]); the unowned-queue failure is that corpus's own review finding, quoted; the verbatim-plus-reconciliation move is a single instance in a different tree. So the *inventory* of altitudes is one design that works, not four that converged — a second corpus organizing its trackers differently would not refute anything here, and none has been compared.

What is argued rather than observed: the same-question test, the consumer requirement, and the independence of audience from altitude. The first two are analytic. The third is a claim about this corpus's contents that generalizes only as a warning — it says the coincidence is a coincidence, which is checkable in any instance and has not been checked in a second one.

The honest caution on the measurements: line counts are a size proxy that a table-heavy file would distort, and the *declared* role of each surface is quoted from the corpus's orientation file rather than verified against its contents. Whether each file does what it says is unmeasured, and it is the check that would actually test the ladder.

## Working Notes

- Owed and cheap: verify declared role against contents on the same seven surfaces. If the item layer contains area-level framing or the navigator contains items, the ladder is aspirational and the census is measuring filenames.
- Not carried here: how a tracker's completed items retire. The estate's practice is to prune at the moment the tracker is touched and migrate the narrative to history in the same commit — that is a history-layer discipline and lives with [[history-layer]].
- Open: whether the portfolio layer survives when a corpus is small. It exists here because the item layer became unreadable; a young instance that ships it from day one may be paying ceremony for a problem it does not have. This is exactly the kind of question a profile should answer rather than a convention ([[instance-profiles]]).
- Adjacent: [[decision-records]] (what a steward-queue pointer should point *at*), [[asked-and-answered]] (why an unresolved item must carry its disposition or it re-bills every fresh agent), [[steward-surfaces]] (ch. 15, which owns the steward's side of this).
