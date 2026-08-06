---
slug: observable-crossings
type: form
depends:
  - integration-metabolism
  - decision-records
---

# Layer crossings are recorded events

*When material moves between layers — a working note resolved, a spike landed, an intake item integrated or archived, a segment's state flag flipped — the crossing is recorded as an event carrying its adjudication, so the backlog is a countable queue and every disposition can be re-examined.*

## The claim

The crossings between fast and slow layers are where a corpus's honesty is decided: they are the moments something is judged settled enough to promote, or dead enough to archive, or done enough to disappear. Today most estate crossings are **silent acts** — a deletion, a `git mv`, a flag edit — whose adjudication exists only in the actor's head at the moment of acting. Three costs follow, each observed live:

- **The backlog becomes a vibe.** With no crossing events, "how much is left" is answered by narrative instead of by count — the condition under which a half-done integration was declared complete in this very corpus.
- **Dispositions cannot be audited.** A crossing with no recorded criterion cannot be checked against the criterion; the delete-test failure was found only because a steward re-ran the judgment by hand on the whole population.
- **The transport and the evidence vanish together.** A working note that is resolved-and-deleted leaves nothing to show the resolution happened, or on what grounds — the disposition (resolved / deferred / promoted) is already a decided, typed act; nothing records it.

The form of the record is a small event, kin to [[decision-records]] at micro grain: *subject, act, grounds or criterion applied, actor, date* — appended somewhere collision-free (per-atom event files or a single-writer ledger, per [[write-safety]]). What it buys: `pending` becomes a query (undispositioned items are countable, not discoverable-by-accident); "done" claims become checkable against the event trail; and the crossing criterion (the delete-test, a promotion bar) is named *at the crossing*, where bending it would be visible, rather than in the actor's private paraphrase.

The boundary honestly held: recording crossings does not make the judgments good — it makes them *inspectable*. A wrong adjudication with a clean event trail is still wrong; it is merely findable.

## Strength & grounds

**Heuristic; the estate has the halves but not the whole.** The event substrate is shipped and multi-agent-safe in the cousin stores (terminology's per-slug decision directories; relata's per-key verification events — "overall verified" is *derived* from latest events, not hand-set). The crossing-without-record failure is documented at both grains: ASF's Gate-4 note-dispositions leave no trace by design review (2026-07 meta-process findings), and this corpus's 2026-08-05 dispatch failure is the intake-grain specimen ([[integration-metabolism]]). No estate corpus yet records *integration* crossings as events; the transfer of the shipped event shape to that crossing is proposed, not demonstrated.

## Working Notes

- The event schema written out in TODO entry R46 (gate dispositions and tribunal records as one family at three altitudes) is the drafting substrate for the schema detail; deliberately not reproduced here until tried once.
- First honest trial available in-house: record this corpus's own INFLUX dispositions as events (the two-line form above) and see whether `check-plan` can derive its INFLUX/integrated counts from the trail instead of the filesystem.
